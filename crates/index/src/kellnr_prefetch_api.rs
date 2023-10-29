use std::sync::Arc;
use super::config_json::ConfigJson;
use appstate::{DbState, SettingsState};
use auth::auth_req_token::AuthReqToken;
use axum::{extract::{Path, State}, http::{StatusCode, HeaderMap}, Json};
use common::{normalized_name::NormalizedName, original_name::OriginalName, prefetch::Prefetch};
use db::DbProvider;

pub async fn config_kellnr(
    State(settings): SettingsState,
    auth_req_token: AuthReqToken,
) -> Json<ConfigJson> {
    _ = auth_req_token;
    Json(ConfigJson::from((&(*settings), "crates")))
}

pub async fn prefetch_kellnr(
    Path((_a, _b, package)): Path<(String, String, OriginalName)>,
    headers: HeaderMap,
    State(db): DbState,
    auth_req_token: AuthReqToken,
) -> Result<Prefetch, StatusCode> {
    _ = auth_req_token;
    let index_name = NormalizedName::from(package);
    internal_kellnr_prefetch(&index_name, &headers, &db).await
}

pub async fn prefetch_len2_kellnr(
    Path((_a, package)): Path<(String, OriginalName)>,
    headers: HeaderMap,
    auth_req_token: AuthReqToken,
    State(db): DbState,
) -> Result<Prefetch, StatusCode> {
    _ = auth_req_token;
    let index_name = NormalizedName::from(package);
    internal_kellnr_prefetch(&index_name, &headers, &db).await
}

async fn internal_kellnr_prefetch(
    name: &NormalizedName,
    headers: &HeaderMap,
    db: &Arc<dyn DbProvider>,
) -> Result<Prefetch, StatusCode> {
    match db.get_prefetch_data(name).await {
        Ok(prefetch) if needs_update(headers, &prefetch) => Ok(prefetch),
        Ok(_prefetch) => Err(StatusCode::NOT_MODIFIED),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}

fn needs_update(headers: &HeaderMap, prefetch: &Prefetch) -> bool {
    let if_none_match = headers.get("if-none-match");
    let if_modified_since = headers.get("if-modified-since");

    match (if_none_match, if_modified_since) {
        (Some(etag), Some(date)) => *etag != prefetch.etag || *date != prefetch.last_modified,
        (_, _) => true,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config_json::ConfigJson;
    use appstate::AppStateData;
    use axum::{Router, routing::get, http::Request, body::Body};
    use db::error::DbError;
    use db::mock::MockDb;
    use mockall::predicate::*;
    use reqwest::header;
    use settings::{Protocol, Settings};
    use tower::ServiceExt;

    #[tokio::test]
    async fn config_returns_config_json() {
        let r = app().await
            .oneshot(Request::get("/api/v1/index/config.json").body(Body::empty()).unwrap())
            .await
            .unwrap();

        let result_msg = hyper::body::to_bytes(r.into_body()).await.unwrap(); 
        let actual = serde_json::from_slice::<ConfigJson>(&result_msg).unwrap();

        assert_eq!(
            ConfigJson::new(&Protocol::Http, "test.api.com", 1234, "crates", false),
            actual
        );
    }

    #[tokio::test]
    async fn prefetch_returns_prefetch_data() {
        let r = app().await
            .oneshot(Request::get("/api/v1/index/me/ta/metadata")
                .header(header::IF_MODIFIED_SINCE, "foo")
                .header(header::ETAG, "bar")
                .body(Body::empty()).unwrap())
            .await
            .unwrap();

        let result_status = r.status();

        assert_eq!(StatusCode::OK, result_status);
        assert_eq!("3", r.headers().get(header::CONTENT_LENGTH).unwrap());
        assert_eq!("date", r.headers().get(header::LAST_MODIFIED).unwrap());
        assert_eq!(vec![0x1, 0x2, 0x3], hyper::body::to_bytes(r.into_body()).await.unwrap());
    }

    #[tokio::test]
    async fn prefetch_returns_not_modified() {
        let r = app().await
            .oneshot(Request::get("/api/v1/index/me/ta/metadata")
                .header(header::IF_MODIFIED_SINCE, "date")
                .header(header::ETAG, "etag")
                .body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(StatusCode::NOT_MODIFIED, r.status());
    }

    #[tokio::test]
    async fn prefetch_returns_not_found() {
        let r = app().await
            .oneshot(Request::get("/api/v1/index/no/tf/notfound")
                .header(header::IF_MODIFIED_SINCE, "date")
                .header(header::ETAG, "etag")
                .body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(StatusCode::NOT_FOUND, r.status());
    }

    async fn app() -> Router {
        let settings = Settings {
            api_address: String::from("test.api.com"),
            api_port: 8000,
            api_port_proxy: 1234,
            ..Settings::new().unwrap()
        };

        let mut mock_db = MockDb::new();
        mock_db
            .expect_get_prefetch_data()
            .with(eq("metadata"))
            .returning(move |_| {
                Ok(Prefetch {
                    data: vec![0x1, 0x2, 0x3],
                    etag: String::from("etag"),
                    last_modified: String::from("date"),
                })
            });
        mock_db
            .expect_get_prefetch_data()
            .with(eq("notfound"))
            .returning(move |_| Err(DbError::CrateNotFound("notfound".to_string())));

        let kellnr_prefetch = Router::new()
            .route("/config.json", get(config_kellnr))
            .route("/:_/:_/:name", get(prefetch_kellnr))
            .route("/:_/:name", get(prefetch_len2_kellnr));

        let state = AppStateData {
            db: Arc::new(mock_db),
            settings: Arc::new(settings),
            ..appstate::test_state().await
        };

        Router::new()
            .nest("/api/v1/index", kellnr_prefetch)
            .with_state(state)
    }
}
