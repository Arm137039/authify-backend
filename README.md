# Authify API

Backend d'authentification en Rust pour une application sociale type Twitter/X.

## Table des matières

- [Documentation API](#documentation-api)
- [Intégration Frontend](#intégration-frontend)

---

## Documentation API

### Base URL

- **Développement**: `http://localhost:8081`
- **Production**: `https://api.authify.armx.be` (à configurer)

### Authentification

Toutes les routes `/api/v1/*` nécessitent un token Firebase dans le header:

```
Authorization: Bearer <firebase-id-token>
```

### Endpoints

#### Health Check

```http
GET /health
```

**Réponse** `200 OK`:
```json
{
  "status": "ok",
  "version": "0.1.0"
}
```

---

#### Créer un profil utilisateur

```http
POST /api/v1/auth/register
Authorization: Bearer <firebase-id-token>
Content-Type: application/json
```

**Body**:
```json
{
  "username": "john_doe",
  "display_name": "John Doe",
  "bio": "Développeur passionné"  // optionnel
}
```

**Contraintes**:
- `username`: 3-30 caractères, uniquement lettres, chiffres et underscores
- `display_name`: 1-100 caractères
- `bio`: max 500 caractères (optionnel)

**Réponse** `201 Created`:
```json
{
  "success": true,
  "data": {
    "uid": "firebase-uid-123",
    "email": "john@example.com",
    "username": "john_doe",
    "display_name": "John Doe",
    "bio": "Développeur passionné",
    "avatar_url": null,
    "followers_count": 0,
    "following_count": 0,
    "posts_count": 0,
    "is_verified": false,
    "is_private": false,
    "created_at": "2024-01-15T10:30:00Z"
  },
  "message": "Profil créé avec succès"
}
```

**Erreurs possibles**:
- `400` - Validation échouée (username invalide, etc.)
- `401` - Token Firebase invalide ou manquant
- `409` - Username déjà pris ou profil existe déjà

---

#### Obtenir le profil courant

```http
GET /api/v1/auth/me
Authorization: Bearer <firebase-id-token>
```

**Réponse** `200 OK`:
```json
{
  "success": true,
  "data": {
    "uid": "firebase-uid-123",
    "email": "john@example.com",
    "username": "john_doe",
    "display_name": "John Doe",
    "bio": "Développeur passionné",
    "avatar_url": null,
    "followers_count": 0,
    "following_count": 0,
    "posts_count": 0,
    "is_verified": false,
    "is_private": false,
    "created_at": "2024-01-15T10:30:00Z"
  }
}
```

**Erreurs possibles**:
- `401` - Token Firebase invalide ou manquant
- `404` - Profil non trouvé (doit d'abord s'enregistrer)

---

### Posts

#### Obtenir la timeline (public)

```http
GET /api/v1/posts?limit=20&offset=0
```

**Query params**:
- `limit` (optionnel): nombre de posts (défaut: 20)
- `offset` (optionnel): décalage pour pagination (défaut: 0)

**Réponse** `200 OK`:
```json
{
  "success": true,
  "data": {
    "posts": [
      {
        "id": "550e8400-e29b-41d4-a716-446655440000",
        "author_uid": "firebase-uid-123",
        "content": "Mon premier post !",
        "likes_count": 5,
        "replies_count": 2,
        "reposts_count": 0,
        "parent_id": null,
        "created_at": "2024-01-15T10:30:00Z"
      }
    ],
    "count": 1
  }
}
```

---

#### Obtenir un post (public)

```http
GET /api/v1/posts/{id}
```

**Réponse** `200 OK`:
```json
{
  "success": true,
  "data": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "author_uid": "firebase-uid-123",
    "content": "Mon premier post !",
    "likes_count": 5,
    "replies_count": 2,
    "reposts_count": 0,
    "parent_id": null,
    "created_at": "2024-01-15T10:30:00Z"
  }
}
```

---

#### Obtenir les réponses d'un post (public)

```http
GET /api/v1/posts/{id}/replies?limit=20&offset=0
```

**Réponse** `200 OK`:
```json
{
  "success": true,
  "data": {
    "posts": [
      {
        "id": "550e8400-e29b-41d4-a716-446655440001",
        "author_uid": "firebase-uid-456",
        "content": "Super post !",
        "likes_count": 1,
        "replies_count": 0,
        "reposts_count": 0,
        "parent_id": "550e8400-e29b-41d4-a716-446655440000",
        "created_at": "2024-01-15T11:00:00Z"
      }
    ],
    "count": 1
  }
}
```

---

#### Obtenir les posts d'un utilisateur (public)

```http
GET /api/v1/users/{uid}/posts?limit=20&offset=0
```

**Réponse** `200 OK`:
```json
{
  "success": true,
  "data": {
    "posts": [...],
    "count": 5
  }
}
```

---

#### Créer un post (authentifié)

```http
POST /api/v1/posts
Authorization: Bearer <firebase-id-token>
Content-Type: application/json
```

**Body**:
```json
{
  "content": "Mon premier post !"
}
```

**Contraintes**:
- `content`: 1-280 caractères

**Réponse** `201 Created`:
```json
{
  "success": true,
  "data": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "author_uid": "firebase-uid-123",
    "content": "Mon premier post !",
    "likes_count": 0,
    "replies_count": 0,
    "reposts_count": 0,
    "parent_id": null,
    "created_at": "2024-01-15T10:30:00Z"
  },
  "message": "Post créé avec succès"
}
```

---

#### Répondre à un post (authentifié)

```http
POST /api/v1/posts/{parent_id}/replies
Authorization: Bearer <firebase-id-token>
Content-Type: application/json
```

**Body**:
```json
{
  "content": "Super post !"
}
```

**Réponse** `201 Created`:
```json
{
  "success": true,
  "data": {
    "id": "550e8400-e29b-41d4-a716-446655440001",
    "author_uid": "firebase-uid-456",
    "content": "Super post !",
    "likes_count": 0,
    "replies_count": 0,
    "reposts_count": 0,
    "parent_id": "550e8400-e29b-41d4-a716-446655440000",
    "created_at": "2024-01-15T11:00:00Z"
  },
  "message": "Réponse créée avec succès"
}
```

---

#### Supprimer un post (authentifié)

```http
DELETE /api/v1/posts/{id}
Authorization: Bearer <firebase-id-token>
```

**Réponse** `204 No Content`

**Erreurs possibles**:
- `401` - Non authentifié
- `403` - Pas autorisé (pas l'auteur du post)
- `404` - Post non trouvé

---

### Format des erreurs

Toutes les erreurs suivent ce format:

```json
{
  "error": {
    "code": "ERROR_CODE",
    "message": "Description de l'erreur"
  }
}
```

**Codes d'erreur**:
| Code | HTTP | Description |
|------|------|-------------|
| `UNAUTHORIZED` | 401 | Token manquant ou invalide |
| `FORBIDDEN` | 403 | Accès non autorisé |
| `NOT_FOUND` | 404 | Ressource non trouvée |
| `VALIDATION_ERROR` | 400 | Données invalides |
| `CONFLICT` | 409 | Conflit (username pris, etc.) |
| `INTERNAL_ERROR` | 500 | Erreur serveur |

---

## Intégration Frontend

### Installation Firebase SDK

```bash
npm install firebase
```

### Configuration Firebase (frontend)

```javascript
// src/lib/firebase.js
import { initializeApp } from 'firebase/app';
import { getAuth } from 'firebase/auth';

const firebaseConfig = {
  apiKey: "AIza...",           // Depuis Firebase Console
  authDomain: "authify-prod-12345.firebaseapp.com",
  projectId: "authify-prod-12345",
  // ... autres configs
};

const app = initializeApp(firebaseConfig);
export const auth = getAuth(app);
```

### Inscription utilisateur

```javascript
import { createUserWithEmailAndPassword } from 'firebase/auth';
import { auth } from './lib/firebase';

async function signUp(email, password, username, displayName) {
  // 1. Créer le compte Firebase
  const userCredential = await createUserWithEmailAndPassword(auth, email, password);

  // 2. Obtenir le token
  const token = await userCredential.user.getIdToken();

  // 3. Créer le profil sur le backend
  const response = await fetch('http://localhost:8081/api/v1/auth/register', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      'Authorization': `Bearer ${token}`
    },
    body: JSON.stringify({
      username,
      display_name: displayName
    })
  });

  if (!response.ok) {
    const error = await response.json();
    throw new Error(error.error.message);
  }

  return response.json();
}
```

### Connexion utilisateur

```javascript
import { signInWithEmailAndPassword } from 'firebase/auth';
import { auth } from './lib/firebase';

async function signIn(email, password) {
  // 1. Se connecter via Firebase
  const userCredential = await signInWithEmailAndPassword(auth, email, password);

  // 2. Obtenir le token
  const token = await userCredential.user.getIdToken();

  // 3. Récupérer le profil
  const response = await fetch('http://localhost:8081/api/v1/auth/me', {
    headers: {
      'Authorization': `Bearer ${token}`
    }
  });

  return response.json();
}
```

### Hook React pour l'authentification

```javascript
// src/hooks/useAuth.js
import { useState, useEffect, createContext, useContext } from 'react';
import { onAuthStateChanged } from 'firebase/auth';
import { auth } from '../lib/firebase';

const AuthContext = createContext(null);

export function AuthProvider({ children }) {
  const [user, setUser] = useState(null);
  const [profile, setProfile] = useState(null);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    const unsubscribe = onAuthStateChanged(auth, async (firebaseUser) => {
      setUser(firebaseUser);

      if (firebaseUser) {
        // Charger le profil depuis le backend
        const token = await firebaseUser.getIdToken();
        try {
          const res = await fetch('http://localhost:8081/api/v1/auth/me', {
            headers: { 'Authorization': `Bearer ${token}` }
          });
          if (res.ok) {
            const data = await res.json();
            setProfile(data.data);
          }
        } catch (err) {
          console.error('Erreur chargement profil:', err);
        }
      } else {
        setProfile(null);
      }

      setLoading(false);
    });

    return unsubscribe;
  }, []);

  // Helper pour obtenir le token
  const getToken = async () => {
    if (!user) return null;
    return user.getIdToken();
  };

  // Helper pour les requêtes API authentifiées
  const apiRequest = async (url, options = {}) => {
    const token = await getToken();
    return fetch(url, {
      ...options,
      headers: {
        ...options.headers,
        'Authorization': `Bearer ${token}`,
        'Content-Type': 'application/json'
      }
    });
  };

  return (
    <AuthContext.Provider value={{ user, profile, loading, getToken, apiRequest }}>
      {children}
    </AuthContext.Provider>
  );
}

export const useAuth = () => useContext(AuthContext);
```

### Utilisation du hook

```jsx
// src/pages/Profile.jsx
import { useAuth } from '../hooks/useAuth';

function Profile() {
  const { profile, loading } = useAuth();

  if (loading) return <div>Chargement...</div>;
  if (!profile) return <div>Non connecté</div>;

  return (
    <div>
      <h1>@{profile.username}</h1>
      <p>{profile.display_name}</p>
      <p>{profile.bio}</p>
      <p>{profile.followers_count} followers</p>
    </div>
  );
}
```