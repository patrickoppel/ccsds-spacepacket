use nom::IResult;
use nom::number::complete::be_u16;
use nom::bytes::complete::take;
use nom::bits;
use nom::sequence::tuple;
use nom::combinator::map;

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

named!(version<&[u8], u8>, bits!(take_bits!(3u8)) );

named!(packet_type<&[u8], u8>, bits!(take_bits!(1u8)) );

named!(sec_header_flag<&[u8], u8>, bits!(take_bits!(1u8)) );

named!(app_proc_id<&[u8], u16>, bits!(take_bits!(11u8)) );

named!(sequence_flags<&[u8], u8>, bits!(take_bits!(2u8)) );

named!(sequence_count<&[u8], u16>, bits!(take_bits!(14u8)) );

named!(data_length<&[u8], u16>, bits!(take_bits!(16u8)) );

named!(primary_header_parser<&[u8], (u8, u8, u8, u16, u8, u16, u16)>, tuple!(version, packet_type, sec_header_flag, app_proc_id, sequence_flags, sequence_count, data_length));

named!(pub primary_header<&[u8], PrimaryHeader>, map!(primary_header_parser, |(version, packet_type, sec_header_flag, app_proc_id, sequence_flags, sequence_count, data_length)| { 
	PrimaryHeader {
        version,
        packet_type,
        sec_header_flag,
        app_proc_id,
        sequence_flags,
        sequence_count,
        data_length
    }
        
}));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_python_spacepacket_primary_header() {
        //this is the equivalent of an all-zero primary header except for a data length of 64 followed by two bytes set to all 1 as a "payload" 
        let raw = b"\x00\x00\x00\x00\x00\x00\x00\x00\x00\x40\xff\xff";
        let expected = PrimaryHeader {
            version: 0,
            packet_type: 0,
            sec_header_flag: 0,
            app_proc_id: 0,
            sequence_flags: 0,
            sequence_count:0,
            data_length: 64
        };
        let (remaining, parsed) = primary_header(raw).expect("failed to parse header");

        assert_eq!(parsed, expected);
        assert_eq!(remaining, &[255, 255])
    }
}


// \x00\x01\x00\x00\x00\x0f\x00\x00\x00\x00\x00\x00\x00o\x05\xdcquery