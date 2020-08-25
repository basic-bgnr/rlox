use super::byte_array::{Chunk, OpCode, Chunkable};

pub trait Disassembable{
	fn disassemble(&self, name: &str);
	fn disassemble_opcode(&self);
}

impl<T:Chunkable> Disassembable for T{ 
	fn disassemble(&self, name : &str) { 
		println!(" ** {} **", name); 
		self.disassemble_opcode();
	}

	fn disassemble_opcode(&self){
		let mut chunk_iterator = self.get_codes().iter();
		loop {
			match chunk_iterator.next() {
					Some(&code) if code == OpCode::OpReturn as u8   => println!("{} OpReturn", code),
					Some(&code) if code == OpCode::OpConstant as u8 => if let Some(&index) = chunk_iterator.next(){
																			// let value = self.get_values()[index as usize];
																			println!("{} OpConstant {}", code, index);
																		},
					None                                            => break,
					_                                               => println!("unknown op code"),
			}
		}
	}
}