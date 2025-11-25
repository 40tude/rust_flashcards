<!-- 
08 10 2024
DONE : Les quizz de la section Python ont été vu

-->




<!-- 
<p align="center">
<img src="../static/md/assets/img1.png" alt="attention" width="577"/>
</p>

$$ E = mc^2 $$




#### Code snippet  

```python
# -----------------------------------------------------------------------------
def preprocessor(df):
    # drop
    df.drop(columns="Unnamed: 7", inplace=True)
    df.drop_duplicates(inplace=True)

    # format
    df.columns = df.columns.str.lower()
    df.columns = df.columns.str.replace("/", "_")
```


Question : 
Answer   : 

#### Code snippet 

```python
# TODO : add sample code
```

-->



<!-- 
############################################################
## Questions issues des quizz
############################################################ 
-->

Question : PYTHON - Data types - Give 2 examples of mutable data collection?
Answer  : 

* List
* Dictionary



Question : PYTHON - Data types - How do you add a single element to the end of a list? How can you remove an item from a list using its value?
Answer  : 

#### Code snippet 

```python
append()
delete()

```

#### Code snippet 

```python
list_villes=["Aix en Provence", "Paris", "Ghisonaccia"]
urls=[]
for ville in list_villes:
    urls.append(
        f"https://www.booking.com/searchresults.fr.html?ss={ville}&checkin=2024-05-02&checkout=2024-05-05&group_adults=2&no_rooms=1&group_children=0",
    )

```



Question : PYTHON - Data types - What is the syntax for creating a slice of a list that includes elements from index 2 to index 5 (excluding index 5)?
Answer  : 

#### Code snippet 

```python
list[2:5]
```





Question : PYTHON - Data types - How can you access the value associated with a specific key in a dictionary?
Answer  : 

Use brackets or .get(). Pay attention when the key does'nt exist yet.

#### Code snippet 

```python
you = {'name': 'Zoubida', 'age': 42}

print(f'First name : {you.get('name')}')
print(f"Age        : {you['age']}")
```




Question : PYTHON - Data types - How do you add a new key-value pair to a dictionary?
Answer  : 

#### Code snippet 

```python
dico["bob"] = 42
```




Question : PYTHON - Data types - How can you iterate over both keys and values in a dictionary?
Answer  : 

#### Code snippet 

```python
person = {
  "nom" : "ROBERT",
  "prenom" : "Zoubida"
}
print(person["nom"])
person.items()
```




Question : PYTHON - Functions - How can you add a default argument to a function?
Answer  : 

By assigning a value to the parameter (param="bob") in the function declaration.

#### Code snippet 

```python
def volume(length=1, width=1, depth=1):
  print(f"Length = {length}")
  return length * width * depth;

volume(42, 2, 3)
volume()
volume(width=4)
```




Question : PYTHON - Functions - What does the acronym DRY stand for in programming?
Answer   : 

Don't repeat yourself



Question : PYTHON - Functions - What is the purpose of giving an alias to exceptions?
Answer  : 

To customize the error message displayed to the user.



Question : PYTHON - Functions - How can you create your own exception in Python?
Answer  : 

By using the ``raise`` statement with a specific error message.

#### Code snippet 

```python
def find_seat(self, n):
    if (not isinstance(n, int) or n < 0):
        raise Exception("n should be a positive integer")
```




Question : PYTHON - Classes - Which method is used to initialize the attributes of a class in Python?
Answer  : 

#### Code snippet 

```python
class Employee():
 
  # Initializing
  def __init__(self, a_name):
    print('Employee created.')
    self.name = a_name

  # Deleting (Calling destructor)
  def __del__(self):
    print('Destructor called, Employee deleted.')
    self.name=""
```




Question : PYTHON - Classes - What does the ``self`` keyword represent in Python classes?
Answer  : 

It refers to the instance of the class.

#### Code snippet 

```python
class MyImputer():
  
  def __init__(self, mylist:list[int]):
    tmp_list = []
    for i in range(len(mylist)):
      if (mylist[i] != "None"):
        tmp_list.append(mylist[i])
    
    avg = sum(tmp_list)/len(tmp_list)
    
    self.list = mylist.copy()
    for i in range(len(self.list)):
      if (self.list[i] == "None"):
        self.list[i] = avg

  def display(self):
    print(self.list)  
```


Question : PYTHON - Classes - What does the ``ValueError`` exception indicate?
Answer  : 

It is raised when a method is called with incorrect arguments. 

#### Code snippet 

```python
class MyCustomImputer(BaseEstimator, TransformerMixin):

    def __init__(self, strategy='mean'):
        self.strategy = strategy

    def fit(self, X, y=None):
        if self.strategy == 'mean':
            self.fill_value = np.nanmean(X, axis=0)
        elif self.strategy == 'median':
            self.fill_value = np.nanmedian(X, axis=0)
        elif self.strategy == 'most_frequent':
            self.fill_value = np.nanmax(X, axis=0)
        else:
            raise ValueError("Invalid strategy. Please choose 'mean', 'median', or 'most_frequent'.")
        return self

    def transform(self, X):
        return np.where(np.isnan(X), self.fill_value, X)
```        


