#[derive(Debug)]
pub enum OpCode{
	//return from the context
	OpReturn,
	//load constant from the `values` array from the Chunk, 
	//this opcode is followed by 1 byte long index to load the value from
	OpConstant,
	//mathematical opcodes
	
	OpAdd,
	OpSub,
	OpMul,
	OpDiv,
	OpNeg,
}

#[derive(Debug)]
pub struct Chunk{
	//this hold the values of the constant in the program
	pub values: ValueArray,
	// this holds the instruction to the vm 
	pub codes : Vec<u8>,
	// this stores the line no corresponding the instruction set, for runtime error identification purpose
	pub lines : Vec<usize>,

}

pub trait Chunkable{
	fn new(capacity: usize) -> Chunk;
	fn write_chunk(&mut self, new_data: u8, atline: usize);
	fn free(self);
	fn get_codes(&self) -> &[u8];
	fn get_values(&self) -> &[Value];
	fn get_lines(&self) -> &[usize];
}

impl Chunkable for Chunk {
	fn new(capacity: usize) -> Chunk {
		Chunk{ codes : Vec::with_capacity(capacity), 
			   values: ValueArray::new(capacity),
			   lines: Vec::with_capacity(capacity)}
	}
	fn write_chunk(&mut self, new_data: u8, atline: usize){
		self.codes.push(new_data);
		self.lines.push(atline);
	}
	fn free(self) {
	}

	fn get_codes(&self) -> &[u8] {
		&self.codes
	}
	fn get_values(&self) -> &[Value]{
		&self.values.get_values()
	}
	fn get_lines(&self) -> &[usize]{
		&self.lines
	}
}


pub type Value = isize;

#[derive(Debug)]
pub struct ValueArray { 
  values: Vec<Value>
}

impl ValueArray {
	pub fn new(capacity: usize) -> ValueArray{
		ValueArray { values: Vec::with_capacity(capacity) }
	}
	// returns the return adderess of the input data
	pub fn write_value(&mut self, value: Value) -> usize {
		self.values.push(value);
		return self.values.len() - 1;
	}

	pub fn free(self){
	}

	pub fn get_values(&self)-> &[Value]{
		&self.values
	}

}