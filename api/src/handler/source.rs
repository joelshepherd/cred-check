use crate::{handler, model, Db};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, PartialEq, Eq)]
pub enum ExpandOptions {
    Opinions,
    Votes,
}

#[derive(Deserialize)]
pub struct FindOptions {
    pub expand: Option<Vec<ExpandOptions>>,
}

#[derive(Deserialize)]
pub struct CreateRequest {
    url: String,
}

#[derive(Serialize)]
pub struct SourceReply {
    id: i64,
    title: String,
    canonical_url: String,
    opinions: Option<Vec<handler::opinion::OpinionReply>>,
    votes: Option<(i64, i64)>,
}

impl From<model::source::Source> for SourceReply {
    fn from(source: model::source::Source) -> Self {
        Self {
            id: source.id,
            title: source.title,
            canonical_url: source.canonical_url,
            opinions: None,
            votes: None,
        }
    }
}

pub async fn find(
    db: Db,
    url: warp::path::Tail,
    opts: FindOptions,
) -> Result<impl warp::Reply, warp::Rejection> {
    let url = url.as_str().to_owned();

    let source = model::source::find_by_url(&db, url).await?;
    let mut reply = SourceReply::from(source);

    if let Some(expand) = opts.expand {
        if expand.contains(&ExpandOptions::Opinions) {
            reply.opinions = Some(
                model::opinion::find_by_source(&db, &reply.id)
                    .await?
                    .into_iter()
                    .map(|opinion| handler::opinion::OpinionReply::from(opinion))
                    .collect(),
            );
        }
        if expand.contains(&ExpandOptions::Votes) {
            reply.votes = Some(model::vote::count_by_position(&db, &reply.id).await?)
        }
    }

    Ok(warp::reply::json(&reply))
}

pub async fn create(db: Db, input: CreateRequest) -> Result<impl warp::Reply, warp::Rejection> {
    let input = model::source::CreateSource { url: input.url };
    let source = model::source::create(&db, input).await?;
    let reply = SourceReply::from(source);

    Ok(warp::reply::with_status(
        warp::reply::json(&reply),
        warp::http::status::StatusCode::CREATED,
    ))
}
