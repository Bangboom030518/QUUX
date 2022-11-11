# TODO

- HOW COMPONENTS MANAGE STATE
  - Stores?
- HOW THEY ARE RENDERED
  - Server Side Static Rendering
  - Client Side Dynamic Rendering
- HOW THEY INTERACT WITH THE DOM TREE
  - Child -> Parent Communication
  - Parent -> Child Communication

- Do we optimise the generated Rust before compiling to WASM?
- How do we store state in the html?
- [Module splitting](https://emscripten.org/docs/optimizing/Module-Splitting.html#module-splitting)?
https://www.arewewebyet.org/topics/frameworks/


## Template Syntax


## Component Struct Interface

## Stores

Stores have an initial value, which is rendered on the server. It is passed when the store is constructed.

```rust
let count: Store<u32> = Store::new(0);
```

The value can be accessed once by calling the `get` method, which accesses the internal `value` property of the store.

## Basic Example

```rust
struct App {
    count: Store<u32>
}

impl Component for App {
    fn init() -> Self {
        Self {
            count: Store::new(0)
        }
    }

    fn render(&self) -> _ {
        html! {
            button(on:click={|| self.count.set(self.count.get() + 1)}) { $self.count }
        }
    }
}

fn main() {
    quux::init_app(App::init())
}
```

### Approach 2

```rust
// server sends
"<button id=\"button\">0</button>" + serialize(Button::new())

// client runs
deserialize(tree).expect("render failed").render(id)

struct Context {
    ids: HashMap<String, String>
}

struct App {
    count: Store<u32>,
}

impl Component for App {
    fn render(&self, context: Context) -> _ {
        let button = getElement(context.selector);
        button.addEvent("click", |_| self.store.count += 1);
        self.store.on_update(|value| button.text = value);
    }
}
```
