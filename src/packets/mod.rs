pub mod play_clientbound;
pub mod serializer;
pub use minecraft_packet_derive::*;
use serializer::*;

#[derive(Debug)]
pub struct VarInt(pub i32);
impl From<VarInt> for usize {
    fn from(val: VarInt) -> Self {
        val.0 as usize
    }
}
impl From<usize> for VarInt {
    fn from(value: usize) -> Self {
        VarInt(value as i32)
    }
}

#[derive(Debug)]
pub struct VarLong(pub i64);
impl From<VarLong> for usize {
    fn from(val: VarLong) -> Self {
        val.0 as usize
    }
}
impl From<usize> for VarLong {
    fn from(value: usize) -> Self {
        VarLong(value as i64)
    }
}

#[derive(Debug)]
pub struct Position {
    pub x: i32,
    pub y: i16,
    pub z: i32,
}

#[minecraft_enum(u8)]
#[derive(Debug)]
pub enum Direction {
    South = 1,
    West,
    North,
    East,
}

pub type UUID = u128;
pub type Angle = u8;
/// Json encoded data, stored in a String.
/// See [the wiki](https://wiki.vg/Chat).
pub type Chat<'a> = &'a str;

/// This is used to replace an unsupported structure by taking all the remaining bytes of a packet.
/// Feel free to make PRs.
#[derive(Debug)]
pub struct RawBytes<'a> {
    data: &'a [u8],
}

#[derive(Debug, MinecraftPacketPart)]
pub struct TestPacket {
    data: u8,
}

#[derive(Debug)]
pub struct Array<'a, T: MinecraftPacketPart<'a> + std::fmt::Debug, U: MinecraftPacketPart<'a>> {
    _len_prefix: std::marker::PhantomData<&'a U>,
    pub items: Vec<T>,
}
