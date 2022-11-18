// Recursive expansion of dbg! macro
// ==================================

match (shared::RenderData {
    html: {
        let res = $crate::fmt::format($crate::fmt::Arguments::new_v1(
            &[],
            &[$crate::fmt::ArgumentV1::new(
                &("Makka Pakka!"),
                $crate::fmt::Display::fmt,
            )],
        ));
        res
    },
    ids: std::collections::HashMap::new(),
}) {
    tmp => {
        {
            $crate::io::_eprint($crate::fmt::Arguments::new_v1(
                &[],
                &[
                    $crate::fmt::ArgumentV1::new(&(""), $crate::fmt::Display::fmt),
                    $crate::fmt::ArgumentV1::new(&(0), $crate::fmt::Display::fmt),
                    $crate::fmt::ArgumentV1::new(
                        &("(view ! {body {h1 (a = \"hello!\") {} makka {}}})"),
                        $crate::fmt::Display::fmt,
                    ),
                    $crate::fmt::ArgumentV1::new(&(&tmp), $crate::fmt::Display::fmt),
                ],
            ));
        };
        tmp
    }
}
