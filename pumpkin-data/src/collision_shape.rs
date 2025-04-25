use pumpkin_util::math::{boundingbox::BoundingBox, position::BlockPos, vector3::Vector3};

use crate::{axis::AxisCycleDirection, block_properties::Axis};

#[derive(Clone, Copy, Debug)]
pub struct CollisionShape {
    pub min: Vector3<f64>,
    pub max: Vector3<f64>,
}

impl CollisionShape {
    pub fn is_empty() -> bool {
        unimplemented!()
    }

    pub fn intersects(&self, other: &BoundingBox) -> bool {
        self.min.x < other.max.x
            && self.max.x > other.min.x
            && self.min.y < other.max.y
            && self.max.y > other.min.y
            && self.min.z < other.max.z
            && self.max.z > other.min.z
    }

    pub fn at_pos(&self, pos: BlockPos) -> Self {
        let vec3 = Vector3 {
            x: pos.0.x as f64,
            y: pos.0.y as f64,
            z: pos.0.z as f64,
        };
        Self {
            min: self.min + vec3,
            max: self.max + vec3,
        }
    }

    pub fn calculate_max_distance(&self, axis: Axis, bounding_box: &BoundingBox, mut max_dist: f64) -> f64 {    
        let cycle_dir = AxisCycleDirection::beetween_x(axis).opposite();
        let lv2 = cycle_dir.cycle(Axis::X);
        let lv3 = cycle_dir.cycle(Axis::Y);
        let lv4 = cycle_dir.cycle(Axis::Z);

        let (f, e) = lv2.get_on_axis(bounding_box);
        let i = self.get_coord_index(lv2, f);
        let j = self.get_coord_index(lv2, e);
        let k = self.get_coord_index(lv3, lv3.get_on_axis(bounding_box).0 + 1.0E-7).max(0);
        let l = (self.get_coord_index(lv3, lv3.get_on_axis(bounding_box).1 - 1.0E-7) + 1).min(self.get_size(lv3) as i32);
        let m = self.get_coord_index(lv4, lv4.get_on_axis(bounding_box).0 + 1.0E-7).max(0);
        let n = (self.get_coord_index(lv4, lv4.get_on_axis(bounding_box).1 - 1.0E-7) + 1).min(self.get_size(lv4) as i32);
        let o = self.get_size(lv2) as i32;

        if max_dist > 0.0 {
            for p in j + 1..o {
               for q in k..l {
                  for r in m..n {
                    if self.in_bounds_and_contains(cycle_dir.choose_ordered(Vector3::new(p as f64, q as f64, r as f64))) {
                        let g = p as f64 / self.get_size(lv2) - e;
                        if g >= -1.0E-7 {
                            max_dist = max_dist.min(g);
                        }

                        return max_dist;
                    }
                  }
               }
            }
         } else if max_dist < 0.0 {
            for p in (0..=i - 1).rev() {
                for q in k..l {
                    for rx in m..n {
                        if self.in_bounds_and_contains(cycle_dir.choose_ordered(Vector3::new(p as f64, q as f64, rx as f64))) {
                            let g = (p as f64 + 1.0) / self.get_size(lv2) - f;
                            if g <= 1.0e-7 {
                                return max_dist.max(g);
                            }
                            return max_dist;
                        }
                    }
                }
            }
         }

         return max_dist;

    }

    #[inline]
    fn get_size(&self, axis: Axis) -> f64 {
        match axis {
            Axis::X => self.max.x - self.min.x,
            Axis::Y => self.max.y - self.min.y,
            Axis::Z => self.max.z - self.min.z,
        }
    }

    fn get_coord_index(&self, axis: Axis, coord: f64) -> i32 {
        let size = self.get_size(axis);
        return (coord * size).clamp(-1.0, size).floor() as i32
    }

    fn contains(&self, other: Vector3<f64>) -> bool {
        self.min.x <= other.x && self.max.x >= other.x && self.min.y <= other.y && self.max.y >= other.y && self.min.z <= other.z && self.max.z >= other.z
    }

    fn in_bounds_and_contains(&self, other: Vector3<f64>) -> bool {
        if other.x < 0.0 || other.y < 0.0 || other.z < 0.0 {
            return false;
        }
        let size = self.max.sub(&self.min);

        if other.x < size.x && other.y < size.y && other.z < size.z {
            return self.contains(other);
        }

        return false;
    }
}
