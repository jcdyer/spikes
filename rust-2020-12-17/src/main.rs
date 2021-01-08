use std::io::Write;

use serde::ser::{
    Serializer,
    SerializeSeq,
};

#[derive(serde::Serialize)]
struct Event {
    origin: &'static str,
    destination: &'static str,
    date: u64,
    product: &'static str,
}
fn main() -> anyhow::Result<()> {
    let rows = vec![
        Event { origin: "Farm", destination: "Pack house", date: 20201210, product: "lettuce" },
        Event { origin: "Pack house", destination: "Distributor", date: 20201212, product: "salad mix" },
        Event { origin: "Distributor", destination: "Restaurant 1", date: 20201213, product: "salad mix" },
        Event { origin: "Distributor", destination: "Restaurant 2", date: 20201213, product: "salad mix" },
        Event { origin: "Distributor", destination: "Restaurant 3", date: 20201217, product: "salad mix" },
        Event { origin: "Distributor", destination: "Restaurant 4", date: 20201214, product: "salad mix" },
    ];

    let out = std::io::stdout();
    let mut ser = serde_json::Serializer::new(&out);
    let mut seq = ser.serialize_seq(Some(rows.len()))?;
    out.lock().write_all(b"\n")?;
    for row in rows {
        seq.serialize_element(&row)?;
        out.lock().write_all(b"\n")?;
    }
    seq.end()?;
    out.lock().write_all(b"\n")?;
    Ok(())
}