<!-- 
############################################################
## 
############################################################ 
-->

Question : PYTHON - Différence entre **arguments** et **paramètres**
Answer  :

* Les **paramètres** d'une fonction sont les noms listés dans la définition de la fonction. 
* Les **arguments** d'une fonction sont les valeurs passées à la fonction.




<!-- 
############################################################
## 
############################################################ 
-->

Question : PYTHON - Pourquoi voudriez-vous implémenter la méthode ``__call__()`` dans la classe d'un de vos modèles?
Answer  : 

If the model class has a ``__call__()`` method then we can call it as a function. 
 
#### Code snippet 

```python
print("model output:", my_model(data))
```











<!-- 
############################################################
## 
############################################################ 
-->
Question : PYTHON - Pouvez-vous nommer certaines des exceptions les plus courantes ?
Answer  : 


| **Exception**              | **Description**                                                                            |
|----------------------------|--------------------------------------------------------------------------------------------|
| `ArithmeticError`          | Classe de base pour les erreurs liées aux calculs arithmétiques.                            |
| `AssertionError`           | Levée lorsqu'une instruction `assert` échoue.                                               |
| `AttributeError`           | Levée lorsqu'un attribut de classe ou d'objet est inaccessible.                             |
| `EOFError`                 | Levée lorsqu'une fonction de lecture rencontre la fin du fichier sans avoir lu de données.  |
| `Exception`                | Classe de base pour la plupart des exceptions définies par l'utilisateur et intégrées.      |
| `FileExistsError`          | Levée lorsqu'une opération tente de créer un fichier ou un répertoire qui existe déjà.      |
| `FileNotFoundError`        | Levée lorsqu'un fichier ou répertoire spécifié est introuvable.                             |
| `FloatingPointError`       | Levée lorsqu'une erreur de calcul en virgule flottante se produit.                          |
| `IndexError`               | Levée lorsqu'un indice de séquence est hors des limites.                                    |
| `KeyboardInterrupt`        | Levée lorsqu'une interruption est demandée (par exemple en appuyant sur Ctrl+C).            |
| `KeyError`                 | Levée lorsqu'une clé est introuvable dans un dictionnaire.                                  |
| `LookupError`              | Classe de base pour les erreurs d'accès à une séquence ou un dictionnaire.                  |
| `NameError`                | Levée lorsqu'un nom local ou global n'est pas trouvé.                                       |
| `NotImplementedError`      | Levée pour indiquer qu'une fonction ou méthode n'est pas implémentée.                       |
| `OSError`                  | Classe de base pour les erreurs du système d'exploitation.                                  |
| `OverflowError`            | Levée lorsqu'une opération arithmétique dépasse les limites de représentation.              |
| `PermissionError`          | Levée lorsqu'une opération échoue en raison de permissions inadéquates.                     |
| `RuntimeError`             | Levée lorsqu'une erreur générique d'exécution se produit.                                   |
| `StopIteration`            | Levée pour indiquer la fin d'une itération.                                                 |
| `SyntaxError`              | Levée lorsqu'une erreur de syntaxe est détectée.                                            |
| `SystemError`              | Levée lorsqu'une erreur interne de Python se produit.                                       |
| `SystemExit`               | Levée par `sys.exit()` pour demander l'arrêt du programme.                                  |
| `TypeError`                | Levée lorsqu'une opération ou fonction est appliquée à un objet de type inapproprié.        |
| `UnboundLocalError`        | Sous-classe de `NameError`, levée lorsqu'une variable locale est utilisée sans être liée.   |
| `ValueError`               | Levée lorsqu'une opération reçoit un argument de type correct mais de valeur inappropriée.  |
| `ZeroDivisionError`        | Levée lorsqu'une division ou un modulo est effectué avec un diviseur égal à zéro.           |

