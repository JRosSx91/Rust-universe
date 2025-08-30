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
const K_B: f64 = 1.380649e-23; // Constante de Boltzmann
const M_SOLAR: f64 = 1.989e30;

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

// --- IMPLEMENTACIÓN DEL MOTOR DE FÍSICA (v18.0 - CON GRADIENTES SUAVES) ---
// --- IMPLEMENTACIÓN DEL MOTOR DE FÍSICA (v18.1 - COMPLETO) ---
impl PhysicsEngine {
    fn new(laws: CosmicLaw) -> Self {
        let alpha = laws.e.powi(2) / (4.0 * PI * EPSILON_0 * H_BAR * C);
        Self { laws, alpha }
    }

    fn chandrasekhar_mass(&self) -> f64 {
        let m_proton = self.laws.mass_up_quark * 2.0 + self.laws.mass_down_quark;
        if m_proton <= 0.0 || self.laws.G <= 0.0 { return 0.0; }
        let mu_e = 2.0;
        (H_BAR * C / self.laws.G).powf(1.5) / (m_proton * mu_e).powi(2)
    }

    fn calculate_stellar_viability(&self) -> f64 {
        let t_core = 1.5e7;
        let thermal_energy = K_B * t_core;
        let m_proton = 2.0 * self.laws.mass_up_quark + self.laws.mass_down_quark;
        if m_proton <= 0.0 { return 0.0; }
        let m_reduced = m_proton / 2.0;

        let gamow_energy = 2.0 * m_reduced * C.powi(2) * (PI * self.alpha).powi(2);
        if thermal_energy <= 0.0 || gamow_energy < 0.0 { return 0.0; }
        let tunnel_exponent = - (gamow_energy / thermal_energy).sqrt();
        let fusion_rate = tunnel_exponent.exp();
        
        if fusion_rate < 1e-30 { return 0.0; }
        let log_rate = fusion_rate.ln();
        
        1.0 / (1.0 + ((-log_rate - 50.0) / 10.0).exp())
    }
    
    fn calculate_black_hole_potential(&self) -> f64 {
        let m_ch = self.chandrasekhar_mass();
        if m_ch.is_nan() || m_ch.is_infinite() || m_ch <= 0.0 { return 0.0; }
        let target_log_mass = (8.0 * M_SOLAR).log10();
        let current_log_mass = m_ch.log10();
        
        let exponent = -((current_log_mass - target_log_mass).powi(2)) / (2.0 * 1.0_f64.powi(2));
        exponent.exp()
    }
    
    fn nuclear_stability_score(&self) -> f64 {
        let m_proton = 2.0 * self.laws.mass_up_quark + self.laws.mass_down_quark;
        if m_proton <= 0.0 { return 0.0; }

        let binding_energy_deuterium = self.laws.alpha_s * m_proton * C.powi(2) * 0.0023;
        let target_binding_joules = 2.22 * 1.602e-13;
        if target_binding_joules <= 0.0 { return 0.0; }
        let relative_error = (binding_energy_deuterium - target_binding_joules).abs() / target_binding_joules;
        
        (-relative_error.powi(2) / 0.5).exp()
    }

    // FUNCIÓN AÑADIDA QUE FALTABA
    fn heavy_elements_viability(&self) -> f64 {
        let alpha_s_optimal = 0.118;
        let alpha_s_error = (self.laws.alpha_s - alpha_s_optimal).abs() / alpha_s_optimal;
        
        if alpha_s_error < 0.5 {
            1.0 - alpha_s_error
        } else {
            0.0
        }
    }
}

