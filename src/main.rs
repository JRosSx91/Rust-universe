use std::error::Error;
use rand::prelude::*;
use rand::Rng;
use std::f64::consts::PI;
use std::fs;
use serde::Deserialize;
use clap::{Parser, Subcommand};

// --- LEVEL 0: CONSTANTES FÍSICAS INMUTABLES ---
const C: f64 = 299_792_458.0;
const H_BAR: f64 = 1.054_571_817e-34;
const EPSILON_0: f64 = 8.854_187_81e-12;
const STABILITY_THRESHOLD_S: f64 = 1.0e-9;

// --- LEVEL 1: PLANTILLAS DE PARTÍCULAS ---
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

// --- LEVEL 2: EL GENOMA DE UN UNIVERSO ---
#[derive(Debug, Clone, Deserialize)] 
struct CosmicLaw {
    G: f64,
    e: f64,
    alpha_s: f64,
    alpha_w: f64,
    mass_up_quark: f64, mass_down_quark: f64, mass_electron: f64,
    mass_charm_quark: f64, mass_strange_quark: f64, mass_muon: f64,
    mass_top_quark: f64, mass_bottom_quark: f64, mass_tauon: f64,
}

// --- MOTOR DE FÍSICA ---
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

    fn calculate_lifetime(&self, particle_mass: f64) -> f64 {
        if particle_mass <= 0.0 { return f64::INFINITY; }
        H_BAR / (particle_mass * C.powi(2) * self.laws.alpha_w)
    }
    
    fn evaluate_viability_path(&self, quark1_mass: f64, quark2_mass: f64, lepton_mass: f64) -> f64 {
        let mass_proton = 2.0 * quark1_mass + quark2_mass;
        let mass_neutron = quark1_mass + 2.0 * quark2_mass;
        if mass_proton >= mass_neutron || mass_proton + lepton_mass <= mass_neutron { return 0.0; }
        
        let mut fitness_score = 0.2;

        let deuterium_energy = (mass_proton + mass_neutron) * (self.laws.alpha_s * 0.0012) * 5.6095886e29;
        let fitness_nucleosynthesis = (-((deuterium_energy - 2.22).powi(2)) / (2.0 * 0.5_f64.powi(2))).exp();
        fitness_score += fitness_nucleosynthesis * 0.2;

        let stellar_index = self.laws.alpha_s / self.alpha;
        let fitness_stellar = (-((stellar_index - 137.0).powi(2)) / (2.0 * 20.0_f64.powi(2))).exp();
        fitness_score += fitness_stellar * 0.2;

        let fitness_fine_tuning = (-((deuterium_energy - 2.22).powi(2)) / (2.0 * 0.1_f64.powi(2))).exp();
        let fitness_chemistry = (-((stellar_index - 137.0).powi(2)) / (2.0 * 5.0_f64.powi(2))).exp();
        fitness_score += (fitness_fine_tuning * fitness_chemistry) * 0.4;
        
        fitness_score
    }
}

// --- FUNCIÓN DE FITNESS ---
fn calculate_fitness(laws: &CosmicLaw) -> (f64, u8) {
    let engine = PhysicsEngine::new(laws.clone());
    let fitness_gen1 = engine.evaluate_viability_path(laws.mass_up_quark, laws.mass_down_quark, laws.mass_electron);
    
    let mut fitness_gen2 = 0.0;
    if MUON.spin == 0.5 && engine.calculate_lifetime(laws.mass_muon) > STABILITY_THRESHOLD_S {
        fitness_gen2 = engine.evaluate_viability_path(laws.mass_strange_quark, laws.mass_charm_quark, laws.mass_muon);
    }
    
    let mut fitness_gen3 = 0.0;
    if TAUON.spin == 0.5 && engine.calculate_lifetime(laws.mass_tauon) > STABILITY_THRESHOLD_S {
        fitness_gen3 = engine.evaluate_viability_path(laws.mass_bottom_quark, laws.mass_top_quark, laws.mass_tauon);
    }

    if fitness_gen1 >= fitness_gen2 && fitness_gen1 >= fitness_gen3 {
        (fitness_gen1, 1)
    } else if fitness_gen2 >= fitness_gen3 {
        (fitness_gen2, 2)
    } else {
        (fitness_gen3, 3)
    }
}

// --- DEFINICIÓN DE LA INTERFAZ DE LÍNEA DE COMANDOS (CLI) ---
#[derive(Parser)]
#[command(author, version, about = "Simulador Cosmológico 'El Armónico 137'", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Modo Mapeo: Simula N universos aleatorios para encontrar candidatos viables.
    Map {
        #[arg(short, long, default_value_t = 5_000_000)]
        universes: u64,
    },
    /// Modo Evolutivo: Evoluciona una población a partir de una semilla.
    Evolve {
        #[arg(short, long)]
        seed: String,
        #[arg(short, long, default_value_t = 500)]
        generations: u32,
    },
}

