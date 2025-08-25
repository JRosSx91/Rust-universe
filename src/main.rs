// main.rs - v7.0 (Holistic Fitness and Full Genetic Evolution)

use std::error::Error;
use csv::Writer;
use rand::prelude::*;
use rand_distr::{Normal, Distribution};
use std::f64::consts::PI;

// --- El Genoma: Todas las constantes que pueden mutar ---
#[derive(Clone, Debug)]
struct CosmicGenes {
    e: f64, alpha_s: f64, alpha_w: f64,
    mass_electron: f64, mass_up_quark: f64, mass_down_quark: f64,
    mass_muon: f64, mass_tauon: f64,
}

// --- El Organismo: El Universo completo ---
#[derive(Debug)]
struct Universe {
    name: String,
    genes: CosmicGenes,
    c: f64, h_bar: f64, g: f64, epsilon_0: f64,
    alpha: f64,
}

// --- Propiedades Físicas Emergentes ---
#[derive(Debug, Clone)]
struct EmergentPhysics {
    deuterium_binding_energy: f64, // en MeV
    stellar_fusion_index: f64,
    stable_lepton_generation: u8,
}

impl Universe {
    fn from_genes(name: String, genes: CosmicGenes) -> Self {
        let c = 299_792_458.0; let h = 6.626_070_15e-34; let h_bar = h / (2.0 * PI);
        let g = 6.674_30e-11; let epsilon_0 = 8.854_187_81e-12;
        let alpha = genes.e.powi(2) / (4.0 * PI * epsilon_0 * h_bar * c);
        Universe { name, genes, c, h_bar, g, epsilon_0, alpha }
    }
}

// La función que calcula las propiedades emergentes
fn calculate_emergent_physics(universe: &Universe) -> EmergentPhysics {
    // 1. Índice de Fusión Estelar
    let stellar_fusion_index = universe.genes.alpha_s / universe.alpha;

    // 2. Energía de Enlace del Deuterio
    let mass_proton = 2.0 * universe.genes.mass_up_quark + universe.genes.mass_down_quark;
    let mass_neutron = universe.genes.mass_up_quark + 2.0 * universe.genes.mass_down_quark;
    let kg_to_mev = 5.6095886e29;
    let binding_energy_kg = (mass_proton + mass_neutron) * (universe.genes.alpha_s * 0.0012);
    let deuterium_binding_energy = binding_energy_kg * kg_to_mev;

    // 3. Estabilidad de Leptones
    const K_DECAY_FACTOR: f64 = 1.0e35;
    let fitness_stability_1gen = 1.0; // Electron
    let muon_decay_rate = universe.genes.alpha_w * universe.genes.mass_muon.powi(2);
    let fitness_stability_2gen = (-muon_decay_rate * K_DECAY_FACTOR).exp();
    let tauon_decay_rate = universe.genes.alpha_w * universe.genes.mass_tauon.powi(2);
    let fitness_stability_3gen = (-tauon_decay_rate * K_DECAY_FACTOR).exp();

    let stable_lepton_generation = if fitness_stability_1gen > 0.9 { 1 }
                                   else if fitness_stability_2gen > 0.9 { 2 }
                                   else if fitness_stability_3gen > 0.9 { 3 }
                                   else { 0 }; // Ninguna es estable

    EmergentPhysics { stellar_fusion_index, deuterium_binding_energy, stable_lepton_generation }
}

// La función de fitness ahora es un juez de estas propiedades emergentes
fn calculate_fitness(physics: &EmergentPhysics) -> f64 {
    // Regla 1: Nucleosíntesis (Deuterio)
    let optimal_deuterium: f64 = 2.22; let sigma_deuterium: f64 = 0.5;
    let fitness_nucleosynthesis: f64 = (-((physics.deuterium_binding_energy - optimal_deuterium).powi(2)) / (2.0f64 * sigma_deuterium.powi(2))).exp();

    // Regla 2: Fusión Estelar
    let optimal_fusion: f64 = 137.036; let sigma_fusion: f64 = 5.0;
    let fitness_stellar = (-((physics.stellar_fusion_index - optimal_fusion).powi(2)) / (2.0f64 * sigma_fusion.powi(2))).exp();
    
    // Regla 3: Debe haber una base para la química (un leptón estable)
    let fitness_chemistry: f64 = if physics.stable_lepton_generation > 0 { 1.0 } else { 0.0 };

    fitness_nucleosynthesis * fitness_stellar * fitness_chemistry
}

