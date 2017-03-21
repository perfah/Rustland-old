// An identifier type for elements of the layout
pub type LayoutElemID = u16;

// The maximum number of available workspaces
pub const MAX_WORKSPACES_LIMIT: usize = 4;

// CALLBACK EVENT SIGNALS
pub const WM_FORWARD_EVENT_TO_CLIENT: bool = false;
pub const WM_CATCH_EVENT: bool = true;

// KEY CODES
pub const LEFT_CLICK: u32 = 0x110;
pub const RIGHT_CLICK: u32 = 0x111;

pub const SOCKET_PORT: u16 = 4451;
pub const SOCKET_DETERMINANT: u8 = b'$';
pub const TAG_PREFIX: &str = "@";