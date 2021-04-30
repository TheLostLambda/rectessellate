mod resize;

use bevy::{prelude::*, window::WindowResized};
use resize::row_boundaries;

#[derive(Debug, Clone, Bundle)]
pub struct Pane {
    pos: (f32, f32),
    size: (f32, f32),
    flex: bool,
}

const WIDTH: f32 = 900.0 + 2.0 * GAP;
const HEIGHT: f32 = 900.0 + 2.0 * GAP;
const GAP: f32 = 10.0;

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
        .run();
}

fn resize_handler(mut events: EventReader<WindowResized>, mut query: Query<&mut Pane>) {
    for event in events.iter() {
        dbg!(event);
        let panes: Vec<Pane> = query.iter_mut().map(|p| p.to_owned()).collect();
        dbg!(row_boundaries(&panes[..]));
    }
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn_bundle(OrthographicCameraBundle {
        transform: Transform::from_xyz(WIDTH / 2.0, -HEIGHT / 2.0, 0.0),
        ..OrthographicCameraBundle::new_2d()
    });

    let panes = vec![
        Pane {
            pos: (0.0, 0.0),
            size: (300.0, 600.0 + GAP),
            flex: true,
        },
        Pane {
            pos: (300.0 + GAP, 0.0),
            size: (600.0 + GAP, 300.0),
            flex: false,
        },
        Pane {
            pos: (300.0 + GAP, 300.0 + GAP),
            size: (300.0, 300.0),
            flex: true,
        },
        Pane {
            pos: (600.0 + 2.0 * GAP, 300.0 + GAP),
            size: (300.0, 600.0 + GAP),
            flex: true,
        },
        Pane {
            pos: (0.0, 600.0 + 2.0 * GAP),
            size: (600.0 + GAP, 300.0),
            flex: false,
        },
    ];

    for pane in panes {
        let (x, y) = pane.pos;
        let (w, h) = pane.size;
        let color = if pane.flex { Color::GREEN } else { Color::RED };

        commands
            .spawn_bundle(SpriteBundle {
                material: materials.add(color.into()),
                transform: Transform::from_xyz(x + w / 2.0, -(y + h / 2.0), 0.0),
                sprite: Sprite::new(Vec2::new(w, h)),
                ..Default::default()
            })
            .insert(pane);
    }
}
