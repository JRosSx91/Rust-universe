# plotter_interactive.py - Interactive Viability Landscape using Plotly

import pandas as pd
import plotly.express as px
import numpy as np

# --- 1. Lectura y Preparación de Datos ---
try:
    df = pd.read_csv('landscape_data.csv', dtype=np.float64)
except FileNotFoundError:
    print("Error: 'landscape_data.csv' not found. Run the Rust simulation first.")
    exit()

df_gen1 = df[df['winning_gen'] == 1].copy()

if df_gen1.empty:
    print("No viable universes found for Generation 1 in the data.")
    exit()

print("Generando gráfico interactivo...")

# --- 2. Creación del Gráfico Interactivo ---
# Usamos Plotly Express, una forma rápida y potente de crear figuras.
fig = px.scatter(
    df_gen1,
    x="mass_up_quark",
    y="mass_down_quark",
    color="fitness",  # El color de cada punto se basa en el fitness
    color_continuous_scale=px.colors.sequential.Viridis, # Mismo esquema de color que nos gusta
    title="Paisaje de Viabilidad Interactivo (Generación 1)",
    labels={ # Etiquetas personalizadas para los ejes
        "mass_up_quark": "Masa del Quark Up (kg)",
        "mass_down_quark": "Masa del Quark Down (kg)",
        "fitness": "Fitness Score"
    },
    # --- LA MAGIA DEL TOOLTIP OCURRE AQUÍ ---
    # hover_data nos permite añadir columnas extra al tooltip que aparece al pasar el ratón.
    hover_data={
        'fitness': ':.6f', # Muestra el fitness con 6 decimales
        'winning_gen': True, # Muestra la generación ganadora
        'mass_up_quark': ':.4e', # Muestra la masa en notación científica
        'mass_down_quark': ':.4e'
    }
)

# --- 3. Mejoras Estéticas del Gráfico ---
# Añadimos la línea de límite de estabilidad (m_up = m_down)
min_val = min(df_gen1['mass_up_quark'].min(), df_gen1['mass_down_quark'].min())
max_val = max(df_gen1['mass_up_quark'].max(), df_gen1['mass_down_quark'].max())
fig.add_shape(
    type="line",
    x0=min_val, y0=min_val,
    x1=max_val, y1=max_val,
    line=dict(color="Red", width=2, dash="dash"),
    name="Límite de Estabilidad"
)

# Ponemos un fondo oscuro y ajustamos los márgenes
fig.update_layout(
    template="plotly_dark",
    margin=dict(l=20, r=20, t=40, b=20)
)

# --- 4. Guardar y Mostrar el Gráfico ---
# Esto guardará el gráfico en un archivo HTML y lo abrirá en tu navegador.
fig.write_html("interactive_landscape.html")
fig.show()

print("\nGráfico interactivo guardado como 'interactive_landscape.html'")
print("Puedes abrir este archivo en tu navegador para explorar los datos.")