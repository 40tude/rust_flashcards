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


<!-- https://app.jedha.co/course/gradient-descent-course-ft/01-gradient-descent-quiz -->

Question : Deep Learning - Gradient Descent - What is the goal of the gradient descent algorithm?
Answer   : To find the set of parameters that minimizes the loss function

Question : Deep Learning - Gradient Descent - What is a loss function?
Answer   : A function that measures how bad the model's prediction errors are

Question : Deep Learning - Gradient Descent - What does the gradient of a function represent?
Answer   : The vector indicating the direction of greatest increase at a given point

Question : Deep Learning - Gradient Descent - What is stochastic gradient descent?
Answer   : A type of gradient descent that uses a batch of samples for each update

Question : Deep Learning - Gradient Descent - Why is a grid search not a suitable method for optimizing model parameters?
Answer   : It requires a large amount of computational power

Question : Deep Learning - Gradient Descent - Which step of the gradient descent algorithm modifies the model parameters iteratively to decrease the loss function?
Answer   : Iteration

Question : Deep Learning - Gradient Descent - What does the learning rate determine in the gradient descent algorithm?
Answer   : The speed at which the parameters are updated

Question : Deep Learning - Gradient Descent - What is the most common stopping criterion in deep learning?
Answer   : Limiting the number of gradient descent steps

Question : Deep Learning - Gradient Descent - What happens if the learning rate is too small in gradient descent?
Answer   : The algorithm converges slowly or may not converge at all

Question : Deep Learning - Gradient Descent - What is the "explosive gradient problem"?
Answer   : When the learning rate is too high, causing the loss function to increase uncontrollably

Question : Deep Learning - Gradient Descent - What is the main difference between gradient descent and stochastic gradient descent?
Answer   : Gradient descent uses all training samples to compute the gradient, while stochastic gradient descent uses a random subset of samples

Question : Deep Learning - Gradient Descent - What is an epoch in the context of training models?
Answer   : The unit of measurement to track model training progress, representing one pass through the entire training dataset



<!-- 
############################################################
## 
############################################################ 
-->
Question : Deep Learning - Gradient Descent - If the ``batch size`` is equal to the ``number of observations``, what would the batch gradient descent be equivalent to?
Answer  : 

* It's called batch gradient descent, or simply gradient descent. 
* In this scenario, the algorithm computes the gradient of the cost function with respect to the parameters using the entire dataset at each iteration.


<!-- 
############################################################
## 
############################################################ 
-->
Question : Deep Learning - Gradient Descent - What is the effect of the batch size on the training of the model? 
Answer  : 

* **Computational Efficiency :** Larger batch sizes result in faster training as more samples are processed in parallel.
* **Stability :** Larger batch sizes provide a more stable estimate of the gradient, which can lead to smoother convergence. They might get stuck in local minima more easily.
* **Generalization :** Smaller batch sizes can help the model generalize better as they introduce more randomness in the updates, which can prevent the model from overfitting.


<!-- 
############################################################
## 
############################################################ 
-->
Question : Deep Learning - Gradient Descent - C'est quoi la back propagation?
Answer  : 

* Un algorithme pour entraîner les réseaux de neurones. 
* Il consiste à calculer les gradients des poids du réseau par rapport à une fonction de coût, puis à ajuster ces poids en utilisant un algorithme d'optimisation tel que la descente de gradient, afin de minimiser la perte lors de la phase d'apprentissage. 
* La rétropropagation permet au réseau de s'ajuster progressivement en fonction des erreurs qu'il commet.


<!-- 
############################################################
## 
############################################################ 
-->
Question : Deep Learning - Gradient Descent - C'est quoi Gradient Descent?
Answer  : 

* Une méthode d'optimisation. 
* Minimisation de la dérivée. 
* Minimisation de la fonction de coût en ML. 
* On ne minimise pas toujours la MSE. 
    * MSE légitime en régression linéaire. 
    * En classification on utilisera log loss AKA cross entropy. 
* On trouve alors les valeurs optimales (poids, biais) qui minimisent la loss function. 


<!-- 
############################################################
## 
############################################################ 
-->
Question : Deep Learning - Gradient Descent - Pouvez-vous donner plus de détails sur l'algo du gradient descent?
Answer  :

1. Initialisation (poids, biais)
2. Itération : calculer le grad par rapport au paramètre à optimiser, prendre l'inverse, avancer d'un pas via learning_rate
3. Condition d'arrêt (n_iter)

* La formule : beta(t+1) = beta (t) - gamma * Grad( C )
* Influence de gamma :
    * Taille du saut d'un beta au suivant
    * Exploding gradient si trop grand
    * Si trop petit on avance pas


<!-- 
############################################################
## 
############################################################ 
-->
Question : Deep Learning - Gradient Descent - Comment choisir gamma et le nb d'itérations?

Answer  :

* On trace la fonction de coût en fonction des itérations
* C doit baisser sur le train set
* Si C baisse puis augmente => exploding gradient
* Sur le val set C va baisser puis augmenter => on trouve alors le bon nb d'itérations (overfiting)


<!-- 
############################################################
## 
############################################################ 
-->
Question : Deep Learning - Gradient Descent - What do you say if I say "Batch Gradient Descent, Stochastic Gradient Descent, Mini-Batch Gradient Descent" ? 
Answer  : 

1. **Batch Gradient Descent** (Batch Size = Number of Observations). Full Batch Gradient Descent : In this scenario, the entire dataset is used to compute the gradient of the cost function. The parameters are updated once per epoch (one pass through the entire dataset). This method is computationally expensive because it requires storing the entire dataset in memory and computing the gradients for all samples before updating the parameters. However, it usually leads to very stable updates and can converge to a good solution.
2. **Stochastic Gradient Descent** (Batch Size = 1) : Here, the gradient is computed and parameters are updated after each individual sample. It's very noisy but can help the model escape local minima more easily and often converges faster, especially with large datasets.
3. **Mini-Batch Gradient Descent** (1 < Batch Size < Number of Observations) : A compromise between batch gradient descent and stochastic gradient descent. It computes the gradient and updates the parameters using a subset of the dataset (a mini-batch) at each iteration. This strikes a balance between the stability of batch gradient descent and the faster convergence of stochastic gradient descent. The batch size is typically chosen to be a power of 2 for efficient memory usage.



<!-- 
############################################################
## 
############################################################ 
-->
Question : Deep Learning - Gradient Descent - You have a dataset of size N and batches of 16 observations. How many times will the parameters of the model be updated before we reach one epoch?  
Answer  : 

``N/16``



<!-- 
############################################################
## 
############################################################ 
-->


Question : Deep Learning - Gradient Descent - Peux tu me faire un point sur **Batch Size** et **Epochs** 
Answer   : 

#### 1. **Batch Size**
Le **batch size** c'est le nombre d'exemples de données sur lesquels le modèle est entraîné avant de mettre à jour les poids via une rétropropagation. C'est la taille des sous-ensembles d'échantillons utilisés pour calculer le gradient et ajuster les poids du modèle.

