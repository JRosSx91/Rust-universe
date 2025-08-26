use std::error::Error;
use rand::prelude::*;
use std::f64::consts::PI;

// --- LEVEL 0 ---
const C: f64 = 299_792_458.0;
const H_BAR: f64 = 1.054_571_817e-34;
const EPSILON_0: f64 = 8.854_187_81e-12;
const STABILITY_THRESHOLD_S: f64 = 1.0e-9;

// --- LEVEL 1 ---
#[derive(Debug, Clone)]
struct ParticleBlueprint {
    name: &'static str,
    spin: f64,
}

lazy_static::lazy_static! {
    // 1st Gen
    static ref UP_QUARK: ParticleBlueprint = ParticleBlueprint { name: "Up", spin: 0.5 };
    static ref DOWN_QUARK: ParticleBlueprint = ParticleBlueprint { name: "Down", spin: 0.5 };
    static ref ELECTRON: ParticleBlueprint = ParticleBlueprint { name: "Electron", spin: 0.5 };
    // 2nd Gen
    static ref CHARM_QUARK: ParticleBlueprint = ParticleBlueprint { name: "Charm", spin: 0.5 };
    static ref STRANGE_QUARK: ParticleBlueprint = ParticleBlueprint { name: "Strange", spin: 0.5 };
    static ref MUON: ParticleBlueprint = ParticleBlueprint { name: "Muon", spin: 0.5 };
    // 3rd Gen
    static ref TOP_QUARK: ParticleBlueprint = ParticleBlueprint { name: "Top", spin: 0.5 };
    static ref BOTTOM_QUARK: ParticleBlueprint = ParticleBlueprint { name: "Bottom", spin: 0.5 };
    static ref TAUON: ParticleBlueprint = ParticleBlueprint { name: "Tauon", spin: 0.5 };
}

// --- LEVEL 2 ---
#[derive(Debug, Clone)]
struct CosmicLaw {
    G: f64,
    e: f64,
    alpha_s: f64,
    alpha_w: f64,
    mass_up_quark: f64, mass_down_quark: f64, mass_electron: f64,
    mass_charm_quark: f64, mass_strange_quark: f64, mass_muon: f64,
    mass_top_quark: f64, mass_bottom_quark: f64, mass_tauon: f64,
}

// --- PHYSIC ENGINE ---
#[derive(Debug)]
struct PhysicsEngine {
    laws: CosmicLaw,
    alpha: f64,
}

impl PhysicsEngine {
    fn new(laws: CosmicLaw) -> Self {
        let alpha = laws.e.powi(2) / (4.0 * PI * EPSILON_0 * H_BAR * C);
        Self { laws, alpha }
    }

    /// Heisenberg's uncertainty principle
    fn calculate_lifetime(&self, particle_mass: f64) -> f64 {
        if particle_mass <= 0.0 { return f64::INFINITY; }
        let natural_lifetime = H_BAR / (particle_mass * C.powi(2));
        natural_lifetime / self.laws.alpha_w
    }

    /// Feasibility logic for a generational path
    fn evaluate_viability_path(&self, quark1_mass: f64, quark2_mass: f64) -> f64 {
        if quark1_mass >= quark2_mass { return 0.0; }
        let mut fitness_score = 0.2;

        let mass_proton = 2.0 * quark1_mass + quark2_mass;
        let mass_neutron = quark1_mass + 2.0 * quark2_mass;
        let binding_energy_kg = (mass_proton + mass_neutron) * (self.laws.alpha_s * 0.0012);
        let deuterium_energy = binding_energy_kg * 5.6095886e29;
        if !(deuterium_energy > 1.5 && deuterium_energy < 3.0) { return fitness_score; }
        fitness_score = 0.4;

        let stellar_index = self.laws.alpha_s / self.alpha;
        if !(stellar_index > 100.0 && stellar_index < 180.0) { return fitness_score; }
        fitness_score = 0.6;

        let optimal_fusion: f64 = 137.036; let sigma_fusion: f64 = 5.0;
        let fitness_bonus_chemistry: f64 = (-((stellar_index - optimal_fusion).powi(2)) / (2.0 * sigma_fusion.powi(2))).exp();
        fitness_score += fitness_bonus_chemistry * 0.2;

        let optimal_deuterium: f64 = 2.22; let sigma_deuterium: f64 = 0.5;
        let fitness_bonus_life: f64 = (-((deuterium_energy - optimal_deuterium).powi(2)) / (2.0 * sigma_deuterium.powi(2))).exp();
        fitness_score += fitness_bonus_life * 0.2;
        
        fitness_score
    }
}

