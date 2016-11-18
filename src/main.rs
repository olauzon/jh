extern crate docopt;
extern crate iron;
extern crate jumphash;
extern crate params;
extern crate rustc_serialize;

use docopt::Docopt;
use iron::prelude::*;
use iron::status;
use jumphash::hash;

const USAGE: &'static str = "
Jump Hash

Usage:
  jh get <key> <buckets>
  jh server <port>
  jh server
  jh (-h | --help)
  jh --version

Options:
  -h --help     Show this screen.
  --version     Show version.
";

#[derive(Debug, RustcDecodable)]
struct Args {
    arg_key: String,
    arg_buckets: u64,
    arg_port: Option<u32>,
    cmd_get: bool,
    cmd_server: bool,
}

fn server(args: &Args) {
    let port = match args.arg_port {
        Some(x) => x.to_string(),
        None => "3000".to_string()
    };

    println!("Starting server on port {}", port);

    fn h(r: &mut Request) -> IronResult<Response> {
        use params::{Params, Value};
        let map = r.get_ref::<Params>().unwrap();
        match (map.find(&["k"]), map.find(&["n"])) {
            (Some(&Value::String(ref k)), Some(&Value::String(ref n))) => {
                let bucket = hash(&k, n.parse::<u32>().unwrap());
                Ok(Response::with((status::Ok, bucket.to_string())))
            },
            (_, _) => Ok(Response::with(status::NotFound)),
        }
    }

    Iron::new(h).http(["0.0.0.0", &port].join(":").as_str()).unwrap();
}

fn main() {
    let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());
    match (args.cmd_get, args.cmd_server) {
        (true, false) => {
            let h = hash(&args.arg_key, args.arg_buckets as u32);
            println!("{}", h);
        },
        (false, true) => {
            server(&args)
        },
        (_, _) => {}
    }
}

#[test]
fn test_jumphash() {
    let idx = hash(&"my key string", 1024);
    assert_eq!(idx, 731);
}
