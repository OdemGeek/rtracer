use nalgebra::Vector3;
use obj::raw::{self, parse_mtl, parse_obj};
use std::{fs::File, collections::HashMap, sync::Arc};
use std::io::BufReader;
use crate::material::Material;
use crate::entity::triangle::Triangle;

pub fn load_model(path: &str) -> Vec<Triangle> {
    let file = File::open(path).unwrap_or_else(|_|
        panic!("Failed to load obj file \"{}\" specified in scene. Reason: Not found", &path)
    );
    let input = BufReader::new(file);
    let model = parse_obj(input).unwrap();
    
    let mut materials: HashMap<String, Arc<Material>> = HashMap::new();
    for mtl in model.material_libraries {
        let file = File::open(&mtl).unwrap_or_else(|_|
            panic!("Failed to load mtl file \"{}\" specified in \"{}\". Reason: Not found", &mtl, &path)
        );
        let input = BufReader::new(file);
        let mat = parse_mtl(input).unwrap();

        materials.extend(mat.materials.iter().map(|(name, raw_material)| {
            let albedo: Vector3<f32> = match raw_material.diffuse.clone().unwrap_or(raw::material::MtlColor::Rgb(0.8, 0.8, 0.8)) {
                raw::material::MtlColor::Rgb(r, g, b) => Vector3::new(r, g, b),
                raw::material::MtlColor::Xyz(_, _, _) => Vector3::new(0.8, 0.8, 0.8),
                raw::material::MtlColor::Spectral(_, _) => Vector3::new(0.8, 0.8, 0.8),
            };
            let emission: Vector3<f32> = match raw_material.emissive.clone().unwrap_or(raw::material::MtlColor::Rgb(0.0, 0.0, 0.0)) {
                raw::material::MtlColor::Rgb(r, g, b) => Vector3::new(r, g, b),
                raw::material::MtlColor::Xyz(_, _, _) => Vector3::new(0.0, 0.0, 0.0),
                raw::material::MtlColor::Spectral(_, _) => Vector3::new(0.0, 0.0, 0.0),
            };
            let material = Arc::new(Material::new(
                albedo,
                emission,
                1.0,
                0.0
            ));
            (name.clone(), material)
        }));
    }

    let mut triangles = vec![];
    model.meshes.iter().for_each(|(mesh_name, mesh)| {
        mesh.polygons.iter().for_each(|pol| {
            for i in pol.start..pol.end {
                match &model.polygons[i] {
                    raw::object::Polygon::P(_) => panic!("Position is not yet implemented"),
                    raw::object::Polygon::PT(_) => panic!("Position + Texture Coordinate is not yet implemented"),
                    raw::object::Polygon::PN(p) => {
                        if p.len() == 3 {
                            let v1 = model.positions[p[0].0];
                            let v2 = model.positions[p[1].0];
                            let v3 = model.positions[p[2].0];

                            let triangle = Triangle::new(
                                Vector3::new(v1.0, v1.1, v1.2),
                                Vector3::new(v2.0, v2.1, v2.2),
                                Vector3::new(v3.0, v3.1, v3.2),
                                materials[mesh_name].clone()
                            );
                            triangles.push(triangle);
                        } else {
                            panic!("Mesh must be triangulated, native triangulation is not yet implemented");
                        }
                    },
                    raw::object::Polygon::PTN(_) => panic!("Position + Texture Coordinate + Normal is not yet implemented"),
                }
            }
        });
    });
    triangles
}