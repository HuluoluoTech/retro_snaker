
#[derive(Debug, Default)]
pub struct Player {
    pub scores: u32,
}
impl Player {
    pub fn update(&mut self, score: u32) {
        self.scores += score;
    }
}
