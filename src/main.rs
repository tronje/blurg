#![feature(plugin)]
#![plugin(rocket_codegen)]


extern crate time;
extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;


mod post;


use std::io;
use std::path::{Path, PathBuf};
use rocket::response::NamedFile;
use rocket_contrib::Json;


use post::{Post, PostMeta};


#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("www/index.html")
}


#[get("/<file..>", rank = 5)]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("www/").join(file)).ok()
}


#[get("/api/posts")]
fn list_posts() -> Json<Vec<usize>> {
    Json(Post::list_posts())
}


#[get("/api/post/meta/<post_id>")]
fn get_post_meta(post_id: usize) -> Option<Json<PostMeta>> {
    match PostMeta::from_id(post_id) {
        Some(meta) => Some(Json(meta)),
        None => None,
    }
}


#[get("/api/post/<id>")]
fn get_post(id: usize) -> Option<Json<Post>> {
    match Post::from_id(id) {
        Some(post) => Some(Json(post)),
        None => None,
    }
}


fn main() {
    let routes = routes![index, files, list_posts, get_post_meta, get_post];
    rocket::ignite().mount("/", routes).launch();
}
