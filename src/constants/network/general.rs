use enum_flags::enum_flags;

// Player State
#[enum_flags]
#[derive(Copy, Clone, PartialEq)]
pub enum SessionStatus {
    Authed,
    LoggedIn,
    Transfer,
    LoggedInOrRecentLogout,
}

#[enum_flags]
#[derive(Copy, Clone, PartialEq)]
pub enum PacketProcessing {
    // Process a packet whenever it is received - mostly for non-handled or non-implemented packets
    Inplace,
    // Packet that is not thread-safe - process it in World.update_sessions
    ThreadUnsafe,
    // Packet is thread-safe - proccess it in Map.update(),
    ThreadSafe
}