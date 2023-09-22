use anyhow::Result;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
	let hc = httpc_test::new_client("http://localhost:8080")?;

	hc.do_get("/health-check")
		.await?
		.print()
		.await?;

	hc.do_post(
		"/eco",
		json!({
			"username": "demo1",
			"pwd": "welcome"
		}),
	)
	.await?
	.print()
	.await?;

	Ok(())
}
