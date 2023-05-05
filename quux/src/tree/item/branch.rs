macro_rules! branch_decl {
    ($name:ident, $($types:ident),*) => {
        #[derive(Clone, Debug)]
        pub enum $name<$($types),*>
        where
            $($types: Item),*
        {
            $($types($types)),*
        }

        impl<$($types),*> Display for $name<$($types),*>
        where
            $($types: Item),*
        {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match self {
                    $($name::$types(child) => Display::fmt(&child, f)),*
                }
            }
        }

        impl<$($types),*> Item for $name<$($types),*>
        where
            $($types: Item),*
        {
            fn insert_id(&mut self, id: u64) -> u64 {
                match self {
                    $($name::$types(child) => child.insert_id(id)),*
                }
            }

            #[client]
            fn hydrate(&mut self) {
                match self {
                    $($name::$types(child) => child.hydrate()),*
                }
            }

            #[client]
            fn dom_representation(&mut self) -> DomRepresentation {
                match self {
                    $($name::$types(value) => value.dom_representation()),*
                }
            }
        }
    };
}
// TODO: consider the Branch enum futher

pub mod prelude {
    use crate::internal::prelude::*;

    // TODO: macroify this

    branch_decl! { Branch2, A, B }

    branch_decl! { Branch3, A, B, C }

    branch_decl! { Branch4, A, B, C, D }

    branch_decl! { Branch5, A, B, C, D, E }

    branch_decl! { Branch6, A, B, C, D, E, F }
}
