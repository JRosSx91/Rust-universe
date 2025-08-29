# plotter_landscape.py - Viability Landscape (v. Final Corregido y Manual)

import pandas as pd
import matplotlib.pyplot as plt
import numpy as np

pd.options.display.float_format = '{:.12e}'.format

try:
    df = pd.read_csv('landscape_data.csv', dtype=np.float64)
except FileNotFoundError:
    print("Error: 'landscape_data.csv' not found. Run the Rust simulation first.")
    exit()

df_gen1 = df[df['winning_gen'] == 1].copy()

if df_gen1.empty:
    print("No viable universes found for Generation 1 in the data.")
    exit()

# --- 2. Análisis Numérico (para referencia, no para impresión aquí) ---
df_elite = df_gen1.sort_values(by='fitness', ascending=False).head(500)
best_universe = df_elite.iloc[0]
max_fitness = best_universe['fitness']


# --- 3. Visualización ---
plt.style.use('dark_background')
fig, (ax1, ax2) = plt.subplots(1, 2, figsize=(24, 12)) # Dos subplots

fig.suptitle('Paisaje de Viabilidad de la Generación 1', fontsize=20)

# GRÁFICO 1: Vista Global del Paisaje
scatter1 = ax1.scatter(
    df_gen1['mass_up_quark'], df_gen1['mass_down_quark'], c=df_gen1['fitness'], 
    cmap='viridis', alpha=0.6, s=10, vmin=0.2, vmax=max_fitness
)
ax1.set_title('Vista Global del Paisaje', fontsize=16)
ax1.set_xlabel('Masa del Quark Up (kg)')
ax1.set_ylabel('Masa del Quark Down (kg)')
ax1.grid(True, linestyle='--', alpha=0.2)
ax1.ticklabel_format(style='sci', axis='both', scilimits=(0,0)) # Notación científica en los ejes


# GRÁFICO 2: Vista de la Élite (Top 500 Universos)
scatter2 = ax2.scatter(
    df_elite['mass_up_quark'], df_elite['mass_down_quark'], c=df_elite['fitness'], 
    cmap='viridis', alpha=0.9, s=50, vmin=df_elite['fitness'].min(), vmax=max_fitness
)
center_up = best_universe['mass_up_quark']
center_down = best_universe['mass_down_quark']
ax2.plot(center_up, center_down, 'r*', markersize=15, label=f'Mejor Fitness: {max_fitness:.4f}')

# Línea de límite de estabilidad (m_up = m_down)
# Aseguramos que la línea se dibuje en el rango visible del zoom
min_val = min(ax2.get_xlim()[0], ax2.get_ylim()[0])
max_val = max(ax2.get_xlim()[1], ax2.get_ylim()[1])
x_line = np.linspace(min_val, max_val, 100)
ax2.plot(x_line, x_line, 'r--', alpha=0.5, label='Límite de Estabilidad')

ax2.set_title('Vista de la Élite (Top 500 Universos)', fontsize=16)
ax2.set_xlabel('Masa del Quark Up (kg)')
ax2.set_ylabel('Masa del Quark Down (kg)')
ax2.grid(True, linestyle='--', alpha=0.3)
ax2.ticklabel_format(style='sci', axis='both', scilimits=(0,0)) # Notación científica en los ejes
ax2.legend()


# Barra de color - AHORA POSICIONADA EN EL EXTREMO DERECHO
# Creamos un eje separado para el colorbar a la derecha de ambos subplots
cbar_ax = fig.add_axes([0.92, 0.15, 0.02, 0.7]) # [left, bottom, width, height]
cbar = fig.colorbar(scatter1, cax=cbar_ax) # Usamos scatter1 para definir el mapeo de color
cbar.set_label('Puntuación de Fitness', fontsize=12)


plt.tight_layout(rect=[0, 0, 0.9, 0.96]) # Ajustamos el rect para dejar espacio al colorbar
plt.show()

print("\n------------------------------------------------------------------------------------------")
print("NOTA SOBRE INTERACTIVIDAD:")
print("Para tooltips interactivos (hover over points), necesitaríamos usar un entorno como Jupyter Notebook")
print("o una librería como Plotly/Bokeh que genere gráficos interactivos en HTML/JavaScript.")
print("Matplotlib en un script simple tiene limitaciones para esta funcionalidad avanzada.")
print("------------------------------------------------------------------------------------------")