3 types de traitement basés sur la taille des batchs :

- **Stochastic Gradient Descent (SGD)** : Lorsque la taille du batch est de 1, c’est-à-dire qu’on met à jour les poids après avoir traité chaque exemple de la base de données. Cela peut être bruité, car chaque exemple peut provoquer de grandes variations dans la mise à jour des poids.
- **Mini-batch Gradient Descent** : La taille du batch est un sous-ensemble de l’ensemble de données complet (par exemple, 32, 64, 128). C’est la méthode la plus courante, car elle équilibre la rapidité des mises à jour et la stabilité de la convergence.
- **Batch Gradient Descent** : Ici, le batch size est égal à la totalité de l’ensemble d’entraînement, donc la mise à jour des poids ne se fait qu'une fois par epoch. C’est plus stable, mais plus lent, surtout avec de grands ensembles de données.

#### 2. **Epoch**
Une **epoch** correspond à une passe complète sur l’ensemble de données d’entraînement. C’est le moment où tous les échantillons de données ont été vus une fois par le modèle.

Si on a un data set de 1000 exemples et que le **batch size** est de 100, il faudra 10 batches pour parcourir tous les exemples une fois, ce qui correspondra à **une epoch**.

Pendant l'entraînement, on utilise souvent plusieurs epochs pour permettre au modèle d’apprendre en répétant le processus plusieurs fois. Après chaque epoch, les poids sont ajustés, et le modèle devient progressivement meilleur à mesure que les gradients se raffinent. Bien sûr avant de redemarrer une epoch il faut mélanger (shuffle) le dataset avant de le découper en B batchs.

##### Exemple :
Imaginons qu'on a un dataset de 10 000 exemples, un batch size de 100, et qu'on souhaite entraîner le modèle sur 20 epochs.

- Chaque epoch consistera à traiter 100 batches (10 000 / 100)
- Le modèle verra chaque exemple 20 fois (1 fois par epoch sur 20 epochs), et après chaque batch, il mettra à jour les poids

Le processus est donc :

* Après chaque batch, les poids sont mis à jour en utilisant les gradients calculés pour ce batch.
* Après chaque epoch, aucun ajustement supplémentaire n'est fait, mais on sait qu'on a parcouru tout ton ensemble de données une fois. Si on fait plusieurs epochs, on répète le processus en passant sur tous les exemples plusieurs fois.

Le choix du **batch size** et du nombre d’**epochs** dépend du compromis entre le temps d’entraînement, la qualité des gradients calculés, et la capacité de généralisation du modèle.




<!-- 
############################################################
## 
############################################################ 
-->
Question : Deep Learning - Gradient Descent - Quelle est la différence entre la descente de gradient stochastique (SGD) et la descente de gradient classique ?
Answer  : 

* La descente de gradient stochastique effectue des mises à jour des poids **après chaque exemple** d'entraînement
    * ce qui rend l'optimisation plus rapide mais plus bruitée. 
* La descente de gradient classique calcule les gradients sur **l'ensemble de données** et met à jour les poids une fois
    * ce qui est plus lent mais moins bruité. 
* Garder en tête que si le calcul de grad C est 100 fois plus rapide sur n' points alors on peut se permettre d'avoir une "route" 20 fois moins directe. 
    * Parler du fait que ça peut nous permettre de sortir d'un minimum local.



<!-- 
############################################################
## 
############################################################ 
-->

Question : Deep Learning - Gradient Descent - Peux tu m'expliquer point par point la back propagation ? Afin de simplifier les choses on se place dans le cas d'un réseau de neurones à 1 neurone et on fait les hypothèses suivantes : 

* Entrée $x_1 = 2$ 
* Poids initial  $w_1 = 0.5 $
* Biais initial $b = 0.0$
* Sortie désirée  $y_{\text{réel}} = 1$ 
* Fonction d'activation sigmoïde : $f(z) = \frac{1}{1 + e^{-z}} $
* La fonction de coût est l'erreur quadratique : $ J = \frac{1}{2} (y_{\text{prévu}} - y_{\text{réel}})^2 $
* Taux d'apprentissage $ \eta = 0.1$


Answer   : 

#### Calcul de la sortie $ y_{\text{prévu}} $

* Somme pondérée : $ z = w_1 \cdot x_1 + b = 0.5 \cdot 2 + 0 = 1.0 $
* Sortie prévisible (à travers la fonction sigmoïde) :
  $$
  y_{\text{prévu}} = f(z) = \frac{1}{1 + e^{-1}} = 0.731
  $$

#### Calcul de la fonction de coût

On injecte $ y_{\text{prévu}} $ et $ y_{\text{réel}} $ dans la fonction de coût.

$$
J = \frac{1}{2} (y_{\text{prévu}} - y_{\text{réel}})^2 = \frac{1}{2} (0.731 - 1)^2 = 0.036
$$

#### Calcul des différentes dérivées partielles pour la rétropropagation

##### Justification :

Dans le cas du poids on va vouloir écrire un truc du style :

$$
w_1 \leftarrow w_1 -\eta \cdot \frac{\partial J}{\partial w_1} = w_1-\eta \cdot \frac{\partial J}{\partial y_{\text{prévu}}} \cdot \frac{\partial y_{\text{prévu}}}{\partial z} \cdot \frac{\partial z}{\partial w_1}
$$

Il faut donc calculer les 3 dérivées partielles suivantes : $\frac{\partial J}{\partial y_{\text{prévu}}}$, $\frac{\partial y_{\text{prévu}}}{\partial z}$ et $\frac{\partial z}{\partial w_1}$

##### Dérivée de la fonction de coût par rapport à la sortie prévisible $ y_{\text{prévu}} $ :

$$
\frac{\partial J}{\partial y_{\text{prévu}}} = \frac{\partial}{\partial y_{\text{prévu}}} \left( \frac{1}{2} (y_{\text{prévu}} - y_{\text{réel}})^2 \right)
$$

$$
\frac{\partial J}{\partial y_{\text{prévu}}} = \frac{1}{2} \cdot 2 \cdot (y_{\text{prévu}} - y_{\text{réel}}) * 1
$$

$$
\frac{\partial J}{\partial y_{\text{prévu}}} = y_{\text{prévu}} - y_{\text{réel}} = 0.731 - 1 = -0.269
$$

##### Dérivée de la sortie prévisible $ y_{\text{prévu}} $ par rapport à $ z $ :



La sortie prévisible est obtenue via la fonction sigmoïde, donc :

$$ y_{\text{prévu}} = f(z) = \frac{1}{1 + e^{-z}} $$

$$
\frac{\partial f(z)}{\partial z} = \frac{-(-e^{-z})}{(1 + e^{-z})^2}  = \frac{e^{-z}}{(1 + e^{-z})^2}
$$

On peut simplifier les choses en remarquant que si

$$ f(z) = \frac{1}{1 + e^{-z}} $$

