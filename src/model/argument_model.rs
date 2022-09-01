use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "aliyun-dns", about = "run aliyun-dns with special config path")]
pub struct Argument {
    /// special config path to run
    #[structopt(short = "c", long = "config-path", parse(from_os_str))]
    pub config_path: Option<PathBuf>,
    /// special log4rs path to run
    #[structopt(short = "l", long = "log-path", parse(from_os_str))]
    pub log4rs_path: Option<PathBuf>,

    /// if true, only run ip detect, skip synchronize
    #[structopt(short, long)]
    pub dry_run: bool,
}
