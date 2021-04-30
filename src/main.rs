mod resize;

use bevy::{prelude::*, render::camera::Camera, window::WindowResized};
use resize::row_boundaries;

const WIDTH: f32 = 900.0 + 2.0 * GAP;
const HEIGHT: f32 = 900.0 + 2.0 * GAP;
const GAP: f32 = 10.0;

pub struct State {
    last_size: (f32, f32),
}

impl Default for State {
    fn default() -> Self {
        Self {
            last_size: (WIDTH, HEIGHT),
        }
    }
}

#[derive(Debug, Clone, Bundle)]
pub struct Pane {
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
        .init_resource::<State>()
        .add_startup_system(setup.system())
        .add_system(resize_handler.system())
        .add_system(draw_panes.system())
        .add_system(move_camera.system())
        .run();
}

fn resize_handler(
    mut events: EventReader<WindowResized>,
    mut state: ResMut<State>,
    mut query: Query<&mut Pane>,
) {
    for event in events.iter() {
        dbg!(event);
        let (last_width, last_height) = state.last_size;
        let (scale_x, scale_y) = (event.width / last_width, event.height / last_height);
        let panes: Vec<Pane> = query
            .iter_mut()
            .map(|mut p| {
                p.size.0 *= scale_x;
                p.size.1 *= scale_y;
                p.pos.0 *= scale_x;
                p.pos.1 *= scale_y;
                p.to_owned()
            })
            .collect();
        dbg!(row_boundaries(&panes[..]));
        state.last_size = (event.width, event.height);
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

fn move_camera(state: Res<State>, mut cameras: Query<(&Camera, &mut Transform)>) {
    for (_camera, mut transform) in cameras.iter_mut() {
        transform.translation.x = state.last_size.0 / 2.0;
        transform.translation.y = -state.last_size.1 / 2.0;
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
        let color = if pane.flex { Color::GREEN } else { Color::RED };

        commands
            .spawn_bundle(SpriteBundle {
                material: materials.add(color.into()),
                ..Default::default()
            })
            .insert(pane);
    }
}
