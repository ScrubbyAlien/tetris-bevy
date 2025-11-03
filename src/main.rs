mod people;

use bevy::prelude::*;
use people::PeoplePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PeoplePlugin)
        .run();
}


