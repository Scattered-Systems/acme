/*
    Appellation: acme
    Context: library
    Creator: FL03 <jo3mccain@icloud.com>
    Description:
        Acme is a complete client library for building data-centric Rust applications

*/
#[doc(inline)]
#[cfg(feature = "derive")]
pub use acme_derive::*;
#[doc(inline)]
#[cfg(feature = "macros")]
pub use acme_macros::*;

pub use crate::{configurations::*, interfaces::*};

mod configurations;
mod interfaces;

mod utils {
    use futures::{Future, Stream, StreamExt, TryStreamExt};
    use coinbase_pro_rs::{WSFeed, CBError, WS_SANDBOX_URL};
    use coinbase_pro_rs::structs::wsfeed::*;

    use coinbase_pro_rs::{Public, ASync, SANDBOX_URL};
    use futures::{TryFutureExt};

    pub async fn get_cb_time() {
        let client: Public<ASync> = Public::new_with_keep_alive(SANDBOX_URL, false);
        // if keep_alive is not disables - tokio::run will hold the connection without exiting the example
        client.get_time().await
            .map_err(|_| ())
            .and_then(|time| {
                println!("Coinbase.time: {}", time.iso);
                Ok(())
            });

    }


    pub async fn stream(ticker: String) {
        let stream = WSFeed::connect(WS_SANDBOX_URL,
            &[ticker], &[ChannelType::Heartbeat]).await.unwrap();

        stream
            .take(10)
            .for_each(|msg: Result<Message, CBError>| async {
            match msg.unwrap() {
                Message::Heartbeat {sequence, last_trade_id, time, ..} => println!("{}: seq:{} id{}",
                                                                                   time, sequence, last_trade_id),
                Message::Error {message} => println!("Error: {}", message),
                Message::InternalError(_) => panic!("internal_error"),
                other => println!("{:?}", other)
            }
        }).await;
    }
}