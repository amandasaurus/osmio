//! Currently unused local experimentation
use super::ObjId;
use std::io::Cursor;
use byteorder::{BigEndian, WriteBytesExt, ReadBytesExt};

pub struct NodeChain {
    _data: Vec<u8>,
}

fn encode_nodes_to_bytes(nodes: &[ObjId]) -> Vec<u8> {
    let mut results = Vec::with_capacity(nodes.len());
    if nodes.len() > 0 {
        let first_node = nodes[0];
        let mut last = first_node;
        results.reserve(8);
        results.write_u64::<BigEndian>(first_node);
        for n in nodes.iter().skip(1) {
            // FIXME might we get overflow here?
            let offset = (*n as i64) - (last as i64);
            last = *n;
            if *n == first_node {
                results.reserve(1);
                results.write_u8(0b1000_0001);
            } else if offset == 0 {
                results.reserve(1);
                results.write_u8(0b0000_0000);
            } else if offset > -126 && offset < 125 {
                let offset: i8 = offset as i8;
                //let offset = offset | 0b1000_0000;
                results.reserve(1);
                results.write_i8(offset);
            } else {
                results.reserve(9);
                results.write_u8(0b1111_1111);
                results.write_u64::<BigEndian>(*n);
            }

        }
    }

    results.shrink_to_fit();

    results
}

fn decode_bytes_to_nodes(bytes: &[u8]) -> Vec<ObjId> {
    let mut results = Vec::with_capacity(bytes.len()/4);  // randomly choose 4

    if bytes.len() > 0 {
        let mut bytes = Cursor::new(bytes);
        let first = bytes.read_u64::<BigEndian>().unwrap();
        results.push(first);
        let mut last = first;
        loop {
            match bytes.read_i8() {
                Err(_) => {
                    // end of input (I think)
                    break;
                },
                Ok(num) => {
                    if num == 0b1000_0001 {
                        results.push(first);
                    } else if num as u8 == 0b1111_1111u8 {
                        let next = bytes.read_u64::<BigEndian>().unwrap();
                        results.push(next);
                        last = next;
                    } else if num == 0 {
                        results.push(last);
                    } else {
                        let offset = num as i64;
                        let new_num: u64 = ((last as i64) + offset) as u64;
                        results.push(new_num);
                        last = new_num;
                    }
                },
            }

        }

    }

    results.shrink_to_fit();
    results
}

impl NodeChain {
    pub fn new(nodes: &[ObjId]) -> Self {
        NodeChain{ _data: encode_nodes_to_bytes(nodes) }
    }

    pub fn nodes(&self) -> Vec<ObjId> {
        decode_bytes_to_nodes(&self._data)
    }
}

mod tests {
    #[test]
    fn test_encode() {
        use super::{encode_nodes_to_bytes, decode_bytes_to_nodes};
        assert_eq!(encode_nodes_to_bytes(&[]), vec![]);
        assert_eq!(encode_nodes_to_bytes(&[10]), vec![0, 0, 0, 0, 0, 0, 0, 10]);
        assert_eq!(encode_nodes_to_bytes(&[10, 11]), vec![0, 0, 0, 0, 0, 0, 0, 10, 1]);
        assert_eq!(encode_nodes_to_bytes(&[10, 9]), vec![0, 0, 0, 0, 0, 0, 0, 10, 0b1111_1111]);
        assert_eq!(encode_nodes_to_bytes(&[10, 11, 12]), vec![0, 0, 0, 0, 0, 0, 0, 10, 1, 1]);
        assert_eq!(encode_nodes_to_bytes(&[10, 11, 12, 13]), vec![0, 0, 0, 0, 0, 0, 0, 10, 1, 1, 1]);
        assert_eq!(encode_nodes_to_bytes(&[10, 256]), vec![0, 0, 0, 0, 0, 0, 0, 10, 255, 0, 0, 0, 0, 0, 0, 1, 0]);
        assert_eq!(encode_nodes_to_bytes(&[10, 256, 10]), vec![0, 0, 0, 0, 0, 0, 0, 10, 255, 0, 0, 0, 0, 0, 0, 1, 0, 0b1000_0001u8]);
    }

    #[test]
    fn test_decode() {
        use super::{encode_nodes_to_bytes, decode_bytes_to_nodes};
        assert_eq!(decode_bytes_to_nodes(&[]), vec![]);
        assert_eq!(decode_bytes_to_nodes(&[0, 0, 0, 0, 0, 0, 0, 10]), vec![10]);
        assert_eq!(decode_bytes_to_nodes(&[0, 0, 0, 0, 0, 0, 0, 10, 1]), vec![10, 11]);
        assert_eq!(decode_bytes_to_nodes(&[0, 0, 0, 0, 0, 0, 0, 10, 255, 0, 0, 0, 0, 0, 0, 1, 0]), vec![10, 256]);
        assert_eq!(decode_bytes_to_nodes(&[0, 0, 0, 0, 0, 0, 0, 10, 255, 0, 0, 0, 0, 0, 0, 1, 0, 0b1000_0001u8]), vec![10, 256, 10]);
    }

    #[test]
    fn test_encode_then_decode() {
        use super::{encode_nodes_to_bytes, decode_bytes_to_nodes};
        use super::super::ObjId;
        fn test(input: &[ObjId]) {
            assert_eq!(decode_bytes_to_nodes(&encode_nodes_to_bytes(input)), input);
        }
        test(&vec![]);
        test(&vec![1, 2, 3]);
        test(&vec![1, 2, 3, 1000, 2]);
        test(&vec![1, 2, 3, 1000, 1]);

    }
}
