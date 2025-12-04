mod pilot;

use pilot::Pilot;

fn main() {
    let pilot: Pilot = Pilot::new(String::from("John"), 30, 100);
    pilot.display_info();
}
