use std::net::TcpStream;
//use std::time::Duration;
use byteorder::WriteBytesExt;
use net2::TcpStreamExt;

use crate::error;
use crate::fcall::*;
use crate::serialize;

pub type Result<T> = ::std::result::Result<T, error::Error>;

macro_rules! io_err {
    ($kind:ident, $msg:expr) => {
        ::std::io::Error::new(::std::io::ErrorKind::$kind, $msg)
    };
}

macro_rules! res {
    ($err:expr) => {
        Err(From::from($err))
    };
}

pub fn parse_proto(arg: &str) -> Option<(&str, String)> {
    let mut split = arg.split('!');
    let (proto, addr, port) = (split.nth(0)?, split.nth(0)?, split.nth(0)?);

    Some((proto, addr.to_owned() + ":" + port))
}

// See also: diod/libdiod/diod_sock.c
pub fn setup_tcp_stream(stream: &TcpStream) -> ::std::io::Result<()> {
    //TcpStreamExt::set_keepalive(stream, Some(Duration::from_secs(120)))?;
    TcpStreamExt::set_nodelay(stream, true)
}

pub fn respond<WExt: WriteBytesExt>(stream: &mut WExt, tag: u16, body: Fcall) -> Result<()> {
    if MsgType::from(&body).is_t() {
        return res!(io_err!(Other, "Invalid 9P message in this context"));
    };

    let msg = Msg { tag, body };
    serialize::write_msg(stream, &msg)?;

    debug!("\t← {:?}", msg);

    Ok(())
}
