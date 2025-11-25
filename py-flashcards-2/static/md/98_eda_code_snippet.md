
<!-- TODO : voir C:\Users\phili\OneDrive\Documents\Programmation\Formations_JEDHA\02_Data_Science_Fullstack_march_2024\12_assets\code-snippets2.ipynb -->

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


<!-- 
############################################################
## 
############################################################ 
-->

<!-- 
Question : EDA Code Snippet -  
Answer  : 

#### Code snippet 

```python

```
-->






<!-- 
############################################################
## 
############################################################ 
-->

Question : EDA Code Snippet - Replace missing values with 42 - fillna
Answer  : 

#### Code snippet 

```python
import pandas as pd

df = pd.DataFrame({
  'A': [1, None, 3], 
  'B': [4, 5, 6]
})
print("Table avant : \n", df.head(), "\n")
df['A'].fillna(42, inplace=True)
print("Table après : \n", df.head(), "\n")
```



<!-- 
############################################################
## 
############################################################ 
-->

Question : EDA Code Snippet - Remplacer les valeurs absentes par la médiane - fillna  
Answer  : 

#### Code snippet 

```python
import pandas as pd

df = pd.DataFrame({
  'A': [1, None, 4], 
  'B': [4, 5, 6]
})
print("Table avant : \n", df.head(), "\n")
median = df["A"].median()  
df["A"].fillna(median, inplace=True)
print("Table après : \n", df.head(), "\n")

# df['A'].fillna(df['A'].median(), inplace=True)
```





<!-- 
############################################################
## 
############################################################ 
-->

Question : EDA Code Snippet - Remplacer les valeurs absentes par 42 - SimpleImputer 
Answer  : 

#### Code snippet 

```python
import pandas as pd
from sklearn.impute import SimpleImputer

df = pd.DataFrame({
  'A': [1, None, 3], 
  'B': [4, 5, 6]
})

print("Table avant : \n", df.head(), "\n")
Zoubida = SimpleImputer(strategy="constant", fill_value=42)
Marcel_ndarray = Zoubida.fit_transform(df)
# ! fit_transform() retourne un ndarray et PAS un DataFrame
print("Table après : \n", Marcel_ndarray, "\n")

# On recréé un dataframe
df = pd.DataFrame(Marcel_ndarray, columns=df.columns, index=df.index)
print("Dataframe   : \n", df, "\n")

# on a accès à Zoubida.statistics_, Zoubida.strategy...
```




<!-- 
############################################################
## 
############################################################ 
-->

Question : EDA Code Snippet - Remplacer une valeur par une autre  
Answer  : 

#### Code snippet 

```python
import pandas as pd

df = pd.DataFrame({'A': [1, 42, 3], 'B': [4, 5, 42]})
print("Table avant : \n", df.head(), "\n")

df['A'].replace([42], '24', inplace=True)
print("Table après : \n", df.head(), "\n")
```




<!-- 
############################################################
## 
############################################################ 
-->

Question : EDA Code Snippet - Afficher les lignes dont le PIB est vide 
Answer  : 

#### Code snippet 

```python
import pandas as pd

df = pd.DataFrame({
  'Pays': ["FR", "UK", "US"], 
  'PIB': [44, None, 0]
})
print("Table : \n", df.head(), "\n")
print(df.loc[df['PIB'].isnull()])

```




<!-- 
############################################################
## 
############################################################ 
-->

Question : EDA Code Snippet - Supprimer les lignes dont le PIB est vaut 42 
Answer  : 

#### Code snippet 

```python
import pandas as pd

df = pd.DataFrame({
  'Pays': ["FR", "UK", "US", "IT", "SP", "POR"], 
  'PIB': [44, None, 42, 28, 42, 31]
})

# Affiche le DataFrame initial
print("DataFrame initial :")
print(df)

# Supprimer les lignes où le contenu de la colonne 2 est égal à 42
df = df[df['PIB'] != 42]

# Affiche le DataFrame après la suppression
print("\nDataFrame après suppression :")
print(df)
```




<!-- 
############################################################
## 
############################################################ 
-->

