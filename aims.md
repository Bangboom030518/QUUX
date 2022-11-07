- Full Stack (like Next.js, Nuxt and SvelteKit)
- Hello World has 0kb of script
- ⚡⚡⚡ Blazingly fast ⚡⚡⚡
- Teeny tiny scripts
- Uses Rust for both server and _CLIENT_ logic

# Rendering

## Approach 1

Convert _ALL_ DOM-related code into simple vanilla operations with the WASM. Take out all the component structs and convert their intialisation logic into simple functions where they need construction. Most components don't require any aditional logic, but where events are present, IDs are randomly generated on the server for elements that need reactivity.

### Advantages

- Minimal WASM, and only used when needed

### Disadvantages

- Difficult to have dynamic SSR-like stuff.

## Approach 2

Send WASM to the client that allows them to hydrate server-rendered static HTML from a serialised representation of the tree. All component structs should be accessible by the client

### Advantages

- Easy to have dynamic SSR-like stuff.

### Disadvantages

- More WASM.
- Client has to do more work hydrating.
