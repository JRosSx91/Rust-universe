use std::error::Error;
use csv::Writer;
use rand::prelude::*;
use rand_distr::Normal;
use std::f64::consts::PI;

// --- El Genoma: Las constantes INDEPENDIENTES que mutarán ---
#[derive(Clone)] // Le decimos a Rust que podemos copiar este struct fácilmente
struct CosmicGenes {
    e: f64,               // Carga elemental
    alpha_s: f64,         // Acoplamiento Fuerte
    alpha_w: f64,         // Acoplamiento Débil
    mass_electron: f64,
    mass_up_quark: f64,
    mass_down_quark: f64,
}

// --- El Organismo: El Universo completo, con sus propiedades derivadas ---
struct Universe {
    name: String,
    genes: CosmicGenes, // Contiene la "receta" genética
    
    // Constantes "Hardware" (las consideramos fijas para nuestra simulación)
    c: f64,
    h_bar: f64,
    g: f64,
    epsilon_0: f64,

    // Constantes DERIVADAS, calculadas a partir de los genes y el hardware
    alpha: f64,
}

impl Universe {
    // El constructor ahora "hace crecer" un Universo a partir de un genoma.
    fn from_genes(name: String, genes: CosmicGenes) -> Self {
        
        // Definimos el "Hardware" como constantes fijas del escenario.
        let c = 299_792_458.0;
        let h = 6.626_070_15e-34;
        let h_bar = h / (2.0 * PI);
        let g = 6.674_30e-11;
        let epsilon_0 = 8.854_187_81e-12;

        // Calculamos las propiedades DERIVADAS.
        let alpha = genes.e.powi(2) / (4.0 * PI * epsilon_0 * h_bar * c);

        // Devolvemos el "organismo" completo.
        Universe {
            name,
            genes,
            c, h_bar, g, epsilon_0,
            alpha,
        }
    }
}

// La función de fitness no cambia, opera sobre el Universo ya construido.
fn calculate_fitness(universe: &Universe) -> f64 {
    // --- NUEVA CONDICIÓN: Estabilidad del Protón ---
    if universe.genes.mass_up_quark >= universe.genes.mass_down_quark {
        return 0.0; // Universo estéril, los átomos no son estables.
    }
    let fitness_proton = 1.0; // Si pasa la prueba, su fitness para esta condición es 1.

    // --- Condición de Alpha (la que ya teníamos) ---
    let alpha = universe.alpha;
    let lower_bound: f64 = 1.0 / 170.0;
    let upper_bound: f64 = 1.0 / 100.0;
    
    if alpha < lower_bound || alpha > upper_bound {
        return 0.0;
    }
    
    let optimal_alpha: f64 = 1.0 / 137.036;
    let sigma: f64 = 0.002;
    let fitness_alpha = (-((alpha - optimal_alpha).powi(2)) / (2.0f64 * sigma.powi(2))).exp();
    
    // El fitness total es el producto de todas las condiciones.
    fitness_proton * fitness_alpha
}

// VERSIÓN CORREGIDA de la función main

fn main() -> Result<(), Box<dyn Error>> {
    const NUM_UNIVERSES: usize = 10_000;
    let base_genes = CosmicGenes {
        e: 1.602_176_63e-19, alpha_s: 1.0, alpha_w: 1.0e-6,
        mass_electron: 9.109_383_7e-31, mass_up_quark: 3.9e-30, mass_down_quark: 8.5e-30,
    };

    println!("Generando un multiverso de {} universos mutantes...", NUM_UNIVERSES);
    let mut multiverse: Vec<Universe> = Vec::with_capacity(NUM_UNIVERSES);
    let mut rng = thread_rng();
    let e_distribution = Normal::new(base_genes.e, 1.0e-21).unwrap();

    for i in 0..NUM_UNIVERSES {
        let mut mutated_genes = base_genes.clone();
        mutated_genes.e = e_distribution.sample(&mut rng);
        multiverse.push(Universe::from_genes(format!("Universo #{}", i + 1), mutated_genes));
    }
    println!("Multiverso generado.");

    // --- Guardar datos en CSV ---
    println!("Guardando datos de viabilidad en 'viability_data.csv'...");
    let mut wtr = Writer::from_path("viability_data.csv")?;
    wtr.write_record(&["alpha", "fitness"])?;

    for u in &multiverse {
        let fitness = calculate_fitness(u);
        wtr.write_record(&[u.alpha.to_string(), fitness.to_string()])?;
    }
    wtr.flush()?;
    println!("Datos guardados.");

    // --- Encontrar e imprimir al ganador ---
    println!("Aplicando selección natural para encontrar al mejor...");
    let fittest_universe = multiverse
        .iter()
        .max_by(|a, b| calculate_fitness(a).partial_cmp(&calculate_fitness(b)).unwrap());

    println!("\n=======================================================");
    println!("SELECCIÓN COMPLETADA. EL UNIVERSO SUPERVIVIENTE ES:");
    
    if let Some(winner) = fittest_universe {
        let fitness_score = calculate_fitness(winner);
        println!("  Nombre: {}", winner.name);
        println!("  Gen 'e' mutado: {:.6e} C", winner.genes.e);
        println!("  (Valor base 'e':   {:.6e} C)", base_genes.e);
        println!("  Alpha resultante: {:.6}", winner.alpha);
        println!("  Puntuación de Fitness: {:.4}", fitness_score);
    } else {
        println!("No se encontró ningún universo viable en la población.");
    }
    println!("=======================================================\n");

    // ESTA DEBE SER LA ÚLTIMA LÍNEA
    Ok(()) // Devolver Ok para indicar que todo ha ido bien
}