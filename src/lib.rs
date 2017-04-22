#[macro_use] extern crate assert_approx_eq;

use std::f32;
use std::error;
use std::fmt;

#[derive(Debug)]
pub struct MalformedCoordinates {}

impl error::Error for MalformedCoordinates {
    fn description(&self) -> &str {
        "Malformed Coordinates"
    }
}

impl fmt::Display for MalformedCoordinates {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Malformed Coordinates")
    }
}

#[derive(Debug)]
pub enum CoordParseError {
    MalformedCoordinates(MalformedCoordinates),
    ParseError(std::num::ParseIntError)
}

impl error::Error for CoordParseError {
    fn description(&self) -> &str {
        // Both underlying errors already impl `Error`, so we defer to their
        // implementations.
        match *self {
            CoordParseError::MalformedCoordinates(ref err) => err.description(),
            CoordParseError::ParseError(ref err) => err.description(),
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            // N.B. Both of these implicitly cast `err` from their concrete
            // types (either `&io::Error` or `&num::ParseIntError`)
            // to a trait object `&Error`. This works because both error types
            // implement `Error`.
            CoordParseError::MalformedCoordinates(ref err) => Some(err),
            CoordParseError::ParseError(ref err) => Some(err),
        }
    }
}

impl fmt::Display for CoordParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use std::error::Error;
        write!(f, "{}", self.cause().unwrap())
    }
}


// Ten digit grids
// 00.001 => 1m
// 10.000 => 10km
// 1.000  => 1km
#[derive(Copy,Clone,Debug,PartialEq)]
pub struct UtmCoordinate {
    pub easting: f32, // In Kilometers
    pub northing: f32, // In Kilometers
}

impl UtmCoordinate {
    pub fn from_coords(coords: String) -> Result<UtmCoordinate, CoordParseError> {
        match coords.len() {
            2 => UtmCoordinate::from_digit(coords, 2),
            4 => UtmCoordinate::from_digit(coords, 4),
            6 => UtmCoordinate::from_digit(coords, 6),
            8 => UtmCoordinate::from_digit(coords, 8),
            10 => UtmCoordinate::from_digit(coords, 10),
            _ => Err(CoordParseError::MalformedCoordinates(MalformedCoordinates{})),
        }
    }

    fn from_digit(coords: String, digits: usize) -> Result<UtmCoordinate, CoordParseError> {
        let scale: f32 = (10.0 as f32).powi(2 - (digits as i32 / 2));
        let (east_s, north_s) = coords.split_at(digits / 2);
        let east = east_s.parse::<u32>().map_err(CoordParseError::ParseError)?;
        let north = north_s.parse::<u32>().map_err(CoordParseError::ParseError)?;
        Ok(UtmCoordinate { easting: scale * east as f32, northing: scale * north as f32 })
    }
}


pub fn utm_range(location: UtmCoordinate, destination: UtmCoordinate) -> f32 {
    let x_diff = destination.easting - location.easting;
    let y_diff = destination.northing - location.northing;

    return (x_diff.powi(2) + y_diff.powi(2)).sqrt()
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn simple_range() {
        let a = UtmCoordinate{ easting: 3.0, northing: 0.0 };
        let b = UtmCoordinate{ easting: 6.0, northing: 4.0 };
        assert_approx_eq!(5.0, utm_range(a, b), 0.001);
        assert_approx_eq!(5.0, utm_range(b, a), 0.001);
    }

    #[test]
    fn zero_range() {
        let a = UtmCoordinate{ easting: 3.0, northing: 4.0 };
        let b = UtmCoordinate{ easting: 3.0, northing: 4.0 };
        assert_approx_eq!(0.0, utm_range(a, b), 0.001);
        assert_approx_eq!(0.0, utm_range(b, a), 0.001);
    }

    #[test]
    fn approx_range_01() {
        let a = UtmCoordinate{ easting: 3.0, northing: 4.001 };
        let b = UtmCoordinate{ easting: 3.0, northing: 4.00 };
        assert_approx_eq!(0.001, utm_range(a, b), 0.001);
        assert_approx_eq!(0.001, utm_range(b, a), 0.001);
    }

    #[test]
    fn approx_range_02() {
        let a = UtmCoordinate{ easting: 15.0, northing: 4.0 };
        let b = UtmCoordinate{ easting: 15.500, northing: 4.025 };
        assert_approx_eq!(0.500, utm_range(a, b), 0.001);
        assert_approx_eq!(0.500, utm_range(b, a), 0.001);
    }

    #[test]
    fn parse_from_string() {
        let a: String = "55341840".to_string();
        let coord = UtmCoordinate::from_coords(a).unwrap();
        assert_eq!(UtmCoordinate { easting: 55.34, northing:18.40}, coord)
    }

    #[test]
    fn parse_from_malformed_string() {
        let a: String = "5341840".to_string();
        let coord = UtmCoordinate::from_coords(a);
        assert!(match coord {
            Ok(_) => false,
            Err(CoordParseError::MalformedCoordinates) => true,
        });
    }

    #[test]
    fn it_works() {
    }
}
