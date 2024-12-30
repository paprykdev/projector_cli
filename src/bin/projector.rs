use clap::Parser;
use rust::{config::Config, opts::Opts, projector::Projector};

use anyhow::Result;

fn main() -> Result<()> {
    let cfg: Config = Opts::parse().try_into()?;
    let config: Config = Opts::parse().try_into()?;
    let mut proj: Projector = config.into();

    match cfg.operation {
        rust::config::Operation::Print(None) => {
            let value = proj.get_value_all();
            let value = serde_json::to_string(&value)?;

            println!("{}", value);
        }
        rust::config::Operation::Print(Some(k)) => {
            proj.get_value(&k).map(|x| println!("{}", x));
        }
        rust::config::Operation::Add(k, v) => {
            proj.set_value(k, v);
            proj.save()?;
        }
        rust::config::Operation::Remove(k) => {
            proj.remove_value(&k);
            proj.save()?;
        }
    }

    return Ok(());
}