// --- FUNCIÓN PRINCIPAL (PUNTO DE ENTRADA) ---
fn main() {
    let cli = Cli::parse();

    let result = match &cli.command {
        Commands::Map { universes } => {
            println!("--- INICIANDO MODO MAPEO ---");
            run_mapping_mode(*universes)
        }
        Commands::Evolve { seed, generations } => {
            println!("--- INICIANDO MODO EVOLUTIVO ---");
            run_evolutionary_mode(seed, *generations)
        }
    };

    if let Err(e) = result {
        eprintln!("Error en la ejecución: {}", e);
    }
}

// --- LÓGICA DEL MODO MAPEO ---
fn run_mapping_mode(num_universes: u64) -> Result<(), Box<dyn Error>> {
    const FITNESS_THRESHOLD_TO_LOG: f64 = 0.0;
    const SAMPLING_FACTOR: u64 = 100;

    let mut rng = thread_rng();
    let mut wtr = csv::Writer::from_path("landscape_data.csv")?;
    wtr.write_record(&[
        "fitness", "winning_gen", "mass_up_quark", "mass_down_quark", "mass_strange_quark", 
        "mass_charm_quark", "mass_bottom_quark", "mass_top_quark"
    ])?;

    println!("Simulando {} universos y muestreando 1 de cada {} candidatos viables...", num_universes, SAMPLING_FACTOR);
    let mut viable_count: u64 = 0;

    for i in 0..num_universes {
        let random_laws = CosmicLaw {
            G: rng.gen_range(6.674e-11..6.674e-10), e: rng.gen_range(0.5e-19..2.5e-19),
            alpha_s: rng.gen_range(0.1..2.0), alpha_w: rng.gen_range(1.0e-9..1.0e-4),
            mass_up_quark: rng.gen_range(1.0e-30..6.0e-30), mass_down_quark: rng.gen_range(1.0e-30..1.3e-29),
            mass_electron: rng.gen_range(1.0e-31..1.0e-30), mass_strange_quark: rng.gen_range(1.0e-29..1.0e-28), 
            mass_charm_quark: rng.gen_range(1.0e-29..1.0e-27), mass_muon: rng.gen_range(1.0e-29..1.0e-27),
            mass_bottom_quark: rng.gen_range(1.0e-28..1.0e-27), mass_top_quark: rng.gen_range(1.0e-28..1.0e-25),
            mass_tauon: rng.gen_range(1.0e-28..1.0e-26),
        };
        
        let (fitness, winning_gen) = calculate_fitness(&random_laws);

        if fitness > FITNESS_THRESHOLD_TO_LOG {
            viable_count += 1;
            if viable_count % SAMPLING_FACTOR == 0 {
                wtr.write_record(&[
                    format!("{:e}", fitness), winning_gen.to_string(),
                    format!("{:e}", random_laws.mass_up_quark), format!("{:e}", random_laws.mass_down_quark),
                    format!("{:e}", random_laws.mass_strange_quark), format!("{:e}", random_laws.mass_charm_quark),
                    format!("{:e}", random_laws.mass_bottom_quark), format!("{:e}", random_laws.mass_top_quark),
                ])?;
            }
        }
        if i > 0 && i % 1_000_000 == 0 {
            println!("... {} millones de universos mapeados.", i / 1_000_000);
        }
    }

    wtr.flush()?;
    println!("--- MAPEO COMPLETADO ---");
    println!("Datos de {} universos guardados en landscape_data.csv", viable_count / SAMPLING_FACTOR);
    Ok(())
}

// Dentro de main.rs...

