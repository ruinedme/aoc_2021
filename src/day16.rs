const BITS_TABLE: [char; 16] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
];

const HEADER_WIDTH: usize = 3;
const BIT_WIDTH: usize = 4;
const GROUP_LEN: usize = 5;
const HEADER_MASK: usize = 7;
const CHUNK_SIZE: usize = 0x10;
const CHUNK_WIDTH: usize = BIT_WIDTH * CHUNK_SIZE;

pub fn run_day16(inputs: &String) {
    let day16_1 = day16_1(&inputs);
    println!("Day 1-1: {day16_1}");

    let day16_2 = day16_2(&inputs);
    println!("Day 1-2: {day16_2}");
}

fn day16_1(inputs: &String) -> usize {
    let packets = decode(inputs);

    let mut version_sum = 0;
    packets.iter().for_each(|x| {
        version_sum += x.version;
    });

    return version_sum;
}

fn day16_2(inputs: &String) -> usize {
    use PacketType::*;

    let mut packets = decode(inputs);

    let mut stack: Vec<usize> = Vec::with_capacity(packets.len()); //track the number of remaining packets for a given operation
    let mut pending_expression: Vec<Packet<PacketType>> = Vec::new(); //stack of pending operations and the associated value packets
    let mut type_0_packets = 0;
    let mut type0_len = 0;
    let mut is_endof_type0 = false;

    while packets.len() > 0 {
        let p = packets.pop().unwrap();

        //flag the end of a type 0 packet
        //type 0 packets can contain any number of sub expressions and can throw the normal stack count off
        if type_0_packets > 0 {
            type_0_packets -= 1;
            if type_0_packets == 0 {
                is_endof_type0 = true;
            }
        }

        //process next packet
        match p.packet_type {
            OpPacket {
                length_type_id,
                sub_packets_len,
            } => {
                #[allow(unused_assignments)]
                let mut packet_count = 0;

                match length_type_id {
                    //total length in bits of the sub-packets contained by this packet.
                    0 => {
                        //type 0 packets will use the next n packets regardless of sub packet depth
                        //scan backwards in the array until the length of this packet is 0
                        //take the count of all packets scanned as the packet count to push on to the stack
                        let mut index = packets.len() - 1;
                        let mut remaining_bits = sub_packets_len;

                        while remaining_bits != 0 {
                            let t = &packets[index];
                            remaining_bits -= t.length;
                            packet_count += 1;
                            //if first 2 packets  are type 0, index will try and subtract past 0
                            if remaining_bits == 0 {
                                break;
                            }
                            index -= 1;
                        }

                        //a type 0 packet can contain any number of sub packets that are also type 0, but will be contained with in the larger one.
                        //only set if we are not currently inside a type 0 packet
                        if type_0_packets == 0 {
                            type_0_packets = packet_count;
                            type0_len = sub_packets_len;
                        }
                    }
                    //number of sub-packets immediately contained by this packet.
                    1 => packet_count = sub_packets_len,
                    _ => unreachable!("Invalid Length Type Id found: {:?}", p),
                }

                //decrement previous count
                if stack.len() > 0 {
                    let depth = stack.len() - 1;
                    stack[depth] -= 1;
                }
                //push packet count to stack
                stack.push(packet_count);
            }
            LitPacket { value: _ } => {
                //decrement previous count
                if stack.len() > 0 {
                    let depth = stack.len() - 1;
                    stack[depth] -= 1;
                }
            }
        }

        //push packet to stack
        pending_expression.push(p);

        #[allow(unused_assignments)]
        let mut packet_len = 0;
        //eval pending expressions
        while *stack.last().unwrap() == 0 || is_endof_type0 {
            stack.pop();

            //get the index of the last operator packet
            let (op_index, op) = pending_expression
                .iter()
                .enumerate()
                .filter(|(_, x)| x.type_id != 4)
                .max_by(|(i, _), (j, _)| i.cmp(&j))
                .map(|(i, x)| (i, x))
                .unwrap();
            let op_len = op.length;

            //take the slice  of the pending expression starting from last entry of the stack index
            let mut slice: Vec<Packet<PacketType>> =
                pending_expression.drain(op_index..).rev().collect();

            let t = eval(&mut slice);
            packet_len = t.length - op_len;
            pending_expression.push(t);

            //evaluated expression matches length of expected type0 packet
            //no longer in a type 0 packet
            if is_endof_type0 && packet_len == type0_len {
                is_endof_type0 = false;
            }

            //Ran out of packets. Evaluate the remaining packets in stack
            //There should not be any nested expressions at this point
            if packets.len() == 0 {
                while pending_expression.len() > 1 {
                    let (op_index, _) = pending_expression
                        .iter()
                        .enumerate()
                        .filter(|(_, x)| x.type_id != 4)
                        .max_by(|(i, _), (j, _)| i.cmp(&j))
                        .unwrap();

                    let mut slice: Vec<Packet<PacketType>> =
                        pending_expression.drain(op_index..).rev().collect();
                    let t = eval(&mut slice);
                    pending_expression.push(t);
                }
                //end of while loop, don't iterate again
                break;
            }
        }
    }

    let ret_value = match pending_expression[0].packet_type {
        PacketType::LitPacket { value } => value,
        _ => unreachable!(
            "Expected value found Operator Packet, {:#?}",
            pending_expression
        ),
    };
    return ret_value;
}

