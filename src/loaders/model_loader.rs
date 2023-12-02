use nalgebra::{Vector3, Vector2};
use obj::ObjResult;
use obj::raw::{self, parse_mtl, parse_obj, RawObj};
use std::path::Path;
use std::{fs::File, collections::HashMap, sync::Arc};
use std::io::BufReader;
use crate::material::Material;
use crate::entity::triangle::Triangle;
use crate::textures::extensions_f32::file_to_texture;
use crate::textures::texture::TextureSamplingMode;

#[inline]
pub fn load_model(path: &str) -> Vec<Triangle> {
    let file = File::open(path).unwrap_or_else(|_|
        panic!("Failed to load obj file \"{}\" specified in scene. Reason: Not found", &path)
    );
    let input = BufReader::new(file);
    let model = parse_obj(input).unwrap();
    
    let materials: HashMap<String, Arc<Material>> = load_materials(&model.material_libraries, path).unwrap_or_else(|x| {
        println!("Can't load materials, loading default materials. Reason: {}", x);
        model.meshes.keys().map(|mesh_name| {
            let albedo: Vector3<f32> = match raw::material::MtlColor::Rgb(0.8, 0.8, 0.8) {
                raw::material::MtlColor::Rgb(r, g, b) => Vector3::new(r, g, b),
                raw::material::MtlColor::Xyz(_, _, _) => Vector3::new(0.8, 0.8, 0.8),
                raw::material::MtlColor::Spectral(_, _) => Vector3::new(0.8, 0.8, 0.8),
            };
            let emission: Vector3<f32> = match raw::material::MtlColor::Rgb(0.0, 0.0, 0.0) {
                raw::material::MtlColor::Rgb(r, g, b) => Vector3::new(r, g, b),
                raw::material::MtlColor::Xyz(_, _, _) => Vector3::new(0.0, 0.0, 0.0),
                raw::material::MtlColor::Spectral(_, _) => Vector3::new(0.0, 0.0, 0.0),
            };
            let material = Arc::new(Material::new(
                albedo,
                emission,
                1.0,
                0.0,
                None
            ));
            (mesh_name.clone(), material)
        }).collect()
    });

    load_meshes(&model, &materials)
}

