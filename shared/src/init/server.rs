use crate::Component;

pub fn init_app<T, P>(component: T) -> String
where
    T: Component<Props = P>,
{
    let render_data = component.render();

    format!("<!DOCTYPE html>\r\n{}", render_data.html)
}
