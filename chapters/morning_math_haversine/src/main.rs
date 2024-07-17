fn main() {
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