Question : EDA Code Snippet - Afficher les lignes d'une même catégorie  
Answer  : 

#### Code snippet 

```python
import pandas as pd
df = pd.DataFrame({
  'Pays': ["FR", "UK", "FR"], 
  'Valeurs': [44, 58, 33]
})
print(df.loc[df['Pays'] == 'FR'])
```




<!-- 
############################################################
## 
############################################################ 
-->

Question : EDA Code Snippet - Afficher les catégories où le pourcentage de valeurs manquantes est non nul  
Answer  : 

#### Code snippet 

```python
import pandas as pd
df = pd.DataFrame({
  'Pays': ["FR", "UK", "US", "FR"], 
  'Feature-1': [10, 58, None, 20],
  'Feature-2': [100, 580, 10, 2000],
  'Feature-3': [10, 58, None, None],
})

print("Table : \n", df.head(), "\n")

Bob = df.isna().sum() / len(df) * 100
print(f"Les '%' de valeurs manquantes dans les colonnes sont :\n{Bob}")

print(f"\nIl y a des valeurs manquantes dans les colonnes suivantes : \n{Bob.loc[Bob.ne(0)]}")

```




<!-- 
############################################################
## 
############################################################ 
-->

Question : EDA Code Snippet -  Créer des colonnes avec le contenu d'une colonne
Answer  : 

#### Code snippet 

```python
import pandas as pd

df = pd.DataFrame({'Colonne': ['Chaine1-Chaine2', 'A-B', 'X-Y']})
print("Table avant : \n", df.head(), "\n")

df[['Colonne1', 'Colonne2']] = df['Colonne'].str.split('-', expand=True)
print("Table après : \n", df.head(), "\n")
```




<!-- 
############################################################
## 
############################################################ 
-->

Question : EDA Code Snippet - Calculer de nouvelles colonnes 
Answer  : 

#### Code snippet 

```python
import pandas as pd
import numpy as np

df = pd.DataFrame({'A': [1, 2, 3], 'B': [4, 5, 6]})
print("Table avant : \n", df.head(), "\n")

df['Somme'] = df['A'] + df['B']
df['Hypothénuse'] = np.sqrt(df['A']**2 + df['B']**2)

taux = 0.5
df["Prix-US"]=df["A"]*taux

print("Table après : \n", df.head(), "\n")
```




<!-- 
############################################################
## 
############################################################ 
-->

Question : EDA Code Snippet - Créer une colonne qui contient des catégories - cut 
Answer  : 

#### Code snippet 

```python
import pandas as pd

df = pd.DataFrame({
  'Id'        : ["Riri", "Fifi", "Loulou", "Avasarala", "Holden", "Naomi", "Razorback", "Apollo", "Soyouz"],
  'Prix'      : [0, 25, 99, 80, 66, 13, 100, 56, 110] 
})
print("Table avant :\n", df.head(10), "\n")

# Les bornes des intervalles
# (0-25, 25-50, 50-75, 75-100) 
intervalles = [0, 25, 50, 75, 100]

# Les étiquettes de chaque catégorie
categories = ['A', 'B', 'C', 'D']

# Utilise cut() pour créer la colonne 'Catégorie'
# include_lowest=True => la 1ere catégorie inclura les valeurs égales à la borne inf de l'intervalle
df['Catégorie'] = pd.cut(df['Prix'], bins=intervalles, labels=categories, include_lowest=True)

print("Table après :\n", df.head(10), "\n")

```




<!-- 
############################################################
## 
############################################################ 
-->

Question : EDA Code Snippet - Difference entre apply et transform  
Answer  : 

## `apply()`
* `apply()` est plus générique que `transform()`
* Utilisée pour appliquer une fonction sur des DataFrames entiers, des colonnes ou des lignes spécifiques
* La fonction passée à `apply()` peut retourner un scalaire, une Serie ou un DataFrame

#### Code snippet 

