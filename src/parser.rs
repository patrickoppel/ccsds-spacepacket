use nom::IResult;
use nom::number::complete::be_u16;
use nom::bytes::complete::take;
use nom::bits;
use nom::sequence::tuple;

#[derive(Clone,Debug,PartialEq,Eq)]
pub struct PrimaryHeader {
    /// Packet Version Number - 3 bits
    pub version: u8,
    /// Packet Type - 1 bit
    pub packet_type: u8,
    /// Secondary Header Flag - 1 bit
    pub sec_header_flag: u8,
    /// Application Process ID - 11 bits
    pub app_proc_id: u16,
    /// Sequence Flags - 2 bits
    pub sequence_flags: u8,
    /// Packet Sequence Count or Packet Name - 14 bits
    pub sequence_count: u16,
    /// Packet Data Length - 2 bytes
    pub data_length: u16,
}

fn version(input: &[u8]) -> IResult<&[u8],u8> {
    bits!( take_bits!(3)(input) )
}

fn packet_type(input: &[u8]) -> IResult<&[u8],u8> {
    bits!( take_bits!(1)(input) )
}

fn sec_header_flag(input: &[u8]) -> IResult<&[u8],u8> {
    bits!( take_bits!(1)(input) )
}


fn app_proc_id(input: &[u8]) -> IResult<&[u8],u16> {
    bits!( take_bits!(11)(input) )
}

fn sequence_flags(input: &[u8]) -> IResult<&[u8],u8> {
    bits!( take_bits!(2)(input) )
}

fn sequence_count(input: &[u8]) -> IResult<&[u8],u16> {
    bits!( take_bits!(14)(input) )
}

fn data_length(input: &[u8]) -> IResult<&[u8],u16> {
    be_u16(input)
}

// fn data_length = |s| {be_u16(s)};

fn primary_header(i: &[u8]) -> IResult<&[u8], PrimaryHeader> {

  // tuple takes as argument a tuple of parsers and will return
  // a tuple of their results
  let (input, (version, packet_type, sec_header_flag, app_proc_id, sequence_flags, sequence_count, data_length)) =
    tuple((version, packet_type, sec_header_flag, app_proc_id, sequence_flags, sequence_count, data_length))(i)?;

  Ok((input, PrimaryHeader { version, packet_type, sec_header_flag, app_proc_id, sequence_flags, sequence_count, data_length }))
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn parse_python_spacepacket_primary_header() {
        let raw = b"\x00\x01\x00\x00\x00\x0f\x00\x00\x00\x00\x00\x00\x00o\x05\xdcquery";
    
        let parsed = parser::primary_header(raw);
        dbg!(parsed);
    }
}


// \x00\x01\x00\x00\x00\x0f\x00\x00\x00\x00\x00\x00\x00o\x05\xdcquery
