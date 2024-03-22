use crate::base::conn::connection::{Conn, ConnPool};
use hyper::client::conn::http1;
use hyper_util::rt::TokioIo;
use log::info;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower::service_fn;

pub struct Listener {
    pool: ConnPool,
}

impl Listener {
    pub fn new() -> Listener {
        Listener {
            pool: ConnPool::new().unwrap(),
        }
    }
    pub async fn listen(&mut self) -> anyhow::Result<()> {
        env_logger::init();
        let addr = SocketAddr::new("127.0.0.1".parse()?, 8080);
        let listener = TcpListener::bind(addr).await?;
        info!("http listener start!");
        loop {
            let (stream, _) = listener.accept().await?;
            let conn = Conn::new(stream);
            self.pool.handle_conn(conn);
        }
    }
}
