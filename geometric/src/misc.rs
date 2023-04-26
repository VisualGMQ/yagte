use math::matrix::Vec3;
use crate::utilitiy::approx_equal;

pub fn get_arbitrary_from_plane(normal: &Vec3, pt: &Vec3) -> Vec3 {
    let x = pt.x() + 1.0;    
    let y = pt.y() + 1.0;    
    let s = normal.dot(&pt);
    if !approx_equal(0.0, normal.z(), 4) {
        let z = (s - (normal.x() * x + normal.y() * y)) / normal.z();
        Vec3::from_xyz(x, y, z)
    } else if !approx_equal(0.0, normal.y(), 4) {
        Vec3::from_xyz(x, y, 0.0)
    } else {
        Vec3::from_xyz(0.0, y, pt.z() + 1.0)
    }
}
