use crate::{
    database::{
        establish_connection,
        models::{article, comment, session},
    },
    router::TEMPLATES,
};
use hyper::{header, Body, Request, Response, StatusCode};
use tera::Context;

pub async fn handle(req: Request<Body>, path: &str) -> Option<Response<Body>> {
    match session::get_from_request(&establish_connection(), &req) {
        Some(s) if s.check_expiration() => {
            let tera = TEMPLATES.get().unwrap();
            log::debug!("Request admin page success: {:?}", s);
            let body = match path {
                "" | "index.html" | "index" => {
                    tera.render("admin/index.html", &Context::new()).unwrap()
                }
                "posts" | "posts.html" => {
                    let mut ctx = Context::new();
                    let articles = article::read_all(&establish_connection()).unwrap();
                    ctx.insert("is_posts", &true);
                    ctx.insert("contents", &articles);
                    tera.render("admin/list.html", &ctx).unwrap()
                }
                "comments" | "comments.html" => {
                    let mut ctx = Context::new();
                    let comments = comment::read_all(&establish_connection()).unwrap();
                    ctx.insert("is_comments", &true);
                    ctx.insert("contents", &comments);
                    tera.render("admin/list.html", &ctx).unwrap()
                }

                _ => return None,
            };
            Some(Response::new(Body::from(body)))
        }
        _ => {
            log::debug!("Request admin page failed, Redirect to /login");
            let mut res = Response::new(Body::empty());
            *res.status_mut() = StatusCode::FOUND;
            res.headers_mut().insert(
                header::LOCATION,
                header::HeaderValue::from_str("/login").unwrap(),
            );
            Some(res)
        }
    }
}
