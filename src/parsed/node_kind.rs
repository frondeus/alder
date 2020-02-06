#[cfg(not(feature = "debug"))]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NodeKind(pub u32);

#[cfg(not(feature = "debug"))]
impl From<u32> for NodeKind {
    fn from(i: u32) -> Self {
        Self(i)
    }
}

#[cfg(not(feature = "debug"))]
impl NodeKind {
    pub const ERROR: NodeKind = NodeKind(0);

    pub fn is_error(&self) -> bool {
        *self == Self::ERROR
    }
}

#[cfg(feature = "debug")]
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct NodeKind(pub &'static str);

#[cfg(feature = "debug")]
impl From<&'static str> for NodeKind {
    fn from(i: &'static str) -> Self {
        Self(i)
    }
}

#[cfg(feature = "debug")]
impl NodeKind {
    pub const ERROR: NodeKind = NodeKind("ERROR");

    pub fn is_error(&self) -> bool {
        *self == Self::ERROR
    }
}
