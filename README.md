# d7engine

## A project by Markus Dick

### d7engine is a homemade games engine for fun

### Installation

Make sure that you have Rust installed on your pc by using:

```
cargo version
```

or install Rust by using the [official Rust installation guid](https://www.rust-lang.org/tools/install).
You also need [cmake](https://cmake.org/download/) installed on your system.

Create a new project:

```
cargo new your_game_name
```

And add the engine to the project:

```
cd your_game_name
cargo add d7engine
```

### Basic setup

```rust
//#![windows_subsystem = "windows"]
use d7engine::*;

struct Runt {
    components: ComponentContainer,
    camera: Transform,
}

impl Runtime for Runt {
    fn load(&mut self) {
        let color = Color::rgb(255, 0, 0);
        let mut rect1 = Component::rect().unwrap();
        rect1.set_color(&color);
        rect1.set_dim(100.0, 100.0);
        rect1.transform.set(50.0, 50.0, 0.0);
        self.components.insert("1", rect1);
    }

    fn draw(&mut self, draw: &Draw) {
        self.components.draw(draw, &self.camera).unwrap();
    }
}

fn main() {
    init(Config::default(), &mut Runt{
        components: ComponentContainer::new(),
        camera: Transform::new(),
    });
}
```
