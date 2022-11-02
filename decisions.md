# TODO

- HOW COMPONENTS MANAGE STATE
  - Stores?
- HOW THEY ARE RENDERED
  - Server Side Static Rendering
  - Client Side Dynamic Rendering
- HOW THEY INTERACT WITH THE DOM TREE
  - Child -> Parent Communication
  - Parent -> Child Communication

## Template Syntax

```rust
html! {
    <h1>{"Hello World!"}</h1>
    <button on:click={|event| clicked.set(true)}>{ "Click Me" }</button>
    <p>
        { "Button clicked: " }
        <span>{$clicked}</span>
    </p>
    <button on:click={}>{ "Create Post" }</button>
    <ul>
        clicked.on_update(|| )
        for post in $posts {
            <li>{ name } { " : " } { description }</li>
        }
    </ul>
}
```

## Component Struct Interface

```rust
struct MyComponent {
    clicked: Store<bool>
}

```

## Stores

Stores have an initial value, which is rendered on the server. It is passed when the store is constructed.

```rust
let count: Store<u32> = Store::new(0);
```

The value can be accessed once by calling the `get` method, which accesses the internal `value` property of the store.

```rust
let value = count.get();
```

```rust
impl A {
    fn init() -> Self {
        let state = Store::new(0);
        if rust_is_cool {
            state.set(1);
        }
    }
}
```

The value can be set on the client using the `set` method.

<Rec>
    <button on:click={
        Self.push_component(Rec::new())
    }>Click Me</button>
</Rec>

<button id="random">Click Me</button>

<script>
const btn = getElem("#random");

function buttonClick() {
    self.addElem(newButton())
}

function newButton() {
    const newbtn = createElem("button");
    newbtn.innerText = "Click Me";
    newbtn.onclick(buttonClick)
    return newbtn;
}

btn.onclick(buttonClick)
</script>
```rust
let mut list = Store::<Vec<String>>::new(vec!["hello", "world"]);

enum Event {
    New(String)
}

html! {
    for item in $list {
        <p>{ item }</p>
    }
}

THE DOM IS:
    <p>hello</p>
    <p>world</p>

#[wasm]
{ 
    let list = ["hello", "world"]

    list.onchange(|event: Event| {
        match Event {
            New(value) => {
                let random = createElem("p");
                random.innerHTML = "world";
                push(random)

            }
        }
    })
}
```

# Landfill

```rust
struct Post {
    title: String,
    description: String,
}


struct ClickedStore {
    value: bool
}

impl Store for ClickedStore {
    type Message = bool;

    fn set(&mut self, message: Self::Message) {
        self.value = message;
    }
}


let clicked = Store::<bool> {
    value: false,
    on_update: |event: | {

    }
}
let posts = Store::<Vec<Post>>::new(vec![]);



html! {
    <h1>{"Hello World!"}</h1>
    <button on:click={|event| clicked.set(true)}>{ "Click Me" }</button>
    <p>
        { "Button clicked: " }
        <span>{$clicked}</span>
    </p>
    <button on:click={}>{ "Create Post" }</button>
    <ul>
        clicked.on_update(|| )
        for post in $posts {
            <li>{ name } { " : " } { description }</li>
        }
    </ul>
}
```
