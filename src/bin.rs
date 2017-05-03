extern crate utm_calc;
extern crate docopt;
extern crate rustc_serialize;

use docopt::Docopt;
use utm_calc::{ UtmCoordinate, utm_range };

const USAGE: &'static str = "
Usage: utm_calc range <coorddst> <coordsrc>
";

#[derive(RustcDecodable)]
struct Args {
    arg_coorddst: String,
    arg_coordsrc: String,
}


fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());
    let a = UtmCoordinate::from_coordinates(args.arg_coorddst).unwrap();
    let b = UtmCoordinate::from_coordinates(args.arg_coordsrc).unwrap();
    println!("Range: {:>6.0}m", utm_range(a, b) * 1000f32);
}
