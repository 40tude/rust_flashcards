<!-- Entretien

	• https://sites.google.com/view/datascience-cheat-sheets
	
	
	
Relire toutes les question et ajouter "<je sais pas pquoi encore>" à la fin si elle est importante
	• Se limiter OBLIGATOIREMENT à 20% des questions
	
Faire le quiz 
	• LIN-REG Quiz: Maximum likelihood estimation 
	• LIN-REG Quiz: How to compute metrics 
	• Time Series : test your knowledge!
	• asynchronous programming

############################################################
## Questions à traiter, ranger plus tard
############################################################


EDA	                    : The recipe
Features Engineering	: The secret sauce
Baseline model	          : The first taste
Metrics Analysis	     : The critics' score
API & App	               : Sharing with friends
Deployment Monitoring	: Serve the dish, maintain quality




Transformer, any comment ?
The attention mechanism that learns contextual relationships between words in a text 


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
Differences entre docker-compose et docker compose ?
Differences entre docker compose up et docker compose run ...

-->



<!-- 
############################################################
## 
############################################################ 
-->
Question : No category yet - Dans un contexte MLflow Tracking Server est ce que les notions de experiments, models, runs et version de modèle sont claires pour vous? 
Answer  : 


Sur le site voilà comment MLflow organise les éléments :

<p align="center">
<img src="../static/md/assets/models_runs_mlflow.png" alt="mlflow" width="577"/>
</p>


1. **Experiment** : 
   - Un *Experiment* est un regroupement de plusieurs *Runs* qui partagent un même objectif de suivi (par exemple, tous les essais d’entraînement pour un certain modèle ou un projet).
   - Un *Experiment* a un nom unique et un ID.

2. **Run** :
   - Un *Run* est une exécution spécifique de l’entraînement d'un modèle ou d'une étape du pipeline. 
   - Chaque *Run* est associé à un *Experiment* et a un ID unique (`run_id`).
   - Les *Runs* contiennent des informations comme les métriques, les paramètres, les artefacts, et les tags enregistrés lors de l'entraînement.
   - Lorsque qu'on sauvegarde un modèle avec MLflow, il est associé à un *Run* dans un *Experiment*.

3. **Model Registry (ou Registered Model)** :
   - Le *Model Registry* permet d'enregistrer un modèle sous un **nom** (le *model_name*) qui le rend accessible pour le chargement, la mise à jour, et la gestion des versions.
   - Un *model_name* dans le registre de modèles peut avoir plusieurs versions (chaque version correspond à un *Run* spécifique associé).

4. **Model Version** :
   - Chaque modèle dans le *Model Registry* a une ou plusieurs **versions** (le *model_version*). Chaque version est liée à un *Run* spécifique d'un *Experiment*.
   - Par exemple, si on entraîne plusieurs fois un modèle et qu'on souhaite sauvegarder une version stable, on enregistres le *Run* correspondant en tant que version dans le *Model Registry*.

### Bref :
- **model_name** correspond au nom d'un modèle dans le *Model Registry*, et il permet de retrouver facilement un modèle sans connaître l'ID d'un *Run*.
- **model_version** fait référence à une version spécifique d'un modèle dans le *Model Registry*. Chaque version correspond à un *Run* précis dans un *Experiment*.

### Code Python :
```python
client = MlflowClient()
model_name = "random_forest"  
model_version = "3"  
model_version_info = client.get_model_version(name=model_name, version=model_version)
print("Version       :", model_version_info.version)  
print("Source Run ID :", model_version_info.run_id)  
```






<!-- 
############################################################
## 
############################################################ 
-->
Question : No category yet - Pouvez-vous me faire un point sur les détails de la création d'une image et d'un container Docker ?
Answer  : 

* Pour lancer un conteneur, il faut une image. Cette image est créée en suivant la "recette" fournie dans un Dockerfile. Lors du processus de création de l'image, Docker empile des **couches** (layers) qui représentent chaque étape définie dans le Dockerfile. Ces couches sont immuables et incluent des instructions comme `RUN`, `COPY`, et `WORKDIR`. Les couches permettent de construire une image efficacement, car Docker peut réutiliser des couches déjà construites si elles n’ont pas changé.

* Les instructions `RUN`, `COPY`, etc., s'exécutent dans un contexte défini par l'instruction `WORKDIR` du Dockerfile. Par exemple, si `WORKDIR` est défini à `/home/app`, l'instruction `COPY bob.txt .` copie le fichier `bob.txt` dans ce répertoire (`/home/app`) au sein de l'image en cours de création.

* Concernant la **source** des fichiers à copier, l'instruction `COPY` utilise un chemin relatif par rapport au **contexte de construction** (build context) défini par `context` dans un fichier `docker-compose.yml`. Ce contexte est le répertoire de l'hôte à partir duquel les fichiers seront copiés vers l'image. Quand on exécute la commande `docker build`, le contexte de construction c'est le répertoire qu'on précises en argument :

```bash
docker build -t mon_image .
docker build -t mon_image /chemin/vers/contexte
```
* Dans le cas des instructions exécutées par le shell (comme `RUN cp docker/requirements_4tests.txt .`), elles s'exécutent dans le contexte de l'image en cours de construction, pas du répertoire de l’hôte. Par conséquent, tous les fichiers et répertoires auxquels ces instructions accèdent doivent avoir été copiés ou créés dans une couche précédente de l'image. Autrement dit, pour que ces fichiers ou répertoires soient accessibles, ils doivent être déjà présents dans l'image.

* Une fois l'image construite, elle peut être instanciée autant de fois que nécessaire. Chaque instance est un **conteneur**. Avec `docker-compose`, grâce à l'instruction `build`, on peut spécifier comment créer une image si elle n'existe pas encore. Si l'image existe déjà, `docker-compose` instancie directement un nouveau conteneur à partir de cette image, sans la reconstruire.



<!-- 
############################################################
## 
############################################################ 
-->
Question : No category yet - Pouvez-vous me faire un point sur les couches dans un contexte Docker ?
Answer  : 

Les **couches** dans Docker sont des niveaux de fichiers qui s'empilent pour constituer une image complète. Chaque couche est un **système de fichiers immutable** qui représente une étape dans la construction de l’image. Elles fonctionnent comme suit :

1. **Chaque instruction dans le Dockerfile génère une couche** : Chaque fois que Docker exécute une instruction comme `RUN`, `COPY`, ou `ADD`, il crée une nouvelle couche. Cette couche contient uniquement les changements effectués par l'instruction. Par exemple, si on a `RUN apt-get update`, cette instruction va générer une couche contenant les modifications du système de fichiers après avoir mis à jour les packages.

2. **Couches de système de fichiers** : Une couche peut inclure de nouveaux fichiers ou dossiers, des modifications apportées aux fichiers existants, ou des suppressions de fichiers. Cependant, comme les couches sont immuables, une "suppression" de fichier est en réalité une opération marquée pour être masquée dans les couches ultérieures.

3. **Empilement en lecture seule** : Lorsqu’une image est terminée, elle est constituée de plusieurs couches en lecture seule, empilées les unes sur les autres. Lorsque vous lancez un conteneur, Docker ajoute une couche finale, qui est en lecture-écriture, au sommet de cette pile. Cela permet aux conteneurs de faire des changements temporaires pendant leur exécution, sans modifier l'image sous-jacente.

4. **Optimisation et réutilisation des couches** : Docker utilise un mécanisme de mise en cache qui permet de réutiliser les couches non modifiées dans les builds successifs. Si une couche existe déjà et n'a pas changé, Docker peut la réutiliser sans la recréer, ce qui accélère le processus de création d’image et optimise le stockage.

### Exemple concret de couches dans une image
Supposons un Dockerfile avec les instructions suivantes :

