//! We wrap all messages in something called a tagged encoding. This encoding wraps over
//! the underlying representation to something we can read without actually decoding the raw data
//! that contains the real broadcast.

use tagged::{TaggedValue, TagType};
use result::{GossipResult, GossipError, io_err, TaggedDecodingError};
use std::io::Reader;

/// A single bytecode traditionally represented as hexadecimal. The specification has a number of
/// default bytecodes that are defined.
pub struct Bytecode(int);

/// A macro to easily create bytecodes.
macro_rules! def(
    ($inp:ident $sp:expr) => (
        pub static $inp: int = $sp;
    );
)

def!(TAG_START 0xCB)
def!(PRIORITY_CACHE_PACKED_START 0x80)
def!(PRIORITY_CACHE_PACKED_END 0xA0)
def!(STRUCT_CACHE_PACKED_START 0xA0)
def!(STRUCT_CACHE_PACKED_END 0xB0)
def!(LONG_ARRAY 0xB0)
def!(DOUBLE_ARRAY 0xB1)
def!(BOOLEAN_ARRAY 0xB2)
def!(INT_ARRAY 0xB3)
def!(FLOAT_ARRAY 0xB4)
def!(OBJECT_ARRAY 0xB5)
def!(MAP 0xC0)
def!(SET 0xC1)
def!(UUID 0xC3)
def!(REGEX 0xC4)
def!(URI 0xC5)
def!(BIGINT 0xC6)
def!(BIGDEC 0xC7)
def!(INST 0xC8)
def!(SYM 0xC9)
def!(KEY 0xCA)
def!(GET_PRIORITY_CACHE 0xCC)
def!(PUT_PRIORITY_CACHE 0xCD)
def!(PRECACHE 0xCE)
def!(FOOTER 0xCF)
def!(FOOTER_MAGIC 0xCFCFCFCF)
def!(BYTES_PACKED_LENGTH_START 0xD0)
def!(BYTES_PACKED_LENGTH_END 0xD8)
def!(BYTES_CHUNK 0xD8)
def!(BYTES 0xD9)
def!(STRING_PACKED_LENGTH_START 0xDA)
def!(STRING_PACKED_LENGTH_END 0xE2)
def!(STRING_CHUNK 0xE2)
def!(STRING 0xE3)
def!(LIST_PACKED_LENGTH_START 0xE4)
def!(LIST_PACKED_LENGTH_END 0xEC)
def!(LIST 0xEC)
def!(BEGIN_CLOSED_LIST 0xED)
def!(BEGIN_OPEN_LIST 0xEE)
def!(STRUCTTYPE 0xEF)
def!(STRUCT 0xF0)
def!(META 0xF1)
def!(ANY 0xF4)
def!(TRUE 0xF5)
def!(FALSE 0xF6)
def!(NULL 0xF7)
def!(INT 0xF8)
def!(FLOAT 0xF9)
def!(DOUBLE 0xFA)
def!(DOUBLE_0 0xFB)
def!(DOUBLE_1 0xFC)
def!(END_COLLECTION 0xFD)
def!(RESET_CACHES 0xFE)
def!(INT_PACKED_1_START 0xFF)
def!(INT_PACKED_1_END 0x40)
def!(INT_PACKED_2_START 0x40)
def!(INT_PACKED_2_ZERO 0x50)
def!(INT_PACKED_2_END 0x60)
def!(INT_PACKED_3_START 0x60)
def!(INT_PACKED_3_ZERO 0x68)
def!(INT_PACKED_3_END 0x70)
def!(INT_PACKED_4_START 0x70)
def!(INT_PACKED_4_ZERO 0x72)
def!(INT_PACKED_4_END 0x74)
def!(INT_PACKED_5_START 0x74)
def!(INT_PACKED_5_ZERO 0x76)
def!(INT_PACKED_5_END 0x78)
def!(INT_PACKED_6_START 0x78)
def!(INT_PACKED_6_ZERO 0x7A)
def!(INT_PACKED_6_END 0x7C)
def!(INT_PACKED_7_START 0x7C)
def!(INT_PACKED_7_ZERO 0x7E)
def!(INT_PACKED_7_END 0x80)

pub fn to_tag<'a>(reader: &'a mut Reader) -> GossipResult<TaggedValue> {

    // Read in the first byte to check whether it's a tagged encoding.
    if try!(reader.read_byte().map_err(io_err)) != 0xCB {
        return Err(GossipError::new("Expected the beginning marker of a tagged encoding.", TaggedDecodingError));
    }

    // Get the type of tag.
    let ty = try!(reader.read_byte().map_err(io_err));

    // Get the length of the tag type.
    let length = try!(reader.read_be_u32().map_err(io_err));
    let buf    = try!(reader.read_exact(length as uint).map_err(io_err));

    let bytes_length = try!(reader.read_be_u32().map_err(io_err));
    let mut bytes = try!(reader.read_exact(bytes_length as uint).map_err(io_err));

    let tag = TaggedValue::new(match ty {
        // String.
        0xE3 => {
            box try!(String::from_utf8(buf).map_err(|e| GossipError::new("Malformed utf8 string.", TaggedDecodingError))) as Box<TagType>
        },
        _ => {
            box 0i as Box<TagType>
        }
    }, bytes);

    Ok(tag)
}