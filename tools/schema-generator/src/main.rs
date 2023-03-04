use std::{fs::File, io::Write};

use iota_client::ClientBuilder;
use schemars::schema_for;

fn main() -> std::io::Result<()> {
    let schema = schema_for!(ClientBuilder);

    let mut file = File::create("ClientBuilderSchema.json")?;
    write!(file, "{}", serde_json::to_string_pretty(&schema)?)?;

    Ok(())
}
