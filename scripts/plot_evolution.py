# plotter.py - Final Version (Analysis of Winning Generation)

import pandas as pd
import matplotlib.pyplot as plt
import matplotlib.ticker as mticker

try:
    log_data = pd.read_csv('final_evolution.csv', decimal='.')
except FileNotFoundError:
    print("Error: Could not find 'final_evolution.csv'.")
    exit()

plt.style.use('dark_background')
fig, (ax1, ax2) = plt.subplots(2, 1, figsize=(15, 15), sharex=True)
fig.suptitle('Análisis Final de la Evolución Cósmica', fontsize=20)

# 1. Gráfico de Fitness
ax1.plot(log_data['generation'], log_data['best_fitness'], 'o-', color='cyan')
ax1.set_ylabel('Mejor Fitness')
ax1.set_ylim(-0.05, 1.05)
ax1.grid(True, linestyle='--', alpha=0.3)
ax1.set_title('Evolución de la Aptitud General', fontsize=16)

# 2. Gráfico de la Generación de Materia Ganadora
ax2.plot(log_data['generation'], log_data['winning_generation'], 'o', color='magenta', markersize=8)
ax2.set_xlabel('Generación')
ax2.set_ylabel('Generación de Materia Dominante')
# Usamos un formateador para poner etiquetas de texto en el eje Y
ax2.yaxis.set_major_formatter(mticker.FuncFormatter(
    lambda x, pos: {1: 'Gen 1 (Nuestro Universo)', 2: 'Gen 2 (Exótico)', 3: 'Gen 3 (Exótico)'}.get(x, 'Extinto')
))
ax2.set_yticks([1, 2, 3])
ax2.grid(True, linestyle='--', alpha=0.3)
ax2.set_title('Base de la Química a lo Largo de la Evolución', fontsize=16)


plt.tight_layout(rect=[0, 0.03, 1, 0.96])
print("\nMostrando el análisis final de la evolución...")
plt.show()