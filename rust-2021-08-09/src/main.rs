use ignore::{WalkBuilder, overrides::OverrideBuilder};

fn main() -> anyhow::Result<()> {
    let overrides = OverrideBuilder::new(".")
        .add("!*-excluded")?
        .build()?;
    let walk = WalkBuilder::new(".")
        .add_custom_ignore_filename("ignore")
        .follow_links(true)
        .overrides(overrides)
        .build();

    for x in walk {
        println!("{:?}", x);
    }
    Ok(())
}
