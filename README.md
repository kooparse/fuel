# Fuel Engine

Fuel is a composable 3D engine.


### How to use the engine
```
extern crate fuel;

use fuel::{Scene, Object};
use fuel::types::Position;

fn main() {
  let mut scene = Scene::new();
  let cube = Object::new(vertices, "cube", "texture");

  let key = scene.add(cube);
  scene.get_component(key).set_position(0., 0., 0.);

  scene.render();
}

```

### How to run an example

```
cargo run --example name_of_an_example
```

### Screenshots
<img width="801" alt="screen shot 2018-05-05 at 7 43 23 pm" src="https://user-images.githubusercontent.com/2859122/39666029-656aa4a4-509d-11e8-9536-971e2eca248e.png">

<img width="794" alt="screen shot 2018-05-05 at 7 46 53 pm" src="https://user-images.githubusercontent.com/2859122/39666030-6b34d864-509d-11e8-991c-3b5bacc838d3.png">
