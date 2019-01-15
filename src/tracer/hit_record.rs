use super::{Material, Vec3};
use std::sync::Arc;

pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub normal: Vec3,
    pub material: Arc<dyn Material>,
}
