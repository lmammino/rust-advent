use std::collections::VecDeque;

#[derive(Debug)]
struct BinaryString {
    bits: VecDeque<u8>,
    consumed_bits: usize,
}

impl BinaryString {
    fn from_hex(hex: &str) -> Self {
        let bits: VecDeque<u8> = hex
            .chars()
            .flat_map(|c| match c {
                '0' => [0, 0, 0, 0],
                '1' => [0, 0, 0, 1],
                '2' => [0, 0, 1, 0],
                '3' => [0, 0, 1, 1],
                '4' => [0, 1, 0, 0],
                '5' => [0, 1, 0, 1],
                '6' => [0, 1, 1, 0],
                '7' => [0, 1, 1, 1],
                '8' => [1, 0, 0, 0],
                '9' => [1, 0, 0, 1],
                'A' => [1, 0, 1, 0],
                'B' => [1, 0, 1, 1],
                'C' => [1, 1, 0, 0],
                'D' => [1, 1, 0, 1],
                'E' => [1, 1, 1, 0],
                'F' => [1, 1, 1, 1],
                _ => unreachable!(),
            })
            .collect();

        BinaryString {
            bits,
            consumed_bits: 0,
        }
    }

    fn has_more(&self) -> bool {
        !self.bits.is_empty()
    }

    fn get_int(&mut self, bits: usize) -> i64 {
        let mut val = 0;
        for _ in 0..bits {
            val = (val << 1) + (self.bits.pop_front().unwrap_or_default() as i64);
            self.consumed_bits += 1;
        }

        val
    }

    fn drop_bits(&mut self, bits: usize) {
        for _ in 0..bits {
            if self.bits.pop_front().is_none() {
                break;
            }
            self.consumed_bits += 1;
        }
    }

    fn hex_align(&mut self) {
        let pos = self.consumed_bits % 4;
        if pos != 0 {
            self.drop_bits(4 - pos);
        }
    }
}

#[derive(Debug)]
enum PacketData {
    Literal(i64),
    Expression(Vec<Packet>),
}

#[derive(Debug)]
struct Packet {
    version: i64,
    type_id: i64,
    data: PacketData,
}

impl Packet {
    fn version_sum(&self) -> i64 {
        let mut version_sum = self.version;
        if let PacketData::Expression(subpackets) = &self.data {
            for packet in subpackets {
                version_sum += packet.version_sum();
            }
        }

        version_sum
    }

    fn eval(&self) -> i64 {
        match &self.data {
            PacketData::Literal(x) => *x,
            PacketData::Expression(subpackets) => match self.type_id {
                0 => subpackets.iter().map(|p| p.eval()).sum(),
                1 => subpackets.iter().map(|p| p.eval()).product(),
                2 => subpackets.iter().map(|p| p.eval()).min().unwrap(),
                3 => subpackets.iter().map(|p| p.eval()).max().unwrap(),
                5 => {
                    if subpackets.get(0).unwrap().eval() > subpackets.get(1).unwrap().eval() {
                        1
                    } else {
                        0
                    }
                }
                6 => {
                    if subpackets.get(0).unwrap().eval() < subpackets.get(1).unwrap().eval() {
                        1
                    } else {
                        0
                    }
                }
                7 => {
                    if subpackets.get(0).unwrap().eval() == subpackets.get(1).unwrap().eval() {
                        1
                    } else {
                        0
                    }
                }
                _ => unreachable!(),
            },
        }
    }
}

#[derive(Debug)]
struct PacketParser(BinaryString);

impl PacketParser {
    fn from_hex(hex: &str) -> Self {
        PacketParser(BinaryString::from_hex(hex))
    }

