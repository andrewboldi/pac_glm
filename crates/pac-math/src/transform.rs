use glam::{Mat4, Quat, Vec3};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Transform {
    pub position: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl Transform {
    pub const IDENTITY: Transform = Transform {
        position: Vec3::ZERO,
        rotation: Quat::IDENTITY,
        scale: Vec3::ONE,
    };

    pub fn new(position: Vec3, rotation: Quat, scale: Vec3) -> Self {
        Self {
            position,
            rotation,
            scale,
        }
    }

    pub fn from_position(position: Vec3) -> Self {
        Self {
            position,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
        }
    }

    pub fn from_rotation(rotation: Quat) -> Self {
        Self {
            position: Vec3::ZERO,
            rotation,
            scale: Vec3::ONE,
        }
    }

    pub fn from_scale(scale: Vec3) -> Self {
        Self {
            position: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale,
        }
    }

    pub fn with_position(mut self, position: Vec3) -> Self {
        self.position = position;
        self
    }

    pub fn with_rotation(mut self, rotation: Quat) -> Self {
        self.rotation = rotation;
        self
    }

    pub fn with_scale(mut self, scale: Vec3) -> Self {
        self.scale = scale;
        self
    }

    pub fn model_matrix(&self) -> Mat4 {
        Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.position)
    }

    pub fn forward(&self) -> Vec3 {
        self.rotation * Vec3::NEG_Z
    }

    pub fn back(&self) -> Vec3 {
        self.rotation * Vec3::Z
    }

    pub fn right(&self) -> Vec3 {
        self.rotation * Vec3::X
    }

    pub fn left(&self) -> Vec3 {
        self.rotation * Vec3::NEG_X
    }

    pub fn up(&self) -> Vec3 {
        self.rotation * Vec3::Y
    }

    pub fn down(&self) -> Vec3 {
        self.rotation * Vec3::NEG_Y
    }

    pub fn translate(&mut self, translation: Vec3) {
        self.position += translation;
    }

    pub fn rotate_by(&mut self, rotation: Quat) {
        self.rotation = rotation * self.rotation;
    }

    pub fn scale_by(&mut self, scale: Vec3) {
        self.scale *= scale;
    }

    pub fn look_at(&mut self, target: Vec3, up: Vec3) {
        let look_dir = (target - self.position).normalize();
        let right = up.cross(look_dir).normalize();
        let corrected_up = look_dir.cross(right);

        let rot_matrix = Mat4::from_cols(
            right.extend(0.0),
            corrected_up.extend(0.0),
            look_dir.extend(0.0),
            Vec3::ZERO.extend(1.0),
        );
        self.rotation = Quat::from_mat4(&rot_matrix);
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self::IDENTITY
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity() {
        let t = Transform::IDENTITY;
        assert_eq!(t.position, Vec3::ZERO);
        assert_eq!(t.rotation, Quat::IDENTITY);
        assert_eq!(t.scale, Vec3::ONE);
    }

    #[test]
    fn test_model_matrix_identity() {
        let t = Transform::IDENTITY;
        assert_eq!(t.model_matrix(), Mat4::IDENTITY);
    }

    #[test]
    fn test_model_matrix_translation() {
        let t = Transform::from_position(Vec3::new(1.0, 2.0, 3.0));
        let expected = Mat4::from_translation(Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(t.model_matrix(), expected);
    }

    #[test]
    fn test_model_matrix_scale() {
        let t = Transform::from_scale(Vec3::new(2.0, 3.0, 4.0));
        let expected = Mat4::from_scale(Vec3::new(2.0, 3.0, 4.0));
        assert_eq!(t.model_matrix(), expected);
    }

    #[test]
    fn test_directions() {
        let t = Transform::from_rotation(Quat::from_rotation_y(std::f32::consts::FRAC_PI_2));

        let forward = t.forward();
        assert!(
            forward.x < -0.9,
            "forward should be approximately -X, got {:?}",
            forward
        );
    }
}