```python
import pandas as pd

df = pd.DataFrame({
  'A': [1, 2, 3], 
  'B': [4, 5, 6]
})

def moyenne(row):
  return (row['A']+ row['B'])/2

print("Table avant : \n", df.head(), "\n")

df["sqrt(A)"] = df['A'].apply(lambda x: x **.5)     # fonction lambda
df["Moyenne"] = df.apply(moyenne, axis=1)
print("Table après : \n", df.head(), "\n")

result = df['A'].apply(lambda x: x **2)                 
print("Serie A après\n", result, "\n")
```

## `transform()`
* `transform()` est spécifiquement conçue pour effectuer des opérations sur des groupes de données.
* La fonction de transformation doit renvoyer une série de la même longueur que l'entrée, et elle est appliquée à chaque groupe de données.
* Utile pour effectuer des opérations de groupe (par exemple, remplacer les valeurs manquantes par la moyenne du groupe).

#### Code snippet 

```python
import pandas as pd

df = pd.DataFrame({
  'Groupe': ['A', 'A', 'B', 'B', 'A', 'B'], 
  'Valeur': [None, 2, None, 4, 5, None]
})
print("Table avant : \n", df.head(), "\n")

# Rempli les valeurs manquantes avec la moyenne du groupe
moyennes_par_groupe = df.groupby('Groupe')['Valeur'].transform('mean')
df['Valeur'] = df['Valeur'].fillna(moyennes_par_groupe)
print("Table après : \n", df.head(), "\n")
```





<!-- 
############################################################
## 
############################################################ 
-->

Question : EDA Code Snippet - Extraire les nombres d'une chaîne et créer une colonne  
Answer  : 

#### Code snippet 

```python
import pandas as pd
import re                                    # regular expression

df = pd.DataFrame({
  'Id'       : ["Riri", "Fifi", "Loulou"],
  'Feature-0': ["Val1", "Val2", "Val3"], 
  'Feature-1': ["Marcel11", "Robert2", "Antoine3"],
  'Feature-2': ["NE-555", "20-USA-48", "Russia"],
})

def get_digit(mystr):
  chiffres = re.findall(r'\d+', str(mystr))  # Trouver tous les chiffres
  try:
    return int(''.join(chiffres))
  except:
    return 0                                 # on peut aussi retourner np.nan

print("Table avant : \n", df.head(), "\n")

df['Feature-3'] = df['Feature-2'].apply(get_digit)         # voir le apply()
df['Feature-1'] = df['Feature-1'].apply(get_digit)
# df['Feature-0'] = df['Feature-0'].apply(keep_numbers)

print("Table après : \n", df.head(), "\n")
```




<!-- 
############################################################
## 
############################################################ 
-->

Question : EDA Code Snippet - Extraire les valeurs d'une chaine, remplir une colonne avec leur moyenne 
Answer  : 

#### Code snippet 

```python
import pandas as pd

def moy(x):
  try : 
    return np.mean([float(i) for i in x.split("~")])
  except: 
    return np.nan     # on peut aussi retrouner une valeur : 42...

df = pd.DataFrame({
  'Id'       : ["Riri", "Fifi", "Loulou"],
  'Feature-0': ["11~13", "10~20~30", "50~100"], 
  'Feature-1': ["18~20", "10~20", "50**100"],
})
print("Avant : \n", df.head(), "\n")

df['Feature-0'] = df['Feature-0'].apply(moy)          # voir le apply()
df['Moy Feature-1'] = df['Feature-1'].apply(moy)

print("Après : \n", df.head())

```




<!-- 
############################################################
## 
############################################################ 
-->

Question : EDA Code Snippet - Transform
Answer  : 

* S'applique à un groupe  

#### Code snippet 

```python
import pandas as pd

data = {'Groupe': ['A', 'A', 'B', 'B', 'A', 'B'],
        'Valeur': [None, 15, None, 25, 30, None]}
df = pd.DataFrame(data)
print("Avant : \n", df.head(), "\n")

# Calculer la moyenne par groupe
moyennes_par_groupe = df.groupby('Groupe')['Valeur'].transform('mean')         # voir le transform sur le groupe
print("Moyennes :\n", moyennes_par_groupe, "\n")

# Remplacer les valeurs manquantes par la moyenne correspondante
df['Valeur'] = df['Valeur'].fillna(moyennes_par_groupe)

# Afficher le DataFrame après transformation
print("Après : \n", df.head(), "\n")

```




