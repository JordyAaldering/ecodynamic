use clap::{Parser, Subcommand};
use controller::*;

fn parse_range(s: &str) -> Result<Vec<u32>, String> {
    let mut cores = Vec::new();

    for comma in s.split(',').collect::<Vec<&str>>() {
        let dash: Vec<&str> = comma.split('-').collect();
        match dash[..] {
            [r1] => {
                let core: u32 = r1.parse().map_err(|_| format!("Invalid range start: {}", r1))?;
                cores.push(core);
            },
            [r1, r2] => {
                let start: u32 = r1.parse().map_err(|_| format!("Invalid range start: {}", r1))?;
                let end: u32 = r2.parse().map_err(|_| format!("Invalid range start: {}", r2))?;
                let mut range: Vec<u32> = (start..=end).collect();
                cores.append(&mut range);
            },
            _ => return Err(format!("Invalid range format: {}", s)),
        }
    }

    println!("{:?}", cores);

    Ok(cores)
}

#[derive(Clone, Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Config {
    /// Controller type.
    #[command(subcommand)]
    pub controller: ControllerType,

    /// Performance (P) cores
    #[arg(short('P'), long, value_parser(parse_range))]
    pub p_cores: std::vec::Vec<u32>,

    /// Efficiency (E) cores
    #[arg(short('E'), long, value_parser(parse_range))]
    pub e_cores: Option<std::vec::Vec<u32>>,

    /// Logical (SMT) cores
    #[arg(short('L'), long, value_parser(parse_range))]
    pub l_cores: Option<std::vec::Vec<u32>>,

    /// Size of the letterbox.
    #[arg(short('s'), long)]
    pub letterbox_size: usize,

    /// Run the resource controller for a single connection only.
    #[arg(long, action)]
    pub single: bool,
}

#[derive(Clone, Debug, Subcommand)]
pub enum ControllerType {
    /// Genetic algorithm approach.
    Genetic(GeneticControllerConfig),
    /// Algorithm based on a performance corridor.
    Corridor(CorridorControllerConfig),
    /// Algorithm based on deltas between runs.
    Delta(DeltaControllerConfig),
    /// Continuously oscilates between 1 and <max-threads>.
    Oscilating,
    /// Always returns <max-threads>.
    Fixed,
}

impl Config {
    pub fn build(&self, req: Request, power_limit_uw: u64) -> Box<dyn Controller> {
        use ControllerType::*;
        match &self.controller {
            Genetic(config) => {
                println!("Building genetic controller with {} max threads, {} max power, and config {:?}",
                         req.max_threads, power_limit_uw, config);
                Box::new(GeneticController::new(req.max_threads, power_limit_uw, self.letterbox_size, config.clone()))
            },
            Corridor(config) => Box::new(CorridorController::new(req.max_threads, config.clone())),
            Delta(config) => Box::new(DeltaController::new(req.max_threads as f32, config.clone())),
            Oscilating => Box::new(OscilatingController::new(req.max_threads)),
            Fixed => Box::new(FixedController::new(req.max_threads)),
        }
    }
}
