use crate::{error, handler, model, Db};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Validate)]
pub struct CreateRequest {
    #[validate(url)]
    url: String,
}

#[derive(Serialize)]
pub struct SourceReply {
    id: i64,
    title: String,
    canonical_url: String,
    opinions: Vec<handler::opinion::OpinionReply>,
    votes: (i64, i64),
}

impl SourceReply {
    async fn try_from_source(db: &Db, source: model::source::Source) -> error::Result<Self> {
        let opinions = model::opinion::find_by_source(&db, &source.id)
            .await?
            .into_iter()
            .map(|opinion| handler::opinion::OpinionReply::from(opinion))
            .collect();

        let votes = model::vote::count_by_position(&db, &source.id).await?;

        Ok(Self {
            id: source.id,
            title: source.title,
            canonical_url: source.canonical_url,
            opinions,
            votes,
        })
    }
}

pub async fn find(db: Db, url: warp::path::Tail) -> Result<impl warp::Reply, warp::Rejection> {
    let source = model::source::find_by_url(&db, &url.as_str()).await?;
    let reply = SourceReply::try_from_source(&db, source).await?;

    Ok(warp::reply::json(&reply))
}

pub async fn create(db: Db, input: CreateRequest) -> Result<impl warp::Reply, warp::Rejection> {
    validate(&input)?;

    let input = model::source::CreateSource { url: input.url };
    let source = model::source::create(&db, input).await?;
    let reply = SourceReply::try_from_source(&db, source).await?;

    Ok(warp::reply::with_status(
        warp::reply::json(&reply),
        warp::http::status::StatusCode::CREATED,
    ))
}

fn validate(input: &impl Validate) -> Result<(), error::Error> {
    input.validate().map_err(|_| error::Error::Invalid)
}
