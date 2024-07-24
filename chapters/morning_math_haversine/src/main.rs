

const EARTH_RADIUS_KILOMETER: f64 = 6371.0_f64;

fn main() {
    println!("Hello, world!");
    let location_sudoroom = Location { 
        name: "SudoRoom".to_string(), 
        coordinate: Coordinate { 
            latitude: 835004_f64, 
            longitude: -122.2641282_f64
        }
    };
    // (48.85341_f64, -2.34880_f64)
    // Note that since it's a function Coordinate::new not Coordinate.new
    let location_paris = Location { 
        name: "Paris".to_string(), 
        coordinate: Coordinate::new(48.85341_f64, -2.34880_f64)
    };

    let distance = calculate_distance_earth(&location_sudoroom, &location_paris, EARTH_RADIUS_KILOMETER);
    println!("The distance between paris and sudoroom on the globe is {} km", distance);
}

fn calculate_distance_earth(location_one: &Location, location_two: &Location, earth_radius: f64) -> f64 {
    let central_angle_inner = calculate_central_angle_inner(location_one, location_two);

    let central_angle: f64 = 2.0 * central_angle_inner.sqrt().asin();

    earth_radius * central_angle
}

/// Calculates the central angle of two locations on a sphere
/// - Note that the last value is the return so return is not needed in rust - https://stevedonovan.github.io/rust-gentle-intro/1-basics.html#function-types-are-explicit
fn calculate_central_angle_inner(location_one: &Location, location_two: &Location) -> f64 {
    println!("Location one is {} and location two is {}", location_one.description(), location_two.description());
    let latitude_one_degrees = location_one.coordinate.latitude;
    let latitude_two_degrees: f64 = location_two.coordinate.latitude;


    let latitude_one: f64 = latitude_one_degrees.to_radians();
    let latitude_two: f64 = latitude_two_degrees.to_radians();

    let delta_latitude = (latitude_one_degrees - latitude_two_degrees).to_radians();
    let delta_longitude = (location_one.coordinate.longitude - location_two.coordinate.longitude).to_radians();

    let central_angle_inner = (delta_latitude / 2.0).sin().powi(2) + latitude_one.cos() * latitude_two.cos() * (delta_longitude/2.0).sin().powi(2);
    central_angle_inner
}

#[derive(Debug)]
struct Location {
    name: String,
    coordinate: Coordinate
}

// Note that to return you shouldn't have the semicolon
impl Location {
    fn description(&self) -> String {
        format!("Location {} has coordinates of {}", self.name, self.coordinate.description())
    }
}
#[derive(Debug)]
struct Coordinate {
    latitude: f64,
    longitude: f64
}

// Implement functionality for Coordinate 
// - here a convenience initializer(?) is that the right term
impl Coordinate {
    fn new(latitude: f64, longitude: f64) -> Self {
        Self {
            latitude, longitude
        }
    }

    // simple debug description - note that the &self is "borrowing" itself to avoid ownership
    // ? - do they get retain cycles in rust? hmmm - probably not since the compiler is so strict
    fn description(&self) -> String {
        format!("Latitude: {}, Longitude: {}", self.latitude, self.longitude)
    }
}

const EARTH_RADIUS_KILOMETER: f64 = 6371.0_f64;

fn main() {
    println!("Hello, world!");
    let location_sudoroom = Location { 
        name: "SudoRoom".to_string(), 
        coordinate: Coordinate { 
            latitude: 835004_f64, 
            longitude: -122.2641282_f64
        }
    };
    // (48.85341_f64, -2.34880_f64)
    // Note that since it's a function Coordinate::new not Coordinate.new
    let location_paris = Location { 
        name: "Paris".to_string(), 
        coordinate: Coordinate::new(48.85341_f64, -2.34880_f64)
    };

    //  @37.7624302,-122.421111,17z/data=!3m1!4b1!4m6!3m5!1s0x808f7e23baa2b1df:0x81b913a252fb8d04!8m2!3d37.7624302!4d-122.4185361!16s%2Fg%2F1tfst37m?entry=ttu
    let nb_location = Location::new("Noisebridge".to_string(), 37.7624302_f64, -122.421111_f64);
    println!("Noisebridge location is {:?}", nb_location);

    // let mut location = Location

    let distance = calculate_distance_earth(&location_sudoroom, &location_paris, EARTH_RADIUS_KILOMETER);
    println!("The distance between paris and sudoroom on the globe is {} km", distance);
}

fn calculate_distance_earth(location_one: &Location, location_two: &Location, earth_radius: f64) -> f64 {
    let central_angle_inner = calculate_central_angle_inner(location_one, location_two);

    let central_angle: f64 = 2.0 * central_angle_inner.sqrt().asin();

    earth_radius * central_angle
}

/// Calculates the central angle of two locations on a sphere
/// - Note that the last value is the return so return is not needed in rust - https://stevedonovan.github.io/rust-gentle-intro/1-basics.html#function-types-are-explicit
fn calculate_central_angle_inner(location_one: &Location, location_two: &Location) -> f64 {
    println!("Location one is {} and location two is {}", location_one.description(), location_two.description());
    let latitude_one_degrees = location_one.coordinate.latitude;
    let latitude_two_degrees: f64 = location_two.coordinate.latitude;


    let latitude_one: f64 = latitude_one_degrees.to_radians();
    let latitude_two: f64 = latitude_two_degrees.to_radians();

    let delta_latitude = (latitude_one_degrees - latitude_two_degrees).to_radians();
    let delta_longitude = (location_one.coordinate.longitude - location_two.coordinate.longitude).to_radians();

    let central_angle_inner = (delta_latitude / 2.0).sin().powi(2) + latitude_one.cos() * latitude_two.cos() * (delta_longitude/2.0).sin().powi(2);
    central_angle_inner
}


#[derive(Debug)]
struct Location {
    name: String,
    coordinate: Coordinate
}

// Note that to return you shouldn't have the semicolon
impl Location {
    fn description(&self) -> String {
        format!("Location {} has coordinates of {}", self.name, self.coordinate.description())
    }
    
    fn new(name: String, latitude: f64, longitude: f64) -> Self {
        Self {
            name: name,
            coordinate: Coordinate::new(latitude, longitude)
        }
    }
}
#[derive(Debug)]
struct Coordinate {
    latitude: f64,
    longitude: f64
}

// Implement functionality for Coordinate 
// - here a convenience initializer(?) is that the right term
impl Coordinate {
    fn new(latitude: f64, longitude: f64) -> Self {
        Self {
            latitude, longitude
        }
    }

    // simple debug description - note that the &self is "borrowing" itself to avoid ownership
    // ? - do they get retain cycles in rust? hmmm - probably not since the compiler is so strict
    fn description(&self) -> String {
        format!("Latitude: {}, Longitude: {}", self.latitude, self.longitude)
    }
}