Alors
 $$ e^{-z} = \frac{1-f(z)}{f(z)} $$

Si on réinjecte dans l'expression de $\frac{\partial f(z)}{\partial z}$ il vient :

$$
\frac{\partial f(z)}{\partial z} = \frac{e^{-z}}{(1 + e^{-z})^2} = \frac{1-f(z)}{f(z)} * \frac{1}{(1 + \frac{1-f(z)}{f(z)})^2}
$$

$$
\frac{\partial f(z)}{\partial z} = (1 - f(z)) \cdot f(z)
$$


$$
\frac{\partial y_{\text{prévu}}}{\partial z} = y_{\text{prévu}} (1 - y_{\text{prévu}}) = 0.731 \cdot (1 - 0.731) = 0.196
$$

##### Dérivée de $z$ par rapport à $w_1$ et $b$ :

On avait :

$$ z = w_1 \cdot x_1 + b $$

Donc : 

* $ \frac{\partial z}{\partial w_1} = x_1 = 2 $
* $ \frac{\partial z}{\partial b} = 1 $

#### Mise à jour des poids et du biais

Les mises à jour se font en tenant compte du taux d'apprentissage $ \eta = 0.1 $.

##### Mise à jour du poids $ w_1 $ :

La variation de poids c'est bien : 

$$
\Delta w_1 = -\eta \cdot \frac{\partial J}{\partial w_1} = -\eta \cdot \frac{\partial J}{\partial y_{\text{prévu}}} \cdot \frac{\partial y_{\text{prévu}}}{\partial z} \cdot \frac{\partial z}{\partial w_1}
$$

Avec les valeurs numériques :

$$ \Delta w_1 = -0.1 \cdot (-0.269) \cdot 0.196 \cdot 2 = 0.0105 $$

Le nouveau poids devient :

$$ w_1 \leftarrow w_1 + \Delta w_1 = 0.5 + 0.0105 = 0.5105 $$






##### Mise à jour du biais $ b $ 

La variation du biais est :

$$
\Delta b = -\eta \cdot \frac{\partial J}{\partial b} = -\eta \cdot \frac{\partial J}{\partial y_{\text{prévu}}} \cdot \frac{\partial y_{\text{prévu}}}{\partial z} \cdot \frac{\partial z}{\partial b}
$$

$$
\Delta b = -0.1 \cdot (-0.269) \cdot 0.196 \cdot 1 = 0.0052
$$

Le nouveau biais devient :
$$
b \leftarrow b + \Delta b = 0 + 0.0052 = 0.0052
$$



























<!-- https://app.jedha.co/course/introduction-to-neural-networks-ft/01-neural-networks-quiz -->


Question : Deep Learning - Neural networks - What is the purpose of an activation function in a neural network? 
Answer   : To add non-linearity to the network's behavior

Question : Deep Learning - Neural networks - Which activation function is known for being computationally efficient and allowing for back propagation? 
Answer   : ReLu

Question : Deep Learning - Neural networks - What is the disadvantage of the ReLu activation function? 
Answer   : It is not zero-centered

Question : Deep Learning - Neural networks - What type of architecture is organized in a sequential manner, with input, hidden, and output layers? 
Answer   : Sequential architecture

Question : Deep Learning - Neural networks -  What is the purpose of the forward pass in a neural network?
Answer   : To transform inputs into outputs

Question : Deep Learning - Neural networks -  How are layers connected in a sequential neural network architecture?
Answer   : Each layer receives inputs from the previous layer

Question : Deep Learning - Neural networks -  What are hyperparameters in a neural network?
Answer   : The neural network model architecture

Question : Deep Learning - Neural networks - How do data scientists typically determine the number of layers and neurons in a neural network? 
Answer   : Through trial and error and comparing models

Question : Deep Learning - Neural networks - What is the purpose of an activation function in a neural network? 
Answer   : To add non-linearity to the network's behavior



<!-- 
############################################################
## 
############################################################ 
-->
Question : Deep Learning - Neural networks - Comment interprétez-vous le bias associé à un neurone particulier d'un réseau ?
Answer  : 

#### Métaphore du biais
* Prenons le cas d'un **thermostat**
* Les **poids** peuvent être vus comme la "température extérieure" (les entrées) qui influence le réglage de la température intérieure
* Le **biais** c'est le réglage manuel du thermostat pour ajuster la température intérieure notre convenance et ce, indépendamment des conditions extérieures. 
* Le biais permet donc de fixer une température de base **avant** même que la température extérieure n'entre en jeu

#### Qu'est-ce que le biais ?
Le biais est une valeur supplémentaire ajoutée dans le calcul des sorties des neurones. Formellement, dans un réseau de neurones, la sortie d'un neurone est calculée à partir de la somme pondérée des entrées, à laquelle on ajoute le biais, avant d'appliquer la fonction d'activation.

Si $x_1, x_2, \dots, x_n$ sont les entrées, et $w_1, w_2, \dots, w_n$ les poids correspondants, alors la sortie d'un neurone (avant d'appliquer la fonction d'activation) est :

$$
z = w_1 x_1 + w_2 x_2 + \dots + w_n x_n + b
$$

où $b$ est le **biais**.

La fonction d'activation (par exemple, ReLU, Sigmoid, etc.) est ensuite appliquée à cette somme pour obtenir la sortie finale du neurone.

#### À quoi sert le biais ?
Le biais permet de **décaler** ou de **déplacer** la fonction d'activation de manière à ce qu'elle ne soit pas forcément centrée sur zéro. Il introduit une flexibilité supplémentaire dans le réseau et permet au modèle d'apprendre des schémas plus complexes.

- **Sans biais**, un neurone avec une entrée nulle produirait toujours une sortie nulle (car $ z = 0 $). Cela rendrait difficile l'apprentissage de certaines fonctions où l'activation ne doit pas être strictement alignée avec l'origine. Pensez au cas où étudie la taille des adultes. La taille ne peut pas être nulle.
- **Avec biais**, même si les entrées sont nulles, la sortie peut être différente de zéro (grâce à $b$), ce qui permet d'introduire plus de diversité dans les résultats possibles.

#### Interprétation du biais

* Le biais peut être interprété comme un paramètre d'ajustement indépendant des entrées. 
* Il permet de contrôler le point à partir duquel un neurone "s'active".
* En d'autres mot, le biais ajuste le seuil à partir duquel la fonction d'activation commence à produire des résultats non nuls.

##### Exemple 
* Si un réseau de neurones doit apprendre à classer des images de chats et de chiens
* Certains neurones pourraient être spécialisés dans la détection de certaines caractéristiques (par exemple, des oreilles pointues). 
* Le biais permet à ces neurones de s'activer ou non, en fonction de la présence ou de l'absence de certaines caractéristiques dans l'image.

#### Fonction du biais dans l'apprentissage

Le biais est un paramètre **appris** par le réseau, tout comme les poids des connexions. Pendant l'entraînement du modèle via la rétropropagation, le biais est ajusté pour minimiser l'erreur globale du réseau.

