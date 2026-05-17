use clap::Parser;
use dsv_reader::*;
use postgres::{Client, GenericClient, NoTls};
use std::fs::File;
use std::io::Write;

#[derive(Parser)]
struct Args {
	#[arg(long, short, help = "input file path")]
	path: String,
}

fn main() {
	let args = Args::parse();
	let path = std::path::Path::new(&args.path);

	let file = File::open(path).unwrap();
	let stream = dsv_reader::BufStream::<_, b'\t'>::try_new(file).unwrap();
	let mut reader = dsv_reader::DsvReader::new(stream);
	let name = path.file_stem().unwrap().to_str().unwrap();

	let mut client =
		Client::connect("host=blue port=5432 dbname=bingo user=tokeiya3", NoTls).unwrap();

	{
		let mut tx = client.transaction().unwrap();

		let cmd = format!(
			r##"
				CREATE  TABLE IF NOT EXISTS {}(
					id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY ,
					player INTEGER NOT NULL ,
					round INTEGER NOT NULL ,
					rank INTEGER NOT NULL ,
					tie INTEGER NOT NULL,
					count INTEGER NOT NULL
				);"##,
			name
		);

		tx.execute(&cmd, &[]).unwrap();
		tx.commit().unwrap();
	}

	{
		let mut tx = client.transaction().unwrap();

		let cmd = format!(
			r##"COPY {}(player, round, rank, tie, count)  FROM STDIN WITH (FORMAT TEXT,DELIMITER E'\t' )"##,
			name
		);

		{
			let mut copy = tx.copy_in(&cmd).unwrap();
			let mut buffer = Vec::<String>::new();

			{
				let mut writer = std::io::BufWriter::with_capacity(8192, &mut copy);
				reader.fill(&mut buffer).unwrap();

				for i in 0..usize::MAX {
					buffer.clear();

					if let None = reader.fill(&mut buffer).unwrap() {
						break;
					}

					if buffer.len() == 0 {
						break;
					}

					let result = writeln!(
						writer,
						"{}\t{}\t{}\t{}\t{}",
						buffer[2], buffer[3], buffer[4], buffer[5], buffer[6]
					);

					if result.is_err() {
						println!("{result:?}")
					}

					if i & 0xFFFF == 0 {
						println!("processed {} lines", i);
					}
				}
			}

			copy.finish().unwrap();
		};

		tx.commit().unwrap();
	}
}
