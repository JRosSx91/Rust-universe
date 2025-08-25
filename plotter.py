# plotter.py - v5.0 (Holistic Analysis)

import pandas as pd
import matplotlib.pyplot as plt

try:
    log_data = pd.read_csv('evolution_log_final.csv', decimal='.')
except FileNotFoundError:
    print("Error: Could not find 'evolution_log_final.csv'.")
    exit()

plt.style.use('dark_background')
fig, (ax1, ax2, ax3) = plt.subplots(3, 1, figsize=(14, 20), sharex=True)
fig.suptitle('Convergencia Evolutiva Holística', fontsize=20)

# 1. Gráfico de Fitness
ax1.plot(log_data['generation'], log_data['best_fitness'], 'o-', color='cyan', label='Mejor Fitness')
ax1.set_ylabel('Puntuación de Fitness')
ax1.set_ylim(-0.05, 1.05)
ax1.grid(True, linestyle='--', alpha=0.3)
ax1.set_title('Evolución de la Aptitud General')

# 2. Gráfico de Propiedades Emergentes
ax2.plot(log_data['generation'], log_data['deuterium_energy'], 'o-', color='lime', label='Energía Enlace Deuterio (MeV)')
ax2.axhline(2.22, color='red', linestyle='--', label='Óptimo Deuterio (~2.22 MeV)')
ax2.set_ylabel('Energía (MeV)')
ax2.grid(True, linestyle='--', alpha=0.3)
ax2.legend()
ax2.set_title('Convergencia de la Nucleosíntesis')

# 3. Gráfico de la Generación de Leptones Estables
# Muestra qué generación fue la base de la química en el universo ganador de cada era.
ax3.plot(log_data['generation'], log_data['stable_lepton_gen'], 'o', color='magenta', markersize=8, label='Generación de Leptón Estable')
ax3.set_xlabel('Generación')
ax3.set_ylabel('Número de Generación')
ax3.set_yticks([0, 1, 2, 3])
ax3.set_yticklabels(['Extinto', 'Gen 1 (Electrón)', 'Gen 2 (Muón)', 'Gen 3 (Tauón)'])
ax3.grid(True, linestyle='--', alpha=0.3)
ax3.set_title('Base de la Química a lo Largo de la Evolución')

plt.tight_layout(rect=[0, 0.03, 1, 0.96])
print("\nMostrando el análisis holístico de la evolución...")
plt.show()