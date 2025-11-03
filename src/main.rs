use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PeoplePlugin)
        .add_systems(Startup, add_people)
        .add_systems(Update, (change_names, whos_here).chain())
        .run();
}


pub struct PeoplePlugin;

impl Plugin for PeoplePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, add_people);
        app.add_systems(Update, (change_names, whos_here).chain());
    }
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

fn add_people(mut commands: Commands) {
    commands.spawn((Person, Name("Adam".to_string())));
    commands.spawn((Person, Name("Bella".to_string())));
    commands.spawn((Person, Name("Claire".to_string())));
}

fn whos_here(query: Query<&Name, With<Person>>) {
    for name in &query {
        println!("{} is here!", name.0);
    }
}

fn change_names(mut query: Query<&mut Name, With<Person>>) {
    for mut name in &mut query {
        if name.0 == "Adam" {
            name.0 = "Alan".to_string();
            break;
        }
    }
}
