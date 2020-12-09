use clap::Clap;
use warp::{Filter, reply, http::{header}};
use rust_embed::RustEmbed;
use chrono::Local;
use pickledb::{PickleDb, PickleDbDumpPolicy};
use serde::{Serialize, Deserialize};
use std::net::IpAddr;

#[derive(Clap)]
#[clap(version = "1.0.1")]
struct Opts {
    #[clap(short, long, default_value = "8088", about = "Listen port")]
    port: u16,
    #[clap(short, long, default_value = "0.0.0.0", about = "Listen ip")]
    ip: String
}

#[derive(RustEmbed)]
#[folder = "public/"]
struct Asset;

#[derive(Serialize, Deserialize, Debug, Default)]
struct SpeedList {
    code: i32,
    data: Vec<Record>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
struct Record {
    ip: String,
    isp: String,
    addr: String,
    download: i32,
    upload: i32,
    ping: i32,
    jitter: i32,
    time: String,
}


#[tokio::main]
async fn main() {
    let opts: Opts = Opts::parse();

    let mut db = PickleDb::load_bin("report.db", PickleDbDumpPolicy::AutoDump).unwrap_or_else(|_| {
        PickleDb::new_bin("report.db", PickleDbDumpPolicy::AutoDump)
    });

    if !db.lexists("report") {
        db.lcreate("report").unwrap();
    }

    let index_html = warp::path::end().map(|| {
        let file = Asset::get("index.html").unwrap();
        reply::html(file)
    });

    let results_html = warp::path("result").map(|| {
        let file = Asset::get("results.html").unwrap();
        reply::html(file)
    });

    let worker = warp::path("worker.js").map(|| {
        let file = Asset::get("worker.js").unwrap();
        reply::html(file)
    }).with(reply::with::header(header::CONTENT_TYPE, "application/javascript"));

    let download = warp::path("download").map(|| {
        let data = [3u8; 1048576];
        String::from_utf8(data.to_vec()).unwrap()
    });

    let report = warp::path("report")
        .and(warp::body::json()).map(|mut data: Record| {
        data.time = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        println!("{:?}", data);
        let mut db = PickleDb::load_bin("report.db", PickleDbDumpPolicy::AutoDump).unwrap();
        db.ladd("report", &data).unwrap();
        reply::reply()
    });

    let results = warp::path("results").map(|| {
        let db = PickleDb::load_bin("report.db", PickleDbDumpPolicy::AutoDump).unwrap();
        let mut result = SpeedList { code: 0, data: vec![] };
        let len = db.llen("report");
        for i in 0..len {
            let mut r = db.lget::<Record>("report", len - 1 - i).unwrap();
            r.ip = r.ip.trim_end_matches(char::is_numeric).to_owned() + "*";
            result.data.push(r);
        }
        reply::json(&result)
    });

    let upload = warp::path("upload").and(warp::body::bytes())
        .map(|_bytes: warp::hyper::body::Bytes| {
            reply::reply()
        });

    let ping = warp::path("ping").map(|| reply::reply());

    let favicon = warp::path("favicon.ico").map(|| {
        let file = Asset::get("favicon.ico").unwrap();
        reply::html(file)
    }).with(reply::with::header(header::CONTENT_TYPE, "image/x-icon"));

    let route = index_html.or(results_html).or(worker).or(ping)
        .or(download).or(upload).or(report).or(results).or(favicon);

    println!("Server listening on {}:{}", opts.ip ,opts.port);
    warp::serve(route).run((opts.ip.parse::<IpAddr>().unwrap(), opts.port)).await
}