fn calculate_fitness(laws: &CosmicLaw) -> (f64, u8) {
    let engine = PhysicsEngine::new(laws.clone());

    let mass_proton = 2.0 * laws.mass_up_quark + laws.mass_down_quark;
    let mass_neutron = laws.mass_up_quark + 2.0 * laws.mass_down_quark;
    
    // Verificación de viabilidad básica
    if mass_proton >= mass_neutron || mass_proton + laws.mass_electron <= mass_neutron {
        return (0.0, 0);
    }

    let mut fitness = 0.0;
    let mut complexity_level = 0;

    // NIVEL 1: Química Básica (0.0-0.2)
    let stability_margin = mass_neutron - mass_proton;
    let atomic_fitness = (stability_margin / mass_proton).min(0.1);
    
    // Bonus por enlace electromagnético estable
    let bohr_radius = 4.0 * PI * EPSILON_0 * H_BAR.powi(2) / (laws.mass_electron * laws.e.powi(2));
    let em_stability = if bohr_radius > 0.0 && bohr_radius < 1e-9 { 0.1 } else { 0.0 };
    
    fitness += atomic_fitness + em_stability;
    
    if fitness >= 0.15 {
        complexity_level = 1; // Universo con átomos
        
        // NIVEL 2: Física Nuclear y Estelar (0.0-0.35)
        let nuclear_score = engine.nuclear_stability_score();
        let stellar_score = engine.calculate_stellar_viability();
        let nuclear_fitness = 0.15 * nuclear_score + 0.2 * stellar_score;
        
        fitness += nuclear_fitness;
        
        if fitness >= 0.4 {
            complexity_level = 2; // Universo con estrellas
            
            // NIVEL 3: Elementos Pesados y Complejidad (0.0-0.25)
            let heavy_elements = engine.heavy_elements_viability();
            let complexity_fitness = 0.25 * heavy_elements;
            
            fitness += complexity_fitness;
            
            if fitness >= 0.6 {
                complexity_level = 3; // Universo con química compleja
                
                // NIVEL 4: Potencial Reproductivo (0.0-0.2)
                let reproductive_fitness = 0.2 * engine.calculate_black_hole_potential();
                fitness += reproductive_fitness;
                
                if fitness >= 0.75 {
                    complexity_level = 4; // Universo auto-reproductivo
                }
            }
        }
    }

    (fitness, complexity_level)
}

