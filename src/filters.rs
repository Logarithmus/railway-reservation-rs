use warp::{filters::BoxedFilter, path, Filter, Reply};

fn templates() -> BoxedFilter<(impl Reply,)> {
    warp::fs::dir("templates").boxed()
}

pub fn all_filters() -> BoxedFilter<(impl Reply,)> {
    templates()
}
