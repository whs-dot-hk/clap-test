use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    name: Option<String>,

    #[arg(short, long, value_name = "AMI_ID", required = true, group = "image")]
    ami_id: Option<String>,
    #[arg(short, long, required = true, group = "image")]
    default: bool,
}

fn main() {
    let cli = Cli::parse();
    
    println!("{:?}", cli.name);
    println!("{:?}", cli.ami_id);
    println!("{:?}", cli.default);
}
