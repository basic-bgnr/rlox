use super::byte_array::{Chunk, OpCode, Chunkable};

pub trait Disassembable{
	fn disassemble(&self, name: &str);
	fn disassemble_opcode(&self);
	fn print(&self, instruction_string:&str, line_num:usize);
}

impl<T:Chunkable> Disassembable for T{ 
	fn disassemble(&self, name : &str) { 
		println!(" ** {} **", name); 
		self.disassemble_opcode();
	}

	fn disassemble_opcode(&self){
		let mut instruction_iterator = self.get_codes().iter();
		let mut line_iterator = self.get_lines().iter();

		let mut zipped_iterator = instruction_iterator.zip(line_iterator);
		loop {
			match zipped_iterator.next() {
					Some((&code, &line_num)) if code == OpCode::OpReturn as u8   => self.print("OP_RETURN", line_num),
					Some((&code, &line_num)) if code == OpCode::OpConstant as u8 => if let Some((&value, &line_num)) = zipped_iterator.next(){
						                                                                let formatted_string = format!("{} {}", "OP_CONSTANT", value);
																						self.print(&formatted_string, line_num);		
																					}, 
					None                                                         => break,
					_                                                            => println!("unknown op code"),
			}
		}
	}
	fn print(&self, instruction_string: &str, line_num: usize){
		println!("line {}: {:}", line_num, instruction_string);
	}
}