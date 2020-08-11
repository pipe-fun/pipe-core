#![feature(duration_zero)]

mod execute_time;
mod original_task;
mod pipe_task;
mod pipe_tasks;
mod device;

#[macro_use]
extern crate serde_derive;

use futures::executor::block_on;
use futures::{StreamExt, AsyncReadExt};
use async_std::net::TcpListener;
use crate::pipe_tasks::PipeTasks;

async fn test() -> std::io::Result<()> {
    let mut buf = [0; 1024];
    let listener = TcpListener::bind("127.0.0.1:1234").await?;

    while let Some(stream) = listener.incoming().next().await {
        let mut socket = stream?;
        let len = socket.read(&mut buf).await?;

        let _socket = socket.clone();
        let token = core::str::from_utf8(&buf[..len]).unwrap();

        let mut tasks = PipeTasks::new(token, _socket);

        async_std::task::spawn(async move {
            loop {
                let fs = tasks.get_all_future().await;
                futures::future::join_all(fs).await;
            }
        });
    }

    Ok(())
}

fn main() {
    block_on(test()).unwrap();
}