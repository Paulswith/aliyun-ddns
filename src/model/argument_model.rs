/*
create at `2019-09-12` by `itachy`
*/
use structopt::StructOpt;
use std::path::PathBuf;


#[derive(Debug, StructOpt)]
#[structopt(name = "aliyun-dns", about = "run aliyun-dns with special config path")]
pub struct Argument {
    /// special config path to run
    #[structopt(short = "c", long = "config-path", parse(from_os_str))]
    config_path: Option<PathBuf>,
    /// special log4rs path to run
    #[structopt(short = "l", long = "log-path", parse(from_os_str))]
    log4rs_path: Option<PathBuf>,
}

impl Argument {
    pub fn config_path(&self) -> &Option<PathBuf> {
        &self.config_path
    }

    pub fn log4rs_path(&self) -> &Option<PathBuf> {
        &self.log4rs_path
    }
}