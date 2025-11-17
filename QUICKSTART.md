# Quick Start Guide

## Prerequisites

1. Install Rust (version 1.70 or later)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. For Linux users, install X11 development libraries:
   ```bash
   # Ubuntu/Debian
   sudo apt-get install libx11-dev

   # Fedora
   sudo dnf install libX11-devel

   # Arch
   sudo pacman -S libx11
   ```

## Running the Game

### Development Mode
```bash
cargo run
```

### Optimized/Release Mode (Recommended)
```bash
cargo run --release
```

## Game Controls

### Keyboard Controls
- **1** - Select Peashooter (100 sun) - Basic shooter
- **2** - Select Sunflower (50 sun) - Generates sun
- **3** - Select Wall-nut (50 sun) - High health blocker
- **4** - Select Snow Pea (175 sun) - Shoots frozen peas
- **5** - Select Repeater (200 sun) - Shoots two peas

### Mouse Controls
- **Left Click on Grid** - Place selected plant (costs sun)
- **Left Click on Sun** - Collect sun

## Basic Strategy

1. **Start with Sunflowers**: Plant sunflowers in the back rows (left side) to generate sun
2. **Build Up Sun**: Wait for sunflowers to produce sun before planting attackers
3. **Place Shooters**: Use peashooters in middle rows to attack zombies
4. **Use Wall-nuts**: Place wall-nuts in front of shooters to protect them
5. **Upgrade**: Once you have enough sun, place repeaters for double damage

## Typical First Minute

```
Time 0:00 - Game starts with 150 sun
Time 0:05 - Place first Sunflower (100 sun remaining)
Time 0:10 - Place second Sunflower (50 sun remaining)
Time 0:10 - First zombie spawns
Time 0:24 - Sunflowers produce sun (100 sun)
Time 0:30 - Place Peashooter (0 sun)
Time 0:48 - More sun generated (25 sun)
Time 0:50 - Collect sun from field
```

## Tips

- **Sun Management**: Always maintain at least 50 sun for emergency plants
- **Layered Defense**: Use multiple rows of plants
- **Front Lines**: Keep wall-nuts in front rows to absorb damage
- **Back Lines**: Keep sunflowers protected in back rows
- **Quick Reflexes**: Collect sun quickly before it disappears
- **Row Priority**: Focus on rows where zombies appear most

## Troubleshooting

### Game Won't Compile
- Ensure you have the latest Rust version: `rustup update`
- Check X11 libraries are installed (Linux)
- Try cleaning and rebuilding: `cargo clean && cargo build`

### Low FPS
- Use release mode: `cargo run --release`
- Close other applications
- Check system meets minimum requirements

### Game Crashes
- Ensure dependencies are up to date: `cargo update`
- Check system resources
- Report issues on GitHub

## Building from Source

```bash
# Clone the repository
git clone https://github.com/TC999/plantsvszombies.git
cd plantsvszombies

# Build the project
cargo build --release

# Run the game
cargo run --release

# Or run the built executable directly
./target/release/plantsvszombies
```

## Testing

Run the test suite:
```bash
cargo test
```

## Performance Tuning

For best performance:
1. Always use `--release` flag
2. Close background applications
3. Ensure graphics drivers are up to date

## Next Steps

Once you're comfortable with basic gameplay:
- Try different plant combinations
- Experiment with defensive strategies
- See how long you can survive
- Explore the source code to understand how it works
- Contribute improvements or new features

## Getting Help

- Check the [README.md](README.md) for detailed documentation
- Review the [IMPLEMENTATION.md](IMPLEMENTATION.md) for technical details
- Report issues on GitHub
- Read the source code - it's well-commented!

## Have Fun!

Enjoy defending your lawn against the zombie horde! 🌻🧟‍♂️
