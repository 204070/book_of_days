use warp::filters::BoxedFilter;
use warp::{Filter, Reply};

use crate::iam::iam_routes::iam_router;

pub fn v1_router() -> BoxedFilter<(impl Reply,)> {
    warp::path("v1").and(iam_router()).boxed()
}