- **Impact sur les fonctions d'activation** : Pour des fonctions d'activation comme la Sigmoid ou Tanh, le biais ajuste le point autour duquel la fonction commence à "switcher" entre deux états (par exemple, entre une activation faible et forte). Pour ReLU, le biais décale le point à partir duquel le neurone devient actif (passe de 0 à une valeur positive).

#### Exemple simplifié
Imaginons un neurone avec deux entrées $x_1$ et $x_2$, des poids $w_1$ et $w_2$, et un biais $b$. La somme pondérée est donc :

$$
z = w_1 x_1 + w_2 x_2 + b
$$

Si les poids sont proches de zéro, mais que le biais $b$ est grand, le neurone peut toujours produire une sortie élevée même si les entrées $x_1$ et $x_2$ sont petites. Cela permet au réseau d'apprendre à ajuster l'activation du neurone indépendamment des entrées directes.



<!-- 
############################################################
## 
############################################################ 
-->
Question : Deep Learning - Neural networks - Would you say that using neural network models compensates the need for feature engineering?
Answer  : 

* It does. 
* The outputs of the neurons in the network may be interpreted as new features that will be used by later neurons to make even more complex features leading to the final prediciton. 
* In addition, these "features" are build by neurons whose parameters get optimized according to the loss function. 
* So it creates features that are linked to the target variable without having to be explicitely coded. 
* The major **downside** is that it all happens in what may be qualified as a "black box" model.  


<!-- 
############################################################
## 
############################################################ 
-->
Question : Deep Learning - Neural networks - If the model overfits, what can we do to limit overfitting?
Answer  : 

* We can reduce the number of neurons and hidden layers in the network. 
* We can also introduce regularization like Ridge (L2) or Lasso (L1)


<!-- 
############################################################
## 
############################################################ 
-->
Question : Deep Learning - Neural networks - What is the effect of adding neurons on a layer?
Answer  : 

Adding a neuron to a layer makes it possible for the model to create an additional "feature" on a given level of complexity



<!-- 
############################################################
## 
############################################################ 
-->
Question : Deep Learning - Neural networks - What happens if we use a linear activation function? 
Answer  : 

* As a hidden layer 
    * Using a linear activation function is **NOT** a good idea. 
    * We loose the capabilities of neural networks to learn complex relation (non linearities). 
* As an output layer 
    * A linear activation function can be used in regression problems



<!-- 
############################################################
## 
############################################################ 
-->
Question : Deep Learning - Neural networks - What is the effect of adding hidden layers?
Answer  : 

* Adding a hidden layer lets the model add one more level of non-linearity by applying one more activation function to the previous output
* This leads to exponentially complex outputs.


<!-- 
############################################################
## 
############################################################ 
-->
Question : Deep Learning - Neural networks - When you use additional features to feed the model, do you need to use as many neurons and layers? Would adding more neurons and layers be an alternative to using additional features?
Answer  : 

* Adding new features may let you use less complex architectures
* the upside is that you know exactly what input features are used which makes the model more interpretable. 
* On the other hand you may be missing some very useful features that model may have created for you.


<!-- 
############################################################
## 
############################################################ 
-->
Question : Deep Learning - Neural networks - Is it more useful to add more neurons on the layers near the bottom or near the top?
Answer  : 

* It is more useful to add neurons towards the bottom because the complexity of the outputs of earlier neurons limit the complexity of the outputs of later neurons
* It is generally good practice to have more neurons on bottom layers and progressively decrease the number of neurons going up the network.





<!-- 
############################################################
## 
############################################################ 
-->
Question : Deep Learning - Neural networks - Can you list most important activation functions
Answer  : 


<p align="center">
<img src="../static/md/assets/activation.png" alt="activation" width="577"/>
</p>



<!-- 
############################################################
## 
############################################################ 
-->
Question : Deep Learning - Neural networks - Avantages et inconvénients de la fonction d'activation ReLU?
Answer  : 

### Avantages de la fonction ReLU (Rectified Linear Unit) :
1. **Simplicité** :
   - ReLU est facile à calculer et ne nécessite pas de calculs exponentiels comme certaines autres fonctions d'activation (par exemple, Sigmoid ou Tanh).
   - Sa formule est simple : $ f(x) = max(0, x) $, où les valeurs négatives sont mises à zéro et les positives sont conservées.

2. **Résolution du problème du vanishing gradient** :
   - Contrairement aux fonctions Sigmoid et Tanh, ReLU permet de mieux éviter le problème du "gradient qui disparaît". En effet, les fonctions d'activation comme Sigmoid et Tanh ont des sorties comprises entre -1 et 1, ce qui signifie que leurs dérivées sont très petites dans une large gamme de valeurs. Lorsque ces petits gradients sont multipliés dans des couches profondes, le résultat est encore plus petit, ce qui empêche les premières couches du réseau de mettre à jour efficacement leurs poids. Le réseau devient alors lent à apprendre, voire incapable d’apprendre, car ces couches "ne ressentent" plus l'effet des erreurs. 
   - Bref, ReLU aide à la propagation efficace du gradient à travers les couches lors de l'entraînement.

3. **Sparsité des activations** :
   - ReLU introduit une forme de **sparsité** dans les réseaux de neurones, car elle met à zéro toutes les valeurs négatives. 
   - Cela conduit à des réseaux de neurones plus efficaces, car seules certaines unités sont activées.

4. **Convergence plus rapide** :
   - ReLU permet à de nombreux modèles d'apprendre plus rapidement car elle accélère la convergence par rapport aux autres fonctions d'activation.

### Inconvénients de la fonction ReLU :
1. **Problème de neurones morts (Dead Neurons)** :
   - Un des inconvénients majeurs est que si un neurone reçoit constamment des valeurs négatives ou nulles à cause de la fonction ReLU, il peut "mourir" et ne plus contribuer au réseau. 
   - Cela peut entraîner des neurones inactifs qui ne se mettent jamais à jour.

2. **Pas de borne supérieure** :
   - La fonction ReLU n'a pas de limite supérieure pour les valeurs positives. 
   - Cela peut entraîner des activations très élevées qui déstabilisent l'entraînement si elles ne sont pas bien régulées, par exemple en utilisant des techniques de **normalisation** [0, 1].

3. **Problème avec les valeurs négatives** :
   - ReLU ne traite pas bien les valeurs négatives (toutes mises à zéro)
   - Ca qui signifie que les neurones peuvent perdre certaines informations si la fonction est appliquée trop strictement. Des variantes type **Leaky ReLU** ou **Parametric ReLU (PReLU)** existent. Elles permettent à une petite portion des valeurs négatives de "passer", ce qui aide à éviter le problème des neurones morts.











<!-- https://app.jedha.co/course/introduction-to-tensorflow-ft/01-neural-networks-with-tf-quiz -->


