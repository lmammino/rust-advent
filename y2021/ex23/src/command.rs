#[derive(Debug)]
pub(crate) enum Command {
    In { pos_hallway: usize, home_x: usize },
    Out { home_x: usize, pos_hallway: usize },
}
