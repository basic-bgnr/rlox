#[derive(Debug)]
pub enum OpCode{
	//return from the function context
	OpReturn,
	//load constant from the `values` array from the Chunk, 
	//this opcode is followed by 1 byte long index to load the value from
	OpConstant,
}

#[derive(Debug)]
pub struct Chunk{
	values: ValueArray,
	codes : Vec<u8>,
}

pub trait Chunkable{
	fn new(capacity: usize) -> Chunk;
	fn write_chunk(&mut self, new_data: u8);
	fn free(self);
	fn get_codes(&self) -> &[u8];
	fn get_values(&self) -> &[Value];
}

impl Chunkable for Chunk {
	fn new(capacity: usize) -> Chunk {
		Chunk{ codes : Vec::with_capacity(capacity), values: ValueArray::new(capacity)}
	}
	fn write_chunk(&mut self, new_data: u8){
		self.codes.push(new_data);
	}
	fn free(self) {
	}

	fn get_codes(&self) -> &[u8] {
		&self.codes
	}
	fn get_values(&self) -> &[Value]{
		&self.values.get_values()
	}
}


type Value = usize;

#[derive(Debug)]
struct ValueArray { 
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