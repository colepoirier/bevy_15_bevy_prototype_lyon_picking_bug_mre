// use bevy::gizmos::aabb::{AabbGizmoPlugin, ShowAabbGizmo};
// use bevy::picking::backend::ray::RayMap;
// use bevy::render::mesh::MeshAabb;
// use bevy::render::view::RenderLayers;
use bevy::{color::palettes::css::*, prelude::*};
// use bevy_prototype_lyon::plugin::BuildShapes;
use bevy_prototype_lyon::prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, ShapePlugin, MeshPickingPlugin))
        .add_systems(Startup, setup_system)
        // .add_systems(PostUpdate, force_update_aabb.after(BuildShapes))
        .run();
}

#[derive(Component)]
struct Name(String);

impl std::fmt::Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
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
            Name("bevy".into()),
            ShowAabbGizmo {
                color: Some(GREEN.into()),
            },
        ))
        .observe(|_: Trigger<Pointer<Over>>| {
            info!("Pointer enter regular mesh 2d!");
        })
        .observe(|_: Trigger<Pointer<Move>>| info!("Pointer over regular mesh 2d!"))
        .observe(|_: Trigger<Pointer<Out>>| info!("Pointer exit regular mesh 2d!"));

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
            Name("lyon".into()),
            ShowAabbGizmo {
                color: Some(GREEN.into()),
            },
        ))
        .observe(|_: Trigger<Pointer<Over>>| info!("Pointer over bevy_prototype_lyon mesh2d!"))
        .observe(|_: Trigger<Pointer<Move>>| info!("Pointer over bevy_prototype_lyon mesh 2d!"))
        .observe(|_: Trigger<Pointer<Out>>| info!("Pointer out bevy_prototype_lyon mesh2d!"));
}

// fn force_update_aabb(
//     mut commands: Commands,
//     // shapes: Query<(Entity, &Name, &Mesh2d, &GlobalTransform, &RenderLayers)>,
//     layers: Query<&RenderLayers>,
//     meshes: Res<Assets<Mesh>>,
//     ray_map: Res<RayMap>,
// ) {
//     for layer in layers.iter() {
//         println!("labyer: {layer:?}");
//     }
//     let mut v = vec![];
//     for (entity, name, mesh, global_transform, render_layers) in shapes.iter() {
//         let mesh = meshes.get(mesh).unwrap();
//         let aabb = mesh.compute_aabb().unwrap();
//         println!("render_layers: {render_layers:?}");
//         let s = format!("{name}: new aabb is {aabb:?}, global_transform: {global_transform:?}, render_layers: {render_layers:?}");
//         commands.entity(entity).insert(aabb);
//         v.push(s)
//     }
//     if ray_map.map().len() > 0 {
//         info!("ray_map: {:?}", ray_map);
//         for s in v {
//             info!("{s}")
//         }
//     }
// }
