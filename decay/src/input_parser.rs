use std::io::BufferedReader;
use std::io::File;
use std::io::IoError;
use std::collections::HashSet;

#[deriving(Show, Clone)]
pub struct SpeciesProbability {
    species: String,
    lambda: f64,
}

#[deriving(Show)]
pub struct DecayScenario {
    t: i64,
    lambdas: Vec<SpeciesProbability>, 
}

impl DecayScenario {
    pub fn from_file(fpath: &str) -> DecayScenario {
        let path = Path::new(fpath);
        let mut file = BufferedReader::new(File::open(&path));
        let lines: Vec<String> = file.lines().map(clean_string).collect();
        let t = from_str(lines[0].as_slice()).unwrap();
        let mut lambdas = Vec::new();
        let mut decay_chain = lines[1].as_slice().split_str("->");

        let mut chain_species_set: HashSet<String> = HashSet::new();
        let mut lambda_species_set: HashSet<String> = HashSet::new();

        for species in decay_chain {
            let trimmed = species.trim_chars(|c: char| c.is_whitespace());
            chain_species_set.insert(String::from_str(trimmed));
        } 

        for line in lines.slice(2, lines.len()).iter() {
            let line_frags: Vec<&str> = line.as_slice().split_str(":")
                .collect();
            let specie: String = line_frags[0].trim_chars(
                |c: char| c.is_whitespace()).to_string();
            let lambda: f64 = from_str(line_frags[1].trim_chars(
                    |c: char| c.is_whitespace())).unwrap();
            lambdas.push(SpeciesProbability { species: specie.clone(), lambda: lambda});
            lambda_species_set.insert(specie.clone());
        }

        if chain_species_set != lambda_species_set {
            fail!("Decay chain does not contain the same species as the \
                  decay probabilities");
        }

        DecayScenario{t: t, lambdas: lambdas}
    }

    pub fn simulate(&self) -> Vec<SpeciesProbability>{
        let mut accum = self.lambdas.clone();

        // initialize simulation structure
        for (i, spec_accum) in accum.iter_mut().enumerate() {
            spec_accum.lambda = match i {
                0 => 1.0,
                _ => 0.0,
            }
        }

        for _ in range(0, self.t) {
            let mut delta = 0_f64;
            for (spec, spec_accum) in self.lambdas.iter().zip(accum.iter_mut()) {
                let delta_0 = spec_accum.lambda * spec.lambda;
                spec_accum.lambda -= delta_0; 
                spec_accum.lambda += delta;
                delta = delta_0;
            }
        }
                      
        accum
    }
}

fn clean_string(s: Result<String, IoError>) -> String {
    let string = s.unwrap();
    let stripped = string.as_slice().trim_chars(|c: char| c.is_whitespace());
    String::from_str(stripped)
}
