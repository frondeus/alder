#[macro_export]
macro_rules! node_ids {
    (pub $name: ident: $first_kind: ident, $($kind: ident),*) => {
        pub struct $name;
        #[allow(non_upper_case_globals)]
        impl $name {
            pub const $first_kind: NodeId = NodeId(stringify!($first_kind));
            $( pub const $kind: NodeId = NodeId(stringify!($kind)); )*
        }
    };
    ($name: ident: $first_kind: ident, $($kind: ident),*) => {
        struct $name;
        #[allow(non_upper_case_globals)]
        impl $name {
            const $first_kind: NodeId = NodeId(stringify!($first_kind));
            $( const $kind: NodeId = NodeId(stringify!($kind)); )*
        }
    };
}
