//!
//! Shape composed from the union of primitives.
//!

use crate::bounding_volume::{BoundingVolume, AABB};
use crate::math::{Isometry, Real};
use crate::partitioning::SimdQuadTree;
use crate::shape::{Shape, SimdCompositeShape};
use std::sync::Arc;

/// A compound shape with an aabb bounding volume.
///
/// A compound shape is a shape composed of the union of several simpler shape. This is
/// the main way of creating a concave shape from convex parts. Each parts can have its own
/// delta transformation to shift or rotate it with regard to the other shapes.
pub struct Compound {
    shapes: Vec<(Isometry<Real>, Arc<dyn Shape>)>,
    quadtree: SimdQuadTree<u32>,
    aabbs: Vec<AABB>,
    aabb: AABB,
}

impl Compound {
    /// Builds a new compound shape.
    ///
    /// Panics if the input vector is empty, of if some of the provided shapes
    /// are also composite shapes (nested composite shapes are not allowed).
    pub fn new(shapes: Vec<(Isometry<Real>, Arc<dyn Shape>)>) -> Compound {
        assert!(
            !shapes.is_empty(),
            "A compound shape must contain at least one shape."
        );
        let mut aabbs = Vec::new();
        let mut leaves = Vec::new();
        let mut aabb = AABB::new_invalid();

        for (i, &(ref delta, ref shape)) in shapes.iter().enumerate() {
            let bv = shape.compute_aabb(delta);

            aabb.merge(&bv);
            aabbs.push(bv.clone());
            leaves.push((i as u32, bv));

            if shape.as_composite_shape().is_some() {
                panic!("Nested composite shapes are not allowed.");
            }
        }

        let mut quadtree = SimdQuadTree::new();
        // NOTE: we apply no dilation factor because we won't
        // update this tree dynamically.
        quadtree.clear_and_rebuild(leaves.into_iter(), 0.0);

        Compound {
            shapes,
            quadtree,
            aabbs,
            aabb,
        }
    }
}

impl Compound {
    /// The shapes of this compound shape.
    #[inline]
    pub fn shapes(&self) -> &[(Isometry<Real>, Arc<dyn Shape>)] {
        &self.shapes[..]
    }

    /// The AABB of this compound in its local-space.
    #[inline]
    pub fn aabb(&self) -> &AABB {
        &self.aabb
    }

    /// The shapes AABBs.
    #[inline]
    pub fn aabbs(&self) -> &[AABB] {
        &self.aabbs[..]
    }
}

impl SimdCompositeShape for Compound {
    #[inline]
    fn nparts(&self) -> usize {
        self.shapes.len()
    }

    #[inline]
    fn map_part_at(&self, shape_id: u32, f: &mut dyn FnMut(Option<&Isometry<Real>>, &dyn Shape)) {
        if let Some(shape) = self.shapes.get(shape_id as usize) {
            f(Some(&shape.0), &*shape.1)
        }
    }

    #[inline]
    fn quadtree(&self) -> &SimdQuadTree<u32> {
        &self.quadtree
    }
}