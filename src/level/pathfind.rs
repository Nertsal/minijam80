use super::*;

impl Level {
    pub fn pathfind(&self, from: Vec2<i32>, to: Vec2<i32>, max_distance: i32) -> Option<Vec2<i32>> {
        let mut queue = std::collections::VecDeque::new();
        let mut used = HashSet::new();
        queue.push_back(to);
        used.insert(to);
        let mut max_iterations = max_distance;
        while let Some(pos) = queue.pop_front() {
            max_iterations -= 1;
            if max_iterations == 0 {
                break;
            }
            for dx in -1..=1 {
                for dy in -1..=1 {
                    let next = pos + vec2(dx, dy);
                    if next == from {
                        return Some(pos);
                    }
                    if !used.contains(&next) {
                        let entity = self.get_entity(next);
                        if entity.is_none() {
                            used.insert(next);
                            queue.push_back(next);
                        }
                    }
                }
            }
        }
        None
    }
    pub fn can_see(
        &self,
        entity: &Entity,
        other: &Entity,
        view_distance: i32,
    ) -> Option<Vec2<i32>> {
        let distance = entity.distance(other);
        if distance > view_distance {
            return None;
        }
        let delta = (other.position - entity.position).map(|x| x as f32);
        let origin = entity.position.map(|x| x as f32);
        let mut direction = if distance == 1 {
            Some(other.position - entity.position)
        } else {
            None
        };
        for i in 1..distance {
            let check_pos = origin + delta * i as f32 / distance as f32;
            let mut empty = false;
            for check_tile in Self::get_tiles(check_pos, delta) {
                if self.is_empty(check_tile) {
                    empty = true;
                    if i == 1 {
                        direction = Some(check_tile - entity.position);
                    }
                    break;
                }
            }
            if !empty {
                return None;
            }
        }
        direction
    }
    fn get_tiles(position: Vec2<f32>, delta: Vec2<f32>) -> Vec<Vec2<i32>> {
        let mut check_tiles = Vec::new();
        if position.x.fract().abs() == 0.5 && position.y.fract().abs() == 0.5 {
            if delta.x * delta.y > 0.0 {
                check_tiles.push(vec2(position.x.ceil() as i32, position.y.floor() as i32));
                check_tiles.push(vec2(position.x.floor() as i32, position.y.ceil() as i32));
            } else {
                check_tiles.push(position.map(|x| x.ceil() as i32));
                check_tiles.push(position.map(|x| x.floor() as i32));
            }
        } else {
            check_tiles.push(position.map(|x| x.round() as i32));
        }
        check_tiles
    }
}
