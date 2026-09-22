# Pong (Rust)

Une implémentation du jeu classique **Pong**, écrite en Rust.

## ⚠️ Statut du projet

**Ce projet est en version bêta.**

- Le jeu ne fonctionne correctement que sur les écrans à **120 Hz**.
- Le support des écrans **60 Hz** est en cours de développement et sera bientôt disponible.

N'hésitez pas à signaler tout bug ou comportement inattendu, surtout si vous testez sur une configuration différente de 120 Hz.

## Installation

```bash
git clone https://github.com/Quicheman83/rs_pong.git
cd rs_pong
cargo build --release
```

## Utilisation

```bash
cargo run --release
```

## Prérequis

- [Rust](https://www.rust-lang.org/) (édition récente recommandée)
- Un écran 120 Hz (pour une expérience optimale, en attendant le support 60 Hz)

## Roadmap

- [ ] Support des écrans 60 Hz
- [ ] Ajustements de la physique de la balle (La balle a pas encore de physique LOL)
- [ ] Menu / options de configuration
- [ ] Mode multijoueur en réseau (à évaluer)
- [ ] support modulaire du moteur (support rudimentaire)

## Contribuer

Les retours et rapports de bugs sont les bienvenus.
