use bevy::prelude::*;
use crate::despawn::Despawn;

pub struct ParticlePlugin;

#[derive(Component)]
pub struct Explosion(pub Vec3);

#[derive(Component)]
struct Particle {
    offset: f32
}

impl Particle {
    pub fn new(offset: f32) -> Self {
        Self {
            offset
        }
    }
}

impl Plugin for ParticlePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                add_explosions,
                update_particles
            ));
    }
}

fn add_explosions(
    time: Res<Time>,
    mut cmds: Commands,
    assets: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    q: Query<(Entity, &Explosion), Added<Explosion>>)
{
    let tex = Some(assets.load("exp1.png"));
    let mesh = meshes.add(Rectangle::new(2.0, 2.0));
    let mat = materials.add(StandardMaterial {
        base_color: Color::hex("ff2244").unwrap(),
        base_color_texture: tex,
        //perceptual_roughness: 1.0,
        alpha_mode: AlphaMode::Blend,
        cull_mode: None,
        ..default()
    });

    for (ent, exp) in q.iter() {
        for i in 0..6 {
            cmds.spawn((
                PbrBundle {
                    mesh: mesh.clone(),
                    material: mat.clone(),
                    transform: Transform::from_translation(exp.0)
                        .with_rotation(
                            Quat::from_rotation_y(i as f32 * 1.0)),
                    ..default()
                },
                Particle::new(time.elapsed_seconds())
            ));
        }
        cmds.entity(ent).remove::<Explosion>();
    }
}

fn update_particles(
    time: Res<Time>,
    mut cmds: Commands,
    mut q: Query<(Entity, &mut Transform, &Particle)>
) {
    let dt = time.delta_seconds();
    for (e, mut t, p) in q.iter_mut() {
        let down = t.down();
        t.translation.y += ((time.elapsed_seconds() + p.offset) * 10.).sin() * dt;
        t.translation += down * dt * 20.0;
        if t.translation.y < -50.0 {
            cmds.entity(e).insert(Despawn);
        }
    }
}