// --- LÓGICA DEL MODO EVOLUTIVO (COMPLETA) ---
fn run_evolutionary_mode(seed_file: &str, num_generations: u32) -> Result<(), Box<dyn Error>> {
    // --- 1. SETUP ---
    let adam_genome: CosmicLaw = serde_json::from_str(&fs::read_to_string(seed_file)?)?;
    let mut rng = thread_rng();
    
    const POPULATION_SIZE: usize = 100;
    const MUTATION_RATE: f64 = 0.10; // 2% de probabilidad de mutación por gen
    const TOURNAMENT_SIZE: usize = 3;

    // Preparamos el archivo CSV para registrar los resultados
    let mut wtr = csv::Writer::from_path("evolution_data.csv")?;
    wtr.write_record(&["generation", "best_fitness"])?;

    // --- 2. POBLACIÓN INICIAL ---
    let mut population: Vec<CosmicLaw> = (0..POPULATION_SIZE)
        .map(|_| adam_genome.mutate(&mut rng, MUTATION_RATE))
        .collect();

    println!("Población inicial creada. Iniciando evolución...");

    // --- 3. BUCLE GENERACIONAL ---
    for generation in 0..num_generations {
        // a. Evaluar a toda la población
        let mut evaluated_population: Vec<(CosmicLaw, f64)> = population.iter()
            .map(|laws| (laws.clone(), calculate_fitness(laws).0))
            .collect();
        
        // Ordenamos para encontrar al campeón de esta generación
        evaluated_population.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        
        let champion = &evaluated_population[0];
        
        // Escribir los datos del campeón en el archivo CSV
        wtr.write_record(&[
            generation.to_string(),
            champion.1.to_string(),
        ])?;
        
        // b, c. Crear la nueva generación
        let mut next_population = Vec::with_capacity(POPULATION_SIZE);
        // Elitismo: El campeón pasa directamente a la siguiente generación sin mutar
        next_population.push(champion.0.clone());

        // Llenar el resto de la población mediante selección y mutación
        for _ in 1..POPULATION_SIZE {
            // Seleccionar un padre mediante torneo
            let mut tournament_contenders = Vec::with_capacity(TOURNAMENT_SIZE);
            for _ in 0..TOURNAMENT_SIZE {
                let random_index = rng.gen_range(0..evaluated_population.len());
                tournament_contenders.push(&evaluated_population[random_index]);
            }
            let parent = tournament_contenders.iter().max_by(|a, b| a.1.partial_cmp(&b.1).unwrap()).unwrap();

            // Crear un hijo mutando al padre y añadirlo a la nueva población
            let child = parent.0.mutate(&mut rng, MUTATION_RATE);
            next_population.push(child);
        }
        
        population = next_population;

        // Informar del progreso en la consola cada 10 generaciones
        if generation % 10 == 0 {
             println!("Generación: {}, Mejor Fitness: {:.6}", generation, champion.1);
        }
    }
    
    // Asegurarse de que todos los datos se escriben en el disco
    wtr.flush()?;
    println!("--- EVOLUCIÓN COMPLETADA ---");
    println!("Resultados guardados en evolution_data.csv");
    Ok(())
}

// --- IMPLEMENTACIÓN DE LA LÓGICA DE MUTACIÓN (VERSIÓN FINAL) ---
// --- IMPLEMENTACIÓN DE LA LÓGICA DE MUTACIÓN (VERSIÓN FINAL CORREGIDA) ---
impl CosmicLaw {
    /// Aplica una mutación a una copia del genoma.
    fn mutate(&self, rng: &mut impl Rng, rate: f64) -> Self {
        let mut new_laws = self.clone();

        // Solución definitiva: Separamos la generación y la comparación para eliminar la ambigüedad.
        let roll = rng.gen::<f64>();
        if roll < rate { new_laws.G *= rng.gen_range(00.95..1.05); }
        
        let roll = rng.gen::<f64>();
        if roll < rate { new_laws.e *= rng.gen_range(0.95..1.05); }

        let roll = rng.gen::<f64>();
        if roll < rate { new_laws.alpha_s *= rng.gen_range(0.95..1.05); }

        let roll = rng.gen::<f64>();
        if roll < rate { new_laws.alpha_w *= rng.gen_range(0.95..1.05); }

        let roll = rng.gen::<f64>();
        if roll < rate { new_laws.mass_up_quark *= rng.gen_range(00.95..1.05); }

        let roll = rng.gen::<f64>();
        if roll < rate { new_laws.mass_down_quark *= rng.gen_range(0.95..1.05); }

        let roll = rng.gen::<f64>();
        if roll < rate { new_laws.mass_electron *= rng.gen_range(0.95..1.05); }

        let roll = rng.gen::<f64>();
        if roll < rate { new_laws.mass_strange_quark *= rng.gen_range(0.95..1.05); }

        let roll = rng.gen::<f64>();
        if roll < rate { new_laws.mass_charm_quark *= rng.gen_range(0.95..1.05); }

        let roll = rng.gen::<f64>();
        if roll < rate { new_laws.mass_muon *= rng.gen_range(0.95..1.05); }

        let roll = rng.gen::<f64>();
        if roll < rate { new_laws.mass_bottom_quark *= rng.gen_range(0.95..1.05); }
        
        let roll = rng.gen::<f64>();
        if roll < rate { new_laws.mass_top_quark *= rng.gen_range(0.95..1.05); }
        
        let roll = rng.gen::<f64>();
        if roll < rate { new_laws.mass_tauon *= rng.gen_range(0.95..1.05); }

        new_laws
    }
}