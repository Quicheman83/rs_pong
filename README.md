# RS_Pong

Une implémentation du jeu classique **Pong**, écrite en Rust.

## ⚠️ Statut du projet

**Ce projet est en version bêta.**

- Le jeu ne fonctionne correctement que sur les écrans à **144 Hz**.
- Le support des écrans autre que **144 Hz** est disponible mais peut etre instable.

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
- Un écran 144 Hz (pour une expérience optimale)

## Roadmap

- [x] Support des écrans 60 Hz
- [x] Ajustements de la physique de la balle
- [ ] Menu / options de configuration (en cours)
- [ ] Mode multijoueur en réseau (impossible avec le moteur actuel)
- [ ] support modulaire du moteur (support rudimentaire)

## Contribuer

Les retours et rapports de bugs sont les bienvenus.
