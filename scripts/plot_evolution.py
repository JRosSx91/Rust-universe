# scripts/plot_evolution.py (Corregido)

import pandas as pd
import matplotlib.pyplot as plt

try:
    # CORRECCIÓN: Cambiado 'final_evolution.csv' a 'evolution_data.csv'
    log_data = pd.read_csv('evolution_data.csv')
except FileNotFoundError:
    print("Error: No se pudo encontrar 'evolution_data.csv'.")
    print("Asegúrate de haber ejecutado el modo 'evolve' primero.")
    exit()

plt.style.use('dark_background')
fig, ax1 = plt.subplots(1, 1, figsize=(15, 8)) # Simplificado a un solo gráfico
fig.suptitle('Evolución del Fitness Cósmico', fontsize=20)

# Gráfico de Fitness
ax1.plot(log_data['generation'], log_data['best_fitness'], 'o-', color='cyan', markersize=4, linewidth=1.5)
ax1.set_xlabel('Generación', fontsize=12)
ax1.set_ylabel('Mejor Fitness de la Generación', fontsize=12)

# Añadir una línea de umbral para referencia
ax1.axhline(y=0.6, color='red', linestyle='--', linewidth=1, label='Umbral de Reproducción (0.6)')

ax1.grid(True, linestyle='--', alpha=0.3)
ax1.set_title('Progreso de la Optimización Evolutiva', fontsize=16)
ax1.legend()

min_fitness = log_data['best_fitness'].min()
max_fitness = log_data['best_fitness'].max()
ax1.set_ylim(min_fitness - 0.000005, max_fitness + 0.000005)

# Mejorar la legibilidad de los números del eje Y si son muy parecidos
plt.ticklabel_format(style='plain', axis='y', useOffset=False)


plt.tight_layout(rect=[0, 0.03, 1, 0.95])
print("\nMostrando el análisis de la evolución...")
plt.show()