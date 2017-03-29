extern crate utm_calc;
extern crate docopt;
extern crate rustc_serialize;

use docopt::Docopt;
use utm_calc::{ UtmCoordinate, utm_range };

const USAGE: &'static str = "
Usage: utm_calc range <aeast> <anorth> <beast> <bnorth>
";

#[derive(RustcDecodable)]
struct Args {
    arg_aeast: f32,
    arg_anorth: f32,
    arg_beast: f32,
    arg_bnorth: f32,
}


fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());
    let a = UtmCoordinate { easting: args.arg_aeast, northing: args.arg_anorth };
    let b = UtmCoordinate { easting: args.arg_beast, northing: args.arg_bnorth };
    println!("Range: {:>6.0}m", utm_range(a, b) * 1000f32);
}
