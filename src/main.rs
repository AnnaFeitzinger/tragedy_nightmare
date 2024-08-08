use core::str;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
	// don't change this line
	let mappings = read_tsv("./data/dmel_orthologs_in_drosophila_species_fb_2019_03.tsv").await?;
	// change this line if you want to add dmel gene ids. you can find the unoffical mappings -> dmel in the xml file.
	let unofficial_names = [
		"FBgn0065109", // 'ppk11'
		"FBgn0022981", // 'rpk'
		"FBgn0261722", // 'fwe'
		"FBgn0263391", // 'hts'
		"FBgn0266570", // 'NO66'
		"FBgn0052069", // 'CG32069'
		"FBgn0034970", // 'yki'
		"FBgn0004698", // 'Xpc'
		"FBgn0000140", // 'asp'
		"FBgn0002872", // 'mu2'
		"FBgn0001137", // 'grk'
	];
	// maps the dmel ids to the other gene ids with the tsv mappings
	let flybase_ids: Vec<String> = unofficial_names
		.iter()
		.flat_map(|name| {
			mappings
				.get(*name)
				.map(|mappings| {
					mappings
						.iter()
						.map(|m| m.flybase_id_other.clone())
						.collect::<Vec<_>>()
				})
				.unwrap_or_default()
		})
		.collect();
	// extracts sequences from species-specific genome in fasta format.
	// CHANGE THIS PATH by downloading from flybase the 2018/2019/latest fasta file and putting it into data/ and copying the path and pasting it in this fn
	let sequences = extract_sequences("./data/dpse-all-gene-r3.04.fasta", &flybase_ids).await?;

	// prints the sequences out, you can also use `noodles::fasta::io::Writer` to write to a new fasta file, as shown below.
	for (id, seq) in sequences {
		println!(">{}\n{}", id, seq);
	}
	Ok(())
}

async fn extract_sequences(
	fasta_file: &str,
	ids: &[String],
) -> tokio::io::Result<HashMap<String, (String, noodles::fasta::record::sequence::Sequence)>> {
	let mut reader = File::open(fasta_file)
		.map(BufReader::new)
		.map(noodles::fasta::io::Reader::new)?;

	let mut sequences = HashMap::new();
	for result in reader.records() {
		let record = result?;
		let id = str::from_utf8(record.name()).unwrap().to_string(); // this is the actual id for the species genome
		let description = str::from_utf8(record.description().as_ref().unwrap()).unwrap().to_string(); 
		if ids.contains(&id) {
			// checks if the non-dmel gene exists in the fasta record, if it does then we add it to `sequences`.
			sequences.insert(id, (description, record.sequence().clone()));
		}
	}

	Ok(sequences)
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
struct GeneMapping {
	/// The Dmel flybase ID (`FBgn<id_num>`).
	flybase_id_dmel: String,
	// The flybase ID for the `species_name` (`FBgn<id_num>`).
	flybase_id_other: String,
	/// The name of the "other" species (D*) (e.g. `Dpse`).
	species_name: String,
}

async fn read_tsv(tsv_path: &str) -> tokio::io::Result<HashMap<String, Vec<GeneMapping>>> {
	let mut reader = csv::ReaderBuilder::new()
		.delimiter(b'\t')
		.from_path(tsv_path)?;
	let mut mappings = HashMap::new();
	for result in reader.records() {
		let record = result?;
		let flybase_id_dmel = record[0].to_string();
		let flybase_id_other = record[5].to_string();
		let species_name = &record[6][0..4];
		mappings
			.entry(flybase_id_dmel.clone())
			.or_insert_with(Vec::new)
			.push(GeneMapping {
				flybase_id_dmel,
				flybase_id_other,
				species_name: species_name.to_string(),
			});
	}

	Ok(mappings)
}
