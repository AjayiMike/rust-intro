enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT
}

fn move_player(d: Direction) {
    match d {
        Direction::UP => println!("Player moved up"),
        Direction::RIGHT => println!("Player moved right"),
        Direction::DOWN => println!("Player moved down"),
        Direction::LEFT => println!("Player moved left")
    }
}
pub fn run () {
    move_player(Direction::DOWN);
    move_player(Direction::LEFT);
    move_player(Direction::UP);
    move_player(Direction::RIGHT);
}