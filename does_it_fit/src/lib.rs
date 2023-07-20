pub mod areas_volumes;
pub use crate::areas_volumes::*;
pub fn area_fit(
	x: usize,
	y: usize,
	objects: areas_volumes::GeometricalShapes,
	times: usize,
	a: usize,
	b: usize,
) -> bool {
    let area = match objects {
        GeometricalShapes::Circle => circle_area(a) as usize,
        GeometricalShapes::Rectangle => rectangle_area(a, b),
        GeometricalShapes::Square => square_area(a),
        GeometricalShapes::Triangle => triangle_area(a, b) as usize
    };
    area * times < x * y
}
pub fn volume_fit(
	x: usize,
	y: usize,
	z: usize,
	objects: areas_volumes::GeometricalVolumes,
	times: usize,
	a: usize,
	b: usize,
	c: usize,
) -> bool {
    let volume = match objects {
		GeometricalVolumes::Cube => cube_volume(a),
		GeometricalVolumes::Sphere => sphere_volume(a) as usize,
		GeometricalVolumes::Cone => cone_volume(a,b) as usize,
		GeometricalVolumes::Pyramid => triangular_pyramid_volume(triangle_area(a, b), c) as usize ,
		GeometricalVolumes::Parallelepiped => parallelepiped_volume(a , b , c) as usize,
	};
	parallelepiped_volume(x, y, z) > volume * times
}