```Dockerfile
FROM python:3.12         # 1ère couche : Image Python de base (grande couche préexistante)
WORKDIR /app             # 2ème couche : Définit le répertoire de travail
COPY requirements.txt .  # 3ème couche : Copie requirements.txt
RUN pip install -r requirements.txt  # 4ème couche : Installe les packages Python
COPY . .                 # 5ème couche : Copie tous les fichiers de l'hôte
CMD ["python", "app.py"] # Instruction finale : commande pour démarrer l'app
```

Dans cet exemple :
- La première couche (l'image de base Python) est souvent téléchargée et réutilisée. 
- Ensuite, chaque instruction ajoute une couche avec uniquement les changements associés. 
- Si `requirements.txt` ne change pas, la couche de l'instruction `RUN pip install -r requirements.txt` sera mise en cache et réutilisée. 

### Avantages des couches
- **Efficacité de stockage** : En empilant uniquement les changements, Docker optimise le stockage et évite les duplications inutiles.
- **Rapidité des builds** : En réutilisant les couches mises en cache, Docker accélère le processus de build, car il ne recrée pas les couches inchangées.


<!-- 
############################################################
## 
############################################################ 
-->
Question : No category yet - What is the purpose of oov_token in ``tokenizer = tf.keras.preprocessing.text.Tokenizer(num_words=k_num_words, oov_token="_UNKNOWN_")``
Answer  : 

* ``oov_token`` = out of vocabulary token
* When the ``oov_token`` is specified in the tokenizer, words which are **NOT** present in the learned vocabulary will be replaced by this token. 
    * This enables the model to handle new words that appear in test or inference data, while reducing the risk of errors or inaccuracies.
    * In a new sentence, 2 unknown words will be represented by the OOV token, preserving information even if the model hasn't seen these words before
* If ``oov_token`` not specified any word not in the vocabulary will be ignored and not tokenized
    * This may result in the loss of important information during inference or testing.




<!-- 
############################################################
## 
############################################################ 
-->
Question : No category yet - Classes déséquilibrées, spam par exemple. Vous faites quoi ?
Answer  : 

1. ``train_test_split()`` avec stratify
1. On touche plus au jeu de test
1. Equilibrer les classes du train set (50/50)
1. Entrainer le modèle avec le train set équilibré
1. Validation/métriques avec le jeu de test déséquilibré 

L'équilibrage des classes se fait par sous ou sur-échantillonnage

* Sur échantillonnage
    * RandomOverSampler from ``imblearn.over_sampling``  
    * SMOTE (Synthetic Minority Oversampling Technique, synthèse de points)

<p align="center">
<img src="../static/md/assets/smote.png" alt="smote" width="577"/>
</p>

* Sous échantillonnage 
    * Tomek Links 
    * NearMiss
* Si on veut garder des classes déséquilibrées lors du training on peut faire de la pondération de classe
    * C'est l'inverse de la freq des classes
    * Voir ``class_weight`` de sklearn qui retourne un dictionnaire qu'on passe ensuite à ``model.fit()`` de tensorflow (param class_weight)
* On peut aussi faire du ``sample_weight`` 
    * Chaque échantillon de ``y_train`` recoit une pondération spécifique à sa classe 
    * Voir param ``sample_weight`` de ``model.fit()`` de tensorflow
* Quand les classes sont déséquilibrées, faire attention aux metrics 
    * conf matrix, precision, recall,F1 score, area under ROC curve
* Lire cet [article](https://www.analyticsvidhya.com/blog/2020/07/10-techniques-to-deal-with-class-imbalance-in-machine-learning/)







<!-- 
############################################################
## 
############################################################ 
-->
Question : No category yet - MSE, RMSE, MAE, R², MAPE ?
Answer  :

**RMSE :** 

* Généralement préférée dans les pb de régression
* Si outliers, faut peut-être prendre en compte la MAE
* RMSE (L2 based) est plus sensible aux outliers que MAE (L1 based)
* MAE et RMSE sont 2 mesures de distance entre vecteurs (prédictions et target)
* Différentes mesures de distances sont possibles:
    * RMSE : norme euclidienne, L2
    * MAE : norme de Manhattan, L1
    * Plus le n de Ln augmente et plus la norme focalise sur les grandes valeurs en négligeant les petites
    * Quand les outliers sont exponentiellement rares (bell curve) RMSE est plus efficace

**MAPE :** 

* Erreur Absolue Moyenne en % de la vraie valeur
* Exprimée en %, c'est une mesure simple et intuitive de l'accuracy d'un modèle
* Important de l'utiliser avec prudence quand les valeurs vraies sont proches de zéro.
* MAPE=0% : modèle parfait. 
* MAPE élevé : erreurs importantes dans les prédictions


<!-- 
############################################################
## 
############################################################ 
-->
Question : No category yet - ANOVA... Ca te dis quoi?
Answer  : 

Analyse de la variance. La variation c'est l'information. ANOVA c'est analyser la quantité d'information captée par le modèle. Variance=écart à la moyenne.


<!-- 
############################################################
## 
############################################################ 
-->
Question : No category yet - Silhouette, un commentaire? 
Answer  : 

* Le coefficient de silhouette évalue la cohésion et la séparation des clusters. 
* Paramètre global du clustering. 
* On veut des clusters bien regroupés autour de leur centroïd et bien séparés entre eux. 
* Coef sans unité. 
* Entre -1 et 1. 
* On veut 1 mais 0.5 c'est OK. 
* On choisit k tel que s soit maximal (voir aussi analyse courbe WCSS, Elbow). 
* Le score de silhouette est calculé pour chaque point de données en mesurant la similarité avec son propre cluster par rapport aux clusters voisins les plus proches.

<!-- 
############################################################
## 
############################################################ 
-->
Question : No category yet - Data Warehouse vs Databases ?
Answer  : 

* **Data warehouses** : optimized to have a performance boost on columns (features)
* **Databases**       : optimized for extracting rows (observations)


<!-- 
############################################################
## 
############################################################ 
-->
Question : No category yet - What is data tidying?
Answer  : 

This is the process of transforming raw, messy data into a clean and organized format that is easier to analyze and interpret. This process involves structuring the data in a way that :

* Each variable forms a column
* Each observation forms a row
* Each type of observational unit forms a table



<!-- 
############################################################
## 
############################################################ 
-->
Question : No category yet - What are the types of categorical data ?
Answer  : 

**Ordinal Data** & **Nominal Data**

* Ordinal data is a ranking list. It’s ordered, **BUT** the intervals between the ranks aren’t necessarily equal. 
* Nominal data is like choosing your favorite ice cream flavor. There’s no logical order to the choices. Whether it’s “Vanilla,” “Chocolate,” or “Strawberry,” one isn’t inherently better or worse than the others. 


<!-- 
############################################################
## 
############################################################ 
-->
Question : No category yet - D'où provient le terme recall ?
Answer  : 

* Le terme "recall" en machine learning vient du domaine de la récupération d'information (Information Retrieval). 
* Dans ce contexte, "recall" se réfère à la capacité d'un système à retrouver toutes les occurrences pertinentes dans un ensemble de données. 
* En d'autres termes, le "recall" mesure le pourcentage de vrais positifs parmi tous les éléments pertinents. Il est utilisé pour évaluer la performance des modèles de classification, en particulier dans les situations où il est crucial de ne pas manquer des éléments pertinents (par exemple, dans la détection de maladies, où il est important de ne pas manquer de cas positifs). 


<!-- 
############################################################
## 
############################################################ 
-->
Question : No category yet - Boostrapping ?  
Answer  : 

* On crée plusieurs sous-ensembles de données en échantillonnant de manière aléatoire avec remplacement à partir de l'ensemble de données d'origine. 
* Chaque sous-ensemble peut donc contenir des exemples répétés et ne pas inclure certains exemples de l'ensemble d'origine.


<!-- 
############################################################
## 
############################################################ 
-->
Question : No category yet - Bagging ?
Answer  : 

* Parallèle (indépendance)
* Le bagging consiste à entraîner plusieurs modèles indépendamment les uns des autres sur différentes versions d'un même ensemble de données, puis à combiner leurs prédictions pour obtenir un modèle final plus robuste. 
* Objectif : Réduction de la variance. Exemple : Random Forest
* 3 phases : 
    1. Bootstrap Sampling
    1. Entraînement des modèles
    1. Agrégation des résultats (Classification => agrégation par vote majoritaire. Régression => agrégation par la moyenne des prédictions)



<!-- 
############################################################
## 
############################################################ 
-->
Question : No category yet - Boosting ?
Answer  : 

* Séquentiel
* Le boosting consiste à entraîner plusieurs modèles de manière séquentielle, chaque modèle cherchant à corriger les erreurs des modèles précédents. 
* Les modèles sont construits de manière dépendante et on pondère les exemples d'entraînement en fonction des erreurs des modèles précédents. 
* Objectif : Réduction du biais et de la variance (correction des erreurs progressives). Exemple : XGBoost, AdaBoost
* 3 phases : 
    1. Initialisation          : Un premier modèle est entraîné sur l'ensemble de données d'origine.
    1. Ajustement              : Les exemples mal classés ou mal prédits par le modèle précédent sont pondérés davantage, de sorte que le modèle suivant se concentre sur ces erreurs.
    1. Combinaison des modèles : Les modèles sont combinés en pondérant leurs prédictions en fonction de leur performance. En général, les modèles performants reçoivent un poids plus important.



<!-- 
############################################################
## 
############################################################ 
-->
Question : No category yet - XGBoost ?
Answer  :

* Extreme Grandient Boosting
* Based on decision trees
* Several weak models (decision trees) are combined to form a robust model
* Model 1 predictions are compared with true values
* Each model is trained to minimize a loss function that measures the residual error
* Residuals are kept and become the target values of the next model
* The new set of weighted observations is injected into model 2
* At the end we have n models
* Submit an observation.
* Sum of the n predictions (each tree predict a residual)


<!-- 
############################################################
## 
############################################################ 
-->
Question : No category yet - XGBoost benefits drawbacks for predictions
Answer  :

#### Benefits

* **Handles complex and heterogeneous data**: XGBoost works well with diverse features (numerical, categorical) and can model complex non-linear relationships often found in price prediction tasks.
* **High performance with large datasets**: It is designed to be highly efficient, even with large volumes of data, thanks to parallelization and memory optimization.
* **Manages missing values and outliers**: XGBoost automatically handles missing data and can incorporate outliers without negatively impacting performance, which is crucial for price prediction scenarios.
* **Prevents overfitting**: With regularization techniques, depth control, and learning rate adjustments, XGBoost helps reduce overfitting, making it suitable for price models with high complexity.
* **Feature importance and interpretability**: XGBoost provides insights into feature importance, helping identify key factors that influence price predictions, which is valuable for decision-making.
* **Robust and adaptable**: It performs well across different data distributions and can handle cases where price variance is high, adapting to various relationships between features and target variables.


#### Potential Drawbacks

* **Complex hyperparameter tuning**: To achieve optimal performance, XGBoost often requires careful hyperparameter tuning, which can increase development complexity.
* **Training time**: While fast, training can be longer on very large datasets compared to simpler algorithms, though the test performance typically makes up for this. 



<!-- 
############################################################
## 
############################################################ 
-->
Question : No category yet - Pouvez-vous expliquer les termes "précision" et "accuracy"
Answer  : 

Métriques utilisées pour évaluer la performance des modèles de classification.

* L'accuracy (exactitude) est une mesure globale de la performance d'un modèle. C'est le pourcentage de prédictions correctes sur le total des prédictions : (TP + TN)/(total de prédictions)
* Précision : une mesure la qualité des prédictions positives (précision, positive). C'est le pourcentage de prédictions positives correctes par rapport au nombre total de prédictions positives. TP/(TP+FP). La précision est cruciale lorsque les faux positifs ont un coût élevé, comme dans le dépistage des maladies, où un faux positif pourrait entraîner des tests supplémentaires inutiles.
* L'accuracy donne une idée globale de la performance du modèle, mais peut être trompeuse si les classes sont déséquilibrées. 


<!-- 
############################################################
## 
############################################################ 
-->
Question : No category yet - C'est quoi le F1 score, que signifie-t-il, quand l'utiliser?
Answer  : 

* Moyenne harmonique du recall et de la précision. 
* Utile quand : 
    1. Les classes sont déséquilibrées 
    1. Si les faux positifs et les faux négatifs ont des coûts comparables, le F1 score fournit un bon compromis. 
* Si proche de 1 =>  le modèle a une bonne performance, équilibrant  précision et rappel. 
* Si faible => soit le modèle a une faible précision, soit un faible rappel, soit les deux. Le modèle ne performe pas bien et nécessite des ajustements.


<!-- 
############################################################
## 
############################################################ 
-->
Question : No category yet - Le F1 score est une moyenne harmonique. Pourquoi ?
Answer  : 

* Penser à la moyenne "harmonieuse"
* Elle est maximale quand les valeurs sont identiques
* F1 : on cherche le compromis Recall & Precision 

<p align="center">
<img src="../static/md/assets/harmonic.png" alt="harmonic" width="577"/>
</p>

* Ci-dessus il faut remarquer  qu'on ne dit pas que la voiture roule pendant une heure à 40 km/h puis pendant une heure à 60 km/h
    * Dans ce cas, elle aurait roulé 2H et parcouru 100 km 
    * La valeur moyenne arithmétique de la vitesse serait alors de 50 km/h
* Non, non, ici on dit qu'elle parcourt la moité de la distance à 40 km/h puis l'autre moitié à 60 km/h 
    * On se demande à quelle vitesse constante elle aurait doit rouler pour parcourir la même distance dans le même temps.

#### Raisonnement :
* La voiture parcourt 40 km en 1H
* Donc elle parcourt $\frac{D}{2}$ en $\frac{D}{2\cdot40}$ heures (c'est une règle de 3, ça reste gérable...)
* De la même façon elle parcourt $\frac{D}{2}$ en $\frac{D}{2\cdot60}$ heures
* La distance totale c'est $D$
* Le temps total du parcours c'est :
$$t = \frac{D}{2\cdot40} + \frac{D}{2\cdot60}$$
* Donc la vitesse moyenne c'est : 
$$V = \frac{d}{t} = \frac{D}{\frac{D}{2\cdot40} + \frac{D}{2\cdot60}} = \frac{2}{\frac{1}{40} + \frac{1}{60}}$$


<!-- 
############################################################
## 
############################################################ 
-->
Question : No category yet - Forward selection, Backward selection ?
Answer  : 

* Forward selection  : On ajoute les variables au modèle qui à chaque étape augmente le R². On arrête si y a plus variable ou si R² baisse.
* Backward selection : Elimination. On part avec toutes les variables. On élimine la variable qui a la plus forte probabilité de ne pas être pertinante (p-value). On arrête quand toutes les variables ont une p-value sup à 5%


<!-- 
############################################################
## 
############################################################ 
-->
Question : No category yet - What is a kernel? 
Answer  : Function that take the observations into a larger dimensional space, in which we hope that the geometric properties of the observations will be linearly separable


<!-- 
############################################################
## 
############################################################ 
-->
Question : No category yet - Objectif de K-Means ?
Answer  : 

* L'objectif du K-Means est de regrouper des données en K clusters de telle manière que les points à l'intérieur d'un cluster soient similaires entre eux et différents des points d'autres clusters
* On fait ça en minimisant la variance intra-cluster. 
* La variance intra-cluster est une mesure de la dispersion des données à l'intérieur de chaque cluster. Elle représente la somme des carrés des distances entre chaque point de données d'un cluster et le centre de ce cluster.


<!-- 
############################################################
## 
############################################################ 
-->
Question : No category yet - WCSS… Any comment ? 
Answer  : 

* Within Cluster Squared Sum. 
* Voir méthode ELBOW (Densité des clusters). 
* Pour chaque exécution de K-Means, on calcule la WCSS (within cluster squared sum, somme des distances au carré entre chaque point de données et le centroïde de son cluster correspondant). 
* C'est un paramètre global sur l'ensemble des clusters. C'est la somme des sommes des carré


<!-- 
############################################################
## 
############################################################ 
-->
Question : No category yet - Homoscédasticité. Ca te dit quoi ? 
Answer  : 

* L'**homoscédasticité** fait référence à l'hypothèse selon laquelle la variance (et donc l'écart type) des erreurs est constante à travers toutes les valeurs prédictives. 

* Dit autrement :  les erreurs (ou résidus) autour de la ligne de régression doivent avoir une dispersion similaire, quelle que soit la valeur de la variable indépendante ($x$ de $y=f(x)$).

* C'est l'une des hypothèses que l'on fait quand on fait de la régression linéaire
    1. linéarité
    1. indépendance des erreurs
    1. normalité des erreurs

* Pour vérifier l'homoscédasticité on peut :
    * Tracer les résidus en fonction des valeurs prédites ou des variables explicatives. En présence d'homoscédasticité, les résidus devraient se répartir de manière aléatoire autour de zéro avec une dispersion constante. Si la variance augmente ou diminue avec les valeurs prédictives (comme une forme d'entonnoir), cela indique de l'hétéroscédasticité.
    * Test de **Breusch-Pagan** ou test de **White** 

* En cas d'hétéroscédasticité
    * Une transformation logarithmique ou racine carrée des variables peut parfois stabiliser la variance.
    * Modèle de régression pondérée : Les observations peuvent être pondérées en fonction de la variance des erreurs.
    * Régression robuste qui ajuste les erreurs standard de manière à rendre les résultats plus fiables en présence d'hétéroscédasticité. 


<!-- 
############################################################
## 
############################################################ 
-->
Question : No category yet - Quelles sont les 3 hypothèses que l'on fait en régression linéaire
Answer : 

* Linéarité : évident
* Indépendance des erreurs : l'erreur sur une observation est indépendante de l'erreur sur une autre. Difficile à prouver à partir d'échantillons. Corrélation vs Causation
* Homoscédasticité : La distribution des erreurs est indépendante de y. Faut que la distribution des erreurs, que l'écart à la droite, soit constant qqsoit y


<!-- 
############################################################
## 
############################################################ 
-->
Question : No category yet - What is Boto3 the SDK (Software Development Kit) of AWS?
Answer  : 

It is a collection of tools and libraries to help you use AWS using code


<!-- 
############################################################
## 
############################################################ 
-->
Question : No category yet - What is a RDBMS (Relational DataBase Management System)?
Answer  : 

A piece of software that lets define, create, maintain, and control access to a database


<!-- 
############################################################
## 
############################################################ 
-->
Question : No category yet - C'est quoi le machine learning ? 
Answer  : 

* Machine Learning (ML) = sous-domaine de l'IA 
* Se concentre sur le développement de techniques permettant aux ordinateurs d'apprendre à partir de données et d'améliorer leurs performances sans être explicitement programmés pour chaque tâche. 
* Le ML permet aux systèmes informatiques :  
	1. de reconnaître des modèles dans les données 
	1. de faire des prédictions ou de prendre des décisions basées sur ces modèles
	1. sans intervention humaine directe pour spécifier explicitement les règles.


<!-- 
############################################################
## 
############################################################ 
-->
Question : No category yet - Expliquez ce qu'est la validation croisée et pourquoi c'est important dans le contexte de l'apprentissage automatique ?
Answer  : 

* Technique utilisée pour évaluer les performances d'un modèle en divisant les données en sous-ensembles d'apprentissage et de test de manière itérative. 
* Cela permet d'estimer la capacité de généralisation du modèle sur des données non vues, et d'identifier le surapprentissage. 
* Les méthodes courantes incluent la validation croisée en **k-fold** et la validation croisée **leave-one-out**.





<!-- 
############################################################
## 
############################################################ 
-->
Question : No category yet - Différence entre régression et classification en apprentissage automatique ?
Answer  : 

* La **régression** est utilisée pour prédire une **valeur continue**
* La **classification** est utilisée pour prédire une classe ou une catégorie discrète.








<!-- 
############################################################
## 
############################################################ 
-->
Question : No category yet - Pouvez-vous expliquer ce qu'est l'overfitting et comment le détecter ?
Answer  : 

* L'overfitting se produit lorsque le modèle s'adapte trop précisément aux données d'entraînement et perd sa capacité de généralisation sur de nouvelles données. 
* Il peut être détecté en observant une performance élevée sur les données d'entraînement mais une performance médiocre sur les données de test
* On peut aussi comparer les performances du modèle sur les données d'entraînement et de validation.






<!-- 
############################################################
## 
############################################################ 
-->
Question : No category yet - Quelle est la différence entre la **normalisation** et la **standardisation** des données ?
Answer  : 

* La **normalisation**   : met à l'échelle les données dans une plage spécifique, souvent entre 0 et 1. 
* La **standardisation** : transforme les données pour qu'elles aient une moyenne nulle et un écart-type de 1. Penser à la courbe de gauss.





<!-- 
############################################################
## 
############################################################ 
-->
Question : No category yet - Qu'est-ce qu'une fonction de coût (ou de perte) et comment est-elle utilisée dans l'apprentissage automatique ?
Answer  : 

* Une fonction de coût mesure l'erreur entre les prédictions d'un modèle et les valeurs réelles de l'ensemble de données. 
* Elle est utilisée dans le processus d'optimisation pour guider l'ajustement des paramètres du modèle afin de minimiser cette erreur. 
* Parler de régression => MSE Classification => LogLoss (entropy)





<!-- 
############################################################
## 
############################################################ 
-->
Question : No category yet - Pouvez-vous expliquer ce qu'est la réduction de la dimensionnalité et pourquoi est-ce important dans l'analyse de données ?
Answer  : 

* La **PCA** (princiapl component analysis, analyse en composantes principales) consiste à réduire le nombre de variables ou de caractéristiques dans un ensemble de données. 
* Cela permet de simplifier les modèles, de réduire le temps de calcul et de prévenir le surapprentissage, tout en préservant autant que possible les informations importantes.





<!-- 
############################################################
## 
############################################################ 
-->
Question : No category yet - Quelles sont les différences entre l'apprentissage supervisé et l'apprentissage non supervisé ?
Answer  : 

* L'apprentissage supervisé implique l'utilisation de données étiquetées pour entraîner un modèle à prédire une sortie
* tandis que l'apprentissage non supervisé explore les données pour découvrir des structures intrinsèques sans étiquettes
* Parler des cas d'usage du non supervisé. Pas une fin en soi









<!-- 
############################################################
## 
############################################################ 
-->
Question : No category yet - Expliquez ce qu'est la régularisation **Lasso** et en quoi elle diffère de la régularisation **Ridge** ?
Answer  : 

* **Lasso :** La régularisation Lasso ajoute, à la fonction de coût, une pénalité proportionnelle à la *valeur absolue des coefficients du modèle*, ce qui favorise la sélection de caractéristiques importantes et conduit à une certaine sparsité. 
* **Ridge :** La régularisation Ridge utilise une pénalité proportionnelle au *carré des coefficients*, ce qui réduit la magnitude des coefficients sans les éliminer complètement.




<!-- 
############################################################
## 
############################################################ 
-->
Question : No category yet - C'est quoi la régularisation ?
Answer  : 

* Une technique pour réduire le surapprentissage (overfitting) en pénalisant les modèles trop complexes. 
* Consiste à ajouter un terme de pénalité à la fonction de coût lors de l'entraînement. 
* Ca encourage le modèle à privilégier des solutions plus simples. 
* La régularisation aide à améliorer la généralisation du modèle en contrôlant sa complexité et en réduisant le risque de surapprentissage.









<!-- 
############################################################
## 
############################################################ 
-->
Question : No category yet - Elbow method, any comment?
Answer  : 

Dans le contexte de l'algorithme K-Means (non supervisé), la méthode Elbow est utilisée pour déterminer le nombre optimal de clusters à utiliser. 

1. Exécution de K-Means pour différents nombres de clusters (de 2 à 20 par exemple). 
2. Calcul de la variance intra-cluster (inertie)
    * Pour chaque exécution de K-Means, on calcule la WCSS 
    * WCSS = within cluster squared sum, somme des distances au carré entre chaque point de données et le centroïde de son cluster correspondant. 
    * Mesure de la dispersion des données à l'intérieur de chaque cluster. 
    * L'inertie intra-cluster. 
    * Plus le nombre de clusters est élevé, plus l'inertie intra-cluster tend à diminuer (si y a autant de cluster que de points elle vaut 0)
    * WCSS est un para global à l'ensemble des clusters. C'est la somme des sommes des carrés
3. Tracé du graphique inertie intra-cluster vs nombre de clusters. 
4. Identification du point de coude sur le graphe. 
    * On recherche le point où la décroissance de l'inertie intra-cluster commence à ralentir de manière significative. 
    * C'est le point où ajouter un cluster de plus k=k+1 ne fait pas basser WCSS de manière significative. 
    * Cela ressemble à un coude sur le graphique. 
    * Ce point est souvent considéré comme le nombre optimal de clusters à utiliser. 
    * Dans certains cas, le point de coude peut ne pas être clairement défini. 
        * Il peut être utile alors d'utiliser d'autres méthodes de validation des clusters (Silhouette).


<!-- 
############################################################
## 
############################################################ 
-->
Question : No category yet - Compromis biais-variance... Ca te parle ?
Answer  : 

1. Le biais mesure à quel point les prédictions d'un modèle diffèrent des valeurs réelles. 
    * Un modèle avec un biais élevé simplifie trop les données d'entraînement et sous-estime la complexité de la relation entre les features et la target. 
    * Conduit à des performances médiocres sur les données d'entraînement et de test. 
    * Les modèles à haut biais sont généralement trop simples pour capturer la complexité des données. 
    * Pour **réduire le biais**, on peut 
        * utiliser des modèles plus complexes 
        * augmenter la taille 
        * augmenter la complexité des caractéristiques utilisées. Features engineering.
2. La variance mesure la sensibilité d'un modèle aux petites variations dans l'ensemble de données d'entraînement. 
    * Un modèle avec une variance élevée est trop sensible au bruit dans les données d'entraînement
    * Cela peut conduire à un surajustement. 
    * Le modèle fonctionne bien sur les données d'entraînement mais il a du mal à généraliser sur de nouvelles données. 
    * Les modèles à haute variance sont souvent complexes (arbres de décision profonds, réseaux neuronaux avec de nombreux paramètres). 
    * Pour **réduire la variance**, on peut utiliser 
        * la régularisation
        * la réduction de la dimensionnalité 
        * augmentation des données.

La **validation croisée** peut être utile pour évaluer comment le compromis biais-variance affecte les performances du modèle. En utilisant la validation croisée, on peut ajuster les hyperparamètres du modèle pour trouver le meilleur compromis entre biais et variance.


<!-- 
############################################################
## 
############################################################ 
-->
Question : No category yet - Connaissez-vous la différence en Corrélation et Causalité ? Pouvez-vous citer quelques critères à vérifier ?
Answer  : 

1. Temporalité : La cause doit précéder l'effet dans le temps.
1. Force de l'association : Une forte association entre une cause potentielle et un effet observé renforce l'idée que la relation pourrait être causale plutôt que simplement corrélée.
1. Plausibilité biologique : Il doit exister une explication scientifique ou un mécanisme pour relier les variables.
1. Relation dose-réponse : Une augmentation de l'exposition doit conduire à une augmentation de l'effet (si la relation est causale).
1. Expérimentation : Des expériences contrôlées, comme des essais randomisés, permettent de tester la causalité.
1. Réversibilité : Si la cause est supprimée, l'effet devrait également disparaître ou diminuer.
1. Consistance : Les résultats doivent être reproduits dans différentes études ou contextes.
1. Spécificité : Un effet spécifique doit être attribué à une cause spécifique.


<!-- 
############################################################
## 
############################################################ 
-->
Question : No category yet - Dans le cas du Machine Learning et du Deep Learning êtes vous d'accord pour dire qu'on laisse le modèle trouver des corrélations et pas des relations de cause à effet?
Answer  : 

**Réponse courte :** Oui car un réseau de neurones peut prédire avec qu'il pleuvra demain si les nuages sont présents, mais cela ne signifie pas qu'il comprend que les nuages **causent** la pluie.

Quoiqu'il en soit, oui oui, je suis d'accord. En ML/DL, les modèles se concentrent sur l'identification de **corrélations** plutôt que sur l'établissement de **relations causales**.

1. **Objectif des modèles ML/DL** : 
   Les algorithmes d'apprentissage supervisé, non supervisé ou par renforcement apprennent à partir des données pour trouver des motifs, des régularités et des corrélations entre les variables d'entrée (features) et les sorties (labels ou classes). Ces corrélations permettent au modèle de faire des prédictions, par exemple, dans la reconnaissance d'images ou la classification de textes. Ces algorithmes ne sont pas conçus pour établir un lien de cause à effet entre les variables.

2. **Corrélations vs Causalité** :
   - **Corrélation** : Les modèles ML/DL détectent des relations statistiques. Ils peuvent trouver des associations entre les données, même si ces associations sont dues au hasard ou à des variables non observées.
   - **Causalité** : Déterminer la causalité nécessite non seulement d'identifier des relations statistiques, mais aussi d'établir un mécanisme expliquant **comment** et **pourquoi** une variable A affecte une variable B. Cela demande généralement une intervention expérimentale (comme les essais contrôlés randomisés), ou des techniques statistiques spécifiques aux modèles causaux.

3. **Pourquoi le DL ne traite pas la causalité directement** :
   - Les réseaux neuronaux (qui sont au cœur du *deep learning*) sont souvent des boîtes noires, c'est-à-dire qu'il est difficile de comprendre **comment** ils arrivent à leurs conclusions. Ils ne sont pas conçus pour interpréter ou expliquer les relations causales. Leur objectif est plutôt de minimiser les erreurs de prédiction à partir des corrélations dans les données d'entraînement.
   - Par exemple, un réseau de neurones peut prédire avec précision qu'il pleuvra demain si les nuages sont présents, mais cela ne signifie pas qu'il comprend que les nuages **causent** la pluie.

4. **Modèles de causalité en ML** :
   Bien que les modèles standards ML/DL ne cherchent pas directement à établir la causalité, il existe des sous-domaines spécialisés du machine learning, comme le **causal inference** ou les **modèles causaux** (e.g., les graphiques de causalité basés sur les travaux de Judea Pearl), qui visent à analyser et à comprendre les relations causales en plus des corrélations.


<!-- 
############################################################
## 
############################################################ 
-->
Question : No category yet - Etes vous d'accord (ou pas) pour dire qu'une IA peut conseiller (il y a x% de chance que l'image soit celle d'un chat) mais qu'elle ne peut pas expliquer (l'image est un chat car...)

Answer  : 
Oui, oui. Un modèle (ML/DL) peut donner des recommandations ou des prédictions basées sur des corrélations, comme « Il y a 85 % de chances que cette image soit celle d'un chat ». Elle se base sur les motifs dans les données qu’elle a appris pendant l'entraînement.

Cependant, elle ne peut pas expliquer le raisonnement derrière cette prédiction de manière causale. Elle ne dit pas « C'est un chat parce qu'il a des oreilles pointues, des moustaches, etc. ». Même si elle détecte ces caractéristiques, elle ne comprend pas **pourquoi** ces éléments définissent un chat. C'est dû à la nature des algorithmes de deep learning, qui se contentent d'optimiser les prédictions sans modéliser explicitement les liens de cause à effet.

C’est un défi de l’IA. Voir les recherches sur l’IA explicable (XAI, Explainable AI).


<!-- 
############################################################
## 
############################################################ 
-->
Question : No category yet - Entre moyenne et médiane. Laquelle des deux est le plus impacté par les outliers? Peux tu illustrer avec un exemple ?

Answer   : 

* La moyenne est plus impactée par les outliers que la médiane.
* Si je suis au bar, que Bill Gates arrive et qu'on calcule la moyenne des salaires... "On sent bien" que cette dernière va être relativement haute. Et ce même si un de mes collègues nous rejoint. 
* Si on calcule alors la médiane de nos 3 salaires, "on sent bien" que cette dernière va se rapprocher de mon collègue et moi.



<!-- 
############################################################
## 
############################################################ 
-->
Question : No category yet - Comment gères-tu le problème de surapprentissage (overfitting) dans un modèle de machine learning ?

Answer   : 

* Régularisation (L1/L2) pour réduire l'effet de la complexité du modèle
* Validation croisée (cross-validation) pour s'assurer que le modèle généralise bien
* Pruning (réduction de la taille du modèle)
* Utiliser des techniques comme dropout dans les réseaux de neurones
* Collecter plus de données ou réduire le nombre de variables d'entrée (feature selection, PCA)


<!-- 
############################################################
## 
############################################################ 
-->
Question : No category yet - Comment choisir entre une régression linéaire et un modèle plus complexe comme les forêts aléatoires (Random Forest) ?

Answer   : 

Ca va dépendre de la nature des données

* **Régression linéaire :** utile si la relation entre les variables d'entrée et de sortie est linéaire ou quasi-linéaire. Modèle simple, facile à interpréter et rapide à entraîner.
* **Forêt aléatoire :** utile lorsque les relations sont plus complexes et non linéaires. Moins sensible aux variables bruitées et outliers. Plus difficile à interpréter. Préférable quand la performance prime sur l'interprétabilité.

Ne pas oublier de mentioner que Scikit-Learn est construit de telle sorte qu'il est très facile de mettre en oeuvre plusieurs modèles avec le même code. Autrement dit, il ne faut pas hésiter à faire des tests et à comparer les résultats des différents modèles. Il faudra alors faire un choix sur LA métrique à laquelle il faudra accorder le plus d'importance.





<!-- 
############################################################
## 
############################################################ 
-->
Question : No category yet - Quelles sont les étapes typiques d'un projet de machine learning ?

Answer   : 

1. Collecte des données : 
1. Prétraitement des données : Nettoyage, gestion des valeurs manquantes, et transformation des données (normalisation, encodage des catégories).
1. Séparation des données : Séparer les données en ensembles d'entraînement, de validation et de test.
1. Choix du modèle : Sélectionner un ou plusieurs algorithmes de machine learning adapté au problème.
1. Entraînement du modèle : 
1. Évaluation du modèle : Utiliser les données de validation/test pour évaluer sa performance (précision, rappel, AUC, etc.).
1. Optimisation : Ajuster les hyperparamètres, utiliser des techniques comme la régularisation pour améliorer les résultats.
1. Déploiement : Mettre le modèle en production
1. Monitoring du modèle : Métriques du modèle + drift dans les données (Evidently AI)
1. CI/CD pour le machine learning : Penser à automatisation le roll-back en cas d’échec d’un déploiement

**Philosophie :**

* Mettre en oeuvre toute la chaîne autour d'un BaseLine Model rapidement. Il sera toujours temps de faire évoluer le modèle ensuite.
* Pour le modèle. Ne pas réinventer la roue. Voir ce qui a été fait, ce qui marche et comment sont obtenus les meilleurs résultats. Transfer Learning



<!-- 
############################################################
## 
############################################################ 
-->
Question : No category yet - Comment gères-tu les biais dans les données et comment t’assures-tu de l’équité dans les modèles ?

Answer   : 

1. Analyse des biais dans les données en amont : identifier les déséquilibres dans les classes, les caractéristiques démographiques, ou autres facteurs.
1. Utilisation de techniques comme la re-sampling (over-sampling/under-sampling) ou la pondération pour traiter les déséquilibres.
1. Audits de modèles réguliers pour surveiller les performances du modèle sur des sous-groupes sensibles.
1. L'application de métriques d'équité comme l'indice de parité de traitement ou la parité démographique.
1. Des approches pour corriger ces biais après le déploiement, telles que algorithmes de post-processing pour ajuster les décisions du modèle.


<!-- 
############################################################
## 
############################################################ 
-->
Question : No category yet - Quelle est ton approche pour mettre à l'échelle un modèle de machine learning en production ?

Answer   : 

* Optimisation des performances : réduire la latence du modèle via la quantification des modèles (réduire la précision des poids), ou distillation de modèle pour créer des versions plus légères.
* Utilisation de frameworks distribués comme Spark MLlib ou TensorFlow Distributed pour gérer des volumes de données importants.
* Déploiement sur des environnements distribués ou dans le cloud (Kubernetes, AWS SageMaker, Google AI Platform) et utilisation de systèmes de streaming pour traiter des données en temps réel.
* Stratégies de mise en cache, de gestion de clusters, et d'utilisation de pipelines CI/CD pour le déploiement continu.
* Méthodologies pour surveiller et ajuster les modèles après leur mise en production.



<!-- 
############################################################
## 
############################################################ 
-->
Question : No category yet - Vous avez un réseau de neurones. Vous ajoutez des features. Le modèle va avoir tendance à ... ? Pourquoi ?

Answer  : 

* Le modèle va avoir tendance à overfitté car on a toutes les chance de rajouter des dimensions non pertinentes qui complexifie la modélisation et empêche un bonne généralisation. 
* On peut penser au cas contraire. Le modèle overfit. Je peux simplifier le réseau de neurones ou bien faire de la PCA et ne retenir que les features les plus pertientes (garder 20% des features qui expliquent 80% des prédictions)


<!-- 
############################################################
## 
############################################################ 
-->
Question : No category yet - Complétez la phrase suivante : "En NLP, la lemmatisation consiste à..."

Answer  : 

* En NLP, la lemmatisation consiste à réduire les mots à leur forme **canonique** ( lemme), c'est-à-dire la forme de base telle qu'on la trouverait dans un dictionnaire. 
* Ainsi "mange", "mangé" et "mangeraient" seraient tous réduits à "manger".
* Contrairement à la racinisation (**stemming**), qui tronque souvent les mots pour obtenir leur radical, la lemmatisation prend en compte le contexte et la morphologie du mot afin de produire un résultat linguistiquement correct.




<!-- 
############################################################
## 
############################################################ 
-->
Question : No category yet - Vous faites de la détection de spam. Vous avez reçu 1_000 mails dont 7% sont des spams. Votre modèle répond toujours ham (vs spam). Dressez la matrice de confusion et calculez precision, recall, accuracy et F1 score. Conclusion ?

Answer  : 

Dans un scénario de détection de spam avec un modèle idiot qui prédit toujours "ham", voici la matrice de confusion ASCII pour un ensemble de données où 93% des emails sont des "ham" et 7% sont des "spam" :

#### Matrice de confusion
On peut la voir comme ça : 

|               | Prédiction Ham | Prédiction Spam |
|---------------|----------------|-----------------|
| **Réel Ham**  | 930            | 0               |
| **Réel Spam** | 70             | 0               |


Ou plus classiquement comme ça :

```
+----------------------------+
Réel |  Ham  |  930  |   0   |
     | Spam  |   70  |   0   |
     +-------+-------+-------+      
     |       |  Ham  | Spam  |
     +-------+-------+-------+      
     |       |   Prédiction  |
+------------+---------------+      
```

#### Métriques

1. **Recall** : proportion de spams détectés correctement.
   $$
   \text{Recall} = \frac{\text{TP}}{\text{TP} + \text{FN}} = \frac{0}{0 + 70} = 0
   $$
   
1. **Précision** : proportion des mails prédits comme spam qui sont effectivement des spams.
   $$
   \text{Précision} = \frac{\text{TP}}{\text{TP} + \text{FP}} = \frac{0}{0 + 0} = 0
   $$
   Ici, on ne peut pas calculer la précision car le dénominateur est nul (aucune prédiction de spam).

1. **Accuracy** (précision globale) : proportion des prédictions correctes (ham ou spam).
   $$
   \text{Accuracy} = \frac{\text{TP} + \text{TN}}{\text{Total}} = \frac{0 + 930}{1000} = 0.93
   $$

1. **F1 Score** : moyenne harmonique entre Recall et Precision   
   $$
   \text{F1 Score} = \frac{2}{\frac{1}{\text{Precision}} + \frac{1}{\text{Recall}}} = \frac{2}{\infty + \infty} = 0
   $$

**Conclusion :** 

1. Bien que l'**Accuracy** soit élevée, le modèle est inutile pour détecter les spams, car il ne les détecte jamais.
1. Dans l'EDA il faut toujours déterminer l'équilibre des classes. On pourra être amené à utiliser SMOTE ou RandomUnderSampler lors du training du modèle.



<!-- 
############################################################
## 
############################################################ 
-->
Question : No category yet - Pouvez-vous me faire un inventaire des commandes git que vous utilisez le plus 

Answer  : 


| Commande         | Exemple de code pour "my_project"                                | Commentaire                                                                 |
|------------------|------------------------------------------------------------------|-----------------------------------------------------------------------------|
| `git clone`      | `git clone https://github.com/user/my_project.git`               | Clone un dépôt Git distant vers le dossier local "my_project"               |
| `git init`       | `git init my_project`                                            | Initialise un nouveau dépôt Git dans le dossier "my_project"                |
| `git remote`     | `git remote add origin https://github.com/user/main_project.git` | Ajoute un dépôt distant (remote) au projet Git local                        |
| `git add`        | `git add my_file.py` ou `git add .`                              | Ajoute "my_file.py" (tous les fichiers) à l'index (staging area) pour le prochain commit |
| `git commit`     | `git commit -m "Ajoute la mémoïsation dans my_file.py"`          | Crée un commit avec un message décrivant les modifications apportées        |
| `git commit`     | `git commit -am "Add graphics in readma.md"`                     | Stage automatiquement les fichiers avant le commit                          |
| `git status`     | `git status`                                                     | Affiche l'état actuel du dépôt (modifications, fichiers en attente, etc.)   |
| `git push`       | `git push origin main`                                           | Pousse les commits locaux vers la branche "main" du dépôt distant "origin"  |
| `git pull`       | `git pull origin main`                                           | Récupère les modifications de la branche "main" et les fusionne avec la branche locale |
| `git branch`     | `git branch feature-branch`                                      | Crée une nouvelle branche nommée "feature-branch"                           |
| `git checkout`   | `git checkout feature-branch`                                    | Change de branche pour travailler sur "feature-branch"                      |
| `git merge`      | `git merge feature-branch`                                       | Fusionne la branche "feature-branch" dans la branche active actuelle        |

<!-- git remote add heroku https://git.heroku.com/py-flashcards.git    Ajouter Heroku comme remote -->
<!-- git subtree push --prefix web_app heroku main                                                 -->

* Les messages dans les commits terminent la phrase en italique ci-dessous:
    * *If applied, this commit will…* Update getting started documentation 
    * Majuscule au début, pas de point à la fin



<!-- 
############################################################
## 
############################################################ 
-->
Question : No category yet - Peux tu me faire un point sur MLflow Projects et me dire à quoi servent les fichiers MLproject ?

Answer  : 

* MLflow Projects est une composante de MLflow 
* Dédiée à la gestion et à la reproductibilité des projets de machine learning
* Objectif = simplifier l’exécution de scripts ML en les encapsulant dans une structure standardisée, portable et partageable. L'idée c'est de garantir que les modèles peuvent être entraînés et évalués de manière cohérente sur différentes machines.

#### Caractéristiques principales de MLflow Projects

1. **Portabilité** : MLflow Projects permet de définir et de documenter toutes les dépendances et configurations nécessaires pour exécuter un projet. Cela inclut les librairies, les paramètres et les scripts.

2. **Définition des dépendances** : Les projets MLflow peuvent inclure des informations sur les dépendances Python (ou d'autres langages) dans des fichiers comme `requirements.txt` ou `conda.yaml`. Ces informations permettent à MLflow de gérer les environnements virtuels pour l'exécution.

3. **Reproductibilité** : En encapsulant les scripts ML dans un projet MLflow avec des configurations bien définies, il est plus facile de reproduire exactement les mêmes résultats à l'avenir, ou de partager le projet avec d'autres.

4. **Exécution standardisée** : Grâce aux définitions de projets, MLflow simplifie l'exécution en ligne de commande ou en script, notamment pour gérer les arguments et les configurations d'entrée.

#### Fichier MLproject

Un fichier `MLproject` est un fichier de configuration YAML qui sert de "point d'entrée" pour définir la structure et les paramètres d’un projet. Ce fichier permet de :

- **Définir les dépendances** : Vous pouvez indiquer quel environnement Conda ou Pip utiliser pour exécuter le projet.
- **Spécifier les entrées et sorties** : Le fichier `MLproject` peut définir des paramètres de manière explicite pour rendre les exécutions flexibles et contrôlables.
- **Déclarer le script principal** : Vous pouvez définir quel script est exécuté lorsque vous lancez le projet MLflow, ainsi que les options de paramètres attendues.
  
Exemple simplifié d’un fichier `MLproject` :

```yaml
name: my_ml_project

conda_env: conda.yaml

entry_points:
  main:
    parameters:
      learning_rate: {type: float, default: 0.01}
      n_estimators: {type: int, default: 100}
    command: "python train.py --learning_rate {learning_rate} --n_estimators {n_estimators}"
```



<!-- 
############################################################
## 
############################################################ 
-->
Question : No category yet - Quels sont les avantages d’un fichier MLproject ?

Answer  : 

Dans le cadre de MLProjects, le fichier `MLproject` 
1. simplifie l'intégration dans des pipelines MLOps 
1. facilite l'exécution reproductible d’expériences

En effet, le fichier `MLproject` rend l'exécution de projets intuitive et standardisée. Par exemple, pour lancer un projet MLflow avec des paramètres spécifiques, on va utiliser :

```bash
mlflow run . -P learning_rate=0.02 -P n_estimators=200
```


<!-- 
############################################################
## 
############################################################ 
-->
Question : No category yet - Qu'est-ce qui se passe après la commande suivante `mlflow run . -P learning_rate=0.02 -P n_estimators=200`

Answer  : 

MLflow va :

1. **Recherche du fichier `MLproject`** : MLflow vérifie dans le répertoire courant (`.`) si un fichier `MLproject` est présent. Ce fichier agit comme une feuille de route pour l’exécution, définissant les dépendances, les paramètres, et le point d’entrée principal.

2. **Configuration de l’environnement d'exécution** : 
   - Si le fichier `MLproject` spécifie un environnement Conda (via `conda_env: conda.yaml` par exemple), MLflow va créer et activer cet environnement.
   - Si `MLproject` définit d'autres fichiers de dépendances, comme `requirements.txt` ou `conda.yaml`, ils seront également pris en compte pour configurer l’environnement.

3. **Exécution de la commande** : Une fois l’environnement configuré, MLflow exécute la commande spécifiée dans le fichier `MLproject` (`command` dans la section `entry_points`). Dans l'exemple donné, cela lance `train.py` en injectant les valeurs de paramètres `learning_rate=0.02` et `n_estimators=200`.

C'est vraiment ce qui permet de garantir une exécution cohérente d’une machine à l’autre car cela permet :
* de reproduire l'exécution 
* avec une configuration stable 
* et des dépendances bien définies



<!-- 
############################################################
## 
############################################################ 
-->
Question : No category yet - Vous êtes dans un environnement Python minimal. Vous utilisez MLProjects et un fichier MLproject dans lequel il y a une ligne `conda_env: conda.yaml`. À quoi ça sert, qu'est ce qui va se passer ?

Answer  : 

* Le champ `conda_env` dans le fichier `MLproject` permet de spécifier un fichier de configuration Conda (`conda.yaml`) 
* Ce dernier contient les dépendances nécessaires, comme `scikit-learn`, `pandas`, etc. 

Quand on exécute le projet dans un environnement minimal, MLflow va :

1. **Créer un nouvel environnement Conda** spécifiquement pour l'exécution du projet, en se basant sur les dépendances listées dans le fichier `conda.yaml`.
2. **Installer toutes les dépendances** spécifiées dans `conda.yaml` (`scikit-learn` par exemple) dans cet environnement.
3. **Exécuter le script Python dans cet environnement isolé**, garantissant que toutes les dépendances sont présentes sans interférer avec l’environnement global ou actuel.

#### Exemple de fichier `MLproject` 

```yaml
name: my_ml_project
conda_env: conda.yaml
entry_points:
  main:
    parameters:
      learning_rate: {type: float, default: 0.01}
    command: "python train.py --learning_rate {learning_rate}"
```

#### Exemple de fichier `conda.yaml`

```yaml
name: my_ml_project_env
channels:
  - defaults
dependencies:
  - python=3.8
  - scikit-learn
  - pandas
  - numpy
```

Lorsque qu'on lance `mlflow run .`, MLflow utilise `conda.yaml` pour créer un environnement avec Python 3.8, `scikit-learn`, `pandas`, et `numpy`. Il active cet environnement et y exécute ensuite `train.py`. C'est ça qui garantit un environnement reproductible sans nécessiter de préinstallation des dépendances sur la machine, et facilite le déploiement dans des environnements variés ou des pipelines MLOps.

Au lieu de ``conda_env`` on peut aussi utiliser `docker_env`. C'est le même principe, on précise l'image docker à lancer (plus paramètres type volume, var d'environnement...). MLflow va alors lancer l'image et y lancer l'exécution du script  

#### Exemple
```yaml
name: fraud_detection 

docker_env:
  image: sklearn_fraud_trainer
  volumes: ["%cd%:/home/app"]
  environment: [ 
      "MLFLOW_TRACKING_URI", 
      "AWS_ACCESS_KEY_ID",
      "AWS_SECRET_ACCESS_KEY",
    ]
    
entry_points:
  main:
    command: "python train.py" 
    # Shows how to pass default parameters
    # train.py will need to : import argparse
    # Uncomment all the lines below
    # parameters:
      # n_estimators: {type: int, default: 15} 
      # min_samples_split: {type: int, default: 3} 
    # command: "python train.py --n_estimators {n_estimators} --min_samples_split {min_samples_split}" 
```


<!-- 
############################################################
## 
############################################################ 
-->
Question : En back propagation on fait des approximations au premier ordre. Quel pourrait être l'intérêt de travailler au second ordre?

Answer  : 

L'objectif principal de l'utilisation des méthodes de second ordre est de **converger vers le minimum avec moins d'itérations** en tenant compte de la courbure de la fonction de coût. En ajustant la direction et l'amplitude des pas en fonction de la courbure locale, ces méthodes permettent souvent de :

1. **Trouver des minima plus rapidement en nombre d'itérations**, ce qui est utile lorsque chaque itération est coûteuse en raison de la taille des données ou du modèle.
2. **Naviguer les régions de plateaux ou de "vallées étroites"** où les gradients sont faibles ou mal orientés. Ces méthodes peuvent ajuster les pas pour éviter de rester bloqué dans de telles régions ou de faire des oscillations, ce qui peut accélérer la convergence.
3. **Améliorer la stabilité des étapes de descente**, car elles adaptent leur approche aux changements locaux de la pente et permettent parfois d'éviter les oscillations dans les directions non souhaitées.

Il y a un compromis à trouver entre moins de pas et plus de temps de calcul à chaque pas. Cela signifie que le bénéfice des méthodes de second ordre dépend du problème spécifique :

- Pour des réseaux profonds modernes, où chaque itération coûte peu relativement à la taille du modèle, la descente de gradient de premier ordre reste souvent préférable.
- Pour des modèles plus petits ou lorsque les itérations sont très coûteuses (par exemple, en formation de grands modèles linguistiques), des approximations de second ordre peuvent être bénéfiques.

#### Exemples

1. **Méthode de Newton** : Cette méthode repose sur l'inversion de la matrice Hessienne (les dérivées secondes) pour ajuster les poids. Elle est plus précise et peut converger plus rapidement que la descente de gradient de premier ordre, car elle utilise des informations de courbure pour prendre en compte la géométrie locale de la fonction de coût. Toutefois, calculer et inverser la Hessienne est coûteux en termes de mémoire et de temps, surtout pour des réseaux de grande taille.

2. **Quasi-Newton (ex. BFGS)** : Les algorithmes quasi-Newton comme BFGS (Broyden-Fletcher-Goldfarb-Shanno) approchent la Hessienne sans avoir à la calculer entièrement. Cela permet de bénéficier de certaines informations de courbure sans le coût complet de calcul de la Hessienne. Cependant, ces méthodes restent souvent plus lourdes que la descente de gradient standard.

3. **Algorithmes basés sur l'approximation de la Hessienne** : Certains algorithmes, comme les méthodes de Hessian-Free Optimization (optimisation sans Hessienne explicite), utilisent des techniques pour estimer des produits matrice-vecteur avec la Hessienne, sans la stocker ni la calculer entièrement. Cela réduit le coût de calcul mais reste plus complexe que la simple descente de gradient.

4. **Optimisation naturelle du gradient** : Cette approche, qui peut être vue comme une méthode de second ordre, adapte le pas de descente en fonction de la métrique de l’espace des paramètres (en utilisant la matrice de Fisher, une approximation de la Hessienne). Cela permet souvent une convergence plus rapide dans les espaces de haute dimension.

**ATTENTION** malgré leurs avantages théoriques, les méthodes de second ordre sont peu utilisées dans les réseaux de neurones profonds pour des raisons de complexité de calcul. Cependant, elles sont plus courantes dans des modèles avec un nombre limité de paramètres ou des réseaux peu profonds.
