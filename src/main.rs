fn main() {
    // Touch workspace crate wiring so this binary stays connected to pac-game.
    let _ = pac_game::math::Transform::default();
}
