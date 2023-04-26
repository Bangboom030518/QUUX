macro_rules! branch_decl {
    ($name:ident, $($types:ident),*) => {
        pub enum $name<$($types),*>
        where
            $($types: Item),*
        {
            $($types($types)),*
        }

        impl<$($types),*> Hydrate for $name<$($types),*>
        where
            $($types: Item),*
        {
            fn hydrate(self)
            where
                Self: Sized,
            {
                match self {
                    $($name::$types(child) => child.hydrate()),*
                }
            }
        }

        impl<$($types),*> Display for $name<$($types),*>
        where
            $($types: Item),*
        {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match self {
                    $($name::$types(child) => child.fmt(f)),*
                }
            }
        }

        impl<$($types),*> Item for $name<$($types),*>
        where
            $($types: Item),*
        {
        }
    };
}
// TODO: consider the Branch enum futher

pub mod prelude {
    use super::super::Hydrate;
    use crate::internal::prelude::*;

    // TODO: macroify this

    branch_decl! { Branch2, A, B }

    branch_decl! { Branch3, A, B, C }

    branch_decl! { Branch4, A, B, C, D }

    branch_decl! { Branch5, A, B, C, D, E }

    branch_decl! { Branch6, A, B, C, D, E, F }
}
