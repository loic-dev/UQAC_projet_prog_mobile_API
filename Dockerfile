# Utiliser une image de base légère avec Rust préinstallé
FROM rust:latest as builder

# Copier les fichiers du projet dans le conteneur
COPY ./ ./

# Construire l'application Rust
RUN cargo build --release

# Exposer le port sur lequel l'application écoute (si nécessaire)
EXPOSE 8000

# Commande pour exécuter l'application au démarrage du conteneur
CMD ["./target/release/uqac_projet_prog_mobile_api"]
