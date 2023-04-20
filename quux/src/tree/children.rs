use super::Hydrate;
use crate::internal::prelude::*;

pub trait Children: Display + Hydrate {
    // TODO: make constants?
    fn is_self_closing(&self) -> bool {
        false
    }

    fn is_empty(&self) -> bool {
        false
    }

    fn boxed<'a>(self) -> Box<dyn Children + 'a>
    where
        Self: Sized + 'a,
    {
        Box::new(self)
    }
}

impl<T: Display> Hydrate for Store<T> {
    fn hydrate(self) {
        todo!()
    }
}

impl<T: Item> Children for T {}

pub struct SelfClosing;

impl Display for SelfClosing {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        Ok(())
    }
}

#[client]
impl Hydrate for SelfClosing {}

impl Children for SelfClosing {
    fn is_self_closing(&self) -> bool {
        true
    }
}

pub struct Empty;

impl Display for Empty {
    fn fmt(&self, _: &mut fmt::Formatter) -> fmt::Result {
        Ok(())
    }
}

#[client]
impl Hydrate for Empty {}

impl Children for Empty {
    fn is_empty(&self) -> bool {
        true
    }
}

pub struct Pair<A: Children, B: Children>(pub A, pub B);

#[client]
impl<A: Children, B: Children> Hydrate for Pair<A, B> {
    fn hydrate(self) {
        self.0.hydrate();
        self.1.hydrate();
    }
}

impl<A: Children, B: Children> Display for Pair<A, B> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)?;
        self.1.fmt(f)?;
        Ok(())
    }
}

impl<A: Children, B: Children> Children for Pair<A, B> {}

// TODO: allow n-length tuple
#[allow(clippy::missing_const_for_fn)]
pub fn children<A, B>(children: (A, B)) -> Pair<A, B>
where
    A: Children,
    B: Children,
{
    Pair(children.0, children.1)
}

// TODO: consider the Branch enum futher

macro_rules! branch_decl {
    ($($types:ident),*) => {
        pub enum Branch<$($types = Empty),*>
        where
            $($types: Children),*
        {
            $($types($types)),*
        }

        impl<$($types),*> Hydrate for Branch<$($types),*>
        where
            $($types: Children),*
        {
            fn hydrate(self)
            where
                Self: Sized,
            {
                match self {
                    $(Branch::$types(child) => child.hydrate()),*
                }
            }
        }

        impl<$($types),*> Display for Branch<$($types),*>
        where
            $($types: Children),*
        {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match self {
                    $(Branch::$types(child) => child.fmt(f)),*
                }
            }
        }

        impl<$($types),*> Item for Branch<$($types),*>
        where
            $($types: Children),*
        {
        }
    };
}

branch_decl! {
    A, B, C, D, E, F
}

// TODO: macroify this
pub type Branch2<A, B> = Branch<A, B, Empty, Empty, Empty, Empty>;
pub type Branch3<A, B, C> = Branch<A, B, C, Empty, Empty, Empty>;
pub type Branch4<A, B, C, D> = Branch<A, B, C, D, Empty, Empty>;
pub type Branch5<A, B, C, D, E> = Branch<A, B, C, D, E, Empty>;
pub type Branch6<A, B, C, D, E, F> = Branch<A, B, C, D, E, F>;
