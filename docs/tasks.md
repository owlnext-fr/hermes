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

### Serveur HTTP (2ème partie)  ✅
- Créer un endpoint de test (uniquement en debug) pour tester l'app (json + html).

### Authentification API ✅
- Créer le guard d'authentification API pour authentifier un APIUser sur l'API.
- Créer un endpoint de test (uniquement en debug) pour tester l'autentification API (json)

### Gestion des User ✅
- Créer le modèle "User" (name, email, password)
- Créer les DTOs
- Créer le service de manipulation des utilisateurs API
- Créer un endpoint pour créer un utilisateur (POST /api/users -> JSON)

### Gestion des Sites
- Créer le modèle "Site"
- Créer les DTOs du modèle "Site"
- Créer le service de manipulation des sites
- Créer une commande pour rafraichir les sites depuis la racine des sites statiques
    - variable d'env pour le chemin root des sites statiques

### Gestion des SiteAccess
- Créer le modèle "SiteAccess"
- Créer les DTOs du modèle "SiteAccess"
- Créer le service de manipulation des accès aux siteaccess.

### Gestion des Sites (2ème partie)
- Créer le endpoint pour consulter un site statique via son sous domaine
    - Ajouter une variable d'env pour la racine des sites statiques.
    - Créer une méthode pour vérifier si un site existe
    - Créer une méthode pour vérifier si un utilisateur a accès à un site (mock -> true)
    - Servir le fichier demandé si l'utilisateur a accès au site

### Authentification User + SiteAccess
- Créer la page de login générique (html)
- Créer le handler du endpoint de login (POST /login -> cookie + redirect to index)
- Créer le guard d'authentification User + SiteAccess pour authentifier un User sur un SiteAccess via cookie.