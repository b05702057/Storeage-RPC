use std::str::FromStr;

use actix_files::Files;
use actix_web::{web, App, HttpServer};
use clap::Parser;
use lab::lab2;
use log::{info, warn, LevelFilter};
use tribbler::config::Config;
use tribbler::config::DEFAULT_CONFIG_LOCATION;
use tribbler::err::{TribResult, TribblerError};
use tribbler::ref_impl::RefServer;
use tribbler::trib::Server;

type Srv = Box<dyn Server + Send + Sync>;

#[derive(Debug, Clone)]
enum ServerType {
    Ref,
    Lab,
}

impl FromStr for ServerType {
    type Err = TribblerError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "ref" => Ok(ServerType::Ref),
            "lab" => Ok(ServerType::Lab),
            _ => Err(TribblerError::Unknown(format!(
                "{} not a valid ServerType",
                s
            ))),
        }
    }
}

/// A program which runs the tribbler front-end service.
#[derive(Parser, Debug)]
#[clap(name = "trib-front")]
struct Cfg {
    /// level to use when logging
    #[clap(short, long, default_value = "INFO")]
    log_level: LevelFilter,

    /// server type to run the front-end against
    #[clap(short, long, default_value = "ref")]
    server_type: ServerType,

    #[clap(short, long, default_value = DEFAULT_CONFIG_LOCATION)]
    config: String,

    /// the host address to bind to. e.g. 127.0.0.1 or 0.0.0.0
    #[clap(long, default_value = "0.0.0.0")]
    host: String,

    /// the host port to bind
    #[clap(long, default_value = "8080")]
    port: u16,
}

#[tokio::main]
async fn main() -> TribResult<()> {
    let args = Cfg::parse();

    env_logger::builder()
        .default_format()
        .filter_level(args.log_level)
        .init();
    let srv_impl: Srv = match args.server_type {
        ServerType::Ref => Box::new(RefServer::new()),
        ServerType::Lab => {
            let cfg = Config::read(Some(&args.config))?;
            let bc = lab2::new_bin_client(cfg.backs).await?;
            lab2::new_front(bc).await?
        }
    };
    let server: web::Data<Srv> = web::Data::new(srv_impl);
    match populate(&server) {
        Ok(_) => info!("Pre-populated test-server successfully"),
        Err(e) => warn!("Failed to pre-populate test server: {}", e),
    }
    let srv = HttpServer::new(move || {
        App::new()
            .app_data(server.clone())
            .service(
                web::scope("/api")
                    .service(api::add_user)
                    .service(api::list_users)
                    .service(api::list_tribs)
                    .service(api::list_home)
                    .service(api::is_following)
                    .service(api::follow)
                    .service(api::unfollow)
                    .service(api::following)
                    .service(api::post),
            )
            .service(Files::new("/", "./www").index_file("index.html"))
    })
    .bind((args.host.as_str(), args.port))?
    .run();
    info!("============================================");
    info!(
        "TRIBBLER SERVING AT ::: http://{}:{}",
        &args.host, &args.port
    );
    info!("============================================");
    srv.await?;
    Ok(())
}

fn populate(server: &web::Data<Box<dyn Server + Send + Sync>>) -> TribResult<()> {
    server.sign_up("h8liu")?;
    server.sign_up("fenglu")?;
    server.sign_up("rkapoor")?;
    server.post("h8liu", "Hello, world.", 0)?;
    server.post("h8liu", "Just tribble it.", 0)?;
    server.post("fenglu", "Double tribble.", 0)?;
    server.post("rkapoor", "Triple tribble.", 0)?;
    server.follow("fenglu", "h8liu")?;
    server.follow("fenglu", "rkapoor")?;
    server.follow("rkapoor", "h8liu")?;
    Ok(())
}

/// this module contains the REST API functions used by the front-end
mod api {
    use std::error::Error;
    use std::{collections::HashMap, sync::Arc};

    use actix_web::{get, http::header::ContentType, post, web, HttpResponse, Responder};
    use log::debug;

    use crate::Srv;

    fn build_resp<T: Serialize>(d: &T) -> HttpResponse {
        HttpResponse::Ok()
            .content_type(ContentType::plaintext())
            .body(serde_json::to_string(d).unwrap())
    }

    fn err_response(err: Box<dyn Error>) -> HttpResponse {
        HttpResponse::InternalServerError().body(err.to_string())
    }

    /// signs up a new user
    #[post("/add-user")]
    pub async fn add_user(
        data: web::Data<Srv>,
        form: web::Form<HashMap<String, String>>,
    ) -> impl Responder {
        let s = form.0;
        debug!("add-user: {:?}", &s);
        match data.sign_up(s.keys().next().unwrap()) {
            Ok(_) => build_resp(&UserList {
                users: data.list_users().unwrap(),
                err: "".to_string(),
            }),
            Err(e) => err_response(e),
        }
    }

