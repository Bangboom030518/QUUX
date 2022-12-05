fn main()
{
    let mut children = context.children.into_iter() ;
    {
        let child =
        children.next().expect("Client and server child lists don't match") ;
        let mut component : QUUXInitialise = shared :: postcard ::
        from_bytes(& child.component).expect("Couldn't deserialize component")
        ; component.render(child.render_context) ;
    }
}