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
    /// Lógica de viabilidad con gradientes de éxito (Gaussiana).
    fn evaluate_viability_path(&self, quark1_mass: f64, quark2_mass: f64, lepton_mass: f64) -> f64 {
        // --- NIVEL 1: Viabilidad Atómica ---
        // Este filtro sigue siendo binario. Un átomo es estable o no lo es.
        let mass_proton = 2.0 * quark1_mass + quark2_mass;
        let mass_neutron = quark1_mass + 2.0 * quark2_mass;
        if mass_proton >= mass_neutron { return 0.0; }
        if mass_proton + lepton_mass <= mass_neutron { return 0.0; }
        
        // Puntuación base por pasar el primer filtro.
        let mut fitness_score = 0.2;

        // --- NIVEL 2: Viabilidad de Nucleosíntesis (AHORA SUAVIZADO) ---
        let deuterium_energy: f64 = (mass_proton + mass_neutron) * (self.laws.alpha_s * 0.0012) * 5.6095886e29;
        let optimal_deuterium: f64 = 2.22; // MeV
        let sigma_deuterium: f64 = 0.5; // MeV, una desviación estándar "razonable"
        let fitness_nucleosynthesis: f64 = (-((deuterium_energy - optimal_deuterium).powi(2)) / (2.0 * sigma_deuterium.powi(2))).exp();
        
        // La puntuación de este nivel es ahora un factor (0 a 1) que multiplica el potencial restante (0.8)
        fitness_score += fitness_nucleosynthesis * 0.2; // Bonus máximo de +0.2 si es perfecto

        // --- NIVEL 3: Viabilidad Estelar (AHORA SUAVIZADO) ---
        let stellar_index: f64 = self.laws.alpha_s / self.alpha;
        let optimal_stellar: f64 = 137.0;
        let sigma_stellar: f64 = 20.0; // Una ventana más amplia para la formación de estrellas
        let fitness_stellar = (-((stellar_index - optimal_stellar).powi(2)) / (2.0 * sigma_stellar.powi(2))).exp();

        fitness_score += fitness_stellar * 0.2; // Bonus máximo de +0.2

        // --- NIVEL 4 & 5: Viabilidad Química y Biológica (ajustes finos) ---
        // Estos se convierten en bonificaciones adicionales si los valores son casi perfectos.
        // Usamos una sigma mucho más pequeña para representar el "ajuste fino".
        let sigma_deuterium_fine: f64 = 0.1;
        let fitness_fine_tuning: f64 = (-((deuterium_energy - optimal_deuterium).powi(2)) / (2.0 * sigma_deuterium_fine.powi(2))).exp();
        
        let sigma_stellar_fine: f64 = 5.0;
        let fitness_chemistry: f64 = (-((stellar_index - optimal_stellar).powi(2)) / (2.0 * sigma_stellar_fine.powi(2))).exp();

        fitness_score += (fitness_fine_tuning * fitness_chemistry) * 0.4; // Bonus máximo de +0.4

        fitness_score
    }
}

// --- FITNESS FUNCTION ---
fn calculate_fitness(laws: &CosmicLaw) -> (f64, u8) {
    let engine = PhysicsEngine::new(laws.clone());

    // --- 1st Gen ---
    let fitness_gen1 = engine.evaluate_viability_path(
        laws.mass_up_quark, laws.mass_down_quark, laws.mass_electron
    );

    // --- 2nd Gen ---
    let mut fitness_gen2 = 0.0;
    if MUON.spin == 0.5 && engine.calculate_lifetime(laws.mass_muon) > STABILITY_THRESHOLD_S {
        fitness_gen2 = engine.evaluate_viability_path(
            laws.mass_strange_quark, laws.mass_charm_quark, laws.mass_muon
        );
    }

    // --- 3rd Gen ---
    let mut fitness_gen3 = 0.0;
    if TAUON.spin == 0.5 && engine.calculate_lifetime(laws.mass_tauon) > STABILITY_THRESHOLD_S {
        fitness_gen3 = engine.evaluate_viability_path(
            laws.mass_bottom_quark, laws.mass_top_quark, laws.mass_tauon
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
    const NUM_UNIVERSES_TO_MAP: u64 = 5_000_000; // Reducimos a 5M para una ejecución más rápida
    const FITNESS_THRESHOLD_TO_LOG: f64 = 0.0;
    const SAMPLING_FACTOR: u64 = 100;

    let mut rng = thread_rng();
    let mut wtr = csv::Writer::from_path("landscape_data.csv")?;
    wtr.write_record(&[
        "fitness", "winning_gen", "mass_up_quark", "mass_down_quark", "mass_strange_quark", 
        "mass_charm_quark", "mass_bottom_quark", "mass_top_quark"
    ])?;

    println!("--- STARTING LANDSCAPE MAPPING (v15.2 - with Sampling) ---");
    println!("Simulating {} universes and sampling 1 in {} viable candidates...", NUM_UNIVERSES_TO_MAP, SAMPLING_FACTOR);

    let mut viable_count: u64 = 0;

    for i in 0..NUM_UNIVERSES_TO_MAP {
        let random_laws = CosmicLaw {
            G: rng.gen_range(6.674e-11..6.674e-10),
            e: rng.gen_range(0.5e-19..2.5e-19),
            alpha_s: rng.gen_range(0.1..2.0),
            alpha_w: rng.gen_range(1.0e-9..1.0e-4),
            mass_up_quark: rng.gen_range(1.0e-30..6.0e-30),
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
            viable_count += 1;
            // Solo escribimos en el archivo si el contador es un múltiplo de nuestro factor
            if viable_count % SAMPLING_FACTOR == 0 {
                wtr.write_record(&[
                    format!("{:e}", fitness),
                    winning_gen.to_string(),
                    format!("{:e}", random_laws.mass_up_quark),
                    format!("{:e}", random_laws.mass_down_quark),
                    format!("{:e}", random_laws.mass_strange_quark),
                    format!("{:e}", random_laws.mass_charm_quark),
                    format!("{:e}", random_laws.mass_bottom_quark),
                    format!("{:e}", random_laws.mass_top_quark),
                ])?;
            }
        }

        if i % 1_000_000 == 0 && i > 0 {
            println!("... {} million universes mapped.", i / 1_000_000);
        }
    }

    wtr.flush()?;
    println!("--- MAPPING COMPLETE ---");
    println!("Data for {} universes saved to landscape_data.csv", viable_count / SAMPLING_FACTOR);
    Ok(())
}