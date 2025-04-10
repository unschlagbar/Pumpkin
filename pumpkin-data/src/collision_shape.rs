use pumpkin_util::math::{boundingbox::BoundingBox, position::BlockPos, vector3::Vector3};

use crate::block_properties::Axis;

#[derive(Clone, Copy, Debug)]
pub struct CollisionShape {
    pub min: Vector3<f64>,
    pub max: Vector3<f64>,
}

impl CollisionShape {
    pub fn is_empty() -> bool {
        unimplemented!()
    }

    pub fn intersects(&self, bounding_box: &BoundingBox) -> bool {
        let box_min = bounding_box.min;
        let box_max = bounding_box.max;

        !(self.max.x < box_min.x
            || self.min.x > box_max.x
            || self.max.y < box_min.y
            || self.min.y > box_max.y
            || self.max.z < box_min.z
            || self.min.z > box_max.z)
    }

    pub fn add_pos(&self, pos: BlockPos) -> Self {
        CollisionShape {
            min: self.min + pos.to_f64(),
            max: self.max + pos.to_f64(),
        }
    }

    pub fn calculate_max_distance(
        &self,
        axis: Axis,
        bounding_box: &BoundingBox,
        max_dist: f64,
    ) -> f64 {
        let cycle = AxisCycle::between(axis, Axis::X).opposite();
        let lv2 = cycle.cycle(Axis::X);
        let lv3 = cycle.cycle(Axis::Y);
        let lv4 = cycle.cycle(Axis::Z);

        let (e, f) = match lv2 {
            Axis::X => (bounding_box.min.x, bounding_box.max.x),
            Axis::Y => (bounding_box.min.y, bounding_box.max.y),
            Axis::Z => (bounding_box.min.z, bounding_box.max.z),
        };

        0.0
    }
}

// Entspricht AxisCycleDirection
#[derive(Debug, Clone, Copy)]
pub enum AxisCycle {
    None,
    Forward,
    Backward,
}

impl AxisCycle {
    fn opposite(&self) -> Self {
        match self {
            Self::None => Self::None,
            Self::Forward => Self::Backward,
            Self::Backward => Self::Forward,
        }
    }

    fn cycle(&self, axis: Axis) -> Axis {
        match self {
            Self::None => axis,
            Self::Forward => match axis {
                Axis::X => Axis::Z,
                Axis::Y => Axis::X,
                Axis::Z => Axis::Y,
            },
            Self::Backward => match axis {
                Axis::X => Axis::Y,
                Axis::Y => Axis::Z,
                Axis::Z => Axis::X,
            },
        }
    }

    fn between(from: Axis, to: Axis) -> Self {
        match (from, to) {
            (Axis::X, Axis::X) | (Axis::Y, Axis::Y) | (Axis::Z, Axis::Z) => Self::None,
            (Axis::X, Axis::Z) | (Axis::Y, Axis::X) | (Axis::Z, Axis::Y) => Self::Forward,
            (Axis::X, Axis::Y) | (Axis::Y, Axis::Z) | (Axis::Z, Axis::X) => Self::Backward,
        }
    }
}
