#[macro_use]
extern crate lazy_static;

use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use argh::FromArgs;
use rand::prelude::*;

const IMAGE_TYPES: [(&str, &'static str); 1] = [("gif", "Image/Gif")];

#[derive(FromArgs)]
/// Headpats
struct HeadpatArgs {
    #[argh(positional, default="\"./images/\".to_string()")]
    image_dir: String,
}

lazy_static! {
    static ref ARGS: HeadpatArgs = argh::from_env();
    static ref GIFS: Vec<(String, String)> = std::fs::read_dir(&ARGS.image_dir).expect("Unable to find Gif Dir").filter_map(|x| {
        let item = x.unwrap().path().as_path().to_str().unwrap().to_string();
        let mut content_type = "";
        let valid = {
            let mut valid = false;
            for ext in &IMAGE_TYPES {
                if item.ends_with(ext.0) {
                    content_type = ext.1;
                    valid = true;
                    break;
                }
            }
            valid
        };
        if valid {
            Some((item, content_type.to_string()))
        } else {
            None
        }
    }).collect::<Vec<(String, String)>>();
    static ref GIFS_LEN: usize = GIFS.len();
}

#[get("/")]
async fn pat() -> impl Responder {
    if &GIFS_LEN as &usize != &0 {
        let mut rng = rand::thread_rng();
        let rand_num: usize = rng.gen_range(0..GIFS.len());
        let val = &GIFS[rand_num];
        if let Ok(data) = std::fs::read(&val.0) {
            return HttpResponse::Ok().content_type(&val.1).body(data);
        }
    }
    HttpResponse::Ok().body("UwU")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    lazy_static::initialize(&GIFS);

    println!("Listening!");
    HttpServer::new(|| {
        App::new().service(pat)
    }).bind("0.0.0.0:80")?.run().await
}