use pumpkin_util::math::{boundingbox::BoundingBox, vector3::Vector3};

use crate::block_properties::Axis;

impl Axis {
    pub fn get_check_order(movement: Vector3<f64>) -> [Axis; 3] {
        if  movement.x.abs() < movement.z.abs() {
            [Axis::Y, Axis::Z, Axis::X]
        } else {
            [Axis::Y, Axis::X, Axis::Z]
        }
    }

    pub fn get_on_axis(&self, bounding_box: &BoundingBox) -> (f64, f64) {
        match self {
            Axis::X => (bounding_box.min.x, bounding_box.max.x),
            Axis::Y => (bounding_box.min.y, bounding_box.max.y),
            Axis::Z => (bounding_box.min.z, bounding_box.max.z),
        }
    }
}

pub enum AxisCycleDirection {
    None,
    Forward,
    Backwards,
}

impl AxisCycleDirection {
    pub fn opposite(&self) -> Self {
        match self {
            Self::None => Self::None,
            Self::Forward => Self::Backwards,
            Self::Backwards => Self::Forward
        }
    }

    pub fn beetween_x(axis: Axis) -> Self {
        match axis {
            Axis::X => Self::None,
            Axis::Y => Self::Backwards,
            Axis::Z => Self::Forward,
        }
    }

    pub fn cycle(&self, axis: Axis) -> Axis {
        match self {
            Self::None => axis,
            Self::Forward => match axis {
                Axis::X => Axis::Y,
                Axis::Y => Axis::Z,
                Axis::Z => Axis::X,
            },
            Self::Backwards => match axis {
                Axis::X => Axis::Z,
                Axis::Y => Axis::X,
                Axis::Z => Axis::Y,
            }
        }
    }

    pub fn choose_ordered(&self, vec: Vector3<f64>) -> Vector3<f64> {
        match self {
            AxisCycleDirection::None => vec,
            AxisCycleDirection::Forward => Vector3::new(vec.z, vec.x, vec.y),
            AxisCycleDirection::Backwards => Vector3::new(vec.y, vec.z, vec.x),
        }
    }
}