Question : Deep Learning - Neural networks with TensorFlow - What is the purpose of the Dense layer in a neural network?
Answer   : To create a fully connected neuron layer

Question : Deep Learning - Neural networks with TensorFlow - Quelle couche est utilisée pour normaliser les sorties de la couche précedente?
Answer   : 

``BatchNormalization`` layer (mean = 0, standard deviation = 1)

On met une couche de BN (et de dropout) entre chaque couche cachée. On peut mettre une couche BN tout de suite aprsè l'entrée. Dans ce cas, pas besoin de normaliser les données.

Lorsqu'on applique la normalisation par batch, chaque sortie d'une couche précédente est normalisée indépendamment. Cette normalisation permet de réajuster les activations (ou sorties) pour qu'elles aient une moyenne de 0 et une écart-type de 1, au sein de chaque mini-batch **d’entraînement**. L’objectif de ce processus est de réduire la variance dans les activations, ce qui permet d’accélérer l'apprentissage en rendant les gradients plus stables.

Pour chaque nouveau mini-batch, les paramètres de normalisation sont recalculés en fonction des activations issues de ce mini-batch spécifique. Cela signifie que la normalisation s'adapte en permanence pendant l’entraînement, ce qui permet au modèle d’apprendre plus efficacement, même si les distributions de données varient.

Pendant l’entraînement, BN se base sur les statistiques (moyenne et variance) calculées pour chaque batch. Cependant, lors de la phase d’**inférence** (prédiction), la couche BN ne fonctionne plus de la même manière. À ce stade, elle utilise des moyennes mobiles et des écarts-types calculés durant l’entraînement, afin de s'adapter aux nouvelles données.

Cela garantit que le modèle utilise des statistiques globales, ajustées pour l'ensemble des données d'entraînement, plutôt que des statistiques sur des mini-batches spécifiques, afin d’assurer une prédiction cohérente et fiable.

Avantages de Batch Normalization

* **Amélioration de la stabilité du réseau** : en normalisant les entrées de chaque couche, BN réduit le problème de disparition ou d'explosion du gradient, souvent rencontré dans les réseaux profonds.
* **Accélération de l’apprentissage** : grâce à la normalisation, le modèle converge plus rapidement, ce qui permet d'utiliser des learning rate plus grands (et de réduire finalement le wall time, le temps d'apprentissage total indiqué par l'horloge au mur)

Régularisation implicite : BN a également un effet de régularisation, réduisant parfois le besoin d’autres techniques comme le dropout.



Question : Deep Learning - Neural networks with TensorFlow - What is the purpose of the Dropout layer in a neural network?
Answer   : To prevent overfitting

Question : Deep Learning - Neural networks with TensorFlow - Which of the following :

1. BatchNormalization
1. Regularization
1. Dense
1. ReLU 

is an activation function used in neural networks?

Answer   : ReLU




Question : Deep Learning - Neural networks with TensorFlow - What is the purpose of regularization in neural networks?
Answer   : To prevent overfitting

Question : Deep Learning - Neural networks with TensorFlow - Which loss function is ideal for most **regression** problems?
Answer   : 

``MeanSquaredError``

#### Code snippet 

```python
optimizer= tf.keras.optimizers.Adam()

model.compile(optimizer=optimizer,
              loss=tf.keras.losses.MeanSquaredError(),
              metrics=[tf.keras.metrics.MeanAbsoluteError()])

```




Question : Deep Learning - Neural networks with TensorFlow - Which loss function is ideal for **binary classification** problems?
Answer   : 

``BinaryCrossentropy``

#### Code snippet 

```python
from tensorflow.keras.losses     import BinaryCrossentropy
from tensorflow.keras.metrics    import BinaryAccuracy
from tensorflow.keras.optimizers import Adam

model.compile(
    optimizer = Adam(0.01),
    loss      = BinaryCrossentropy(),
    metrics   = [BinaryAccuracy()]         # This is a list
)              


```



Question : Deep Learning - Neural networks with TensorFlow - Which loss function is ideal for **multi-class classification** problems where the target variable is in **dummy** form?

Answer   : 

``CategoricalCrossentropy``

#### Code snippet 

```python
model.compile(
    optimizer=tf.keras.optimizers.Adam(learning_rate = 0.001),
    loss = tf.keras.losses.CategoricalCrossentropy(),
    metrics = [tf.keras.metrics.CategoricalAccuracy()])

```




Question : Deep Learning - Neural networks with TensorFlow - Which loss function is ideal for **multi-class classification** problems where the target variable is in **index** form?
Answer   : 

``SparseCategoricalCrossentropy``

#### Code snippet 

```python
optimizer = tf.keras.optimizers.Adam(learning_rate=3e-5, epsilon=1e-08)
loss = tf.keras.losses.SparseCategoricalCrossentropy(from_logits=True)
metrics = tf.keras.metrics.SparseCategoricalAccuracy('accuracy')

model.compile(optimizer=optimizer, loss=loss, metrics=metrics)

history = model.fit(
    train_dataset, 
    epochs=k_epochs, 
    validation_data=test_dataset,
    callbacks=[early_stopping, reduce_lr, tensorboard]
)


```



Question : Deep Learning - Neural networks with TensorFlow - What is the name of the adaptive optimizer that increases or decreases the learning rate based on the gradient value?
Answer   : Adam






<!-- 
############################################################
## 
############################################################ 
-->


Question : Deep Learning - Neural networks with TensorFlow - L'éxécution de la ligne de code ``model.summary()`` génère la sortie ci-dessous. Pouvez-vous l'expliquer en détail et retrouver le nombre d'entrées du réseau ?


```bash
Model: "sequential_13"
_________________________________________________________________
 Layer (type)                Output Shape              Param #   
=================================================================
 dense_74 (Dense)            (None, 8)                 24        
                                                                 
 batch_normalization_16 (Ba  (None, 8)                 32        
 tchNormalization)                                               
                                                                 
 dense_75 (Dense)            (None, 4)                 36        
                                                                 
 dense_76 (Dense)            (None, 1)                 5         
                                                                 
=================================================================
Total params: 97 (388.00 Byte)
Trainable params: 81 (324.00 Byte)
Non-trainable params: 16 (64.00 Byte)
_________________________________________________________________

```

Answer   : 
* Réseau de neurones avec 4 couches
* Couche 1 
    * Nb paramètres = 24
    * 24 = 8 biais + 8 x Nb_Input
    * Donc Nb_Input = 2
* Couche 2 
    * C'est une couche de normalisation (``BatchNormalization``)
    * On part de 8 on arrive à 8 
    * 8 neurones de normalisation 
    * Nb paramètres = 8 * 4 = 32   
    * Il y a 8 * 2 paramètres pour le training et 8 * 2 paramètres pour les inférences
    * Les 2 correspondent au facteur d'échelle ($\gamma$) et ainsi qu'à l'offset ($\beta$) à utiliser pour ramener les valeurs des différents batchs dans la gamme [0, 1]
