use std::env;

mod byte_array;
use byte_array::{Chunk, OpCode, Chunkable};

mod disassembler;
use disassembler::Disassembable;

mod vm;
use vm::Vm;

fn main(){
	for argument in env::args() {
		println!("passed argument is : {:?}", argument);
	}


	let mut chunk = Chunk::new(100);

	///////////////////////////////////////////////
	let constant_to_write = 10;
	let index = chunk.values.write_value(constant_to_write);

	chunk.write_chunk(OpCode::OpConstant as u8, 1);
	chunk.write_chunk(index as u8, 1);
 
	///////////////////////////////////////////////
	let constant_to_write = 100;
	let index = chunk.values.write_value(constant_to_write);

	chunk.write_chunk(OpCode::OpConstant as u8, 1);
	chunk.write_chunk(index as u8, 2);
	///////////////////////////////////////////////
	chunk.write_chunk(OpCode::OpAdd as u8, 3);
	/////###################################//////

	///////////////////////////////////////////////
	let constant_to_write = 110;
	let index = chunk.values.write_value(constant_to_write);

	chunk.write_chunk(OpCode::OpConstant as u8, 1);
	chunk.write_chunk(index as u8, 3);
	///////////////////////////////////////////////
	chunk.write_chunk(OpCode::OpDiv as u8, 3);
	/////###################################//////

	chunk.write_chunk(OpCode::OpReturn as u8, 3);
	chunk.disassemble("main code");


	let mut v_machine = Vm::new(&chunk);

	v_machine.run();



	chunk.free();

}