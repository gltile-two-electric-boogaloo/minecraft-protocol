pub mod play_clientbound;
pub mod serializer;
use std::convert::TryFrom;
pub use minecraft_packet_derive::*;
use serializer::*;

#[derive(Debug)]
pub struct VarInt(pub i32);
impl TryFrom<VarInt> for usize {
    type Error = std::num::TryFromIntError;
    fn try_from(value: VarInt) -> Result<Self, Self::Error> {
        TryFrom::try_from(value.0)
    }
}
impl From<usize> for VarInt {
    fn from(value: usize) -> Self {
        VarInt(value as i32)
    }
}

#[derive(Debug)]
pub struct VarLong(pub i64);
impl TryFrom<VarLong> for usize {
    type Error = std::num::TryFromIntError;
    fn try_from(value: VarLong) -> Result<Self, Self::Error> {
        TryFrom::try_from(value.0)
    }
}
impl TryFrom<usize> for VarLong {
    type Error = std::num::TryFromIntError;
    fn try_from(value: usize) -> Result<Self, Self::Error> {
        let value: i64 = TryFrom::try_from(value)?;
        Ok(VarLong(value))
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
/// Identifiers are a namespaced location, in the form of `minecraft:thing`.
/// If the namespace is not provided, it defaults to `minecraft` (i.e. thing is `minecraft:thing`).
/// Custom content should always be in its own namespace, not the default one.
/// The namespace should only use the characters `01​​234​5​6​78​9abcdefghijklmnopqrstuvwxyz-_`; actual names may contain more symbols.
/// The naming convention is `lower_case_with_underscores`. [More information](https://minecraft.net/en-us/article/minecraft-snapshot-17w43a).
pub type Identifier<'a> = &'a str;

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
