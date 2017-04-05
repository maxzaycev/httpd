extern crate argparse;
mod Http;

use argparse::{ArgumentParser, Store};

fn main() {
    let mut n_cpu = 1;
    let mut root_dir = "./".to_string();

    let ip = "127.0.0.1".to_string();
    let port = "80".to_string();
    {
    	let mut ap = ArgumentParser::new();
        ap.set_description("httpd");
        ap.refer(&mut n_cpu)
            .add_option(&["-n"], Store,
            "CPU Number");
        ap.refer(&mut root_dir)
            .add_option(&["-r"], Store,
            "Root dir");
        ap.parse_args_or_exit();
    }

    Http::Server::new(ip, root_dir, n_cpu).listen(port);
}