use aliyun_ddns::model::argument_model::Argument;
use structopt::StructOpt;
use anyhow::Result;


fn main() -> Result<()> {
    let ref argument: Argument = Argument::from_args();
    aliyun_ddns::entry(argument)?;
    Ok(())
}