/**
 * version: Version number of the header. Valid ranges 0-7
 * type_id: Type of packet. Valid ranges 0-7
 * length: length of the packet in bits.
 */
#[derive(Debug, Clone, Copy, PartialEq)]
struct Packet<T: PacketTrait> {
    version: usize,
    type_id: usize,
    length: usize,
    packet_type: T,
}

//if length_type_id == 0, sub_packet_len is in bits
//if length_type_id == 1, sub_packet_len is in total packets
#[derive(Debug, Clone, PartialEq)]
enum PacketType {
    OpPacket {
        length_type_id: usize,
        sub_packets_len: usize,
    },
    LitPacket {
        value: usize,
    },
}

trait PacketTrait {}
impl PacketTrait for PacketType {}

fn decode(inputs: &str) -> Vec<Packet<PacketType>> {
    let mut total_chunks = (inputs.len() as f32 / CHUNK_SIZE as f32).ceil() as usize;
    if total_chunks == 0 {
        total_chunks = 1;
    }

    let chunk_pad = CHUNK_SIZE - (inputs.len() % CHUNK_SIZE);
    let pad_str = std::iter::repeat('0').take(chunk_pad);
    let mut aligned_input = inputs.to_owned();
    aligned_input.push_str(&String::from_iter(pad_str));
    let mut chunks: Vec<usize> = Vec::with_capacity(total_chunks);

    //break input into usize chunks
    for chunk in 0..total_chunks {
        let offset = chunk * CHUNK_SIZE;
        let encoded_chunk = &aligned_input[offset..(CHUNK_SIZE + offset)];

        //decode the chunks
        let mut value = 0;
        for c in encoded_chunk.chars() {
            BITS_TABLE
                .iter()
                .enumerate()
                .filter(|(_, &x)| c == x)
                .for_each(|(i, _)| {
                    value = value << BIT_WIDTH;
                    value += i;
                });
        }
        chunks.push(value);
    }
    //flip the Vec so we can work from the end using pop()
    chunks.reverse();

    //Convert chunks to Vec of Packets
    return read_chunks(&mut chunks);
}

fn read_chunks(chunks: &mut Vec<usize>) -> Vec<Packet<PacketType>> {
    let mut packets: Vec<Packet<PacketType>> = Vec::new();

    let mut buffer_offset: usize = 0;
    let mut buffer = chunks.pop().expect("Expected Non Empty Vec");
    let mut buffer2: usize = 0;
    let mut buff_read = CHUNK_WIDTH;

    if chunks.len() > 0 {
        buffer2 = chunks.pop().unwrap();
    }

    while buffer != 0 {
        let version = buffer >> (CHUNK_WIDTH - HEADER_WIDTH);
        let type_id: usize = (buffer >> (CHUNK_WIDTH - (HEADER_WIDTH * 2))) & HEADER_MASK;
        buffer_offset += HEADER_WIDTH * 2;

        match type_id {
            4 => {
                let continue_mask: usize = 0x10;
                let mut has_next = true;
                let mut value = 0;
                //values are groups of 5 bits
                //left most bit determines if there is another group
                while has_next {
                    buffer_offset += GROUP_LEN;
                    let loop_offset = CHUNK_WIDTH - buffer_offset;

                    has_next =
                        ((continue_mask << loop_offset) & buffer) >> loop_offset == continue_mask;
                    //get the value of this group of packets
                    let g = ((0xF << loop_offset) & buffer) >> loop_offset;
                    //shift and group value
                    value = (value << BIT_WIDTH) + g;
                }

                packets.push(Packet {
                    version,
                    type_id,
                    length: buffer_offset,
                    packet_type: PacketType::LitPacket { value },
                });
            }
            0..=7 => {
                //Get length type bit
                let length_type_id_len = 1;
                let length_type_id_mask = 1 << (CHUNK_WIDTH - buffer_offset - length_type_id_len);
                let _length_type_id =
                    (buffer & length_type_id_mask) >> length_type_id_mask.trailing_zeros();

                //add offset and set subpacket mask
                //0 is length in bits
                //1 is length in packets
                let length_mask = match _length_type_id {
                    0 => {
                        buffer_offset += 15;
                        0x7FFF << (CHUNK_WIDTH - buffer_offset - length_type_id_len)
                    }
                    1 => {
                        buffer_offset += 11;
                        0x7FF << (CHUNK_WIDTH - buffer_offset - length_type_id_len)
                    }
                    _ => unimplemented!("Invalid Packet"),
                };

                buffer_offset += length_type_id_len;
                //get sub packet length
                let sub_packets_len: usize = (buffer & length_mask) >> length_mask.trailing_zeros();

                packets.push(Packet {
                    version,
                    type_id,
                    length: buffer_offset,
                    packet_type: PacketType::OpPacket {
                        length_type_id: _length_type_id,
                        sub_packets_len,
                        // sub_packets: Vec::new()
                    },
                })
            }
            _ => unreachable!("Invalid Packet alignment"),
        }

        //mask away used bits in the offset
        //shift buffer left by the offset
        let buff_mask =
            ((2usize.pow(buffer_offset as u32) - 1) as usize) << (CHUNK_WIDTH - buffer_offset);

        //shift buff left
        buffer = (buffer & !buff_mask) << buffer_offset;

        //add in left most bits from buff2
        buffer = buffer + ((buffer2 & buff_mask) >> (CHUNK_WIDTH - buffer_offset));

        //shift buff2 left
        buffer2 = (buffer2 & !buff_mask) << buffer_offset;

        //read in next chunk
        if buff_read < buffer_offset && chunks.len() > 0 {
            //push remaining bits into buffer
            if buff_read != 0 {
                let m0 = (2usize.pow(buff_read as u32) - 1) << (CHUNK_WIDTH - buff_read);
                buffer += (buffer2 & m0) >> (CHUNK_WIDTH - buff_read);
                buffer_offset -= buff_read;
            }

            //refill buffer2
            buffer2 = chunks.pop().unwrap();

            //shift remaining offset into buffer
            let t_mask = (2usize.pow(buffer_offset as u32) - 1) << (CHUNK_WIDTH - buffer_offset);
            buffer += (buffer2 & t_mask) >> (CHUNK_WIDTH - buffer_offset);
            buffer2 = (!t_mask & buffer2) << buffer_offset;
            buff_read = CHUNK_WIDTH;
        }

        //prepare for next iteration
        buff_read = buff_read.abs_diff(buffer_offset);
        buffer_offset = 0;
    }

    packets.reverse();
    return packets;
}

