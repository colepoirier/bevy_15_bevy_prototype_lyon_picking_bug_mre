use bevy::{color::palettes::css::*, prelude::*};
use bevy_prototype_lyon::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, ShapePlugin, MeshPickingPlugin))
        .add_systems(Startup, setup_system)
        .run();
}

fn setup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn((
            Mesh2d(meshes.add(Rectangle::new(50.0, 50.0))),
            MeshMaterial2d(materials.add(ColorMaterial::from_color(RED))),
            Transform::from_translation(Vec3::ZERO.with_x(250.0)),
        ))
        .observe(|_: Trigger<Pointer<Over>>| {
            info!("Pointer over regular mesh 2d!");
        })
        .observe(|_: Trigger<Pointer<Out>>| info!("Pointer left regular mesh 2d!"));

    let rect = shapes::Rectangle {
        extents: Vec2::splat(250.0),
        origin: RectangleOrigin::Center,
        radii: Some(BorderRadii::single(25.0)),
    };

    commands.spawn((Camera2d, Msaa::Sample4));
    commands
        .spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&rect),
                ..default()
            },
            Stroke::new(WHITE, 10.0),
            Fill::color(RED),
        ))
        .observe(|_: Trigger<Pointer<Over>>| println!("Pointer over bevy_prototype_lyon mesh2d!"))
        .observe(|_: Trigger<Pointer<Out>>| println!("Pointer out bevy_prototype_lyon mesh2d!"));
}