* Couche 3 
    * On part de 8 on arrive à 4 
    * Nb paramètres = 4 * 8 poids + 4 biais  = 36
* Couche 4 
    * On part de 4 on arrive à 1 
    * Nb paramètres = 4 * 1 poids + 1 biais = 5
* Nombre total de paramètres 97 (24+32+36+5)
* Il y a 16 paramètres non entrainables. Les 16 paramètres de normalisation utilisé en inférence
* Il y a 81 paramètres entrainables (97 - 16)

#### Rappel
* Pendant l'entraînement, la couche de normalisation apprend les paramètres ($\gamma$ et $\beta$), ce qui explique pourquoi il y a 8 * 2 = 16 paramètres entraînables. 8 facteurs d'échelle (pour les 8 neurones de la couche) et 8 offsets.
* Lors de des inférences, la couche de normalisation utilise la moyenne et la variance globale calculées sur l'ensemble des lots pendant l'entraînement, plutôt que sur le batch actuel. Ces statistiques (moyenne et variance globales) sont précalculées et non modifiables pendant les inférences. C'est pourquoi Keras les considère comme paramètres non entraînables. Cela donne 8 * 2 = 16 paramètres non entraînables (8 pour la moyenne, 8 pour la variance).

#### À propos du None qu'on voit dans la colonne Output Shape
* None fait référence à la dimension du batch size lors de l'entraînement ou de l'inférence du modèle 
* La taille du lot n'est pas encore spécifiée à ce stade du modèle
* Pour les tenseurs on aura : 
    * nb lignes   = batch size 
    * nb colonnes = nb de neurones




<!-- 
############################################################
## 
############################################################ 
-->


Question : Deep Learning - Neural networks with TensorFlow - L'éxécution de la ligne de code ``model.summary()`` génère la sortie ci-dessous. Comment dessineriez-vous la situation ?


```bash
Model: "sequential_13"
_________________________________________________________________
 Layer (type)                Output Shape              Param #   
=================================================================
 dense_74 (Dense)            (None, 8)                 24        
                                                                 
 batch_normalization_16 (Ba  (None, 8)                 32        
 tchNormalization)                                               
                                                                 
 dense_75 (Dense)            (None, 4)                 36        
                                                                 
 dense_76 (Dense)            (None, 1)                 5         
                                                                 
=================================================================
Total params: 97 (388.00 Byte)
Trainable params: 81 (324.00 Byte)
Non-trainable params: 16 (64.00 Byte)
_________________________________________________________________

```

Answer   : 

* Couche 1 
    * Nb paramètres = 24
    * 24 = 8 biais + 8 x Nb_Input
    * Donc Nb_Input = 2

<p align="center">
<img src="../static/md/assets/neural_network_01.png" alt="attention" width="577"/>
</p>

* L'entrée du modèle est dans le fond et la sortie à l'avant
* Tous les neurones d'une couche sont connectés à tous les neurones de la couche précédente (dense)
* On ne tient pas compte du Batch_Size qui est à None dans le model.summary()



<!-- 
############################################################
## 
############################################################ 
-->


Question : Deep Learning - Neural networks with TensorFlow - L'éxécution de la ligne de code ``model.summary()`` génère la sortie ci-dessous. Je vous annonce que ``batch_size = 4``. Comment dessineriez-vous la situation ?


```bash
Model: "sequential_13"
_________________________________________________________________
 Layer (type)                Output Shape              Param #   
=================================================================
 dense_74 (Dense)            (None, 8)                 24        
                                                                 
 batch_normalization_16 (Ba  (None, 8)                 32        
 tchNormalization)                                               
                                                                 
 dense_75 (Dense)            (None, 4)                 36        
                                                                 
 dense_76 (Dense)            (None, 1)                 5         
                                                                 
=================================================================
Total params: 97 (388.00 Byte)
Trainable params: 81 (324.00 Byte)
Non-trainable params: 16 (64.00 Byte)
_________________________________________________________________

```

Answer   : 

* Couche 1 
    * Nb paramètres = 24
    * 24 = 8 biais + 8 x Nb_Input
    * Donc Nb_Input = 2

<p align="center">
<img src="../static/md/assets/neural_network_02.png" alt="attention" width="577"/>
</p>

* L'entrée du modèle est dans le fond et la sortie à l'avant
* Dans le model_summary(), dans la colonne Output Shape, None sera remplacé par Batch_Size (4 ici)
* On traite Batch_Size éléments en parallèle
* Par couche, chaque tenseur possède Batch_Size lignes et Nb_Neurons colonnes
    * Ils sont représenté par les matrices de couleurs
    * On retrouve les tailles : 4x2, 4x8, 4x8, 4x4 et 4x1







<!-- 
############################################################
## Questions issues des quizz
############################################################ 
-->


<!-- https://app.jedha.co/course/convolutional-neural-network-ft/01-understand-cnn-quiz -->

Question : Deep Learning - Understand CNN - What is the main advantage of using convolutional neural networks (CNNs) for image analysis?
Answer   : They can take advantage of the spatial structure and hierarchical organization of images.

Question : Deep Learning - Understand CNN - Why is it inefficient to use fully connected neural networks on images?
Answer   : Fully connected neurons require a different parameter for each pixel in the image.

Question : Deep Learning - Understand CNN - What does the kernel size represent in a convolutional layer?
Answer   : The size of the patterns that the neuron can detect in an image.

Question : Deep Learning - Understand CNN - What does the stride represent in a convolutional layer?
Answer   : The distance the convolutional filter moves across the input image.

Question : Deep Learning - Understand CNN - What is the purpose of padding in a convolutional layer? 
Answer   : 

1. To detect information located near the border of the input image.
1. To prevent the spatial dimension of the output from shrinking.

Question : Deep Learning - Understand CNN - What is the main purpose of pooling layers in convolutional neural networks?
Answer   : To summarize the information flowing through the network.

Question : Deep Learning - Understand CNN - Which aggregation function is commonly used in MaxPooling?
Answer   : Max

Question : Deep Learning - Understand CNN - What does building deeper convolutional networks help with?
Answer   : Taking advantage of the hierarchical structure of images.




<!-- 
############################################################
## 
############################################################ 
-->

Question : Deep Learning - CNN with TensorFlow - L'éxécution de la ligne de code ``model.summary()`` génère la sortie ci-dessous. Je vous confirme qu'on alimente le modèle avec des images RGB 100x100. Pouvez-vous recalculer les valeurs de la colonne Param ?


