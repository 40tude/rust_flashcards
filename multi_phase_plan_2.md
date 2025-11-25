# Multi-Phase Plan: Category/Subcategory Implementation

## Vue d'ensemble

Ajouter category et subcategory aux flashcards:
- Extraire depuis format "Question : CATEGORY - SUBCATEGORY - ..."
- Stocker en base de données
- Rendre searchable via FTS5
- Afficher sur format "CATEGORY - SUBCATEGORY" (une ligne)

## Phase 1: Validation des fichiers markdown

**Objectif**: ~~Vérifier que tous les fichiers .md respectent le format requis~~ **DÉJÀ FAIT**

**Pattern validé**: `Question : CATEGORY - SUBCATEGORY - La question...`
- Regex validation PowerShell: `^Question\s*:\s*.+\s-\s.+\s-\s.+`
- Regex extraction Rust: `^\s*:\s*([^-]+?)\s-\s([^-]+?)\s-\s(.+)`
  - Capture group 1: CATEGORY
  - Capture group 2: SUBCATEGORY
  - Capture group 3: QUESTION_TEXT

**Status**: ✅ Validation effectuée, fichiers conformes

**→ Passer directement à Phase 2**

---

## Phase 2: Modification du schéma de base de données

**Objectif**: Ajouter colonnes category et subcategory

**Fichier**: `src/db/schema.rs`

**Modifications**:
```rust
CREATE TABLE flashcards (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    category TEXT,           -- NOUVEAU
    subcategory TEXT,        -- NOUVEAU
    question_html TEXT NOT NULL,
    answer_html TEXT NOT NULL
)

CREATE VIRTUAL TABLE flashcards_fts USING fts5(
    id UNINDEXED,
    category,                -- NOUVEAU (searchable)
    subcategory,             -- NOUVEAU (searchable)
    question_html,
    answer_html
)
```

**Critères de succès**:
- ✅ Table flashcards a colonnes category/subcategory
- ✅ FTS5 inclut category/subcategory (searchable)
- ✅ Code compile sans erreur

---

## Phase 3: Modification du modèle Flashcard

**Objectif**: Ajouter champs au struct Rust

**Fichier**: `src/db/models.rs`

**Modifications**:
```rust
pub struct Flashcard {
    pub id: i64,
    pub category: Option<String>,      // NOUVEAU
    pub subcategory: Option<String>,   // NOUVEAU
    pub question_html: String,
    pub answer_html: String,
}
```

**Critères de succès**:
- ✅ Struct modifié avec Option<String>
- ✅ Constructor updated si nécessaire
- ✅ Code compile

---

## Phase 4: Modification des requêtes SQL

**Objectif**: Modifier toutes les fonctions d'insertion/sélection

**Fichier**: `src/db/queries.rs`

**Fonctions à modifier**:
1. `insert_flashcard()` - Ajouter params category/subcategory
2. `get_random_flashcard()` - SELECT avec nouveaux champs
3. `get_random_searched_flashcard()` - SELECT avec nouveaux champs
4. `populate_fts_table()` - Sync avec nouveaux champs

**Critères de succès**:
- ✅ INSERT inclut category/subcategory
- ✅ SELECT récupère category/subcategory
- ✅ FTS5 sync inclut nouveaux champs
- ✅ Recherche FTS5 cherche aussi dans category/subcategory
- ✅ Code compile

---

## Phase 5: Extraction depuis markdown

**Objectif**: Parser "Question : CAT - SUBCAT - ..." avec regex

**Fichier**: `src/content/markdown.rs`

**Regex d'extraction**:
```rust
let category_regex = Regex::new(r"^\s*:\s*([^-]+?)\s-\s([^-]+?)\s-\s(.+)").unwrap();
```
- Capture group 1: CATEGORY
- Capture group 2: SUBCATEGORY
- Capture group 3: QUESTION_TEXT

**Gestion erreurs**:
- Question non-conforme → category/subcategory = None
- Log warning avec chemin fichier

**Critères de succès**:
- ✅ Extraction regex fonctionne
- ✅ Questions conformes → category/subcategory extraites
- ✅ Questions non-conformes → None + warning
- ✅ Code compile

