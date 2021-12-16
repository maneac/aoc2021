use super::*;
use std::fs::write;
use test::Bencher;

const PART_1: usize = 877;
const PART_2: usize = 194435634456;

#[test]
fn test_part_1_real() {
    let data = read_data("../../data");

    assert_eq!(PART_1, part_1(&data));
}

#[test]
fn test_part_2_real() {
    let data = read_data("../../data");

    assert_eq!(PART_2, part_2(&data));
}

#[test]
fn test_read_data() {
    let input = "8A004A801A8002F478";

    let expected = Packet::Operator(OperatorPacket {
        version: 4,
        operator: Operation::Minimum,
        sub_packets: vec![Packet::Operator(OperatorPacket {
            version: 1,
            operator: Operation::Minimum,
            sub_packets: vec![Packet::Operator(OperatorPacket {
                version: 5,
                operator: Operation::Minimum,
                sub_packets: vec![Packet::Literal(LiteralPacket {
                    version: 6,
                    value: 15,
                })],
            })],
        })],
    });

    write(Path::new("/tmp").join("day_16.txt"), input).unwrap();

    assert_eq!(expected, read_data("/tmp"));
}

#[test]
fn test_input_parse_literal() {
    let data = Input::from("D2FE28");

    let expected = Packet::Literal(LiteralPacket {
        version: 6,
        value: 2021,
    });

    assert_eq!(expected, data);
}

#[test]
fn test_input_parse_operator_type_0() {
    let data = Input::from("38006F45291200");

    let expected = Packet::Operator(OperatorPacket {
        version: 1,
        operator: Operation::LessThan,
        sub_packets: vec![
            Packet::Literal(LiteralPacket {
                version: 6,
                value: 10,
            }),
            Packet::Literal(LiteralPacket {
                version: 2,
                value: 20,
            }),
        ],
    });

    assert_eq!(expected, data);
}

#[test]
fn test_input_parse_operator_type_1() {
    let data = Input::from("EE00D40C823060");

    let expected = Packet::Operator(OperatorPacket {
        version: 7,
        operator: Operation::Maximum,
        sub_packets: vec![
            Packet::Literal(LiteralPacket {
                version: 2,
                value: 1,
            }),
            Packet::Literal(LiteralPacket {
                version: 4,
                value: 2,
            }),
            Packet::Literal(LiteralPacket {
                version: 1,
                value: 3,
            }),
        ],
    });

    assert_eq!(expected, data);
}

#[test]
fn test_part_1_example_1() {
    let data = Input::from("8A004A801A8002F478");

    assert_eq!(16, part_1(&data));
}

#[test]
fn test_part_1_example_2() {
    let data = Input::from("620080001611562C8802118E34");

    assert_eq!(12, part_1(&data));
}

#[test]
fn test_part_1_example_3() {
    let data = Input::from("C0015000016115A2E0802F182340");

    assert_eq!(23, part_1(&data));
}

#[test]
fn test_part_1_example_4() {
    let data = Input::from("A0016C880162017C3686B18A3D4780");

    assert_eq!(31, part_1(&data));
}

#[test]
fn test_part_2_sum() {
    let data = Input::from("C200B40A82");

    assert_eq!(3, part_2(&data));
}

#[test]
fn test_part_2_product() {
    let data = Input::from("04005AC33890");

    assert_eq!(54, part_2(&data));
}

#[test]
fn test_part_2_minimum() {
    let data = Input::from("880086C3E88112");

    assert_eq!(7, part_2(&data));
}

#[test]
fn test_part_2_maximum() {
    let data = Input::from("CE00C43D881120");

    assert_eq!(9, part_2(&data));
}

#[test]
fn test_part_2_less_than() {
    let data = Input::from("D8005AC2A8F0");

    assert_eq!(1, part_2(&data));
}

#[test]
fn test_part_2_greater_than() {
    let data = Input::from("F600BC2D8F");

    assert_eq!(0, part_2(&data));
}

#[test]
fn test_part_2_equal_to() {
    let data = Input::from("9C005AC2F8F0");

    assert_eq!(0, part_2(&data));
}

#[test]
fn test_part_2_nested_equal_to() {
    let data = Input::from("9C0141080250320F1802104A08");

    assert_eq!(1, part_2(&data));
}

#[bench]
fn bench_read_data(b: &mut Bencher) {
    b.iter(|| {
        let data = read_data("../../data");

        assert_ne!(data, Packet::Empty);
    })
}

#[bench]
fn bench_part_1(b: &mut Bencher) {
    let data = read_data("../../data");

    b.iter(|| {
        assert_eq!(PART_1, part_1(&data));
    })
}

#[bench]
fn bench_part_2(b: &mut Bencher) {
    let data = read_data("../../data");

    b.iter(|| {
        assert_eq!(PART_2, part_2(&data));
    })
}
