extern crate publicip;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt()]
struct Opt {
    #[structopt(
        short = "4",
        long = "ipv4",
        parse(try_from_str),
        default_value = "true",
        help = "Return the IPv4 address"
    )]
    ipv4: bool,
    #[structopt(
        short = "6",
        long = "ipv6",
        parse(try_from_str),
        default_value = "false",
        help = "Return the IPv6 address"
    )]
    ipv6: bool,
}

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let opt = Opt::from_args();
    if opt.ipv4 {
        println!("{}", publicip::v4().await?);
    } else if opt.ipv6 {
        println!("{}", publicip::v6().await?);
    }
    Ok(())
}
