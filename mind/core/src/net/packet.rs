// internally used by generated code
static NODATA: Vec<u8> = Vec::new();


// todo convert to enum
/// Does not get handled unless client is connected.
pub static PRIORITY_LOW: i32 = 0;
/// Gets put in a queue and processed if not connected.
pub static PRIORITY_NORMAL: i32 = 1;
/// Gets handled immediately, regardless of connection status.
pub static PRIORITY_HIGH: i32 = 2;

pub trait Packet {
    fn handled() {}
    fn get_priority() -> i32 {
        PRIORITY_NORMAL
    }
    fn handle_client() {}
    // todo: add more stuff
}