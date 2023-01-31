# d7engine
## A project by Markus Dick
### d7engine is a homemade games engine for fun.

### Instalation

Make sure that you have Rust installed on your pc by using:

```
cargo version
```

or install Rust by using the [official Rust installation guid](https://www.rust-lang.org/tools/install).

Create a new project:

```
cargo new your_game_name
```

And add the engine to the project:

```
cd your_game_name
cargo add d7engine
```

### Basic setup:

```
//#![windows_subsystem = "windows"]
use d7engine::prelude::*;

struct Runt {
    components: Vec<Box<dyn Component>>,
    camera: Transform,
}

impl Runtime for Runt {
    fn load(&mut self) {
        
    }

    fn draw(&mut self, draw: &Draw) {
        for component in &self.components {
            component.draw(draw, &self.camera).unwrap();
        }
    }
}

fn main() {
    init(Config::default(), &mut Runt{
        components: vec![],
        camera: Transform::new(),
    });
}
```