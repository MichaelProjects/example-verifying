use structopt::StructOpt;



#[derive(StructOpt)]
#[structopt(
    name = "examples",
    about = "Test examples in languages that dont have a standard way of executing them."
)]
#[derive(Debug, Clone)]
pub struct Examples {
    #[structopt(help = "Root directory of examples")]
    pub dir: String,
    #[structopt(help = "Language entry eg. python = if__name__ == __main__")]
    pub language_entry: String,
    #[structopt()]
    pub dependency_filename: String,
    #[structopt(help = "The base command like: python or ruby")]
    pub execute_command: String,
    #[structopt(help = "How you install dependencys from a file: pip install -r ")]
    pub install_deps_command: String
}