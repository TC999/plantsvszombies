# Architecture Diagram

```
┌─────────────────────────────────────────────────────────────────────┐
│                         BEVY GAME ENGINE                            │
│                     (ECS Architecture - 0.11)                       │
└─────────────────────────────────────────────────────────────────────┘
                                  │
                    ┌─────────────┴─────────────┐
                    │                           │
         ┌──────────▼──────────┐    ┌──────────▼──────────┐
         │    COMPONENTS       │    │      SYSTEMS        │
         │      (Data)         │    │      (Logic)        │
         └─────────────────────┘    └─────────────────────┘
                  │                            │
        ┌─────────┴─────────┐       ┌─────────┴─────────┐
        │                   │       │                   │
   ┌────▼────┐         ┌────▼────┐  │  ┌────▼────┐    ┌────▼────┐
   │ Plant   │         │ Zombie  │  │  │  Input  │    │Movement │
   ├─────────┤         ├─────────┤  │  ├─────────┤    ├─────────┤
   │ type    │         │ type    │  │  │ mouse   │    │ zombies │
   │ health  │         │ health  │  │  │ keyboard│    │ plants  │
   │ attack  │         │ speed   │  │  └─────────┘    └─────────┘
   │ cooldown│         │ attack  │  │
   └─────────┘         └─────────┘  │  ┌─────────┐    ┌─────────┐
                                    │  │Shooting │    │Collision│
   ┌─────────┐         ┌─────────┐  │  ├─────────┤    ├─────────┤
   │Projectle│         │   Sun   │  │  │ peas    │    │ damage  │
   ├─────────┤         ├─────────┤  │  │ frozen  │    │ hit     │
   │ damage  │         │ value   │  │  │ double  │    │ detect  │
   │ speed   │         │lifetime │  │  └─────────┘    └─────────┘
   │is_frozen│         └─────────┘  │
   └─────────┘                      │  ┌─────────┐    ┌─────────┐
                                    │  │   Sun   │    │Spawning │
   ┌─────────┐                      │  ├─────────┤    ├─────────┤
   │Position │                      │  │ produce │    │ zombies │
   ├─────────┤                      │  │ collect │    │ random  │
   │   x, y  │                      │  │ decay   │    │ timer   │
   └─────────┘                      │  └─────────┘    └─────────┘
                                    │
                                    │  ┌─────────┐    ┌─────────┐
                                    │  │  Game   │    │GameOver │
                                    │  ├─────────┤    ├─────────┤
                                    │  │ combat  │    │ detect  │
                                    │  │ damage  │    │ display │
                                    │  └─────────┘    └─────────┘

┌─────────────────────────────────────────────────────────────────────┐
│                          RESOURCES                                  │
│                      (Global State)                                 │
├─────────────────────────────────────────────────────────────────────┤
│  SunResource    │  SelectedPlant  │  ZombieSpawnTimer  │ GameState │
│  ──────────────────────────────────────────────────────────────────│
│  amount: u32    │  plant_type     │  timer: f32        │ game_over │
│                 │  Option<Type>   │  interval: f32     │ bool      │
└─────────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────────┐
│                         GAME LOOP                                   │
├─────────────────────────────────────────────────────────────────────┤
│                                                                     │
│  Startup:                                                           │
│  ├─ Setup Camera                                                    │
│  ├─ Create Grid (9x5)                                               │
│  ├─ Initialize Resources (150 sun)                                  │
│  └─ Create UI Elements                                              │
│                                                                     │
│  Update (Every Frame):                                              │
│  ├─ Handle Input (keyboard/mouse)                                   │
│  ├─ Move Zombies (left direction)                                   │
│  ├─ Plant Shooting (check for zombies in row)                       │
│  ├─ Move Projectiles (right direction)                              │
│  ├─ Check Collisions (projectile vs zombie)                         │
│  ├─ Sunflower Produce Sun (every 24s)                               │
│  ├─ Sun Lifetime Decay (10s limit)                                  │
│  ├─ Collect Sun (mouse click)                                       │
│  ├─ Spawn Zombies (every 10s)                                       │
│  ├─ Game Logic (zombie vs plant combat)                             │
│  ├─ Check Game Over (zombie at left edge)                           │
│  ├─ Display Game Over (if needed)                                   │
│  └─ Update UI (sun counter)                                         │
│                                                                     │
└─────────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────────┐
│                      DATA FLOW EXAMPLE                              │
├─────────────────────────────────────────────────────────────────────┤
│                                                                     │
│  Player Clicks Grid:                                                │
│  1. Input System detects mouse click                                │
│  2. Calculate grid position (x, y)                                  │
│  3. Check if position is occupied (query existing plants)           │
│  4. Check if enough sun (SunResource)                               │
│  5. Deduct sun cost from SunResource                                │
│  6. Spawn Plant entity with components:                             │
│     - Plant (stats based on type)                                   │
│     - Position (grid coordinates)                                   │
│     - SpriteBundle (visual representation)                          │
│                                                                     │
│  Peashooter Shoots:                                                 │
│  1. Shooting System queries all plants                              │
│  2. Update attack timer (delta time)                                │
│  3. Check if cooldown complete                                      │
│  4. Query zombies in same row                                       │
│  5. If zombie exists, spawn Projectile entity:                      │
│     - Projectile (damage, speed)                                    │
│     - Position (plant's row)                                        │
│     - SpriteBundle (pea visualization)                              │
│                                                                     │
│  Projectile Hits Zombie:                                            │
│  1. Projectile Movement System moves projectile right               │
│  2. Collision System checks distance to all zombies                 │
│  3. If distance < threshold:                                        │
│     - Reduce zombie health by projectile damage                     │
│     - Despawn projectile entity                                     │
│     - If zombie health = 0, despawn zombie entity                   │
│                                                                     │
└─────────────────────────────────────────────────────────────────────┘
```