    fn next_packet(&mut self) -> Option<Packet> {
        if !self.0.has_more() {
            return None;
        }

        // version: first 3 bits
        let version = self.0.get_int(3);
        // type_id: next 3 bits
        let type_id = self.0.get_int(3);

        let data = match type_id {
            4 => {
                let mut groups: VecDeque<i64> = Default::default();
                loop {
                    let is_last = self.0.get_int(1) == 0;
                    groups.push_front(self.0.get_int(4));

                    if is_last {
                        break;
                    }
                }
                // calculates the value of n using the 4bit ints groups
                let mut val = 0;
                for _ in 0..groups.len() {
                    val = (val << 4) + (groups.pop_back().unwrap() as i64);
                }

                PacketData::Literal(val)
            }
            _ => {
                let length_type = self.0.get_int(1);
                let subpackets_length_num_bits = match length_type {
                    0 => 15,
                    _ => 11,
                };
                let subpackets_length = self.0.get_int(subpackets_length_num_bits);
                let mut subpackets: Vec<Packet> = Default::default();
                if length_type == 0 {
                    // number of bits of all sub packets
                    let bits_before_packet = self.0.consumed_bits;
                    while self.0.consumed_bits - bits_before_packet < subpackets_length as usize {
                        let packet = self.next_packet().unwrap();
                        subpackets.push(packet);
                    }
                } else {
                    // number of sub packets
                    for _ in 0..subpackets_length {
                        subpackets.push(self.next_packet().unwrap()); // consumes all the sub-packets
                    }
                }

                PacketData::Expression(subpackets)
            }
        };

        Some(Packet {
            version,
            type_id,
            data,
        })
    }
}

impl Iterator for PacketParser {
    type Item = Packet;

    fn next(&mut self) -> Option<Self::Item> {
        let packet = self.next_packet();
        self.0.hex_align();
        packet
    }
}

pub fn part1(input: &str) -> i64 {
    let parser = PacketParser::from_hex(input);
    parser.map(|p| p.version_sum()).sum()
}

pub fn part2(input: &str) -> i64 {
    let mut parser = PacketParser::from_hex(input);
    let packet = parser.next().unwrap();
    packet.eval()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_literal_packet() {
        let mut parser = PacketParser::from_hex("D2FE28");
        let packet = parser.next().unwrap();
        assert_eq!(packet.version, 6);
        assert_eq!(packet.type_id, 4);
        assert!(matches!(packet.data, PacketData::Literal(2021)));
    }

    #[test]
    fn test_parse_simple_expression_packets() {
        let mut parser = PacketParser::from_hex("38006F45291200");
        let packet = parser.next().unwrap();
        assert_eq!(packet.version, 1);
        assert_eq!(packet.type_id, 6);

        let mut parser = PacketParser::from_hex("EE00D40C823060");
        let packet = parser.next().unwrap();
        assert_eq!(packet.version, 7);
        assert_eq!(packet.type_id, 3);

        let mut parser = PacketParser::from_hex("8A004A801A8002F478");
        let packet = parser.next().unwrap();
        assert_eq!(packet.version, 4);
        assert_eq!(packet.version_sum(), 16);

        let mut parser = PacketParser::from_hex("620080001611562C8802118E34");
        let packet = parser.next().unwrap();
        assert_eq!(packet.version, 3);
        assert_eq!(packet.version_sum(), 12);

        let mut parser = PacketParser::from_hex("C0015000016115A2E0802F182340");
        let packet = parser.next().unwrap();
        assert_eq!(packet.version_sum(), 23);

        let mut parser = PacketParser::from_hex("A0016C880162017C3686B18A3D4780");
        let packet = parser.next().unwrap();
        assert_eq!(packet.version_sum(), 31);
    }

    #[test]
    fn test_eval() {
        let mut parser = PacketParser::from_hex("C200B40A82");
        let packet = parser.next().unwrap();
        assert_eq!(packet.eval(), 3);

        let mut parser = PacketParser::from_hex("04005AC33890");
        let packet = parser.next().unwrap();
        assert_eq!(packet.eval(), 54);

        let mut parser = PacketParser::from_hex("880086C3E88112");
        let packet = parser.next().unwrap();
        assert_eq!(packet.eval(), 7);

        let mut parser = PacketParser::from_hex("CE00C43D881120");
        let packet = parser.next().unwrap();
        assert_eq!(packet.eval(), 9);

        let mut parser = PacketParser::from_hex("D8005AC2A8F0");
        let packet = parser.next().unwrap();
        assert_eq!(packet.eval(), 1);

        let mut parser = PacketParser::from_hex("F600BC2D8F");
        let packet = parser.next().unwrap();
        assert_eq!(packet.eval(), 0);

        let mut parser = PacketParser::from_hex("9C005AC2F8F0");
        let packet = parser.next().unwrap();
        assert_eq!(packet.eval(), 0);

        let mut parser = PacketParser::from_hex("9C0141080250320F1802104A08");
        let packet = parser.next().unwrap();
        assert_eq!(packet.eval(), 1);
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 938);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 1495959086337);
    }
}
