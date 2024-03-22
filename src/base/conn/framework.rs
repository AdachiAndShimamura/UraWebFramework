use crate::base::conn::listener::Listener;
use anyhow::Result;
use hyper::server::*;

struct UraFrameWork {
    listener: Listener,
}

impl UraFrameWork {
    pub fn new() -> UraFrameWork {
        UraFrameWork {
            listener: Listener::new(),
        }
    }
    pub async fn start(&mut self) -> Result<()> {
        self.listener.listen().await?;
        Ok(())
    }
}

#[tokio::test]
async fn test() {
    UraFrameWork::new().start().await.unwrap();
}
