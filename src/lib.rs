#[macro_use] extern crate assert_approx_eq;


// Ten digit grids
// 00.001 => 1m
// 10.000 => 10km
// 1.000  => 1km
#[derive(Copy,Clone)]
pub struct UtmCoordinate {
    pub easting: f32, // In Kilometers
    pub northing: f32, // In Kilometers
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
    fn it_works() {
    }
}