<!-- 
############################################################
## 
############################################################ 
-->

Question : EDA Code Snippet - groupby, remplacer valeurs manquantes par moyenne de la catégorie V1  
Answer  : 

* Marche pas dans tous les cas
* Il faut que chaque catégorie ait une moyenne à calculer
* Si une catégorie n'a aucune moyenne à calculer rien n'est fait

#### Code snippet 

```python
data = {
  'Pays': ['A', 'A', 'B', 'B', 'A', 'B'],
  'PIB': [None, None, 10, 25, None, 45]
}
df = pd.DataFrame(data)

print("Avant : \n", df.head(), "\n")
moyennes_par_pays = df.groupby('Pays')['PIB'].mean()
df['PIB'] = df.apply(lambda row: moyennes_par_pays[row['Pays']] if pd.isna(row['PIB']) else row['PIB'], axis=1)
print("Après : \n", df.head(), "\n")

```







<!-- 
############################################################
## 
############################################################ 
-->

Question : EDA Code Snippet - groupby, remplacer valeurs manquantes par moyenne de la catégorie V2  
Answer  : 

* Met 0 si la moyenne n'a pas pu être calculée

#### Code snippet 

```python
data = {
  'Pays': ['A', 'A', 'B', 'B', 'A', 'B'],
  'PIB': [None, None, 10, 25, None, 45]
}
df = pd.DataFrame(data)

print("Avant : \n", df.head(), "\n")
moyennes_par_pays = df.groupby('Pays')['PIB'].mean()
moyennes_par_pays.fillna(0.0, inplace=True)
df['PIB'] = df.apply(lambda row: moyennes_par_pays[row['Pays']] if pd.isna(row['PIB']) else row['PIB'], axis=1)
print("Après : \n", df.head(), "\n")


```




<!-- 
############################################################
## 
############################################################ 
-->

Question : EDA Code Snippet - Supprimer une colonne  
Answer  : 

#### Code snippet 

```python

import pandas as pd
df = pd.DataFrame({
  'Id'      : ["Riri", "Fifi", "Loulou"],
  'Feature0': [1, 2, 3], 
  'Feature1': [10, 20, 30], 
  'Feature2': [100, 200, 300], 
})
print("Table avant : \n", df.head(), "\n")
df.drop("Feature1", inplace=True, axis=1)
print("Table après : \n", df.head(), "\n")


```




<!-- 
############################################################
## 
############################################################ 
-->

Question : EDA Code Snippet - Supprimer des colonnes en fonction d'un motif dans le titre 
Answer  : 

#### Code snippet 

```python
import pandas as pd
df = pd.DataFrame({
  'Id'      : ["Riri", "Fifi", "Loulou"],
  'Feature0': [1, 2, 3], 
  'Feature1': [4, 5, 6],
  'Feature2': [7, 8, 9],
  'Feature3': [1, 2, 3],
  'Feature4': [4, 5, 6],
  'Feature5': [7, 8, 9],
  'Feature6': [1, 2, 3]
})
print("Table avant : \n", df.head(1), "\n")
columns_to_drop=[f'Feature{i}' for i in range (2,5)]
df.drop(columns=columns_to_drop, inplace=True)
print("Table après : \n", df.head(1), "\n")

```





<!-- 
############################################################
## 
############################################################ 
-->

Question : EDA Code Snippet - Merger 2 fichiers csv ou dataframe  
Answer  : 

#### Code snippet 

```python

import pandas as pd

# Charger les deux fichiers CSV
# df1 = pd.read_csv('fichier1.csv', delimiter=";")
# df2 = pd.read_csv('fichier2.csv', delimiter=";")
df1 = pd.DataFrame({
  'Id' : [101, 1001, 200, 2000, 300],
  'Valeur1' : ['a', 'b', 'c', 'd', 'e'],
  'Valeur2' : ['aa', 'bb', 'cc', 'dd', 'ee']
})

df2 = pd.DataFrame({
  'Id' : [101, 102, 200, 201, 300, 500],
  'Valeur1' : [1,2,3,4,5,10],
  'Valeur2' : [11,12,13,14,15,100]
})

# Effectue le merge sur la colonne 'Id'
resultat_merge = pd.merge(df1, df2, on='Id')

# Afficher le résultat
print(resultat_merge)

```




