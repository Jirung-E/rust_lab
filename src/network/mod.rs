/// ### Intel Windows환경에서 serialize해서, little-endian으로 변환됨.
/// #### 해결방법
/// 1. 최초접속시 패킷을 검사해서 endian이 일치하지 않는다면 접속 거부
/// 2. 통신시 big-endian으로 변환해서 통신(네트워크 통신 표준?)

mod packet;
mod protocol;

pub use packet::*;
pub use protocol::*;