# d7engine
## A project by Markus Dick
### d7engine is a homemade games engine for fun.

Basic setup:

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