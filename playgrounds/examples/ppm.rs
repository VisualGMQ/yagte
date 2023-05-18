use graphics::ppm::PPM;
use math::matrix::*;

fn main() {
    let mut ppm = PPM::new(400, 300);
    for y in 0..ppm.height() {
        for x in 0..ppm.width() {
            ppm.set_pixel(
                x,
                y,
                Vec3::from_xyz(
                    x as f64 / ppm.width() as f64,
                    y as f64 / ppm.height() as f64,
                    1.0,
                ),
            );
        }
    }

    ppm.write_to_file("test.ppm").unwrap();
}
