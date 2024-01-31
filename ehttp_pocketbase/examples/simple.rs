use ehttp_pocketbase::client;
use futures_lite::{future, FutureExt};

fn main() {
    let client = client::Client::default();
    let _result = future::block_on(async{client.health_check()});
    // println!("{:?}", result);
}
