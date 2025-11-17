use bevy::prelude::*;
use crate::components::projectile::Projectile;

pub fn move_projectiles(
    mut commands: Commands,
    mut projectiles_query: Query<(Entity, &mut Transform, &Projectile)>,
    time: Res<Time>,
) {
    for (entity, mut transform, projectile) in projectiles_query.iter_mut() {
        transform.translation.x += projectile.speed * time.delta_seconds();

        // 移除超出屏幕的子弹
        if transform.translation.x > 1000.0 {
            commands.entity(entity).despawn();
        }
    }
}
