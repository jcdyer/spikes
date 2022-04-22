use std::sync::Arc;

use dropshot::{ApiDescription, ApiEndpoint, ConfigDropshot, ConfigLogging, ConfigLoggingLevel, HttpServerStarter, endpoint};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let log =
        ConfigLogging::StderrTerminal {
            level: ConfigLoggingLevel::Trace,
        }
        .to_logger("minimal-example")?
        ;


    let endpoint = ApiEndpoint::new(
        "abc",

    );
    // Describe the API.
    let api = ApiDescription::new()
        .register(endpoint).map_err(anyhow::Error::msg)?;
    // Register API functions -- see detailed example or ApiDescription docs.

    // Start the server.
    let server =
        HttpServerStarter::new(
            &ConfigDropshot {
                bind_address: "127.0.0.1:8384".parse().unwrap(),
                request_body_max_bytes: 1024,
            },
            api,
            Arc::new(0u64),
            &log,
        )?
        .start();
    server.await
        .map_err(anyhow::Error::msg)
}
/*
#[endpoint(
    method = POST,
    path = "/counter/{n}",
)]
async fn increment_counter() {

}
*/

#[cfg(feature = "derp")]
fn herp() -> &'static str {
    "i'm a herp derp"
}