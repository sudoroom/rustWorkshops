
// use std::io;

const EARTH_RADIUS_KILOMETER: f64 = 6371.0_f64;

fn main() {
    online_sample();
 
    let distance_paris_sudoroom = calculate_distance((835004_f64, -122.2641282_f64), (48.85341_f64, -2.34880_f64));
    println!("The distance from paris to sudoroom is {} kilometers", distance_paris_sudoroom);

    println!("What is the name of the first location?");

    let sudo_coordinate = Coordinate::new(835004_f64, -122.2641282_f64);

    println!("{:?}", sudo_coordinate);
    println!("{}", sudo_coordinate.description());
    let location_sudoroom = Location { 
        name: "SudoRoom".to_string(), 
        coordinate: Coordinate { 
            latitude: 835004_f64, 
            longitude: -122.2641282_f64
        }
    };
    // (48.85341_f64, -2.34880_f64)
    let location_paris = Location { 
        name: "Paris".to_string(), 
        coordinate: Coordinate { 
            latitude: 48.85341_f64, 
            longitude: -2.34880_f64
        }
    };

    println!("the location is {:?}", location_sudoroom);

    let second_distance = calculate_distance_earth(&location_sudoroom, &location_paris);
    println!("The distance between {:?} and {:?} is {:1} kilometers", 
        location_sudoroom.name, 
        location_paris.name, 
        second_distance);
 }

fn calculate_distance_earth(location_one: &Location, location_two: &Location) -> f64 {
    let place_one = (location_one.coordinate.latitude, location_one.coordinate.longitude);
    let place_two = (location_two.coordinate.latitude, location_two.coordinate.longitude);

    let distance = calculate_distance(place_one, place_two);
    distance
}
 
 #[derive(Debug)]
struct Location {
    name: String,
    coordinate: Coordinate
}
#[derive(Debug)]
struct Coordinate {
    latitude: f64, 
    longitude: f64,
}

impl Coordinate {
    fn new(latitude: f64, longitude: f64) -> Self {
        Self { latitude, longitude }
    }

    fn description(&self) -> String {
        format!("Latitude: {}, Longitude: {}", self.latitude, self.longitude)
    }
}


// destructuring - fn calculate_distance((latitude1, longitude1): (f64, f64), (latitude2, longitude2): (f64, f64)) -> f64 {
fn calculate_distance(location1: (f64, f64), location2: (f64, f64)) -> f64 {
    // 
    let (latitude1_degrees, longitude1_degrees) = location1;
    let (latitude2_degrees, longitude2_degrees) = location2;

    let latitude1 = latitude1_degrees.to_radians();
    let latitude2 = latitude2_degrees.to_radians();

    let delta_latitude = (latitude1_degrees - latitude2_degrees).to_radians();
    let delta_longitude = (longitude1_degrees - longitude2_degrees).to_radians();

    let central_angle_inner = (delta_latitude / 2.0).sin().powi(2) + 
        latitude1.cos() * latitude2.cos() * (delta_longitude/2.0).sin().powi(2);

    let central_angle = 2.0 * central_angle_inner.sqrt().asin();

    let distance = EARTH_RADIUS_KILOMETER * central_angle;
    distance
}

fn online_sample() {
    let earth_radius_kilometer = 6371.0_f64;

    let (paris_latitude_degrees, paris_longitude_degrees) = (48.85341_f64, -2.34880_f64);

    // 835004,-122.2641282,15z
    let (sudoroom_latitude_degrees, sudoroom_longitude_degrees) = (835004_f64, -122.2641282_f64);
    
    let paris_latitude = paris_latitude_degrees.to_radians();
    // let paris_longitude = paris_longitude_degrees.to_radians();

    let sudoroom_latitude = sudoroom_latitude_degrees.to_radians();
    // let sudoroom_longitude = sudoroom_longitude_degrees.to_radians();

    let delta_latitude = (paris_latitude_degrees - sudoroom_latitude_degrees).to_radians();
    let delta_longitude = (paris_longitude_degrees - sudoroom_longitude_degrees).to_radians();

    let central_angle_inner = (delta_latitude / 2.0).sin().powi(2) +
        paris_latitude.cos() * sudoroom_latitude.cos() * (delta_longitude/2.0).sin().powi(2);

    let central_angle = 2.0 * central_angle_inner.sqrt().asin();

    let distance = earth_radius_kilometer * central_angle;

    println!(
        "Distance between sudoroom and Paris on the surface of the earth is {:.1} km",
        distance
    );
}