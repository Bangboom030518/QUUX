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

## Template Syntax

```rust
struct InitInfo {
    stores: Vec<Store>,

}

Button, id

--> client
--> Button::new()
--> "hydrate" the button with id

struct Button {
    fn render() -> Vec<Store> {
        html! {
            <button onclick={|_| alert("hi")}>a</button>
        }
    }
}
// On the server
struct Button {
    fn render() -> "<button id=random>a</button>"
}

struct Button {
    fn render() ->
        // On client
        getElem("#random").addClickEvent(|_| alert("hi"))
}
```

```rust

struct Text {
    fn render() -> {
        html! {
            <p>hello</p>
        }
    }
}

struct Button {
    fn render() {
        // generic rust
        let store = Store::new(0);
        let list = Store::new([]);
        // maybe changes, maybe not
        html! {
            <button onclick={|_| {
                store.set(store.get() + 1);
                list.push(())
            }}>{$store}</button>
            <ul id="random2">
                {@for _ in $list}
                    <Text />
                {@endfor}
            </ul>
        }
    }
}

// Server



struct Button {
    fn render() -> {
        // generic rust

        // create a store - internal impl generates random id
        let store = Store::new(0);
        // maybe changes, maybe not
        "<button id='random'>0 <!-- format!('{}', store) --></button>"
    }
}


// Client

static mut STORE: Mutex<HashMap<String, String>>

static mut random_store = // construct store

static mut random_list = // construct list

getElem("#random").addClickEvent(|| {
    random_store.set(random_store.get() + 1);
    getElem("#random").innerText=random_store.get(); // code called by onchange.
    random_list.push(());
    // random_list.on_push(|_| {
        // BEGIN TEXT INIT LOGIC
        let elem = createElem("p")
        elem.innerText = "lolz";
        // END TEXT INIT LOGIC

        getElem("#random2").append(elem);
    // }
})

```

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
// https://sycamore-rs.netlify.app/

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

// https://en.wikipedia.org/wiki/OSI_model#Layer_architecture

```rust

fn init() -> {
    let store = Store::new(0);

}

fn render() -> {

}
```

NOTE: N static children OR 1 store????????????

```rust
html! {
    <meta 1>
    <meta 2>
    <button id="random" $store=store on:click={|| {}}><p>{ $store }</p><p></p></button>
}

// Expands to

#[cfg!(target = wasm)]
{
    addEvent("click")
}

#[cfg!(not(target = wasm))]
{
    compileToHTML(Button {

    })
}
```
