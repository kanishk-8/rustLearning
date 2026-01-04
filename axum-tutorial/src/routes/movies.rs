use axum::{Json, Router, extract::Path, routing::get};
use serde::Serialize;

#[derive(Serialize)]
struct Movie {
    id: u32,
    title: &'static str,
    year: u16,
}

pub fn routes() -> Router {
    Router::new()
        // GET /movies
        .route("/", get(get_movies))
        // GET /movies/movie-info
        .route("/{id}", get(get_movie_info))
}

async fn get_movies() -> Json<Vec<Movie>> {
    Json(vec![Movie {
        id: 1,
        title: "Interstellar",
        year: 2014,
    }])
}

async fn get_movie_info(Path(id): Path<u32>) -> Json<Movie> {
    Json(Movie {
        id: 1,
        title: "Interstellar",
        year: 2014,
    })
}
