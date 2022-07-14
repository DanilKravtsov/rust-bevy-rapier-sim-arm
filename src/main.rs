use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_obj::*;
use bevy_infinite_grid::{InfiniteGridBundle, InfiniteGridPlugin};
use bevy_mod_picking::{
    DebugCursorPickingPlugin, DebugEventsPickingPlugin, DefaultPickingPlugins, PickableBundle,
   };
use bevy_flycam::PlayerPlugin;
    fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(ObjPlugin)
        //.add_plugin(PlayerPlugin)//камера
        .add_plugin(InfiniteGridPlugin)
        .add_plugins(DefaultPickingPlugins)
        .add_plugin(DebugCursorPickingPlugin) // <- Adds the green debug cursor.
        .add_plugin(DebugEventsPickingPlugin) // <- Adds debug event logging.
        .add_startup_system(setup)

        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup_physics)
        .add_startup_system(setup_graphics)
        .run();
    }
        
fn setup_physics(mut commands: Commands) {
            /* Create the ground. */
            commands
                .spawn()
                .insert(Collider::cuboid(100.0, 0.1, 100.0))
                .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)));
        
        }

fn setup_graphics(mut commands: Commands) {
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_matrix(
            Mat4::look_at_rh(
                Vec3::new(5.0, 5.0, 22.0),
                Vec3::new(1.0, 1.0, 1.0),
                Vec3::new(0.0, 1.0, 0.0),
            )
            .inverse(),
        ),
        ..Default::default()
    }).insert_bundle(bevy_mod_picking::PickingCameraBundle::default());
}
// */ //

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    //поле
    commands.spawn_bundle(InfiniteGridBundle::default());


    // */ //  рука С КОЛАЙДЕРОМ 
    let con1 = commands.spawn_bundle(PbrBundle {
            mesh: asset_server.load("arm/con1.obj"),
            material: materials.add(StandardMaterial {
                base_color_texture: Some(asset_server.load("red.png")),
                ..Default::default()
            }),
        ..Default::default()
    }).insert_bundle(PickableBundle::default())                                               // <- Sets the camera to use for picking.
    .insert(RigidBody::Fixed)
    .insert(Collider::cylinder(0.17, 1.1))
    .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)))
    .insert(Friction::coefficient(0.7))
    .insert(Restitution::coefficient(0.3))
    .insert(ColliderMassProperties::Density(2.0))
    .id();

        // */ //  рука С КОЛАЙДЕРОМ 
    let con2 = commands.spawn_bundle(PbrBundle {
            mesh: asset_server.load("arm/con2.obj"),
            material: materials.add(StandardMaterial {
                base_color_texture: Some(asset_server.load("blue.png")),
                ..Default::default()
            }),
            ..Default::default()    
        }).insert_bundle(PickableBundle::default())// <- Sets the camera to use for picking.
        .insert(RigidBody::Dynamic)
        .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, 0.4, 0.0)))
        .insert(Friction::coefficient(0.7))
        .insert(Restitution::coefficient(0.3))
        .insert(ColliderMassProperties::Density(0.1))
        .with_children(|children| {
            children.spawn()
                .insert(Collider::cylinder(0.06, 1.1))
               .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)));
            })
        .id();
        
              // */ // рука С КОЛАЙДЕРОМ
   let arm = commands.spawn_bundle(PbrBundle {
    mesh: asset_server.load("arm/arm.obj"),
    material: materials.add(StandardMaterial {
        base_color_texture: Some(asset_server.load("white.png")),
        ..Default::default()
    }),
    ..Default::default()
}).insert_bundle(PickableBundle::default())// <- Sets the camera to use for picking.
.insert(RigidBody::Dynamic)
.insert(Friction::coefficient(0.7))
.insert(Restitution::coefficient(0.3))
.insert(ColliderMassProperties::Density(0.1))
.with_children(|children| {
    children.spawn()
        .insert(Collider::cuboid(0.1,0.5,0.6))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(0.5, 3.4, 0.0)));
}).id();


 // */ //  рука С КОЛАЙДЕРОМ 
    
 let hand = commands.spawn_bundle(PbrBundle {
    mesh: asset_server.load("arm/hand.obj"),
    material: materials.add(StandardMaterial {
        base_color_texture: Some(asset_server.load("white.png")),
        ..Default::default()
    }),
    ..Default::default()
}).insert_bundle(PickableBundle::default())// <- Sets the camera to use for picking.
.insert(RigidBody::Dynamic)
.insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, 4.3, 0.0)))
.insert(Friction::coefficient(0.7))
.insert(Restitution::coefficient(0.3))
.insert(ColliderMassProperties::Density(0.01))
.with_children(|children| {
    children.spawn()
        .insert(Collider::cylinder(0.1,0.53,))
        .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, 2.45, 0.0)));
})
.id();
        // */ // рука С КОЛАЙДЕРОМ
 let paint = commands.spawn_bundle(PbrBundle {
        mesh: asset_server.load("arm/paint.obj"),
        material: materials.add(StandardMaterial {
            base_color_texture: Some(asset_server.load("white.png")),
            ..Default::default()
        }),
            ..Default::default()
        }).insert_bundle(PickableBundle::default())// <- Sets the camera to use for picking.
        .insert(RigidBody::Dynamic)
        .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, 6.5, 0.0)))
        .insert(Friction::coefficient(0.7))
        .insert(Restitution::coefficient(0.3))
        .insert(ColliderMassProperties::Density(2.0))
        .with_children(|children| {
             children.spawn()
                .insert(Collider::cuboid(0.1,0.1,0.5))
                .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, 0.4, 0.0)));
        })
        .id();
    
        // */ //  рука С КОЛАЙДЕРОМ 
   commands.spawn_bundle(PbrBundle {
            mesh: asset_server.load("arm/kles2.obj"),
                material: materials.add(StandardMaterial {
                base_color_texture: Some(asset_server.load("red.png")),
            ..Default::default()
        }),
        ..Default::default()
        }).insert_bundle(PickableBundle::default())// <- Sets the camera to use for picking.
        .insert(RigidBody::Dynamic)
        .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, 7.5, 0.0)))
        .insert(Friction::coefficient(0.7))
        .insert(Restitution::coefficient(0.3))
        .insert(ColliderMassProperties::Density(2.0))
        .with_children(|children| {
           children.spawn()
                .insert(Collider::cuboid(0.1,0.15,0.1))
                .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, 0.95, 0.0)));
            });  
        // */ //  рука С КОЛАЙДЕРОМ 
    commands.spawn_bundle(PbrBundle {
        mesh: asset_server.load("arm/kles1.obj"),
        material: materials.add(StandardMaterial {
            base_color_texture: Some(asset_server.load("red.png")),
            ..Default::default()
        }),
            ..Default::default()
        }).insert_bundle(PickableBundle::default())// <- Sets the camera to use for picking.
        .insert(RigidBody::Dynamic)
        .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, 7.0, 0.0)))
        .insert(Friction::coefficient(0.7))
        .insert(Restitution::coefficient(0.3))
        .insert(ColliderMassProperties::Density(2.0))
        .with_children(|children| {
            children.spawn()
            .insert(Collider::cuboid(0.1,0.15,0.1))
                .insert_bundle(TransformBundle::from(Transform::from_xyz(0.0, 0.95, 10.0)));
        });
    // */ //    
 commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(3.0, 4.0, 3.0)),
        ..Default::default()
    });
    // */ //  
  let joint = RevoluteJointBuilder::new(Vec3::Y)
  .local_anchor1(Vec3::new(0.0, 0.4, 0.0))
  .motor_velocity(0.1, 0.0)
  ;
  commands
  .entity(con2)
  .insert(ImpulseJoint::new(con1, joint));
  
  let joint1 = RevoluteJointBuilder::new(Vec3::X)
  .local_anchor1(Vec3::new(0.0, 0.5, 0.0))
  .motor_velocity(0.1, 0.0)
  ;
   commands
  .entity(arm)
  .insert(ImpulseJoint::new(con2, joint1));
  
  let joint2 = RevoluteJointBuilder::new(Vec3::X)
  .local_anchor1(Vec3::new(0.0, 3.4, 0.0))
  //.motor_velocity(-0.1, 0.0)
  ;
   commands
  .entity(hand)
  .insert(ImpulseJoint::new(arm, joint2));
  
  let joint3 = RevoluteJointBuilder::new(Vec3::Y)
  .local_anchor1(Vec3::new(0.0, 2.61, 0.0)) 
  .motor_velocity(0.1, 0.0)
  ;
   commands
  .entity(paint)
  .insert(ImpulseJoint::new(hand, joint3));
  /*
  let joint4 = PrismaticJointBuilder::new(Vec3::Z)
  .local_anchor1(Vec3::new(0.0, 0.0, 0.0));
  //.limit_axis([-2.0, 5.0);
commands
  .entity(kles1)
  .insert(ImpulseJoint::new(paint, joint4));
  
  let joint5 = PrismaticJointBuilder::new(Vec3::Z)
  .local_anchor1(Vec3::new(0.0, 0.0, 0.0));
  //.limit_axis([-2.0, 5.0);
commands
  .entity(kles2)
  .insert(ImpulseJoint::new(paint, joint5));
*/
}