<!-- 
| `IndentationError`         | Sous-classe de `SyntaxError`, levée lorsqu'un problème d'indentation est rencontré.         |
| `ImportError`              | Levée lorsqu'une tentative d'importation échoue.                                            |
| `BaseException`            | Classe de base pour toutes les exceptions en Python.                                        |
| `GeneratorExit`            | Levée pour terminer un générateur.                                                         |
| `StopAsyncIteration`       | Levée pour indiquer la fin d'une itération asynchrone.                                      |
| `BufferError`              | Levée lorsqu'une opération sur un tampon ne peut pas être effectuée.                        |
| `ModuleNotFoundError`      | Sous-classe de `ImportError` levée lorsque le module spécifié est introuvable.              |
| `MemoryError`              | Levée lorsque Python manque de mémoire.                                                     |
| `BlockingIOError`          | Levée lorsqu'une opération d'entrée/sortie est bloquée.                                     |
| `ChildProcessError`        | Levée lorsqu'une opération relative à un processus enfant échoue.                           |
| `ConnectionError`          | Classe de base pour les erreurs liées aux connexions.                                       |
| `BrokenPipeError`          | Levée lorsqu'une connexion de pipe ou de socket est rompue.                                 |
| `ConnectionAbortedError`   | Levée lorsqu'une connexion est interrompue de manière inattendue.                           |
| `ConnectionRefusedError`   | Levée lorsqu'une connexion est refusée par l'hôte.                                          |
| `ConnectionResetError`     | Levée lorsqu'une connexion est réinitialisée par l'hôte.                                    |
| `InterruptedError`         | Levée lorsqu'une fonction système est interrompue par un signal d'interruption.             |
| `IsADirectoryError`        | Levée lorsqu'une opération sur un fichier attend un fichier mais trouve un répertoire.      |
| `NotADirectoryError`       | Levée lorsqu'une opération attend un répertoire mais trouve un fichier.                     |
| `ProcessLookupError`       | Levée lorsqu'un processus spécifié est introuvable.                                         |
| `TimeoutError`             | Levée lorsqu'une opération système dépasse le temps imparti.                                |
| `ReferenceError`           | Levée lorsqu'une référence faible à un objet inaccessible est utilisée.                     |
| `RecursionError`           | Levée lorsqu'une récursion dépasse la profondeur maximale autorisée.                        |
| `TabError`                 | Levée lorsqu'un mélange de tabulations et d'espaces est utilisé pour l'indentation.         |
| `UnicodeError`             | Classe de base pour les erreurs liées à l'encodage ou au décodage Unicode.                 |
| `UnicodeDecodeError`       | Levée lorsqu'une chaîne Unicode ne peut pas être décodée.                                   |
| `UnicodeEncodeError`       | Levée lorsqu'une chaîne Unicode ne peut pas être encodée.                                   |
| `UnicodeTranslateError`    | Levée lorsqu'une erreur de traduction Unicode se produit.                                   |
| `Warning`                  | Classe de base pour les avertissements (non fatals).                                        |
| `DeprecationWarning`       | Avertissement lorsqu'une fonctionnalité est obsolète.                                       |
| `FutureWarning`            | Avertissement pour indiquer un changement prévu dans le futur.                              |
| `UserWarning`              | Avertissement générique émis par les utilisateurs.                                          |
| `SyntaxWarning`            | Avertissement relatif à des problèmes syntaxiques mineurs.                                  |
| `RuntimeWarning`           | Avertissement concernant des événements d'exécution suspectés mais non fatals.             |
 -->



<!-- 
############################################################
## 
############################################################ 
-->
Question : PYTHON - Pouvez-vous nommer certaines des exceptions les plus courantes ?
Answer  : 

#### Code snippet 

```python
from pathlib import Path

def lire_fichier(chemin_fichier):
    fichier = Path(chemin_fichier)
    try:
        with fichier.open('r') as f:
            contenu = f.read()
    except FileNotFoundError:
        print(f"Erreur : Le fichier '{chemin_fichier}' n'existe pas.")
    except PermissionError:
        print(f"Erreur : Permission refusée pour accéder au fichier '{chemin_fichier}'.")
    else:
        print("Contenu du fichier :")
        print(contenu)
    finally:
        # Ce bloc est toujours exécuté, qu'une exception soit levée ou non
        print("Fin de l'opération de lecture.")

lire_fichier('chemin/inexistant.txt')

```


#### Code snippet 

```python
from pathlib import Path

def ecrire_dans_fichier(chemin_fichier, texte):
    fichier = Path(chemin_fichier)
    try:
        with fichier.open('w') as f:
            f.write(texte)
    except PermissionError:
        print(f"Erreur : Impossible d'écrire dans le fichier '{chemin_fichier}'.")
    else:
        print(f"Le texte a été écrit dans '{chemin_fichier}'.")
    finally:
        # Ce bloc est toujours exécuté, qu'une exception soit levée ou non
        print("Fin de l'opération d'écriture.")

ecrire_dans_fichier('fichier_output.txt', 'Zoubida for ever.')

```



<!-- 
############################################################
## 
############################################################ 
-->
Question : PYTHON - Est-ce que les paramètres sont passés par valeur ou par référence ?
Answer  : 

* Les paramètres des fonctions sont passés par **référence** pour les objets **mutables** (listes, dictionnaires, objets définis par l'utilisateur)
* Ils sont passés par **valeur** pour les objets **immuables** (comme les entiers, chaînes de caractères, tuples).

Cela signifie que si on passe un objet :

* **mutable** à une fonction (une liste, ...), et que la fonction modifie cet objet, cette modification affectera l'objet d'origine en dehors de la fonction.
* **immuable** (un entier, une chaîne de caractères, ...), une modification au sein de la fonction ne changera pas l'objet d'origine. Si il est modifié, Python créera une nouvelle instance de cet objet.

#### Code snippet 

```python
def modifier_liste(l):
    l.append(4)  # Modifie l'objet d'origine

def changer_entier(x):
    x = 10       # Crée une nouvelle instance de x

ma_liste = [1, 2, 3]
mon_entier = 5

modifier_liste(ma_liste)
changer_entier(mon_entier)

print(ma_liste)    # [1, 2, 3, 4]
print(mon_entier)  # 5
```
