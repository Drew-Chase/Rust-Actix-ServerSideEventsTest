use std::{io, time::Duration};

use actix_web::{get, App, HttpServer, Responder};
use actix_web_lab::sse;
use time::format_description::well_known::Rfc3339;
use tokio::time::sleep;


/// Endpoint that provides Server-Sent Events (SSE) with the current timestamp in UTC.
#[get("/time")]
async fn timestamp() -> impl Responder {
	let (sender, receiver) = tokio::sync::mpsc::channel(2);

	// Spawn a new task to periodically send the current time.
	actix_web::rt::spawn(async move {
		loop {
			let time = time::OffsetDateTime::now_utc();
			let msg = sse::Data::new(time.format(&Rfc3339).unwrap()).event("timestamp");

			// Send the message via the channel.
			if sender.send(msg.into()).await.is_err() {
				break;
			}

			// Wait for 1 second before sending the next timestamp.
			sleep(Duration::from_secs(1)).await;
		}
	});

	// Return the SSE response with a keep-alive interval of 3 seconds.
	sse::Sse::from_infallible_receiver(receiver).with_keep_alive(Duration::from_secs(3))
}

#[actix_web::main]
async fn main() -> io::Result<()> {
	HttpServer::new(|| {
		App::new()
			.service(timestamp) // Register the timestamp service.
	})
		.workers(2) // Use 2 workers for the server.
		.bind(("127.0.0.1", 8080))? // Bind the server to localhost on port 8080.
		.run()
		.await
}