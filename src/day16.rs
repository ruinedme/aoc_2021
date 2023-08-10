#[allow(dead_code)]
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
    let mut total_chunks = (inputs.len() as f32/ CHUNK_SIZE as f32).ceil() as usize;
    if total_chunks == 0 {
        total_chunks = 1;
    }
    
    let chunk_pad = CHUNK_SIZE - (inputs.len() % CHUNK_SIZE);
    let pad_str = std::iter::repeat('0').take(chunk_pad);
    let mut aligned_input = inputs.to_owned();
    aligned_input.push_str(&String::from_iter(pad_str));

    let mut chunks: Vec<usize> = Vec::with_capacity(total_chunks);
    for c in 0..total_chunks {
        let offset = c * CHUNK_SIZE;
        if offset + CHUNK_SIZE < aligned_input.len() {
            chunks.push(decode(&aligned_input[offset..(CHUNK_SIZE+offset)]));
        }else {
            chunks.push(decode(&aligned_input[offset..]));
        }
    }
    
    //read_chunks() uses pop() when reading in the next chunk.
    chunks.reverse();
    let packets = read_chunks(&mut chunks);
    let mut version_sum = 0;
    packets.iter().for_each(|x|{
        version_sum += x.header.version;
    });

    return version_sum;
}

fn day16_2(_inputs: &String) -> usize {
    return 0;
}


#[derive(Debug)]
struct Packet<T: PacketTrait> {
    header: Header,
    _packet_type: T,
}

#[derive(Debug)]
struct Header {
    version: usize,
    _type_id: usize,
}

#[derive(Debug)]
enum PacketType {
    OpPacket {
        _length_type_id: usize,
        _sub_packets: usize,
    },
    LitPacket {
        _value: usize,
    },
}

trait PacketTrait {}
impl PacketTrait for PacketType {}

fn decode(inputs: &str) -> usize {
    let mut value = 0;
    for c in inputs.chars() {
        BITS_TABLE
            .iter()
            .enumerate()
            .filter(|(_, &x)| c == x)
            .for_each(|(i, _)| {
                value = value << BIT_WIDTH;
                value += i;
            });
    }
    return value;
}

fn read_chunks(chunks: &mut Vec<usize>) -> Vec<Packet<PacketType>> {
    
    let mut packets: Vec<Packet<PacketType>> = Vec::new();
    
    let mut buffer_offset: usize = 0;
    let mut buffer = chunks.pop().unwrap();
    let mut buffer2: usize = 0;
    let mut buff_read = CHUNK_WIDTH;

    if chunks.len() > 0 {
        buffer2 = chunks.pop().unwrap();    
    }
    
    
    while buffer != 0 {
        let version = buffer >> (CHUNK_WIDTH - HEADER_WIDTH );
        let type_id: usize = (buffer >> (CHUNK_WIDTH - (HEADER_WIDTH*2) )) & HEADER_MASK;
        buffer_offset += HEADER_WIDTH * 2;

        match type_id {
            4 => {
                let continue_mask: usize = 0x10;
                let mut has_next = true;
                let mut value = 0;
                while has_next {
                    buffer_offset += GROUP_LEN;
                    let loop_offset = CHUNK_WIDTH - buffer_offset;

                    has_next = ((continue_mask << loop_offset) & buffer) >> loop_offset == continue_mask;
                    let g = ((0xF << loop_offset) & buffer) >> loop_offset;
                    value = (value << BIT_WIDTH) + g;
                }

                packets.push(Packet { header: Header { version, _type_id: type_id }, _packet_type: PacketType::LitPacket { _value: value } });
            },
            0..=7 => {
                let length_type_id_len = 1;
                let length_type_id_mask = 1 << (CHUNK_WIDTH - buffer_offset - length_type_id_len);
                let length_type_id = (buffer & length_type_id_mask) >> length_type_id_mask.trailing_zeros();
                
                let length_mask = match length_type_id  {
                    0 => {                        
                        buffer_offset += 15;
                        0x7FFF << (CHUNK_WIDTH - buffer_offset - length_type_id_len)
                    },
                    1 => {
                        buffer_offset += 11;
                        0x7FF << (CHUNK_WIDTH - buffer_offset - length_type_id_len)
                    },
                    _ => unimplemented!("Invalid Packet")
                };

                buffer_offset += length_type_id_len;

                let sub_packet_length = (buffer & length_mask) >> length_mask.trailing_zeros();

                packets.push(Packet { header: Header { version, _type_id: type_id }, _packet_type: PacketType::OpPacket { _length_type_id: length_type_id, _sub_packets: sub_packet_length } })               
            },
            _ => unimplemented!("Invalid Packet alignment")
        }

        feed_buffer(&mut buffer, &mut buffer_offset, &mut buffer2);
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
        
        //decrement buff_read
        buff_read = buff_read.abs_diff(buffer_offset);
        //reset offset
        buffer_offset = 0;
    }
    
    return packets;
}

//mask away used bits in the offset
//shift buffer left by the offset
fn feed_buffer(buffer: &mut usize, offset: &mut usize, buffer2: &mut usize){
    
    let buff_mask = ((2usize.pow(*offset as u32) -1) as usize) << (CHUNK_WIDTH - *offset);
    
    //shift buff left
    *buffer = (*buffer & !buff_mask) << *offset;

    //add in left most bits from buff2
    *buffer = *buffer + ((*buffer2 & buff_mask) >> (CHUNK_WIDTH - *offset));
    
    //shift buff2 left
    *buffer2 = (*buffer2 & !buff_mask) << *offset;
}
