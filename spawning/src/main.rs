use std::collections::HashMap;
use bytes::Bytes;
use mini_redis::{Command, Connection, Frame};
use tokio::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};

type Db = Arc<Mutex<HashMap<String, Bytes>>>;

// this is a mini_redis lite
#[tokio::main]
async fn main() {
	let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
	// make a shareable db across processes
	let db: Arc<Mutex<HashMap<String, Bytes>>> = Arc::new(Mutex::new(HashMap::new()));

	loop {
		let (socket, _) = listener.accept().await.unwrap();
		let db = db.clone();
		// spawn each process for each incoming request, move socket to a new task
		tokio::spawn(async move {
			// spawn will spawn a task, which is an async green thread
			process(socket, db).await;
		});
	}
}

async fn process(socket: TcpStream, db: Db) {
	use mini_redis::Command::{Get,Set};

	let mut conn = Connection::new(socket);

	while let Some(frame) = conn.read_frame().await.unwrap() {
		let resp = match Command::from_frame(frame).unwrap() {
			Get(cmd) => {
				// locking mutex like this may cause a contention if the process take a long time (not likely in this use case)
				// in a likely hood of a contention, we could use either message passing, or sharding the mutex
				// sharding the mutex is just making a vector/has map of mutexes. Then we could reroute an incoming key to each mutex by hashing
				// think of this like database sharing
				let db = db.lock().unwrap();
				if let Some(val) = db.get(cmd.key()) {
					Frame::Bulk(val.clone().into())
				} else {
					Frame::Null
				}
			}
			Set(cmd) => {
				let mut db = db.lock().unwrap();
				// need to_string, because insert's arguments are mutable, need to borrow
				db.insert(cmd.key().to_string(), cmd.value().clone());
				Frame::Simple("OK".to_string())
			}
			cmd => panic!("unimplemented {:?}", cmd),
		};

		conn.write_frame(&resp).await.unwrap();
	}
}
