# 🚀 The Harmonic 137

[![Simulation Status](https://img.shields.io/badge/simulation-Phase%201%20Complete-green.svg)](https://github.com/JRosSx91/Harmonic-137)
[![Version](https://img.shields.io/badge/version-16.0-blue.svg)](
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

A cosmological simulator in Rust that explores the **Cosmic Natural Selection** hypothesis and the fundamental nature of physical constants.

## 📜 The Theory: The Harmonic 137

Our fundamental premise is that the laws of physics are not an arbitrary set of constants, but the result of an evolutionary process on a cosmic scale. Universes "reproduce" through black holes, and with each new generation, their governing laws (`CosmicLaw`) can mutate.

The core hypothesis, "The Harmonic 137," posits that the dimensionless constants we observe (like the fine-structure constant, $\alpha \approx 1/137$) are "harmonics": fundamentally stable, resonant states that are favored by natural selection over eons.

A universe's **fitness** is measured by its ability to form massive stars that collapse into black holes, thus enabling "reproduction."

## 🛠️ Current Project Status: Phase 1 Complete

The current version (`v16.0`) has completed **Phase 1: Viability Landscape Mapping**.

- A brute-force simulation was run with 5 million universes featuring randomly generated physical laws.
- We have discovered that only **Generation 1** matter appears to be viable under the explored parameters.
- We have identified a **"Pinnacle of Viability"**: an extremely small and dense region in the parameter space where stable universes emerge.
- We have isolated our champion, "Adam," a universe with a fitness score of ~0.59, right on the threshold of reproduction (set at 0.6).

## ⚙️ Getting Started

1.  **Clone the Repository:**
    ```bash
    git clone [https://github.com/JRosSx91/Harmonic-137.git](https://github.com/JRosSx91/Harmonic-137.git)
    cd universo-rust
    ```
2.  **Run the Mapping Simulation:**
    - This will generate the `landscape_data.csv` file. The execution may take some time.
    ```bash
    cargo run --release
    ```
3.  **Visualize the Results:**
    - We recommend the interactive script for a better exploration of the data.
    ```bash
    pip install pandas plotly
    python scripts/plot_interactive.py
    ```
    - An `interactive_landscape.html` file will be generated and will open in your browser.

## 🗺️ Roadmap (The Inkwell)

- ✅ **Phase 1:** Viability Landscape Mapping.
- ⏳ **Phase 2:** **Evolutionary Simulation.** Use Adam's genome as a seed for an evolutionary simulation to see if the 0.6 fitness threshold can be crossed.
- 📝 **Phase 3:** **Physics Engine Enhancements.** Implement a more sophisticated model for gravity (`G`) and black hole formation.
- 🌌 **Phase 4:** **Exploration of New Premises.** Test deeper hypotheses (the primacy of Alpha, evolving constants, etc.).
- 📚 **Phase 5:** **Documentation and Dissemination.** Formalize the project for potential publication.
