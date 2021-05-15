use super::*;

mod creature;
mod tile;

pub use creature::*;
pub use tile::*;

pub struct Model {
    pub tiles: HashMap<Vec2<i32>, Tile>,
    pub creatures: Vec<Creature>,
}

impl Model {
    pub fn new() -> Self {
        Self {
            tiles: {
                let mut tiles = HashMap::new();
                for x in -10..=10 {
                    for y in -10..=10 {
                        tiles.insert(vec2(x, y), Tile::Empty);
                    }
                }
                tiles
            },
            creatures: vec![Creature {
                next_move: None,
                position: vec2(0, 0),
                creature_type: CreatureType::Player,
            }],
        }
    }

    pub fn update(&mut self, delta_time: f32) {}

    pub fn handle_event(&mut self, event: &geng::Event) {
        match event {
            geng::Event::KeyDown { key } => {
                if let Some(player_move) = match key {
                    geng::Key::Right => Some(Move::Right),
                    geng::Key::Left => Some(Move::Left),
                    geng::Key::Up => Some(Move::Up),
                    geng::Key::Down => Some(Move::Down),
                    _ => None,
                } {
                    let player = self.get_player_mut().unwrap();
                    player.next_move = Some(player_move);
                    self.make_move();
                }
            }
            _ => (),
        }
    }

    fn get_player_mut(&mut self) -> Option<&mut Creature> {
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

    fn make_move(&mut self) {
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
