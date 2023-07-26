# Tasks

## Techniques

### Initialisation du projet ✅
- rustup update
- initialiser le projet
- lancer la base de donnée
    - créer un docker compose pour surrealDB
- créer la CLI avec CLAP
    - créer un bootstrap
    - créer le lancement du serveur
    - créer le lancement de l'interface de commande (console)

### Database (Première partie) ✅
- connecter la base de donnée

### Serveur HTTP (Première partie) ✅
- créer un serveur HTTP avec Rocket
    - créer un builder qui va créer l'instance de Rocket
    - charger les states de Rocket
- dispatcher l'instance de Rocket dans le bootstrap

### Command interface ✅
- Créer le trait de commande
- Créer la registry de commande
- Charger la registry en tant que state dans rocket

### Database (2eme partie) ✅
- Créer le modèle "APIUser"
- Créer les DTOs
- Créer le service de manipulation des utilisateurs API
- Créer une commande pour créer un utilisateur API
- Créer une commande pour supprimer un utilisateur API

### Serveur HTTP (2ème partie)
- Créer un endpoint de test (uniquement en debug) pour tester la connexion.
- Créer un endpoint de test (uniquement en debug) pour tester le mécanisme de connexion.
- Créer un endpoint API pour créer un utilisateur.

### Serveur HTTP (3eme partie)
- Créer le endpoint pour consulter un site statique via son sous domaine
    - Ajouter une variable d'env pour la racine des sites statiques.
    - Créer un service pour vérifier si un site existe (mock)
    - Créer un service pour vérifier si un utilisateur a accès à un site (mock)
    - Servir le fichier demandé si l'utilisateur a accès au site

### Database (3eme partie)
- Créer le modèle "Site"
- Créer les DTOs du modèle "Site"
- Créer le service de manipulation des sites
- Créer une commande pour rafraichir les sites depuis la racine des sites statiques
- Créer le modèle "SiteAccess"
- Créer les DTOs du modèle "SiteAccess"
- Créer le service de manipulation des accès aux siteaccess.
- Remplacer le mock du service de vérification d'accès par le service de manipulation des accès aux sites.
- Remplacer le mock du service de vérification de l'existence d'un site par le service de manipulation des sites.

### Authentification
> TBD
