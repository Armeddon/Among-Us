use amongus;
use amongus::Command;
use amongus::Color;

fn main() {
    let mut sus = amongus::AmongUs::new();

    amongus::interpret! {
        mut sus,

        YELLOW SUS
        YELLOW SUS
        LIME SUS
        GREEN SUS
        PURPLE SUS
        LIME SUS
        LIME SUS
        BLACK SUS
    }
}
