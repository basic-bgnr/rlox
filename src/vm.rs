use super::byte_array::{Chunk, OpCode, Chunkable, Value};

pub struct Vm<'a>{
	chunk: &'a Chunk,
	ip: usize,
	stack: Vec<Value>,
}

enum BinaryOperation{
	Add,
	Sub,
	Mul,
	Div,
}

impl<'a> Vm<'a>{
	pub fn new(chunk: &'a Chunk) -> Vm<'a>{
		Vm{chunk: chunk, ip: 0, stack: Vec::new()}
	}

	pub fn run(&mut self){
		loop{

			match self.get_instruction() {

				op_return if op_return == OpCode::OpReturn as u8 => {
												let ret_val = self.stack.pop();
												println!("returned {:?}", ret_val);
												break;	
										  },

				op_constant if op_constant == OpCode::OpConstant as u8 => {					                                        
												let constant = self.load_constant();
												self.stack.push(constant);
												println!("Constant Loaded: {:?}", constant);
											},
				op_add if op_add == OpCode::OpAdd as u8 => {let result = self.perform_binary_operation(BinaryOperation::Add);self.stack.push(result);println!("Add: {:?}", result);},
				op_sub if op_sub == OpCode::OpSub as u8 => { let result = self.perform_binary_operation(BinaryOperation::Sub); self.stack.push(result); println!("Sub: {:?}", result);},
				op_mul if op_mul == OpCode::OpMul as u8 => { let result = self.perform_binary_operation(BinaryOperation::Mul); self.stack.push(result); println!("Mult: {:?}", result);},
				op_div if op_div == OpCode::OpDiv as u8 => { let result = self.perform_binary_operation(BinaryOperation::Div); self.stack.push(result); println!("Div: {:?}", result);},

				op_neg if op_neg == OpCode::OpNeg as u8 => {if let Some(operand1) = self.stack.pop() { let result = -1 * operand1; self.stack.push(result); println!("{:?}", result);} else { panic!("insufficient operand to perform neg operation")} },

				_                   => break,

			}

		};
	}
	fn get_instruction(&mut self) -> u8 {
		return self.chunk.get_codes()[self.advance()]
	}

	fn load_constant(&mut self) -> Value {
		let index = self.get_instruction();
		let constant = self.chunk.get_values()[index as usize];
		return constant;

	}
	fn get_stack_items(&mut self, size:usize){

	}

	fn perform_binary_operation(&mut self, operation:BinaryOperation) -> Value {
		// println!("{:?}", self.stack);
		if let (Some(operand1), Some(operand2)) = (self.stack.pop(), self.stack.pop()){

			match operation {
				BinaryOperation::Add => operand1 + operand2,
				BinaryOperation::Sub => operand1 - operand2,
				BinaryOperation::Mul => operand1 * operand2,
				BinaryOperation::Div => operand1 / operand2,
			}

		}else{
			panic!("insufficient operands to perform binary operation")
		}

	}

	fn advance(&mut self) -> usize {
		self.ip += 1;
		self.ip-1
	}
}