// Which mode the server operates in.
#[derive(Clone, Debug)]
pub enum ServerMode {
    /// Write back received bytes
    Echo,
    /// Do one read, then write a bodged HTTP response and
    /// cleanly close the connection.
    Http,
    /// Forward traffic to/from given port on localhost. From now hardcoded the forward
    /// port: 9000
    Forward(u16),
}