<!-- 
############################################################
## 
############################################################ 
-->

Question : EDA Code Snippet - Sauver un Dataframe  
Answer  : 

#### Code snippet 

```python
df.to_csv("mon-fichier.csv", index=False)
```




<!-- 
############################################################
## 
############################################################ 
-->

Question : EDA Code Snippet - Merge avancé de 2 Dataframes 
Answer  : 

* La seconde table contient des Types dont on veut calculer la moyennes des features
* Voir le contenu de la Table 3
* Une des features de la Table 1 est le type
* On veut étendre la Table 1, avec pour chaque ligne, en fonction du type, les moyennes calculées avec la Table 2 

#### Code snippet 

```python

df2 = pd.DataFrame({
  'Id'        : ["Riri", "Fifi", "Loulou", "Avasarala", "Holden", "Naomi", "Razorback", "Apollo", "Soyouz"],
  'Type'      : ["Type1", "Type2", "Type3", "Type1", "Type2", "Type3", "Type1", "Type2", "Type3"], 
  'Feature-1' : [0, 1, 2, 3, 4, 5, 6, 7, 8],
  'Value'     : [10, 20, 30, 11, 21, 31, 12, 22, 32],
})
print("Table 2 :\n", df2.head(10), "\n")

# df3 = df2.groupby('Type')["Value"].transform("mean")
# df3 = df2.groupby('Type')["Value"].mean()
# print("Table 3 : Depuis Table 2, les moyennes de Value par Type\n", df3, "\n")

df3 = df2.groupby('Type')[["Feature-1", "Value"]].mean()
print("Table 3 : Depuis Table 2, les moyennes de Value et Feature-1 par Type\n", df3, "\n")

df1 = pd.DataFrame({
  'Id'        : ["Riri", "Avasarala", "Apollo", "Soyouz"],
  'Colonne-00': ["BB", "SF", "Space", "Space"], 
  'Colonne-01': [3.14, 42, 2.718, 1.618],
  'Colonne-02': ["Type1", "Type2", "Type2", "Type1"],
})
print("Table 1 :\n", df1.head(), "\n")

# Voir que dans le join les colonnes n'ont pas le même nom
df4 = pd.merge(df1, df3, left_on='Colonne-02', right_on='Type', how='inner' )
# df4 = pd.merge(df1, df2.groupby('Type')[["Feature-1","Value"]].mean(),left_on='Colonne-02', right_on='Type', how='inner' )
print("Merge de Table 1 et Table 3 sur le Type :\n", df4.head(10), "\n")


```




<!-- 
############################################################
## 
############################################################ 
-->

Question : EDA Code Snippet - Classe pour valider les saisies 
Answer  : 

#### Code snippet 

```python
class UserInput:
  """ A simple class to validate user input  """
  def __init__(self, datatype, prompt=""):
    while True:
      try:
        if(prompt!=""):
          self.value = datatype(input(prompt + " :"))
        else:
          self.value = datatype(input())
        break
      except ValueError:
        print("Le montant devrait être un nombre")
        # self.value = 0

# Testing
deposit = UserInput(float, "Montant initial").value
print (deposit)

deposit = UserInput(float).value
print (deposit)

deposit = UserInput(int, "Saisir un entier").value
print (deposit)

```




<!-- 
############################################################
## 
############################################################ 
-->

Question : EDA Code Snippet - Ajouter une ligne à un DataFrame - concat 
Answer  : 

#### Code snippet 

```python

import pandas as pd

# Add new row at the end of the DataFrame
df = pd.DataFrame({"first_name":["Lucien", "Jocelyne", "Brigitte"], "age":[29, 43, 32]})
new_row = pd.DataFrame ({
  'first_name': ['Joséphine'],
  'age': [43],
})

df = pd.concat([df, new_row], ignore_index=True)
display(df)


```







