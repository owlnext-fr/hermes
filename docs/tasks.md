# Tasks

## Techniques

### Initialisation du projet
- rustup update
- initialiser le projet
- lancer la base de donnée
    - créer un docker compose pour surrealDB
- créer la CLI avec CLAP
    - créer un bootstrap
    - créer le lancement du serveur
    - créer le lancement de l'interface de commande (console)

### TBD


```mermaid
mindmap
  root((HERMES))
    %% technos
    Rust
        CLI
            Serveur
            Console
            "CLAP"
    SurrealDB
        Lancer un serveur à côté
            Conteneur
        Gestion des modèles
    %% concepts
    Serveur HTTP
        "Rocket"
    Sous domaine + site statique
        Préciser la racine des sites
        Sécuriser l'accès aux fichiers
            Statuts HTTP
    authentification
        Login + password
        argon2id
        poser un cookie pour le subdomain
    API
        Sécurité
            header avec clé
        Format
            REST
                JSON
        Manipulation des accès

```
