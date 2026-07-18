use super::*;

impl Mesh {
    //  create a cube
    pub fn cube(length: f32) -> Self {
        let mut vertices = Vec::new();
        let mut faces = Vec::new();
        let mut normals = Vec::new();
        let mut texcoords = Vec::new();

        let half_length = length / 2.0f32;

        /* xyz coord
           v5----v4
          / |  / |
        v1--|-v0 |
        | v7--|-v6
        | /   | /
        v3----v2 */
        vertices.push(Vec3::new(half_length, half_length, half_length));//v0
        vertices.push(Vec3::new(-half_length, half_length, half_length));//v1
        vertices.push(Vec3::new(half_length, half_length, -half_length));//v2
        vertices.push(Vec3::new(-half_length, half_length, -half_length));//v3
        vertices.push(Vec3::new(half_length, -half_length, half_length));//v4
        vertices.push(Vec3::new(-half_length, -half_length, half_length));//v5
        vertices.push(Vec3::new(half_length, -half_length, -half_length));//v6
        vertices.push(Vec3::new(-half_length, -half_length, -half_length));//v7

        /*
              t
              ^   b
              | /
        l<—— a ——> r
            /|
          f  b
        */

        normals.push(Vec3::new(0.0, 1.0, 0.0));
        normals.push(Vec3::new(0.0, -1.0, 0.0));
        normals.push(Vec3::new(0.0, 0.0, 1.0));
        normals.push(Vec3::new(0.0, 0.0, -1.0));
        normals.push(Vec3::new(-1.0, 0.0, 0.0));
        normals.push(Vec3::new(1.0, 0.0, 0.0));

        texcoords.push(Vec2::new(0.0, 0.0));

        //front
        faces.push([1, 2, 3, 0, 0, 0, 0, 0, 0]);
        faces.push([0, 1, 2, 0, 0, 0, 0, 0, 0]);

        //back
        faces.push([4, 5, 6, 1, 1, 1, 0, 0, 0]);
        faces.push([5, 6, 7, 1, 1, 1, 0, 0, 0]);

        // top
        faces.push([0, 1, 4, 2, 2, 2, 0, 0, 0]);
        faces.push([1, 4, 5, 2, 2, 2, 0, 0, 0]);

        // bottom
        faces.push([2, 3, 6, 3, 3, 3, 0, 0, 0]);
        faces.push([3, 6, 7, 3, 3, 3, 0, 0, 0]);

        // left
        faces.push([1, 3, 5, 4, 4, 4, 0, 0, 0]);
        faces.push([3, 5, 7, 4, 4, 4, 0, 0, 0]);

        // right
        faces.push([0, 2, 4, 5, 5, 5, 0, 0, 0]);
        faces.push([2, 4, 6, 5, 5, 5, 0, 0, 0]);

        Self {
            origin: Vec3::new(0.0, 0.0, 0.0),
            vertices,
            normals,
            texcoords,
            faces,
            bvh_opt: None,
        }
    }
}
