use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use hyper::server::*;
use hyper::server::conn::http1::Connection;
use anyhow::Result;
use log::info;
use tokio::net::TcpListener;

struct HttpListener{
    listener:conn,

}

impl HttpListener {
    pub async fn start()->Result<()>{
        env_logger::init();
        let addr=SocketAddr::new("127.0.0.1".parse()?,8080).into();
        let listener=TcpListener::bind(addr).await?;
        info!("http listener start!");
        loop {
            let (stream,_)=listener.accept().await?;
            let io=hyper::server::conn::http1::Builder::new().serve_connection(stream)
        }
    }

}
fn handler(){

}
#[test]
fn test(){
    let mut h1_conn_builder=conn::http1::Builder::new();
    h1_conn_builder.keep_alive(true).
}