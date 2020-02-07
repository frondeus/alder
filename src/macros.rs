#[cfg(not(feature = "debug"))]
#[macro_export]
macro_rules! node_kinds {
    ($name: ident: $first_kind: ident, $($kind: ident),*) => {
        mod counter {
            // 0 = error
            // 1 = token
            pub enum $name { $first_kind = 2, $($kind),* }
        }

        struct $name;
        #[allow(non_upper_case_globals)]
        impl $name {
            const $first_kind: NodeKind = NodeKind(counter::$name::$first_kind as u32);
            $( const $kind: NodeKind = NodeKind(counter::$name::$kind as u32); )*
        }
    };
}

#[cfg(feature = "debug")]
#[macro_export]
macro_rules! node_kinds {
    ($name: ident: $first_kind: ident, $($kind: ident),*) => {
        struct $name;
        #[allow(non_upper_case_globals)]
        impl $name {
            const $first_kind: NodeKind = NodeKind(stringify!($first_kind));
            $( const $kind: NodeKind = NodeKind(stringify!($kind)); )*
        }
    };
}