---

## Phase 6: Gestion des images PNG

**Objectif**: Images n'ont pas de category/subcategory

**Fichier**: `src/content/images.rs`

**Modification**:
```rust
queries::insert_flashcard(pool, None, None, &question_html, &answer_html)?;
```

**Critères de succès**:
- ✅ Images insérées avec category=NULL, subcategory=NULL
- ✅ Code compile

---

## Phase 7: Modification des routes

**Objectif**: Passer category/subcategory aux templates

**Fichiers**:
- `src/routes/index.rs`
- `src/routes/search_results.rs`

**Modifications structs**:
```rust
struct IndexTemplate {
    category: Option<String>,
    subcategory: Option<String>,
    q_html: String,
    a_html: String,
    nb_cards: i64,
}
```

**Critères de succès**:
- ✅ Templates reçoivent category/subcategory
- ✅ Code compile

---

## Phase 8: Modification des templates

**Objectif**: Afficher "CATEGORY - SUBCATEGORY" sur une ligne

**Fichiers**:
- `templates/index.html`
- `templates/search_results.html`

**Format d'affichage**:
```html
{% if category.is_some() %}
<div class="mt-2">
    <p class="text-muted">
        <small>
            <strong>{{ category.unwrap() }}{% if subcategory.is_some() %} - {{ subcategory.unwrap() }}{% endif %}</strong>
        </small>
    </p>
</div>
{% endif %}
```

**Critères de succès**:
- ✅ Category et subcategory sur même ligne séparées par " - "
- ✅ Si pas de subcategory: juste category
- ✅ Si pas de category: rien affiché
- ✅ Rendu HTML correct

---

## Phase 9: Migration base de données

**Objectif**: Reconstruire DB avec nouveau schéma

**Commandes**:
```powershell
rm flashcards.db
cargo run
```

**Critères de succès**:
- ✅ DB supprimée
- ✅ App redémarre
- ✅ Nouveau schéma créé
- ✅ Markdown chargés avec category/subcategory
- ✅ PNG chargés avec NULL
- ✅ Warnings affichés pour questions non-conformes

---

## Phase 10: Tests et validation

**Objectif**: Vérifier que tout fonctionne

**Tests à effectuer**:
1. **Affichage**: Vérifier "CATEGORY - SUBCATEGORY" sur fiches
2. **Recherche**: Chercher par nom de catégorie (ex: "Python", "Deep Learning")
3. **Images PNG**: Vérifier qu'elles n'ont pas de category affichée
4. **Session**: Vérifier navigation Next fonctionne
5. **Search results**: Vérifier recherche + affichage category

**Commandes test**:
```powershell
cargo run
# Ouvrir http://localhost:8080
# Tester navigation Next
# Tester /search avec "Python"
# Tester /search avec "Deep Learning"
```

**Critères de succès**:
- ✅ Category/subcategory affichées correctement
- ✅ Recherche par category fonctionne
- ✅ PNG sans category
- ✅ Navigation et session OK
- ✅ Aucune erreur console

---

## Phase 11: Commit et déploiement

**Objectif**: Sauvegarder et déployer sur Heroku

**Actions**:
1. Commit changements
2. Push sur GitHub
3. Deploy sur Heroku
4. Tester en production

**Commandes**:
```bash
git add .
git commit -m "Add category/subcategory support with FTS5 search"
git push
git push heroku main
```

**Critères de succès**:
- ✅ Code commité
- ✅ Poussé sur GitHub
- ✅ Déployé sur Heroku
- ✅ Production fonctionne correctement

---

## Fichiers à modifier (résumé)

1. ✅ `check_categories.ps1` - Script validation (déjà créé)
2. `src/db/schema.rs` - Schéma DB
3. `src/db/models.rs` - Struct Flashcard
4. `src/db/queries.rs` - 4 fonctions SQL
5. `src/content/markdown.rs` - Extraction regex
6. `src/content/images.rs` - Passer None
7. `src/routes/index.rs` - Template struct
8. `src/routes/search_results.rs` - Template struct
9. `templates/index.html` - Affichage
10. `templates/search_results.html` - Affichage

**Total**: ~200-250 lignes modifiées