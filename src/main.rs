//! Demonstrates use of the Server-Sent Events (SSE) responder.

use std::{io, time::Duration};

use actix_web::{get, App, HttpServer, Responder};
use actix_web_lab::sse;
use time::format_description::well_known::Rfc3339;
use tokio::time::sleep;


#[get("/time")]
async fn timestamp() -> impl Responder {
	let (sender, receiver) = tokio::sync::mpsc::channel(2);

	actix_web::rt::spawn(async move {
		loop {
			let time = time::OffsetDateTime::now_utc();
			let msg = sse::Data::new(time.format(&Rfc3339).unwrap()).event("timestamp");

			if sender.send(msg.into()).await.is_err() {
				break;
			}

			sleep(Duration::from_secs(1)).await;
		}
	});

	sse::Sse::from_infallible_receiver(receiver).with_keep_alive(Duration::from_secs(3))
}

#[actix_web::main]
async fn main() -> io::Result<()> {
	HttpServer::new(|| {
		App::new()
			.service(timestamp)
	})
		.workers(2)
		.bind(("127.0.0.1", 8080))?
		.run()
		.await
}