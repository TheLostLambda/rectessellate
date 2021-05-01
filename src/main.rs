mod resize;

use std::collections::HashMap;

use bevy::{prelude::*, render::camera::Camera, window::WindowResized};
use resize::resize_horizontally;

const WIDTH: f32 = 900.0 + 2.0 * GAP;
const HEIGHT: f32 = 900.0 + 2.0 * GAP;
const GAP: f32 = 10.0;

#[derive(Debug, Clone, Bundle)]
pub struct Pane {
    id: u32,
    pos: (f32, f32),
    size: (f32, f32),
    flex: bool,
}

fn main() {
    App::build()
        .insert_resource(WindowDescriptor {
            title: "Layout Solver".to_string(),
            width: WIDTH,
            height: HEIGHT,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_system(resize_handler.system())
        .add_system(draw_panes.system())
        .run();
}

fn resize_handler(
    mut events: EventReader<WindowResized>,
    mut cameras: Query<(&Camera, &mut Transform)>,
    mut query: Query<&mut Pane>,
) {
    for event in events.iter() {
        // Update camera position
        for (_camera, mut transform) in cameras.iter_mut() {
            transform.translation.x = event.width / 2.0;
            transform.translation.y = -event.height / 2.0;
        }
        let panes: Vec<_> = query.iter_mut().map(|p| p.to_owned()).collect();
        let mut mapping = HashMap::new();
        for pane in resize_horizontally(event.width, &panes) {
            mapping.insert(pane.id, pane);
        }
        for mut pane in query.iter_mut() {
            *pane = mapping.remove(&pane.id).unwrap();
        }
    }
}

fn draw_panes(mut query: Query<(&mut Transform, &mut Sprite, &Pane), Changed<Pane>>) {
    for (mut transform, mut sprite, pane) in query.iter_mut() {
        let (x, y) = pane.pos;
        let (w, h) = pane.size;
        transform.translation.x = x + w / 2.0;
        transform.translation.y = -(y + h / 2.0);
        sprite.size = Vec2::new(w, h);
    }
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn_bundle(OrthographicCameraBundle {
        transform: Transform::from_xyz(WIDTH / 2.0, -HEIGHT / 2.0, 0.0),
        ..OrthographicCameraBundle::new_2d()
    });

    let mut panes = vec![
        Pane {
            id: 1,
            pos: (0.0, 0.0),
            size: (300.0, 600.0 + GAP),
            flex: true,
        },
        Pane {
            id: 2,
            pos: (300.0 + GAP, 0.0),
            size: (600.0 + GAP, 300.0),
            flex: false,
        },
        Pane {
            id: 3,
            pos: (300.0 + GAP, 300.0 + GAP),
            size: (300.0, 300.0),
            flex: true,
        },
        Pane {
            id: 4,
            pos: (600.0 + 2.0 * GAP, 300.0 + GAP),
            size: (300.0, 600.0 + GAP),
            flex: true,
        },
        Pane {
            id: 5,
            pos: (0.0, 600.0 + 2.0 * GAP),
            size: (600.0 + GAP, 300.0),
            flex: false,
        },
    ];

    // Just to make my life harder and force me to sort later
    panes.reverse();

    for pane in panes {
        let color = if pane.flex { Color::GREEN } else { Color::RED };

        commands
            .spawn_bundle(SpriteBundle {
                material: materials.add(color.into()),
                ..Default::default()
            })
            .insert(pane);
    }
}
