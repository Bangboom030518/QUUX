use crate::internal::prelude::*;

macro_rules! element {
    ($name:ident) => {
        #[must_use]
        pub fn $name<'a>() -> Element<'a, item::Empty> {
            Element::new(stringify!($name))
        }
    };
}

macro_rules! elements {
    ($($names:ident),*) => {
        $(element!($names);)*

        pub mod prelude {
            pub use super::{$($names),*};
        }
    };
}

elements!(
    a, abbr, address, area, article, aside, audio, b, base, bdi, bdo, blockquote, body, br, button,
    canvas, caption, cite, code, col, colgroup, data, datalist, dd, del, details, dfn, dialog, div,
    dl, dt, em, embed, fieldset, figcaption, figure, footer, form, h1, head, header, hgroup, hr,
    html, i, iframe, img, input, ins, kbd, label, legend, li, link, main, map, mark, menu, meta,
    meter, nav, noscript, object, ol, optgroup, option, output, p, picture, portal, pre, progress,
    q, rp, rt, ruby, s, samp, script, section, select, slot, small, source, span, strong, style,
    sub, summary, sup, table, tbody, td, template, textarea, tfoot, th, thead, time, title, tr,
    track, u, ul, var, video, wbr
);