/// Returns a LiteralPacket from the evaluated expression
fn eval(packets: &mut Vec<Packet<PacketType>>) -> Packet<PacketType> {
    use PacketType::*;

    let bit_length: usize = packets.iter().map(|x| x.length).sum();
    let packet_version: usize = packets.iter().map(|x| x.version).sum();
    let mut ret_value = 0;

    let p = packets.pop().unwrap();
    match p.packet_type {
        OpPacket {
            length_type_id: _,
            sub_packets_len: _,
        } => match p.type_id {
            0 => {
                ret_value = packets
                    .iter()
                    .map(|x| match x.packet_type {
                        LitPacket { value } => value,
                        _ => unreachable!(),
                    })
                    .sum();
            }
            1 => {
                ret_value = match packets.pop().unwrap().packet_type {
                    LitPacket { value } => value,
                    _ => unreachable!(),
                };
                packets.iter().for_each(|x| match x.packet_type {
                    LitPacket { value } => ret_value *= value,
                    _ => unreachable!(),
                });
            }
            2 => {
                ret_value = packets
                    .iter()
                    .map(|x| match x.packet_type {
                        LitPacket { value } => value,
                        _ => unreachable!(),
                    })
                    .min_by(|a, b| a.cmp(&b))
                    .unwrap();
            }
            3 => {
                ret_value = packets
                    .iter()
                    .map(|x| match x.packet_type {
                        LitPacket { value } => value,
                        _ => unreachable!(),
                    })
                    .max_by(|a, b| a.cmp(&b))
                    .unwrap();
            }
            4 => unreachable!(),
            5 => {
                let a = match packets
                    .pop()
                    .expect("Expected exactly 2 values found None.")
                    .packet_type
                {
                    LitPacket { value } => value,
                    _ => unreachable!(),
                };
                let b = match packets
                    .pop()
                    .expect("Expected exactly 2 values, found only 1")
                    .packet_type
                {
                    LitPacket { value } => value,
                    _ => unreachable!(),
                };

                if a > b {
                    ret_value = 1;
                }
            }
            6 => {
                let a = match packets
                    .pop()
                    .expect("Expected exactly 2 values found None.")
                    .packet_type
                {
                    LitPacket { value } => value,
                    _ => unreachable!(),
                };
                let b = match packets
                    .pop()
                    .expect("Expected exactly 2 values, found only 1")
                    .packet_type
                {
                    LitPacket { value } => value,
                    _ => unreachable!(),
                };
                if a < b {
                    ret_value = 1;
                }
            }
            7 => {
                let a = match packets
                    .pop()
                    .expect("Expected exactly 2 values found None.")
                    .packet_type
                {
                    LitPacket { value } => value,
                    _ => unreachable!(),
                };
                let b = match packets
                    .pop()
                    .expect("Expected exactly 2 values, found only 1")
                    .packet_type
                {
                    LitPacket { value } => value,
                    _ => unreachable!(),
                };
                if a == b {
                    ret_value = 1;
                }
            }
            _ => unreachable!("Invalid Type Id found: {:?}", p),
        },
        LitPacket { value } => ret_value = value,
    }

    return Packet {
        version: packet_version,
        type_id: 4,
        length: bit_length,
        packet_type: PacketType::LitPacket { value: ret_value },
    };
}