#[inline]
fn load_meshes(model: &RawObj, materials: &HashMap<String, Arc<Material>>) -> Vec<Triangle> {
    let mut triangles = vec![];
    model.meshes.iter().for_each(|(mesh_name, mesh)| {
        mesh.polygons.iter().for_each(|pol| {
            for i in pol.start..pol.end {
                match &model.polygons[i] {
                    raw::object::Polygon::P(p) => {
                        if p.len() == 3 {
                            let v1 = model.positions[p[0]];
                            let v2 = model.positions[p[1]];
                            let v3 = model.positions[p[2]];
                            let v1: Vector3<f32> = Vector3::new(v1.0, v1.1, -v1.2);
                            let v2: Vector3<f32> = Vector3::new(v2.0, v2.1, -v2.2);
                            let v3: Vector3<f32> = Vector3::new(v3.0, v3.1, -v3.2);

                            // Calculate the normal vector of the triangle (cross product of two edges)
                            let edge1: Vector3<f32> = v2 - v1;
                            let edge2: Vector3<f32> = v3 - v1;
                            let n: Vector3<f32> = edge1.cross(&edge2).normalize();
                            // For some reason Z coordinate is negative, so just reverse it
                            let triangle = Triangle::new(
                                v1, v2, v3,
                                n, n, n,
                                Vector2::zeros(),
                                Vector2::zeros(),
                                Vector2::zeros(),
                                materials[mesh_name].clone()
                            );
                            triangles.push(triangle);
                        } else {
                            panic!("Mesh must be triangulated, native triangulation is not yet implemented");
                        }
                    },
                    raw::object::Polygon::PT(p) => {
                        if p.len() == 3 {
                            let v1 = model.positions[p[0].0];
                            let v2 = model.positions[p[1].0];
                            let v3 = model.positions[p[2].0];
                            let v1: Vector3<f32> = Vector3::new(v1.0, v1.1, -v1.2);
                            let v2: Vector3<f32> = Vector3::new(v2.0, v2.1, -v2.2);
                            let v3: Vector3<f32> = Vector3::new(v3.0, v3.1, -v3.2);

                            // Calculate the normal vector of the triangle (cross product of two edges)
                            let edge1: Vector3<f32> = v2 - v1;
                            let edge2: Vector3<f32> = v3 - v1;
                            let n: Vector3<f32> = edge1.cross(&edge2).normalize();

                            let u1 = model.tex_coords[p[0].1];
                            let u2 = model.tex_coords[p[1].1];
                            let u3 = model.tex_coords[p[2].1];

                            // For some reason Z coordinate is negative, so just reverse it
                            let triangle = Triangle::new(
                                v1, v2, v3,
                                n, n, n,
                                Vector2::new(u1.0, u1.1),
                                Vector2::new(u2.0, u2.1),
                                Vector2::new(u3.0, u3.1),
                                materials[mesh_name].clone()
                            );
                            triangles.push(triangle);
                        } else {
                            panic!("Mesh must be triangulated, native triangulation is not yet implemented");
                        }
                    },
                    raw::object::Polygon::PN(p) => {
                        if p.len() == 3 {
                            let v1 = model.positions[p[0].0];
                            let v2 = model.positions[p[1].0];
                            let v3 = model.positions[p[2].0];
                            let v1: Vector3<f32> = Vector3::new(v1.0, v1.1, -v1.2);
                            let v2: Vector3<f32> = Vector3::new(v2.0, v2.1, -v2.2);
                            let v3: Vector3<f32> = Vector3::new(v3.0, v3.1, -v3.2);

                            let n1 = model.normals[p[0].1];
                            let n2 = model.normals[p[1].1];
                            let n3 = model.normals[p[2].1];
                            // For some reason Z coordinate is negative, so just reverse it
                            let triangle = Triangle::new(
                                v1, v2, v3,
                                Vector3::new(n1.0, n1.1, n1.2),
                                Vector3::new(n2.0, n2.1, n2.2),
                                Vector3::new(n3.0, n3.1, n3.2),
                                Vector2::zeros(),
                                Vector2::zeros(),
                                Vector2::zeros(),
                                materials[mesh_name].clone()
                            );
                            triangles.push(triangle);
                        } else {
                            panic!("Mesh must be triangulated, native triangulation is not yet implemented");
                        }
                    },
                    raw::object::Polygon::PTN(p) => {
                        if p.len() == 3 {
                            let v1 = model.positions[p[0].0];
                            let v2 = model.positions[p[1].0];
                            let v3 = model.positions[p[2].0];
                            let v1: Vector3<f32> = Vector3::new(v1.0, v1.1, -v1.2);
                            let v2: Vector3<f32> = Vector3::new(v2.0, v2.1, -v2.2);
                            let v3: Vector3<f32> = Vector3::new(v3.0, v3.1, -v3.2);
                            
                            let n1 = model.normals[p[0].2];
                            let n2 = model.normals[p[1].2];
                            let n3 = model.normals[p[2].2];
                            
                            let u1 = model.tex_coords[p[0].1];
                            let u2 = model.tex_coords[p[1].1];
                            let u3 = model.tex_coords[p[2].1];
                            // For some reason Z coordinate is negative, so just reverse it
                            let triangle = Triangle::new(
                                v1, v2, v3,
                                Vector3::new(n1.0, n1.1, n1.2),
                                Vector3::new(n2.0, n2.1, n2.2),
                                Vector3::new(n3.0, n3.1, n3.2),
                                Vector2::new(u1.0, u1.1),
                                Vector2::new(u2.0, u2.1),
                                Vector2::new(u3.0, u3.1),
                                materials[mesh_name].clone()
                            );
                            triangles.push(triangle);
                        } else {
                            panic!("Mesh must be triangulated, native triangulation is not yet implemented");
                        }
                    },
                }
            }
        });
    });
    triangles
}

#[inline]
fn load_materials(libs: &[String], path: &str) -> ObjResult<HashMap<String, Arc<Material>>> {
    let mut materials: HashMap<String, Arc<Material>> = HashMap::new();
    let parent_path = Path::new(path).parent().unwrap_or(Path::new(""));
    
    for mtl in libs {
        let mtl_path = parent_path.join(mtl);
        let file = File::open(&mtl_path).unwrap_or_else(|_|
            panic!("Failed to load mtl file \"{:?}\" specified in \"{}\". Reason: Not found", &mtl_path, &path)
        );
        let input = BufReader::new(file);
        let mat: raw::RawMtl = parse_mtl(input)?;

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
            let albedo_map = raw_material.diffuse_map.as_ref().map(|x| {
                let map_path = parent_path.join(&x.file);
                file_to_texture(&map_path, TextureSamplingMode::Repeat)
            });
            let material = Arc::new(Material::new(
                albedo,
                emission,
                1.0,
                0.0,
                albedo_map
            ));
            (name.clone(), material)
        }));
    }
    Ok(materials)
}
