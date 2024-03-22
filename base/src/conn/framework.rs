
use anyhow::Result;
use crate::conn::listener::Listener;

struct UraFrameWork {
    listener: Listener,
}

impl UraFrameWork {
    pub fn new() -> Result<UraFrameWork> {
        Ok(UraFrameWork {
            listener: Listener::new()?,
        })
    }
    pub async fn start(&mut self) -> Result<()> {
        self.listener.listen().await?;
        Ok(())
    }
}

#[tokio::main]
async fn main()->Result<()> {
    Ok(UraFrameWork::new()?.start().await.unwrap())
}
