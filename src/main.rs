use aliyun_ddns::model::argument_model::Argument;
use structopt::StructOpt;


fn main() {
    let ref argument: Argument = Argument::from_args();
    aliyun_ddns::entry(argument);
}