## Component Interaction Matrix

```
                 │ Plant │ Zombie │Projectle│  Sun  │Position│
─────────────────┼───────┼────────┼─────────┼───────┼────────┤
Input System     │   ✓   │        │         │   ✓   │        │
Movement System  │       │   ✓    │         │       │   ✓    │
Shooting System  │   ✓   │   ✓    │    ✓    │       │   ✓    │
Projectile System│       │        │    ✓    │       │        │
Collision System │       │   ✓    │    ✓    │       │        │
Sun System       │   ✓   │        │         │   ✓   │        │
Spawning System  │       │   ✓    │         │       │   ✓    │
Game Logic       │   ✓   │   ✓    │         │       │   ✓    │
Game Over System │       │   ✓    │         │       │        │
UI System        │       │        │         │   ✓   │        │
─────────────────┴───────┴────────┴─────────┴───────┴────────┘

✓ = System reads/writes this component
```

## File Structure

```
plantsvszombies/
├── Cargo.toml              # Project configuration
├── Cargo.lock              # Dependency lock file
├── README.md               # Main documentation
├── QUICKSTART.md           # Quick start guide
├── IMPLEMENTATION.md       # Technical details
├── ARCHITECTURE.md         # This file
│
└── src/
    ├── main.rs             # Entry point, app setup
    │
    ├── components/         # ECS Components (data)
    │   ├── mod.rs
    │   ├── plant.rs        # Plant types and stats
    │   ├── zombie.rs       # Zombie types and stats
    │   ├── projectile.rs   # Bullet properties
    │   ├── sun.rs          # Sun resource
    │   └── position.rs     # Grid coordinates
    │
    └── systems/            # ECS Systems (logic)
        ├── mod.rs
        ├── input.rs        # Player input handling
        ├── movement.rs     # Zombie movement
        ├── shooting.rs     # Plant shooting
        ├── projectile.rs   # Bullet movement
        ├── collision.rs    # Damage calculation
        ├── sun.rs          # Sun mechanics
        ├── spawning.rs     # Zombie generation
        ├── game.rs         # Combat logic
        └── game_over.rs    # End game detection
```
