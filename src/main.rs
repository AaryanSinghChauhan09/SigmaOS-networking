use ::networking::{networking, Config};

fn main() -> anyhow::Result<()> {
    let config = Config::default();
    let mut component = networking::new(config);
    let _ = &component;
    println!("SigmaOS-networking operational");
    Ok(())
}
