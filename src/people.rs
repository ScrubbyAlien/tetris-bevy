use bevy::prelude::*;

pub struct PeoplePlugin;

impl Plugin for PeoplePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(WhosHereTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));
        app.add_systems(Startup, add_people);
        app.add_systems(Update, (change_names, whos_here).chain());
    }
}

#[derive(Component)]
struct Person;

#[derive(Component)]
struct Name(String);

#[derive(Resource)]
struct WhosHereTimer(Timer);

fn add_people(mut commands: Commands) {
    commands.spawn(create_person("Adam"));
    commands.spawn(create_person("Bella"));
    commands.spawn(create_person("Claire"));
}

fn create_person(name: &str) -> (Person, Name) {
    (Person, Name(name.to_string()))
}

fn whos_here(time: Res<Time>, mut timer: ResMut<WhosHereTimer>, query: Query<&Name, With<Person>>) {
    if timer.0.tick(time.delta()).just_finished() {
        for name in &query {
            println!("{} is here!", name.0);
        }
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
