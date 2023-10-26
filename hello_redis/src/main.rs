use mini_redis::{client, Result};
use mini_redis::client::Client;

// tokio runtime to make main an async
#[tokio::main]
async fn main() -> Result<()> {
	let mut client: Client;
	// connect to the mini-redis
  match client::connect("127.0.0.1:6379").await {
	  Ok(c) => {
		  client = c;
	  }
	  Err(err) => {
		  panic!("{}", err);
	  }
  }

	// await? is a mini-redis's macro
	// set hello = world
	// client.set("hello", "world".into()).await?;

	let result = client.get("hello").await?;
	println!("got value from server; result={:?}", result);

	Ok(())
}
