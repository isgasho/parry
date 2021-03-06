use crate::bounding_volume::AABB;
use crate::math::{Isometry, Real};
use crate::shape::ConvexPolygon;

impl ConvexPolygon {
    /// Computes the world-space AABB of this convex polygon, transformed by `pos`.
    #[inline]
    pub fn aabb(&self, pos: &Isometry<Real>) -> AABB {
        super::details::point_cloud_aabb(pos, self.points())
    }

    /// Computes the local-space AABB of this convex polygon.
    #[inline]
    pub fn local_aabb(&self) -> AABB {
        super::details::local_point_cloud_aabb(self.points())
    }
}
