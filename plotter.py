# plotter.py - Lee los datos de Rust y crea el gráfico de viabilidad

import pandas as pd
import matplotlib.pyplot as plt

# 1. Leer el archivo CSV generado por Rust
try:
    data = pd.read_csv('viability_data.csv')
except FileNotFoundError:
    print("Error: No se encontró el archivo 'viability_data.csv'.")
    print("Asegúrate de ejecutar el programa de Rust primero con 'cargo run'.")
    exit()

# 2. Crear el gráfico
plt.style.use('dark_background')
fig, ax = plt.subplots(figsize=(12, 7))

ax.scatter(data['alpha'], data['fitness'], alpha=0.6, s=15,
           c=data['fitness'], cmap='viridis')

# 3. Añadir detalles y etiquetas
ax.axvline(1/137.036, color='red', linestyle='--', label='Alpha Observado (~1/137)')
ax.set_title('Paisaje de Viabilidad de Universos Simulados', fontsize=16)
ax.set_xlabel('Constante de Estructura Fina (alpha)', fontsize=12)
ax.set_ylabel('Puntuación de Fitness Evolutiva', fontsize=12)
ax.legend()
ax.grid(True, linestyle='--', alpha=0.2)

# 4. Mostrar el gráfico
print("\nMostrando el gráfico de viabilidad...")
plt.show()