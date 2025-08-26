# plotter_landscape.py - Viability Landscape Visualization

import pandas as pd
import matplotlib.pyplot as plt
import numpy as np

try:
    df = pd.read_csv('landscape_data.csv')
except FileNotFoundError:
    print("Error: 'landscape_data.csv' not found. Run the Rust simulation first.")
    exit()

# Filtramos solo los datos relevantes para la Generación 1 para simplificar el análisis inicial
df_gen1 = df[df['winning_gen'] == 1]

if df_gen1.empty:
    print("No viable universes found for Generation 1 in the data.")
    exit()

plt.style.use('dark_background')
fig, ax = plt.subplots(figsize=(15, 12))

# El gráfico de dispersión (scatter plot)
# x -> masa del quark up
# y -> masa del quark down
# c -> color basado en el fitness
scatter = ax.scatter(
    df_gen1['mass_up_quark'], 
    df_gen1['mass_down_quark'], 
    c=df_gen1['fitness'], 
    cmap='viridis',  # Un mapa de color popular (puedes probar 'plasma', 'inferno', 'magma')
    alpha=0.7,
    s=15 # Tamaño del punto
)

# Línea de límite de estabilidad (m_up = m_down)
x_limit = np.linspace(min(df_gen1['mass_up_quark']), max(df_gen1['mass_up_quark']), 100)
ax.plot(x_limit, x_limit, 'r--', label='Límite de Estabilidad (m_up = m_down)')

# Barra de color
cbar = fig.colorbar(scatter, ax=ax)
cbar.set_label('Puntuación de Fitness', fontsize=12)

# Etiquetas y títulos
ax.set_xlabel('Masa del Quark Up (kg)', fontsize=12)
ax.set_ylabel('Masa del Quark Down (kg)', fontsize=12)
ax.set_title('Paisaje de Viabilidad: Masas de los Quarks de 1ª Generación', fontsize=16)
ax.grid(True, linestyle='--', alpha=0.3)
ax.legend()
ax.set_aspect('equal', adjustable='box') # Asegura que los ejes tengan la misma escala

# Formato de notación científica para los ejes
ax.ticklabel_format(style='sci', axis='both', scilimits=(0,0))

plt.show()