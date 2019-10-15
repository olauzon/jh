extern crate actix_web;
extern crate clap;
extern crate jumphash;

#[macro_use]
extern crate serde_derive;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use jumphash::hash;

use std::str::FromStr;

#[derive(Deserialize)]
pub struct GetBucketParams {
    k: String,
    n: u32,
}

impl GetBucketParams {
    fn get_bucket(&self) -> u32 {
        hash(&self.k, self.n)
    }
}

fn server(ip: &str, port: &str) {
    fn get_bucket(params: web::Query<GetBucketParams>) -> impl Responder {
        let bucket = params.get_bucket();
        HttpResponse::Ok().body(bucket.to_string())
    }

    // HttpServer automatically starts a number of http workers.
    // By default this number is equal to number of logical CPUs in the system.
    HttpServer::new(|| App::new().service(web::resource("/").route(web::get().to(get_bucket))))
        .bind(format!("{}:{}", ip, port))
        .unwrap()
        .run()
        .unwrap();
}

fn main() {
    let matches = clap::App::new("jh: Jump Hash consistent hashing utility")
        .version("0.2")
        .author("Olivier Lauzon <olauzon@gmail.com>")
        .about("Returns a bucket for a key and number of buckets")
        .subcommand(
            clap::SubCommand::with_name("get")
                .about("Get bucket from CLI invocation")
                .arg(
                    clap::Arg::with_name("key")
                        .short("k")
                        .long("key")
                        .value_name("KEY")
                        .help("Value for KEY")
                        .required(true)
                        .index(1),
                )
                .arg(
                    clap::Arg::with_name("buckets")
                        .short("n")
                        .long("buckets")
                        .value_name("BUCKETS")
                        .help("Number of buckets")
                        .required(true)
                        .index(2),
                ),
        )
        .subcommand(
            clap::SubCommand::with_name("server")
                .about("Start jh http server")
                .arg(
                    clap::Arg::with_name("port")
                        .short("p")
                        .long("port")
                        .value_name("PORT")
                        .help("PORT number")
                        .default_value("8088"),
                )
                .arg(
                    clap::Arg::with_name("ip")
                        .short("i")
                        .long("ip")
                        .value_name("IP")
                        .help("IP Interface")
                        .default_value("0.0.0.0"),
                ),
        )
        .get_matches();

    match matches.subcommand_name() {
        Some("get") => {
            let args = matches.subcommand_matches("get").unwrap();
            let k = args.value_of("key").unwrap().to_string();
            let n: u32 = FromStr::from_str(args.value_of("buckets").unwrap()).unwrap();

            let bucket = GetBucketParams { k, n }.get_bucket();
            println!("{}", bucket)
        }
        Some("server") => {
            let args = matches.subcommand_matches("server").unwrap();
            let port = args.value_of("port").unwrap();
            let ip = args.value_of("ip").unwrap();

            println!("Starting jh server on {}:{}", ip, port);
            server(ip, port)
        }
        _ => {}
    }
}

#[test]
fn test_jumphash() {
    let idx = hash(&"my key string", 1024);
    assert_eq!(idx, 731);
}
