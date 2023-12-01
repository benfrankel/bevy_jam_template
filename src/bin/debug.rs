#![allow(dead_code)]

use bevy::app::MainScheduleOrder;
use bevy::ecs::schedule::ScheduleLabel;
use bevy::prelude::*;
use bevy_jam_template::AppPlugin;
use bevy_mod_debugdump::schedule_graph::Settings;
use bevy_mod_debugdump::schedule_graph_dot;

// Usage: Disable logging with RUST_LOG=off, then pipe the output into `dot`
// Example: `RUST_LOG=off cargo run --release --bin debug_dump | dot -Tsvg | feh -`
fn main() {
    let mut app = App::new();
    app.add_plugins(AppPlugin);

    let mut labels = app
        .world
        .resource::<Schedules>()
        .iter()
        .map(|(label, _)| format!("{:?}", label))
        .collect::<Vec<_>>();
    labels.sort();
    println!("All schedules: {}\n", labels.join(", "));

    let main_labels = app
        .world
        .resource::<MainScheduleOrder>()
        .labels
        .iter()
        .map(|label| format!("{:?}", label))
        .collect::<Vec<_>>();
    println!("Main schedules: {}\n", main_labels.join(", "));

    //print_schedule_graph(&mut app, Update);
    print_schedule(&mut app, Update);
}

fn print_schedule_graph(app: &mut App, label: impl ScheduleLabel) {
    let settings = Settings {
        prettify_system_names: false,
        ..default()
    };
    let dot = schedule_graph_dot(app, label, &settings);
    println!("{dot}");
}

fn print_schedule(app: &mut App, label: impl ScheduleLabel + Clone) {
    app.world
        .resource_scope::<Schedules, _>(|world, mut schedules| {
            let schedule = schedules.get_mut(label.clone()).unwrap();
            let graph = schedule.graph_mut();
            graph.initialize(world);
            graph
                .build_schedule(world.components(), label.intern(), &default())
                .unwrap();

            // List systems
            let mut systems = vec![];
            for (node_id, _, _) in graph.systems() {
                systems.push(node_id);
            }

            // Sort topologically based on dependency graph
            let mut system_order = vec![];
            for &node in graph.dependency().cached_topsort() {
                if graph.get_system_at(node).is_some() {
                    system_order.push(node);
                }
            }
            systems.sort_by_key(|&x| {
                system_order
                    .iter()
                    .position(|&y| x == y)
                    .unwrap_or(usize::MAX)
            });

            // Print systems
            for system in systems {
                let system_name = graph.system_at(system).name().to_string();
                println!("[{:?}] {}", label, system_name);
            }
        });
}
