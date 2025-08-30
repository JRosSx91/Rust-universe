# scripts/find_adam.py

import pandas as pd
import json
import os

try:
    # --- CÓDIGO MEJORADO ---
    # Encuentra la ruta del directorio donde se encuentra este script
    script_dir = os.path.dirname(os.path.abspath(__file__))
    
    # Construye la ruta al archivo CSV, que está en el directorio padre (../)
    csv_path = os.path.join(script_dir, '..', 'landscape_data.csv')

    df = pd.read_csv(csv_path)
    # -----------------------

except FileNotFoundError:
    print(f"Error: No se pudo encontrar '{csv_path}'.")
    print("Asegúrate de haber ejecutado el modo 'map' primero.")
    exit()


# Encontrar el universo con el mayor fitness
adam = df.loc[df['fitness'].idxmax()]

print("--- Adán Encontrado ---")
print(f"Fitness Máximo: {adam['fitness']:.6f}")
print(adam)

# Crear el diccionario del genoma para guardarlo en JSON
# NOTA: Los nombres de las claves deben coincidir con los de la struct CosmicLaw en Rust
adam_genome = {
    # Necesitamos añadir valores por defecto para las constantes que no guardamos en el CSV
    "G": 6.67430e-11, # Valor estándar como placeholder
    "e": 1.60217663e-19, # Valor estándar como placeholder
    "alpha_s": 1.0, # Valor placeholder
    "alpha_w": 1.0e-6, # Valor placeholder
    "mass_electron": 9.10938356e-31, # Valor placeholder
    "mass_muon": 1.883531594e-28, # Valor placeholder
    "mass_tauon": 3.16754e-27, # Valor placeholder

    # Estos son los valores clave de nuestro Adán
    "mass_up_quark": adam['mass_up_quark'],
    "mass_down_quark": adam['mass_down_quark'],
    "mass_strange_quark": adam['mass_strange_quark'],
    "mass_charm_quark": adam['mass_charm_quark'],
    "mass_bottom_quark": adam['mass_bottom_quark'],
    "mass_top_quark": adam['mass_top_quark'],
}

# Guardar el genoma en un archivo JSON
with open('adam_genome.json', 'w') as f:
    json.dump(adam_genome, f, indent=4)

print("\nGenoma de Adán guardado en 'adam_genome.json'")