use aliyun_ddns::model::argument_model::Argument;
use anyhow::Result;
use structopt::StructOpt;

#[tokio::main]
async fn main() -> Result<()> {
    let ref argument: Argument = Argument::from_args();
    aliyun_ddns::entry(argument).await?;
    Ok(())
}