// --- FITNESS FUNCTION ---
fn calculate_fitness(laws: &CosmicLaw) -> (f64, u8) {
    let engine = PhysicsEngine::new(laws.clone());

    // --- 1st Gen ---
    let fitness_gen1 = engine.evaluate_viability_path(
        laws.mass_up_quark, laws.mass_down_quark
    );

    // --- 2nd Gen ---
    let mut fitness_gen2 = 0.0;
    if MUON.spin == 0.5 && engine.calculate_lifetime(laws.mass_muon) > STABILITY_THRESHOLD_S {
        fitness_gen2 = engine.evaluate_viability_path(
            laws.mass_strange_quark, laws.mass_charm_quark
        );
    }

    // --- 3rd Gen ---
    let mut fitness_gen3 = 0.0;
    if TAUON.spin == 0.5 && engine.calculate_lifetime(laws.mass_tauon) > STABILITY_THRESHOLD_S {
        fitness_gen3 = engine.evaluate_viability_path(
            laws.mass_bottom_quark, laws.mass_top_quark
        );
    }

    // --- Final viability check ---
    if fitness_gen1 >= fitness_gen2 && fitness_gen1 >= fitness_gen3 {
        (fitness_gen1, 1)
    } else if fitness_gen2 >= fitness_gen3 {
        (fitness_gen2, 2)
    } else {
        (fitness_gen3, 3)
    }
}

// --- MAIN SIMULATE FUNCTION ---
fn main() -> Result<(), Box<dyn Error>> {
    const NUM_UNIVERSES_TO_MAP: u64 = 10_000_000;
    const FITNESS_THRESHOLD_TO_LOG: f64 = 0.2;

    let mut rng = thread_rng();
    let mut wtr = csv::Writer::from_path("landscape_data.csv")?;
    wtr.write_record(&[
        "fitness", "winning_gen", "mass_up_quark", "mass_down_quark", 
        "alpha_s", "alpha", "G"
    ])?;

    println!("--- STARTING LANDSCAPE MAPPING (v15.1) ---");
    println!("Simulating {} universes to map the viability landscape...", NUM_UNIVERSES_TO_MAP);

    for i in 0..NUM_UNIVERSES_TO_MAP {
        let random_laws = CosmicLaw {
            G: rng.gen_range(6.674e-11..6.674e-10),
            e: rng.gen_range(0.5e-19..2.5e-19),
            alpha_s: rng.gen_range(0.1..2.0),
            alpha_w: rng.gen_range(1.0e-9..1.0e-4),
            mass_up_quark: rng.gen_range(1.0e-30..6.0e-30), // Rango ampliado para mejor visualización
            mass_down_quark: rng.gen_range(1.0e-30..1.3e-29),
            mass_electron: rng.gen_range(1.0e-31..1.0e-30),
            mass_strange_quark: rng.gen_range(1.0e-29..1.0e-28), 
            mass_charm_quark: rng.gen_range(1.0e-29..1.0e-27),
            mass_muon: rng.gen_range(1.0e-29..1.0e-27),
            mass_bottom_quark: rng.gen_range(1.0e-28..1.0e-27), 
            mass_top_quark: rng.gen_range(1.0e-28..1.0e-25),
            mass_tauon: rng.gen_range(1.0e-28..1.0e-26),
        };
        
        let (fitness, winning_gen) = calculate_fitness(&random_laws);

        if fitness > FITNESS_THRESHOLD_TO_LOG {
            let engine = PhysicsEngine::new(random_laws.clone());
            wtr.write_record(&[
                fitness.to_string(),
                winning_gen.to_string(),
                random_laws.mass_up_quark.to_string(),
                random_laws.mass_down_quark.to_string(),
                random_laws.alpha_s.to_string(),
                engine.alpha.to_string(),
                random_laws.G.to_string(),
            ])?;
        }

        if i % 1_000_000 == 0 && i > 0 {
            println!("... {} million universes mapped.", i / 1_000_000);
        }
    }

    wtr.flush()?;
    println!("--- MAPPING COMPLETE ---");
    println!("Data saved to landscape_data.csv");
    Ok(())
}