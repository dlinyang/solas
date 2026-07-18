use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader};
use super::Mesh;

impl Mesh {
    pub fn load_obj<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let file = File::open(&path).map_err(|e| format!("Failed to open file: {}", e))?;
        let reader = BufReader::new(file);
        let mut mesh = Mesh::new();

        // // Temporary storage for raw vertex data
        // let mut raw_vertices: Vec<[f32; 3]> = Vec::new();
        // let mut raw_normals: Vec<[f32; 3]> = Vec::new();
        // let mut raw_texcoords: Vec<[f32; 2]> = Vec::new();

        // // Store face indices as (vertex, texcoord, normal) tuples
        // let mut face_indices: Vec<(u32, Option<u32>, Option<u32>)> = Vec::new();

        for (line_num, line) in reader.lines().enumerate() {
            let line = line.map_err(|e| format!("Line {}: {}", line_num + 1, e))?;
            let line = line.trim().to_string();

            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.is_empty() {
                continue;
            }

            match parts[0] {
                "v" => {
                    if parts.len() < 4 {
                        return Err(format!("Line {}: Invalid vertex definition", line_num + 1));
                    }
                    let x = parts[1].parse::<f32>()
                        .map_err(|e| format!("Line {}: Invalid vertex x: {}", line_num + 1, e))?;
                    let y = parts[2].parse::<f32>()
                        .map_err(|e| format!("Line {}: Invalid vertex y: {}", line_num + 1, e))?;
                    let z = parts[3].parse::<f32>()
                        .map_err(|e| format!("Line {}: Invalid vertex z: {}", line_num + 1, e))?;
                    mesh.vertices.push([x, y, z].into());
                }
                "vn" => {
                    if parts.len() < 4 {
                        return Err(format!("Line {}: Invalid normal definition", line_num + 1));
                    }
                    let x = parts[1].parse::<f32>()
                        .map_err(|e| format!("Line {}: Invalid normal x: {}", line_num + 1, e))?;
                    let y = parts[2].parse::<f32>()
                        .map_err(|e| format!("Line {}: Invalid normal y: {}", line_num + 1, e))?;
                    let z = parts[3].parse::<f32>()
                        .map_err(|e| format!("Line {}: Invalid normal z: {}", line_num + 1, e))?;
                    mesh.normals.push([x, y, z].into());
                }
                "vt" => {
                    if parts.len() < 3 {
                        return Err(format!("Line {}: Invalid texcoord definition", line_num + 1));
                    }
                    let u = parts[1].parse::<f32>()
                        .map_err(|e| format!("Line {}: Invalid texcoord u: {}", line_num + 1, e))?;
                    let v = parts[2].parse::<f32>()
                        .map_err(|e| format!("Line {}: Invalid texcoord v: {}", line_num + 1, e))?;
                    mesh.texcoords.push([u, v].into());
                }
                "f" => {
                    if parts.len() < 4 {
                        return Err(format!("Line {}: Face requires at least 3 vertices", line_num + 1));
                    }

                    let mut face_verts: Vec<(usize, usize, usize)> = Vec::new();

                    for part in &parts[1..] {
                        let vert_parts: Vec<&str> = part.split('/').collect();
                        if vert_parts.len() != 3 {
                            return  Err(format!("line {}: Wrong Parameter number", line_num));
                        }
                        let v_idx = vert_parts[0].parse::<usize>()
                            .map_err(|e| format!("Line {}: Invalid vertex index: {}", line_num + 1, e))?;

                        let vt_idx = vert_parts[1].parse::<usize>()
                                .map_err(|e| format!("Line {}: Invalid texcoord index: {}", line_num + 1, e))?;

                        let vn_idx = vert_parts[2].parse::<usize>()
                                .map_err(|e| format!("Line {}: Invalid normal index: {}", line_num + 1, e))?;

                        face_verts.push((v_idx - 1, vn_idx - 1, vt_idx - 1));
                    }

                    if face_verts.len()  < 3 {
                        return  Err(format!("line {}: wrong face point",line_num));
                    }

                    for i in 0..face_verts.len() - 2 {
                        let (av, avn, avt) = face_verts[i];
                        let (bv, bvn, bvt) = face_verts[i+1];
                        let (cv, cvn, cvt) = face_verts[i+2];

                        mesh.faces.push([av, bv, cv, avn, bvn, cvn, avt, bvt, cvt]);
                    }


                }
                _ => {} // Ignore other lines (usemtl, mtllib, etc.)
            }
        }

        if mesh.faces.len() > 1000 {
            mesh.build_bvh();
        }

        Ok(mesh)
    }
}