```bash
Model: "sequential"
 
+--------------------------------------+-----------------------------+-----------------+
| Layer (type)                         | Output Shape                |           Param |
+--------------------------------------+-----------------------------+-----------------+
| conv2d_1 (Conv2D)                    | (None, 50, 50, 32)          |             896 |
+--------------------------------------+-----------------------------+-----------------+
| conv2d_2 (Conv2D)                    | (None, 25, 25, 64)          |          18,496 |
+--------------------------------------+-----------------------------+-----------------+
| conv2d_3 (Conv2D)                    | (None, 13, 13, 64)          |          36,928 |
+--------------------------------------+-----------------------------+-----------------+
| conv2d_4 (Conv2D)                    | (None, 7, 7, 128)           |          73,856 |
+--------------------------------------+-----------------------------+-----------------+
| flatten (Flatten)                    | (None, 6272)                |               0 |
+--------------------------------------+-----------------------------+-----------------+
| dense (Dense)                        | (None, 1)                   |           6,273 |
+--------------------------------------+-----------------------------+-----------------+
 Total         params: 136,449 (533.00 KB)
 Trainable     params: 136,449 (533.00 KB)
 Non-trainable params:       0 (  0.00 KB)
```

Answer   : 

* On a  Nb_Params = Nb_Filtres x ( Nb_Channels_In x kernel x kernel + BIAIS)
* On part d'images 100x100 en RGB
* En mode "télégraphique" on peut écrire : 

```bash
Conv 2D f= 32 k=3x3 s=2 p=same  => sortie 32 matrices de 50x50  Params =  32 ( 3*3*3 + 1) =    896  
Conv 2D f= 64 k=3x3 s=2 p=same  => sortie 64 matrices de 25x25  Params =  64 (3*3*32 + 1) = 18_496
Conv 2D f= 64 k=3x3 s=2 p=same  => sortie 64 matrices de 3x3    Params =  64 (3*3*64 + 1) = 36_928
Conv 2D f=128 k=3x3 s=2 p=same  => sortie 128 matrices de 7x7   Params = 128 (3*3*64 + 1) = 73_856
Flatten                                                         Params                    =      0
Dense   En entrée il y a 128 matrices 7x7 => 128*7*7 = 6_272    Params =        6_272 + 1 =  6_273
```
* Soit un total de 136_449 paramètres




#### À garder en tête
* Les filtres sont des filtres 3D
* Chacun des filtres d'une couche s'applique simultanément à l'ensemble des feature maps qui sont en entrée
    * Au niveau de la couche 2 ci-dessus, les filtres sont de dimension (3,3,32) car en sortie de la couche 1 on a 32 feature maps
    * Au niveau de la couche 1 ci-dessus, les filtres sont de dimension (3,3, 3) car en entrée de la couche 1 on a une image RGB

* Dans la couche 2, le filtre va convoluer simultanément sur les 32 feature maps d'entrée.
* Il effectue une convolution sur chaque feature map séparément, **mais** les résultats de ces convolutions sont **additionnés** pour former une seule feature map de sortie
    * Les 32 résultats des convolutions (un pour chaque feature map) sont additionnés pixel par pixel pour produire une seule feature map de sortie.
    * Le filtre peut également avoir un biais ajouté après cette somme, pour ajuster la valeur finale.
* En sortie il y a autant de features maps que de filtres



<!-- 
############################################################
## 
############################################################ 
-->

Question : Deep Learning - CNN with TensorFlow - Pouvez-vous définir les termes ``stride``, ``padding`` et ``kernel_size`` ?

Answer   : 

* **Kernel size** 
    * Résumé = Taille du filtre (ex. : 3x3 ou 5x5).
    * Le **kernel** (filtre) est une matrice appliquée sur l'image (ou sur les feature maps en entrée) pour extraire des caractéristiques comme des lignes verticales, des bords, des textures, etc.
    * La taille du kernel (**kernel_size**) c'est la taille de la matrice en question
    * La taille du kernel c'est la taille max du motif qu'on pourra reconnaitre
    * Si il y a 32 features en entrée, le filtre s'applique simultanément aux 32 features
    * Le filtre est donc un filtre 3D : x, y, z où z est le nombre features en entrée
    * Exemple : filtre 3x3 ou 5x5 
  
* **Stride** (Pas)
    * Résumé = Nombre de pixels par lequel le filtre se déplace.
    * Le **stride** détermine comment le filtre se déplace sur l'image d'entrée. 
    * Il spécifie le nombre de pixels dont le filtre se déplace à chaque pas.
    * Si le stride est de `1`, le filtre se déplace d'un pixel à la fois. Si le stride est de `2`, le filtre saute de deux pixels à chaque mouvement.
    * Un stride plus grand réduit la taille de la sortie (moins de positions sont évaluées), tandis qu'un stride plus petit conserve plus de détails.
    * Un stride de `(1, 1)` signifie que le filtre se déplace d'un pixel horizontalement et verticalement à chaque pas.
    * Typique 1 ou 2 pas plus

* **Padding** (Remplissage)
    * Résumé = Gestion des bords de l'image pour ajuster la taille de sortie (e.g. `same` ou `valid`). 
    * C'est l'ajout de pixels autour des bords de l'image d'entrée **avant** d'appliquer la convolution. 
    * Permet de contrôler la taille de la sortie.
    * Il existe principalement deux types de padding :
        * **Same** : Le padding est ajouté de manière à ce que la sortie ait la même dimension que l'entrée.
        * **Valid** : Aucune extension n'est effectuée, et la taille de la sortie diminue en fonction de la taille du filtre et du stride.
    * Le padding permet de conserver plus de détails aux bords de l'image. 
    * Sans padding (**valid**) on perd des pixels à chaque couche de convolution ce qui diminue progressivement la taille de l'image.


<!-- 
############################################################
## 
############################################################ 
-->

Question : Deep Learning - CNN with TensorFlow - Data augmentation? Ca vous dit quoi ?

Answer   : 

* Avec les réseaux de neurones convolutifs (CNN) une fois que les poids du filtre sont fixés, il retrouve bien le motif quelque soit l'endroit où il se trouve
* Par contre il faut que le motif ait toujours la même orientation
* Le fitre détecte des lignes verticales mais pas des lignes droites (qui peuvent être horizontales, en biais...) 
    * Invariance translation : oui
    * Invariance rotation : non
* Bref, il ya des soucis de rotation mais aussi de contraste, d'ombrage…
* On va faire de la rotation sur les images = **data augmentation**
* Idem pour contraste et luminosité


<!-- 
############################################################
## 
############################################################ 
-->

Question : Deep Learning - CNN with TensorFlow - Qu'est-ce qu'un réseau de neurones convolutif (CNN) ?

Answer   : 

* Un CNN est un type de réseau de neurones spécialement conçu pour traiter les données structurées sous forme de grille (images par exemple). 
* Il utilise des convolutions pour en extraire les caractéristiques principales


<!-- 
############################################################
## 
############################################################ 
-->

Question : Deep Learning - CNN with TensorFlow - Quels sont les principaux types de couches dans un CNN ?

Answer   : 

* **Couche convolutionnelle** (de convolution): applique des filtres sur l'image d'entrée pour détecter des caractéristiques locales comme des bords, des textures.
* **Couche de pooling** (de sous-échantillonnage) : réduit la dimensionnalité en conservant les informations les plus importantes.
* **Couche entièrement connectée** (dense) : connecte chaque neurone de la couche précédente à chaque neurone de la couche suivante. Utilisées en fin de réseau pour la classification.
* **Couche d'activation** : applique une fonction non linéaire (comme ReLU) pour ajouter de la complexité au modèle.
* **Dropout** : Active uniquement pendant l'entrainement. Eteint aléatoirement des neurones. 

