use crate::{Capabilities, Letterbox, controller::genetic::chromosome::ChromosomeConfig};

use super::{Chromosome, GeneticConfig, GeneticController};

pub struct GeneticControllerBuilder {
    config: GeneticConfig,
    capabilities: Capabilities,
    thread_control: bool,
    pinning_control: bool,
    power_control: bool,
}

impl GeneticControllerBuilder {
    pub fn new(config: GeneticConfig, capabilities: Capabilities) -> Self {
        Self {
            config,
            capabilities,
            thread_control: false,
            power_control: false,
            pinning_control: false,
        }
    }

    pub fn thread_control(mut self, enabled: bool) -> Self {
        self.thread_control = enabled;
        self
    }

    pub fn power_control(mut self, enabled: bool) -> Self {
        self.power_control = enabled;
        self
    }

    pub fn pinning_control(mut self, enabled: bool) -> Self {
        self.pinning_control = enabled;
        self
    }

    /// Instead of randomly initialized values, use an even spread over valid thread
    /// counts and power limits to reduce duplication and increase the chances of
    /// finding an optimum immediately.
    pub fn build(self) -> GeneticController {
        let bounds = ChromosomeConfig {
            thread_control: self.thread_control,
            pinning_control: self.pinning_control,
            power_control: self.power_control,
            max_threads: self.capabilities.max_threads,
            min_power: self.capabilities.power_min,
            max_power: self.capabilities.power_max,
        };

        let population = (0..self.config.population_size)
            .map(|mut i| {
                if self.config.initial_population_descending {
                    i = self.config.population_size - i - 1;
                }

                let t = i as f32 / (self.config.population_size - 1) as f32;
                Chromosome::lerp(&bounds, t)
            })
            .collect();

        log::trace!("Init: {:?}", population);

        GeneticController {
            letterbox: Letterbox::new(self.config.population_size),
            population,
            immigration_cooldown: self.config.immigration_cooldown_generations,
            sort_descending: !self.config.initial_population_descending,
            effective_survival_rate: self.config.survival_rate,
            effective_mutation_rate: self.config.mutation_rate,
            config: self.config,
            bounds,
            generation: 0,
            immigration_was_triggered: false,
        }
    }
}
