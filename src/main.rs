mod cli;
mod service;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let args = cli::parse();
	service::service_entry(args).await?;

	Ok(())
}
