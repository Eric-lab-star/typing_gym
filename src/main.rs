use clap::{
    Parser,
    Args,
};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(flatten)]
    vers: Vers,


    /// Some regular input
    #[arg(group = "input")]
    input_file: Option<String>,

    /// some special input argument
    #[arg(long, group = "input")]
    spec_in: Option<String>,

    #[arg(short, requires = "input")]
    config: Option<String>,
}

#[derive(Args)]
#[group(required = true, multiple = false)]
struct Vers {
    #[arg(long, value_name = "VER")]
    set_ver: Option<String>,

    /// auto inc major
    #[arg(long)]
    major: bool,

    /// auto inc minor
    #[arg(long)]
    minor: bool,

    /// auto inc patchk
    #[arg(long)]
    patch: bool,
}

fn main() {
    let cli = Cli::parse();

    let mut major = 1;
    let mut minor = 2;
    let mut patch = 3;

    let vers = &cli.vers;
    let version = if let Some(ver) = vers.set_ver.as_deref() {
        ver.to_string()
    } else {
        let (maj, min, pat) = (vers.major, vers.minor, vers.patch);
        match (maj, min, pat) {
            (true, _, _) => major += 1,
            (_, true, _) => minor += 1,
            (_, _, true) => patch += 1,
            _ => unreachable!(),
        };
        format!("{major}.{minor}.{patch}")
    };

    print!("Version: {version}");

    if let Some(config) = cli.config.as_deref() {
        let input = cli
            .input_file
            .as_deref()
            .unwrap_or_else(|| cli.spec_in.as_deref().unwrap());
        println!("Doing work using input {input} and config {config}");
    }


}
