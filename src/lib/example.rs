/// A component that tracks when an entity was last damaged,
/// used for health regeneration calculations
#[derive(Component, Default, Copy, Clone, Debug)]
pub struct LastDamaged {
    /// The tick we were last damaged
    pub tick: i64,
}

// A module (think like a Bukkit plugin or Bevy plugin)
// We can import these in main.rs and have modules depend on each other
impl Module for RegenerationModule {
  fn module(world: &World) {
    // Register our component in the Entity Component System world
    world.component::<LastDamaged>();

    // When a Health component is added to an entity, automatically add LastDamaged
    // Entities are composed of components instead of using inheritance
    world
        .component::<Health>()
        .add_trait::<(flecs::With, LastDamaged)>();

    // This uses a Rust macro to define a system that handles health regeneration
    // This system queries all entities that have:
    // - LastDamaged: to track time since last damage
    // - Health (previous): the entity's health in the previous tick
    // - Health (current): the entity's current health value
    // - MaxHealth: the maximum health value of the entity
    // - Compose: a singleton component providing global game state
    system!(
      "regenerate_health",
      world,
      &mut LastDamaged,
      &(Prev, Health), // The health value from the previous game tick
      &mut Health, // The current health value
      &MaxHealth, // The maximum health value of the entity
      &Compose($)  // Singleton ($) component with global game state
    )
    .multi_threaded() // Enable parallel execution of this system (it is that easy!)
    .each(|(last_damaged, prev_health, health, max_health, compose)| {
      // Skip dead entities
      if health.is_dead() {
          return;
      }

      let current_tick = compose.global().tick;

      // Update damage timestamp if health decreased since last tick
      if health < prev_health {
          last_damaged.tick = current_tick;
      }

      let ticks_since_damage = current_tick - last_damaged.tick;


      // Calculate health regeneration using a linear ramp-up approach
      const BASE_REGEN: f32 = 0.01;  // 1% base
      const MAX_REGEN: f32 = 0.10;   // 10% maximum
      const RAMP_TICKS: f32 = 900.0; // Ticks to reach maximum

      // Calculate progress from 0.0 to 1.0 based on time since last damage
      let progress = ((current_tick - last_damaged.tick) as f32 / RAMP_TICKS).min(1.0);

      // Interpolate between base and max regeneration rates
      let regen_rate = BASE_REGEN + progress * (MAX_REGEN - BASE_REGEN);

      // Apply regeneration, capped at max health
      health.heal(regen_rate, max_health);
    });
  }
}