// La función de promediado ahora incluye todos los genes que mutan
fn calculate_average_genes(pool: &[CosmicGenes]) -> CosmicGenes {
    if pool.is_empty() { panic!("Breeding pool cannot be empty!"); }
    let pool_size = pool.len() as f64;
    CosmicGenes {
        e: pool.iter().map(|g| g.e).sum::<f64>() / pool_size,
        alpha_s: pool.iter().map(|g| g.alpha_s).sum::<f64>() / pool_size,
        alpha_w: pool.iter().map(|g| g.alpha_w).sum::<f64>() / pool_size,
        mass_electron: pool.iter().map(|g| g.mass_electron).sum::<f64>() / pool_size,
        mass_up_quark: pool.iter().map(|g| g.mass_up_quark).sum::<f64>() / pool_size,
        mass_down_quark: pool.iter().map(|g| g.mass_down_quark).sum::<f64>() / pool_size,
        mass_muon: pool.iter().map(|g| g.mass_muon).sum::<f64>() / pool_size,
        mass_tauon: pool.iter().map(|g| g.mass_tauon).sum::<f64>() / pool_size,
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    const NUM_GENERATIONS: u32 = 200;
    const POPULATION_SIZE: usize = 10_000;
    const BREEDING_POOL_SIZE: usize = 100;

    let mut rng = thread_rng();
    let mut wtr = Writer::from_path("evolution_log_final.csv")?;
    wtr.write_record(&["generation", "best_fitness", "deuterium_energy", "stable_lepton_gen"])?;
    
    println!("--- STARTING EVOLUTIONARY SIMULATION (v7.0 - Holistic Fitness) ---");

    // --- GENERACIÓN 0: CALDO PRIMORDIAL ---
    let mut population: Vec<(Universe, EmergentPhysics)> = Vec::with_capacity(POPULATION_SIZE);
    for i in 0..POPULATION_SIZE {
        let random_genes = CosmicGenes {
            e: rng.gen_range(0.5e-19..2.5e-19), alpha_s: rng.gen_range(0.1..2.0),
            alpha_w: rng.gen_range(1.0e-7..1.0e-4),
            mass_up_quark: rng.gen_range(1.0e-30..1.0e-29), mass_down_quark: rng.gen_range(1.0e-30..1.0e-29),
            mass_electron: rng.gen_range(1.0e-31..1.0e-30),
            mass_muon: rng.gen_range(1.0e-29..1.0e-28),
            mass_tauon: rng.gen_range(1.0e-28..1.0e-27),
        };
        let universe = Universe::from_genes(format!("G0U{}", i), random_genes);
        let physics = calculate_emergent_physics(&universe);
        population.push((universe, physics));
    }

    population.sort_by(|a, b| calculate_fitness(&b.1).partial_cmp(&calculate_fitness(&a.1)).unwrap());
    
    let breeding_pool: Vec<CosmicGenes> = population.iter().take(BREEDING_POOL_SIZE).map(|(u, _p)| u.genes.clone()).collect();
    
    if breeding_pool.is_empty() || calculate_fitness(&population[0].1) < 0.01 {
        println!("BIG BANG FAILED: Total extinction in the primordial generation.");
        return Ok(());
    }

    let mut current_genes = calculate_average_genes(&breeding_pool);
    let best_physics = &population[0].1;
    let best_fitness = calculate_fitness(best_physics);
    println!("Generation 0   | First Survivors Found! Best Fitness: {:.4} | Deuterium: {:.2} MeV | Stable Lepton: Gen {}",
        best_fitness, best_physics.deuterium_binding_energy, best_physics.stable_lepton_generation);
    wtr.write_record(&[
        "0".to_string(), best_fitness.to_string(),
        best_physics.deuterium_binding_energy.to_string(),
        best_physics.stable_lepton_generation.to_string(),
    ])?;
        
    // --- BUCLE DE EVOLUCIÓN ---
    for generation in 1..=NUM_GENERATIONS {
        let mut population: Vec<(Universe, EmergentPhysics)> = Vec::with_capacity(POPULATION_SIZE);
        let mut create_dist = |mean: f64| Normal::new(mean, mean * 0.10).unwrap();
        
        for i in 0..POPULATION_SIZE {
            let mutated_genes = CosmicGenes {
                e: create_dist(current_genes.e).sample(&mut rng),
                alpha_s: create_dist(current_genes.alpha_s).sample(&mut rng),
                alpha_w: create_dist(current_genes.alpha_w).sample(&mut rng),
                mass_electron: create_dist(current_genes.mass_electron).sample(&mut rng),
                mass_up_quark: create_dist(current_genes.mass_up_quark).sample(&mut rng),
                mass_down_quark: create_dist(current_genes.mass_down_quark).sample(&mut rng),
                mass_muon: create_dist(current_genes.mass_muon).sample(&mut rng),
                mass_tauon: create_dist(current_genes.mass_tauon).sample(&mut rng),
            };
            let universe = Universe::from_genes(format!("G{}U{}", generation, i), mutated_genes);
            let physics = calculate_emergent_physics(&universe);
            population.push((universe, physics));
        }

        population.sort_by(|a, b| calculate_fitness(&b.1).partial_cmp(&calculate_fitness(&a.1)).unwrap());
        let breeding_pool: Vec<CosmicGenes> = population.iter().take(BREEDING_POOL_SIZE).map(|(u, _p)| u.genes.clone()).collect();
        
        if breeding_pool.is_empty() { println!("GENERATION {}: EXTINCTION EVENT!", generation); break; }

        current_genes = calculate_average_genes(&breeding_pool);
        let best_physics = &population[0].1;
        let best_fitness = calculate_fitness(best_physics);
        println!("Generation {:<3} | Best Fitness: {:.4} | Deuterium: {:.2} MeV | Stable Lepton: Gen {}",
            generation, best_fitness, best_physics.deuterium_binding_energy, best_physics.stable_lepton_generation);
        wtr.write_record(&[
            generation.to_string(), best_fitness.to_string(),
            best_physics.deuterium_binding_energy.to_string(),
            best_physics.stable_lepton_generation.to_string(),
        ])?;
    }
    
    wtr.flush()?;
    println!("\n--- EVOLUTION COMPLETE ---");
    println!("Final optimized genome: {:?}", current_genes);
    println!("Evolution log saved to 'evolution_log_final.csv'");
    
    Ok(())
}