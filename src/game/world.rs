use super::Player;


#[derive(Debug, Clone)]
pub struct World {
    players: Vec<Player>,
}

impl World {
    pub fn new() -> World {
        World {
            players: Vec::new(),
        }
    }

    pub fn add_player(&mut self, player: Player) {
        self.players.push(player);
    }
}