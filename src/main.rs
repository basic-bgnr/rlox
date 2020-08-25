use std::env;

mod byte_array;
use byte_array::{Chunk, OpCode, Chunkable};

mod disassembler;
use disassembler::Disassembable;

fn main(){
	for argument in env::args() {
		println!("passed argument is : {:?}", argument);
	}


	let mut chunk = Chunk::new(100);

	chunk.write_chunk(OpCode::OpConstant as u8);
	chunk.write_chunk(123_u8);

	chunk.write_chunk(OpCode::OpReturn as u8);

	chunk.disassemble("main code");

	chunk.free();

}