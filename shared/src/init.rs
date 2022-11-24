use crate::Component;

#[cfg(not(target = "wasm"))]
pub fn init_app<T, P>(component: T)
where
    T: Component<Props = P>,
{
    let render_data = component.render();

    println!("{}", render_data.html);
    // todo!("Implement `init_app`");
}