// Función auxiliar para análisis del paisaje
fn analyze_universe_type(fitness: f64, level: u8) -> &'static str {
    match level {
        0 => "Estéril",
        1 => "Químico",
        2 => "Estelar", 
        3 => "Complejo",
        4 => "Reproductivo",
        _ => "Desconocido"
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

        // Añadir al modo mapping
        if fitness > FITNESS_THRESHOLD_TO_LOG {
            let universe_type = analyze_universe_type(fitness, winning_gen);
            if i % 10_000_000 == 0 {
                println!("Muestra #{}: Fitness {:.4}, Tipo: {}", i, fitness, universe_type);
            }
        }

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

fn run_evolutionary_mode(seed_file: &str, num_generations: u32) -> Result<(), Box<dyn Error>> {
    // --- 1. SETUP ---
    let adam_genome: CosmicLaw = serde_json::from_str(&fs::read_to_string(seed_file)?)?;
    let mut rng = thread_rng();
    
    const POPULATION_SIZE: usize = 100;
    const MUTATION_RATE: f64 = 0.10; // 10% de probabilidad por gen
    const TOURNAMENT_SIZE: usize = 3;
    const HYPERMUTATION_CHANCE: f64 = 0.05; // 5% de las mutaciones serán 'saltos de fe'

    // Preparamos el archivo CSV para registrar los resultados
    let mut wtr = csv::Writer::from_path("evolution_data.csv")?;
    wtr.write_record(&["generation", "best_fitness"])?;

    // --- 2. POBLACIÓN INICIAL ---
    let mut population: Vec<CosmicLaw> = (0..POPULATION_SIZE)
        .map(|_| adam_genome.mutate(&mut rng, MUTATION_RATE, HYPERMUTATION_CHANCE))
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
            let child = parent.0.mutate(&mut rng, MUTATION_RATE, HYPERMUTATION_CHANCE);
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

// --- IMPLEMENTACIÓN DE LA LÓGICA DE MUTACIÓN (CON HIPERMUTACIÓN) ---
impl CosmicLaw {
    /// Aplica una mutación a una copia del genoma, con posibilidad de hipermutación.
    fn mutate(&self, rng: &mut impl Rng, rate: f64, hypermutation_chance: f64) -> Self {
        let mut new_laws = self.clone();

        if rng.gen::<f64>() < rate {
            if rng.gen::<f64>() < hypermutation_chance {
                new_laws.G = rng.gen_range(6.674e-11..6.674e-10);
            } else {
                new_laws.G *= rng.gen_range(0.95..1.05);
            }
        }
        if rng.gen::<f64>() < rate {
            if rng.gen::<f64>() < hypermutation_chance {
                new_laws.e = rng.gen_range(0.5e-19..2.5e-19);
            } else {
                new_laws.e *= rng.gen_range(0.95..1.05);
            }
        }
        if rng.gen::<f64>() < rate {
            if rng.gen::<f64>() < hypermutation_chance {
                new_laws.alpha_s = rng.gen_range(0.1..2.0);
            } else {
                new_laws.alpha_s *= rng.gen_range(0.95..1.05);
            }
        }
        if rng.gen::<f64>() < rate {
            if rng.gen::<f64>() < hypermutation_chance {
                new_laws.alpha_w = rng.gen_range(1.0e-9..1.0e-4);
            } else {
                new_laws.alpha_w *= rng.gen_range(0.95..1.05);
            }
        }
        if rng.gen::<f64>() < rate {
            if rng.gen::<f64>() < hypermutation_chance {
                new_laws.mass_up_quark = rng.gen_range(1.0e-30..6.0e-30);
            } else {
                new_laws.mass_up_quark *= rng.gen_range(0.95..1.05);
            }
        }
        if rng.gen::<f64>() < rate {
            if rng.gen::<f64>() < hypermutation_chance {
                new_laws.mass_down_quark = rng.gen_range(1.0e-30..1.3e-29);
            } else {
                new_laws.mass_down_quark *= rng.gen_range(0.95..1.05);
            }
        }
        if rng.gen::<f64>() < rate {
            if rng.gen::<f64>() < hypermutation_chance {
                new_laws.mass_electron = rng.gen_range(1.0e-31..1.0e-30);
            } else {
                new_laws.mass_electron *= rng.gen_range(0.95..1.05);
            }
        }
        if rng.gen::<f64>() < rate {
            if rng.gen::<f64>() < hypermutation_chance {
                new_laws.mass_strange_quark = rng.gen_range(1.0e-29..1.0e-28);
            } else {
                new_laws.mass_strange_quark *= rng.gen_range(0.95..1.05);
            }
        }
        if rng.gen::<f64>() < rate {
            if rng.gen::<f64>() < hypermutation_chance {
                new_laws.mass_charm_quark = rng.gen_range(1.0e-29..1.0e-27);
            } else {
                new_laws.mass_charm_quark *= rng.gen_range(0.95..1.05);
            }
        }
        if rng.gen::<f64>() < rate {
            if rng.gen::<f64>() < hypermutation_chance {
                new_laws.mass_muon = rng.gen_range(1.0e-29..1.0e-27);
            } else {
                new_laws.mass_muon *= rng.gen_range(0.95..1.05);
            }
        }
        if rng.gen::<f64>() < rate {
            if rng.gen::<f64>() < hypermutation_chance {
                new_laws.mass_bottom_quark = rng.gen_range(1.0e-28..1.0e-27);
            } else {
                new_laws.mass_bottom_quark *= rng.gen_range(0.95..1.05);
            }
        }
        if rng.gen::<f64>() < rate {
            if rng.gen::<f64>() < hypermutation_chance {
                new_laws.mass_top_quark = rng.gen_range(1.0e-28..1.0e-25);
            } else {
                new_laws.mass_top_quark *= rng.gen_range(0.95..1.05);
            }
        }
        if rng.gen::<f64>() < rate {
            if rng.gen::<f64>() < hypermutation_chance {
                new_laws.mass_tauon = rng.gen_range(1.0e-28..1.0e-26);
            } else {
                new_laws.mass_tauon *= rng.gen_range(0.95..1.05);
            }
        }

        new_laws
    }
}