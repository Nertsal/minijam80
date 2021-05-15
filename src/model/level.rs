use super::*;

#[derive(Serialize, Deserialize, Clone)]
pub struct Level {
    pub tiles: HashMap<Vec2<i32>, Tile>,
    pub creatures: Vec<Creature>,
}

impl Level {
    pub fn test() -> Self {
        Self {
            tiles: {
                let mut tiles = HashMap::new();
                for x in -10..=10 {
                    for y in -10..=10 {
                        tiles.insert(vec2(x, y), Tile::Empty);
                    }
                }
                tiles.insert(vec2(3, 0), Tile::Bush);
                tiles
            },
            creatures: vec![Creature {
                next_move: None,
                position: vec2(0, 0),
                creature_type: CreatureType::Player,
            }],
        }
    }

    pub fn get_player_mut(&mut self) -> Option<&mut Creature> {
        if let Some(i) = self
            .creatures
            .iter()
            .enumerate()
            .find(|(_, creature)| {
                if let CreatureType::Player = creature.creature_type {
                    true
                } else {
                    false
                }
            })
            .map(|(i, _)| i)
        {
            self.creatures.get_mut(i)
        } else {
            None
        }
    }

    pub fn make_move(&mut self) {
        for creature in &mut self.creatures {
            if let Some(next_move) = &creature.next_move {
                let direction = match next_move {
                    Move::Up => vec2(0, 1),
                    Move::Down => vec2(0, -1),
                    Move::Right => vec2(1, 0),
                    Move::Left => vec2(-1, 0),
                };
                let next_pos = creature.position + direction;
                if let Some(tile) = self.tiles.get(&next_pos) {
                    match tile {
                        Tile::Empty => creature.position = next_pos,
                        Tile::Bush => (),
                    }
                }
            }
        }
    }
}
