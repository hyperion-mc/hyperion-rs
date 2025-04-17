#[derive(Component, Default, Copy, Clone, Debug)]
pub struct LastDamaged {
  // The tick we were last damaged
  pub tick: i64,
}

impl Module for RegenerationModule {
  fn module(world: &World) {
    // Register our new component
    world.component::<LastDamaged>();

    // when a Player component is added onto an entity, add a LastDamaged component
    // Player is a "tag" component to mark an entity as a player
    // Note: if we wanted other entities to regenerate, we would add the trait to other
    // components as well
    world
        .component::<Player>()
        .add_trait::<(flecs::With, LastDamaged)>();

    // Query all entities with LastDamaged, Prev -> Health, and Health components
    system!(
      "regenerate_health",
      world,
      &mut LastDamaged,
      &(Prev, Health), // The health value last gametick
      &mut Health, // The current health value
      &Compose($) // This is a singleton component that is shared across all entities
    )
    .multi_threaded() // it is this easy to add multi-threading
    .each(|(last_damaged, prev_health, health, compose)| {
      // we cannot regenerate if we are dead
      if health.is_dead() {
          return;
      }

      let current_tick = compose.global().tick;

      // Update damage timestamp if health decreased
      if health < prev_health {
          last_damaged.tick = current_tick;
      }

      let ticks_since_damage = current_tick - last_damaged.tick;


      // Calculate regeneration using a simpler linear approach
      const BASE_REGEN: f32 = 0.01;  // 1% base
      const MAX_REGEN: f32 = 0.10;   // 10% maximum
      const RAMP_TICKS: f32 = 900.0; // Ticks to reach maximum

      let progress = ((current_tick - last_damaged.tick) as f32
          / RAMP_TICKS).min(1.0);

      let regen_rate = BASE_REGEN + progress * (MAX_REGEN - BASE_REGEN);

      // Apply regeneration, capped at max health
      health.heal(regen_rate);
      **health = health.min(MAX_HEALTH);
    });
  }
}
