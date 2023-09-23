use anyhow::Result;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
	let hc = httpc_test::new_client("http://localhost:3000")?;

	hc.do_get("/health-check")
		.await?
		.print()
		.await?;

	hc.do_post("/health-check", json!({}))
		.await?
		.print()
		.await?;

	Ok(())
}
