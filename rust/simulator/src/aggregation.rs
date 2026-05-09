use dsv_writer::*;

const ROUND_OFFSET:usize=4;
const ROUND_SIZE:usize=68;
pub struct Table<const N:usize>{
	data:[[usize;N];ROUND_SIZE]
}

impl<const N:usize> Default for Table<N>{
	fn default() -> Self {
		Self {
			data: [[0; N]; ROUND_SIZE]
		}
	}
}

impl<const N:usize> Table<N>{
	pub fn set(&mut self,round:usize,value:usize){
		todo!()
	}
	
}

impl<const N:usize> ToDsv<std::io::Error> for Table<N>{
	fn to_dsv<T: Encoder>(&self, writer: &mut T) -> ToDsvResult<(),std::io::Error> {
		writer.write_str_field("rank",QuoteMode::AutoDetect)?;
		
		for idx in ROUND_OFFSET..=ROUND_OFFSET+ROUND_SIZE{
			writer.write_value_field(&idx, QuoteMode::AutoDetect)?;
		}
		
		writer.end_of_record(false)?;
		
		for (rank,arr) in self.data.iter().enumerate(){
			writer.write_value_field(&rank, QuoteMode::AutoDetect)?;
			for value in arr.iter(){
				writer.write_value_field(&value, QuoteMode::AutoDetect)?;
			}
			writer.end_of_record(false)?;
		}
		
		Ok(())
	}
}