
Je souhaite que l'application puisse gérer différents jeux de cartes

On va appeler ces jeux de cartes des "decks".

- La base de données par défaut doit dorénavant s'appeler "./deck.db" (plus ./flashcards.db)
    - Si elle n'existe pas l'application la construit en utilisant le répertoire associé par défaut qui se nomme "deck" (lire la discussion plus bas concernant son emplacement)

- Il faut que l'application puisse créer différents decks
    - Exemple : "rust-flashcard -r my_deck" ou en version longue "rust-flashcard --rebuild-deck my_deck"
    - L'option "--rebuild-db" doit donc être renommée "--rebuild-deck"
    - Les 2 exemples précédents génèrent un fichier "./my_deck.db"
    - L'application utilise le contenu du répertoire associé qui se nomme "my_deck" (lire la discussion plus bas concernant son emplacement)

- Il faut que l'application puisse charger différents decks
    - Exemple : "rust-flashcard -d my_deck" ou en version longue "rust-flashcard --deck my_deck"
    - Il faut se poser la question concernant le nom du deck qui est affiché dans les pages HTML (actuellement `deck_name` issu de `config.rs`).
    - Faut il garder le nom du deck par défaut dans `config.rs` (je pense que oui)
    - A quel moment préciser le nom à afficher quand le deck n'est pas le deck par défaut?
        - rust-flashcard -d my_deck -n "Nom du Deck" ou encore rust-flashcard -deck my_deck -deck-name "Nom du Deck"
        - Si on utilise pas le paramètre deck-name alors le nom à afficher est le nom du deck plutôt que rine (""). Exemple : "my_deck" avec l'exemple précédent
    - Sans option l'application charge le deck par défaut "./deck.db"
    - Le nom de la base à charger c'est le nom du deck, c'est à dire le nom du répertoire où on retrouve les sous-répertoires "img/" et "md/" (lire la discussion plus bas concernant l'emplacement du répertoire en question)

## Discussion concernant l'organisation des répertoires
* Actuellement la base est construite en utilisant le contenu du répertoire static/
* Ce dernier est aussi envoyé sur Heroku
* Afin d'illustrer les 2 strategies possibles, ci-dessous on duplique le contenu du deck par défaut dans un jeu qui s'appelle deck_42. Les 2 decks ont le même contenu mais c'est pas grave, c'est pour l'exemple.
* Il faut choisir entre :
1. Renommer le répertoire "static/" en "deck/", le dupliquer et nommer sa copie "deck_42/" par exemple.
    - On peut customiser css/ js/ et le favicon par deck mais on duplique le code
2. Créer "static/deck" et y copier les sous-répertoires "img/" et "md/" puis dupliquer "static/deck" en "static/dec_42" par exemple.
    - On ne duplique pas le css, js et le favicon est le même

Dans les 2 cas
- il y a un impact conséquent sur le contenu du répertoire md/ car dans les documents il y a des références à "../static/md/assets/my_image.png" par exemple
- il y a aussi un impact fort sur le code existant

```
# Organisation 1
deck/
    css/
    js/
    favicon
    img/
    md/
deck1/
    css/
    js/
    favicon
    md/
deck2/
    css/
    js/
    favicon
    img/
deck3/
    css/
    js/
    favicon
    img/
    md/
```

```
# Organisation 2
static/
    css/
    js/
    favicon
    deck/
        img/
        md/
    deck1/
        md/
    deck2/
        img/
    deck3/
        img/
        md/
```

J'ai envie de proposer les phases suivantes:
1. Discuter puis décider du meilleur type d'organisation des répertoires pour les decks (1 ou 2)
2. Utiliser l'application avec le deck par défaut
    1. Idéalement il faut que l'application s'occupe du deck par défaut comme n'importe quel autre deck sauf qu'au lieu d'aller chercher son nom via la ligne de commande, elle l'obtient via le fichier de config. Comme ça on anticipe l'utilisation des autres decks.
    1. Renommer les répertoires en conséquences
    1. Réorganiser les répertoires en conséquence
    1. modifier le code
    1. modifier les .md pour tenir compte des nouveaux chemins
    1. tester
3. Ajouter un second deck
    1. tester









