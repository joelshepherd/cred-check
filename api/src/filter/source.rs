use crate::{handler, Context};
use warp::Filter;

pub fn find(
    context: impl Filter<Extract = (Context,), Error = std::convert::Infallible> + Clone,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    context
        .and(warp::path("source"))
        .and(warp::path::tail())
        .and_then(handler::source::find)
}

pub fn create(
    context: impl Filter<Extract = (Context,), Error = std::convert::Infallible> + Clone,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    context
        .and(warp::path("source"))
        .and(warp::post())
        .and(warp::body::json())
        .and_then(handler::source::create)
}

#[cfg(test)]
mod tests {
    use crate::database;
    use warp::test::request;
    use warp::{http::StatusCode, Filter};

    async fn setup() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> {
        let db = database::connect().await;

        // Insert test data
        sqlx::query!(
            "insert into source (title, url) \
            values ('Test', 'https://example.com/existing') \
            on conflict do nothing"
        )
        .execute(&db)
        .await
        .unwrap();

        super::super::filters(db)
    }

    #[tokio::test]
    async fn test_find() {
        let filters = setup().await;

        let res = request()
            .path("/source/https://example.com/existing")
            .reply(&filters)
            .await;

        assert_eq!(res.status(), StatusCode::OK);
        assert!(std::str::from_utf8(res.body())
            .unwrap()
            .contains("https://example.com/existing"));
    }

    #[tokio::test]
    async fn test_create() {
        let filters = setup().await;

        let res = request()
            .method("POST")
            .path("/source")
            .body(r#"{ "url": "https://example.com/" }"#)
            .reply(&filters)
            .await;

        assert_eq!(res.status(), StatusCode::CREATED);
    }
}