    /// lists all the users registered
    #[get("list-users")]
    pub async fn list_users(data: web::Data<Srv>) -> impl Responder {
        match data.list_users() {
            Ok(v) => {
                let ul = UserList {
                    users: v,
                    err: "".to_string(),
                };
                build_resp(&ul)
            }
            Err(e) => err_response(e),
        }
    }

    /// lists all the tribs for a particular user
    #[post("list-tribs")]
    pub async fn list_tribs(
        data: web::Data<Srv>,
        form: web::Form<HashMap<String, String>>,
    ) -> impl Responder {
        let s = form.0;
        match data.tribs(s.keys().next().unwrap()) {
            Ok(v) => {
                let ul = TribList {
                    tribs: v,
                    err: "".to_string(),
                };
                build_resp(&ul)
            }
            Err(e) => err_response(e),
        }
    }

    /// lists the home page for a particular user
    #[post("list-home")]
    pub async fn list_home(
        data: web::Data<Srv>,
        form: web::Form<HashMap<String, String>>,
    ) -> impl Responder {
        let s = form.0;
        match data.home(s.keys().next().unwrap()) {
            Ok(v) => {
                let ul = TribList {
                    tribs: v,
                    err: "".to_string(),
                };
                build_resp(&ul)
            }
            Err(e) => err_response(e),
        }
    }

    /// determines whether a user is following another user or not
    #[post("is-following")]
    pub async fn is_following(
        data: web::Data<Srv>,
        form: web::Form<HashMap<String, String>>,
    ) -> impl Responder {
        let s = form.0;
        let raw = s.keys().next().unwrap();
        let t = serde_json::from_str::<WhoWhom>(raw).unwrap();
        match data.is_following(&t.who, &t.whom) {
            Ok(v) => {
                let ul = Bool {
                    v,
                    err: "".to_string(),
                };
                build_resp(&ul)
            }
            Err(e) => err_response(e),
        }
    }

    /// makes a user follow another user
    #[post("follow")]
    pub async fn follow(
        data: web::Data<Srv>,
        form: web::Form<HashMap<String, String>>,
    ) -> impl Responder {
        let s = form.0;
        let raw = s.keys().next().unwrap();
        let t = serde_json::from_str::<WhoWhom>(raw).unwrap();
        match data.follow(&t.who, &t.whom) {
            Ok(_) => {
                let ul = Bool {
                    v: true,
                    err: "".to_string(),
                };
                build_resp(&ul)
            }
            Err(e) => err_response(e),
        }
    }

    /// makes a user unfollow another user
    #[post("unfollow")]
    pub async fn unfollow(
        data: web::Data<Srv>,
        form: web::Form<HashMap<String, String>>,
    ) -> impl Responder {
        let s = form.0;
        let raw = s.keys().next().unwrap();
        let t = serde_json::from_str::<WhoWhom>(raw).unwrap();
        match data.unfollow(&t.who, &t.whom) {
            Ok(_) => {
                let ul = Bool {
                    v: true,
                    err: "".to_string(),
                };
                build_resp(&ul)
            }
            Err(e) => err_response(e),
        }
    }

    /// gets the list of users following a particular user
    #[post("following")]
    pub async fn following(
        data: web::Data<Srv>,
        form: web::Form<HashMap<String, String>>,
    ) -> impl Responder {
        let s = form.0;
        match data.following(s.keys().next().unwrap()) {
            Ok(v) => {
                let ul = UserList {
                    users: v,
                    err: "".to_string(),
                };
                build_resp(&ul)
            }
            Err(e) => err_response(e),
        }
    }

    /// adds a post for a particular user
    #[post("post")]
    pub async fn post(
        data: web::Data<Srv>,
        form: web::Form<HashMap<String, String>>,
    ) -> impl Responder {
        let s = form.0;
        let raw = s.keys().next().unwrap();
        match serde_json::from_str::<Post>(raw) {
            Ok(p) => {
                let x = match data.post(&p.who, &p.message, p.clock) {
                    Ok(_) => Bool {
                        v: true,
                        err: "".to_string(),
                    },
                    Err(e) => Bool {
                        v: false,
                        err: e.to_string(),
                    },
                };
                build_resp(&x)
            }
            Err(e) => err_response(Box::new(e)),
        }
    }

    use serde::{Deserialize, Serialize};
    use tribbler::trib::Trib;

    #[derive(Serialize, Deserialize, Debug, Clone)]
    struct UserList {
        err: String,
        users: Vec<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    struct TribList {
        err: String,
        tribs: Vec<Arc<Trib>>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    struct Bool {
        err: String,
        v: bool,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    struct Clock {
        err: String,
        n: u64,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    struct WhoWhom {
        who: String,
        whom: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    struct Post {
        who: String,
        message: String,
        clock: u64,
    }
}