<!-- 
############################################################
## 
############################################################ 
-->

Question : Deep Learning - CNN with TensorFlow - Qu'est-ce que la convolution et comment fonctionne-t-elle dans un CNN ?

Answer   : 

* La convolution est une opération mathématique où un filtre (ou noyau) de petite taille est appliqué à l'entrée (par exemple, une image). 
* Le filtre glisse sur l'image, et à chaque position, il calcule un produit scalaire entre le filtre et la région correspondante de l'image
* Ca produit une carte de caractéristiques. 
* La convolution permet d'extraire dans la feature maps des motifs locaux comme des bords ou des textures.


<!-- 
############################################################
## 
############################################################ 
-->

Question : Deep Learning - CNN with TensorFlow - Pourquoi utilise-t-on la fonction d'activation ReLU ?

Answer   : 

* La fonction ReLU (**Rectified Linear Unit**) remplace les valeurs négatives par zéro et conserve les valeurs positives. 
* Elle introduit de la non-linéarité dans le réseau, ce qui permet de modéliser des relations plus complexes. 
* ReLU est beaucoup plus simple à calculer que les autres fonctions d'activation telles que Sigmoïde ou Tanh (fonctions transcendantes)


<!-- 
############################################################
## 
############################################################ 
-->

Question : Deep Learning - CNN with TensorFlow - Quel est le rôle de la couche de pooling dans un CNN ?

Answer   : 

* La couche de pooling réduit la dimensionnalité de la carte de caractéristiques tout en conservant les informations les plus importantes
* Par exemple, max pooling ou average pooling (max pool prefered) 
* Permet 
    1. de rendre le modèle plus efficace
    1. de réduire le surapprentissage
    1. de rendre les caractéristiques extraites plus robustes aux variations dans les images (translation mais aussi un peu en rotation)


<!-- 
############################################################
## 
############################################################ 
-->

Question : Deep Learning - CNN with TensorFlow - Comment se fait la régularisation dans un CNN ?

Answer   : 

* Faut simplifier le modèle !   
* **Dropout** : Pendant l'entraînement. Consistant à **désactiver** aléatoirement des neurones (20% par exemple), réduisant ainsi le surapprentissage.
* **Régularisation L2** : ajoute une pénalité basée sur les poids du modèle pour encourager des poids plus petits.
* **Early stopping** : arrêt de l'entraînement lorsque la performance sur les données de **validation** commence à se dégrader.


<!-- 
############################################################
## 
############################################################ 
-->

Question : Deep Learning - CNN with TensorFlow - Qu'est-ce qu'une carte de caractéristiques (feature map) ?

Answer   : 

* Une carte de caractéristiques est le résultat de l'application d'un filtre (ou noyau) sur l'image d'entrée. 
* Elle contient les réponses locales détectées par le filtre à chaque position de l'image 
* Ell met en évidence des motifs spécifiques comme des bords, des lignes verticales, horizontales ou des textures.



<!-- 
############################################################
## 
############################################################ 
-->

Question : Deep Learning - CNN with TensorFlow - Quelle est la différence entre un CNN et un MLP (perceptron multicouche) ?

Answer   : 

* Un CNN exploite la structure spatiale des données (comme les images) grâce à la convolution et le pooling
* Un MLP connecte chaque neurone de chaque couche à tous les neurones de la couche suivante (dense). 
* Les CNN sont mieux adaptés aux données ayant une structure locale, comme les images
* Les MLP sont plus adaptés aux données vectorielles ou tabulaires.


<!-- 
############################################################
## 
############################################################ 
-->

Question : Deep Learning - CNN with TensorFlow - Comment choisir la taille d'un filtre dans un CNN ?

Answer   : 

* La taille du filtre est un hyperparamètre que l'on choisit souvent par expérimentation. 
* Des petits filtres (comme 3x3) sont souvent utilisés car ils capturent efficacement les motifs locaux tout en nécessitant moins de calculs. 
* Des filtres plus grands peuvent être utilisés pour capturer des motifs plus globaux, mais ils peuvent entraîner plus de surapprentissage et une augmentation du coût en calcul.


<!-- 
############################################################
## 
############################################################ 
-->

Question : Deep Learning - CNN with TensorFlow - Qu'est-ce que le "stride" dans un CNN et quel est son impact ?

Answer   : 

* Le stride est le nombre de pixels que le filtre déplace à chaque fois qu'il glisse sur l'image. 
* Un stride de 1 signifie que le filtre se déplace pixel par pixel, tandis qu'un stride de 2 signifie qu'il se déplace de deux pixels à chaque étape. 
* Un stride plus élevé réduit la taille de la carte de caractéristiques, augmentant ainsi l'efficacité du calcul, mais peut également perdre des détails.


<!-- 
############################################################
## 
############################################################ 
-->

Question : Deep Learning - CNN with TensorFlow - Quelle est l'importance de la régularisation dans les CNN et quels types de régularisation sont couramment utilisés ?

Answer   : 

* La régularisation aide à prévenir le surapprentissage (overfitting) en pénalisant les grands poids dans le réseau. 
* Les types de régularisation courants sont la régularisation L2 (weight decay), L1 (lasso), L1L2 (elastic), le dropout, et la normalisation par batch (batch normalization).


<!-- 
############################################################
## 
############################################################ 
-->

Question : Deep Learning - CNN with TensorFlow - Qu'est-ce que le "padding" dans le contexte des convolutions et pourquoi est-il utilisé ?

Answer   : 

* Le padding consiste à ajouter des pixels (généralement des zéros) autour de l'image d'entrée pour préserver la taille spatiale de la sortie après la convolution. 
* Cela permet également aux filtres de balayer toutes les parties de l'image d'entrée, incluant les bords.



<!-- 
############################################################
## 
############################################################ 
-->

Question : Deep Learning - CNN with TensorFlow - Peux-tu expliquer comment fonctionne une architecture populaire de CNN, comme AlexNet, VGGNet ou ResNet ?

Answer   : 

* **AlexNet** utilise 
    * des couches convolutives avec des tailles de filtres et des strides différents
    * suivies de couches de pooling 
    * et de plusieurs couches entièrement connectées. 
    * Il utilise également le dropout pour réduire le surapprentissage.

* **VGGNet** 
    * est connu pour sa simplicité avec l'utilisation de petites convolutions (3x3) 
    * des couches très profondes
    * lui permet d'extraire des caractéristiques complexes.

* ResNet (Residual Network) 
    * introduit les connexions résiduelles qui permettent d'ajouter directement l'entrée à la sortie de certaines couches
    * ça facilite l'apprentissage et permettant de construire des réseaux extrêmement profonds sans problème de dégradation.






































