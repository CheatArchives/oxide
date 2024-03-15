use super::Cheat;

#[derive(Debug)]
pub struct Visuals {}

impl Visuals {
    pub fn init() -> Visuals {
        Visuals {}
    }
    pub fn name() -> &'static str {
        "Visuals"
    }
}
impl Cheat for Visuals {
    fn handle_event(&mut self, _: &mut crate::draw::event::Event) {
    }
}
