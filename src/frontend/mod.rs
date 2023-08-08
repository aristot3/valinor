pub mod server;

use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "src/frontend/"]
struct Asset;
