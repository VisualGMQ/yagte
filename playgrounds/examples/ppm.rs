use graphics::ppm::PPM;
use math::{matrix::*, precision::Real};

fn main() {
    let mut ppm = PPM::new(400, 300);
    for y in 0..ppm.height() {
        for x in 0..ppm.width() {
            ppm.set_pixel(
                x,
                y,
                Vec3::from_xyz(
                    x as Real / ppm.width() as Real,
                    y as Real / ppm.height() as Real,
                    1.0,
                ),
            );
        }
    }

    ppm.write_to_file("test.ppm").unwrap();
}
