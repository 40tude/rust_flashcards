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


Question : INTRO-ML - What is machine learning?
Answer  : The use and development of computer systems that are able to learn and adapt without following explicit instructions, by using algorithms and statistical models to analyse and draw inferences from patterns in data.

Question : INTRO-ML - What is supervised machine learning?
Answer  : Solving problems with solved examples.

Question : INTRO-ML - What are the two most common types of supervised machine learning problems?
Answer  : Classification and regression.

Question : INTRO-ML - How can a machine learning model be formally presented in regression problems?
Answer  : y = f(x) + epsilon

Question : INTRO-ML - What is the goal of a data scientist in supervised learning?
Answer  : Approximate the true function f(X).

Question : PREPROCESSING - What is the purpose of preprocessing in machine learning?
Answer  : To clean and transform structured data before training models

Question : PREPROCESSING - Which of the following is an example of structured data?
Answer  : Tabular data

Question : PREPROCESSING - What are the four main families of variables in structured data?
Answer  : Continuous, discrete, ordinal, and nominal

Question : PREPROCESSING - In supervised machine learning, what is the target variable?
Answer  : The variable being predicted

Question : PREPROCESSING - Which method is used to replace missing values with the mean of a quantitative variable?
Answer  : Mean imputation

Question : PREPROCESSING - How are categorical variables encoded when using One Hot Encoding?
Answer  : By creating a binary variable for each modality

Question : PREPROCESSING - What is the purpose of dropping columns in preprocessing?
Answer  : To exclude variables with unique identifiers

Question : PREPROCESSING - What does standardization of quantitative variables involve?
Answer  : Scaling variables to a specific range

Question : PREPROCESSING - Why is it important to encode categorical variables in machine learning?
Answer  : To make sure categorical variables are represented in numerical format

Question : SVM - What is the primary objective of Support Vector Machines (SVM)?
Answer  : To find a hyperplane that maximizes the margin between classes

Question : SVM - Which of the following represents a hyperplane?
Answer  : A vector subspace of dimension d-1 in the original space

Question : SVM - How do Support Vector Machines (SVM) handle non-linear data? (3 answers)
Answer  : By applying polynomial transformations to the data + By increasing the dimensionality of the observation space + By using radial kernel functions for mapping data

Question : SVM - What is the purpose of the C parameter in SVM models?
Answer  : To penalize errors on the training data

Question : SVM - Which parameter controls the influence area of each observation in a radial kernel SVM model?
Answer  : Gamma parameter

Question : SVM - What is the effect of increasing the C parameter in SVM models?
Answer  : Higher variance and lower bias models

Question : SVM - How does the gamma parameter influence the decision boundaries in SVM models?
Answer  : It defines the size of influence areas for observations

Question : SVM - What happens when the gamma parameter is too large in a radial kernel SVM model? (one or more correct answers)
Answer  : The model becomes highly sensitive to individual observations + The model is more likely to overfit and have high variance + The decision boundary fits the data too closely, leading to overfitting

Question : LIN-REG - What is the purpose of model evaluation criteria and variable selection methods?
Answer  : To determine the best set of explanatory variables for a model

Question : LIN-REG - What does SST (Sum of Square Total) represent in multiple linear regression?
Answer  : The total amount of information contained in the target variable

Question : LIN-REG - How is $R^{2}$ calculated in multiple linear regression?
Answer  : $$R^{2} = 1-\frac{SSR}{SST}$$ 

Question : LIN-REG - Which model selection method starts with a model using all explanatory variables and eliminates one variable at a time based on p-values? 
Answer  : Backward elimination

Question : LIN-REG - What is the main advantage of the stepwise model selection algorithm?
Answer  : It is computationally efficient for large numbers of explanatory variables.

Question : LIN-REG - True or False? R2 increases with each additional explanatory variable. 
Answer  : True

Question : LIN-REG - How does SSE (Sum of Square Explained) relate to the target variable in multiple linear regression?
Answer  : SSE measures the dispersion of the model's predictions compared to the average of the target variable

Question : LIN-REG - What is the purpose of Analysis of Variance (ANOVA) in model evaluation?
Answer  : To compare the performance of different models based on their F-statistics.

Question : LIN-REG - Which of the following is a valid use case for the model evaluation and selection methods discussed?
Answer  : All linear models and logistic regression models.

Question : REGUL - What is the purpose of regularization in linear regression?
Answer  : To reduce the importance of variables or remove irrelevant variables

Question : REGUL - Which regularization technique introduces a penalty term based on the squared magnitude of the coefficients?
Answer  : Ridge model

Question : REGUL - What is the purpose of the cost function in linear regression?
Answer  : To minimize the error between predictions and actual values

Question : REGUL - What does the bias-variance trade-off represent?
Answer  : The trade-off between bias and variance in model predictions + The trade-off between underfitting and overfitting

Question : REGUL - In the Ridge model, what happens to bias and variance as the penalty parameter (λ) increases?
Answer  : Bias increases, variance decreases

Question : REGUL - Why is the intercept term not penalized in the Ridge model?
Answer  : It represents the average level of the target variable

Question : REGUL - Which regularization technique is suitable for variable selection?
Answer  : Lasso model

Question : REGUL - What is the sparsity hypothesis in the context of the Lasso model?
Answer  : The assumption that only a few coefficients are non-zero

Question : REGUL - How does the Lasso model differ from the Ridge model?
Answer  : Lasso introduces a stronger penalty, leading to more sparse solutions compared to Ridge

Question : REGUL - What is the main drawback of overfitting in machine learning models?
Answer  : It increases the generalization error on unseen data

Question : CROSS-VALID - What is the purpose of cross-validation?
Answer  : To evaluate model performance on unknown data

Question : CROSS-VALID - What does overfitting mean?
Answer  : The model performs well on known data and poorly on unknow data

Question : CROSS-VALID - What is k-fold cross-validation?
Answer  : Splitting data into k equal groups and evaluating the model k times

Question : CROSS-VALID - Which situation indicates that a model is underfitting?
Answer  : score(train) ~ score(test) ~ 0

Question : CROSS-VALID - What is hyperparameter optimization?
Answer  : Choosing optimal hyperparameter values for a learning algorithm

Question : CROSS-VALID - What is grid search?
Answer  : Searching for optimal hyperparameters through an exhaustive search

Question : CROSS-VALID - Why is cross-validated grid search time-consuming?
Answer  : Because it requires training the model multiple times

Question : CROSS-VALID - What is the purpose of hyperparameter tuning?
Answer  : To choose the best hyperparameter values for a learning algorithm

Question : LOG-REG - What is the purpose of logistic regression?
Answer  : To predict a category or probability distribution for a binary classification problem.

Question : LOG-REG - What function is used to estimate the relationship between the features and the target variable in logistic regression?
Answer  : Sigmoid function

Question : LOG-REG - What is the purpose of cross-validation in logistic regression?
Answer  : To detect overfitting of the model.

Question : LOG-REG - Which of the following is a metric used for performance assessment in classification?
Answer  : Accuracy

Question : LOG-REG - What is the purpose of a confusion matrix in evaluating a classification model?
Answer  : To analyze the number of true positives, true negatives, false positives, and false negatives.

Question : LOG-REG - What does the ROC curve represent? 
Answer  : The performance of the model in detecting positive observations across all possible thresholds.
Cette courbe montre le compromis entre le True Positive Rate (Recall, TP/(TP+FN), ligne du bas de la matrice de confusion, proportion de vrais positifs détectés par rapport à tous les positifs réels, taux élevé préférable) en ordonnée et le False Positive Rate (FP/(FP+TN, ligne haut de la matrice de confusion, proportion de faux positifs par rapport à tous les négatifs réels, se sont de fausses alarmes, taux bas préférable) en abscisse pour différents seuils de décision. 
Le seuil passe de 1 à 0 quand on va de la gauche vers la droite. 
Seuil = 1 => On classe tout en 0. Pas de vrais positif détecté (TPR=0)          aucun faux négatif (FPR=0) car tout est classé négatif
Seuil = 0 => On classe tout en 1. Tous les vrais positifs sont détectés (TPR=1) mais tous les négatifs réel sont mal classés en 1 (FPR=1)
Quand on passe le seuil de 1 à 0 (de gche à dte), le TPR (y) augmente ainsi que le FPR (x)

Question : LOG-REG - What does the AUC (Area Under the Curve) measure?
Answer  : C'est un indicateur global, une métrique unique qui résume la performance du modèle sur l'ensemble des seuils possibles. 
Un AUC plus élevé indique que le modèle a une bonne capacité à différencier les classes positives et négatives
Comme indicateur global l'AUC est utile pour comparer plusieurs modèles de classification sur un même jeu de données. 
Un AUC de 0.5 indique que le modèle n'est pas meilleur qu'un tirage au sort aléatoire. 
Un AUC de 1.0 indique une séparation parfaite entre les classes 0 et 1.

Question : Pourquoi la Courbe ROC est nécessaire en plus de l'AUC
Answer  : 3 points
1 - L'AUC n'indique pas comment la performance change avec différents seuils. Un modèle peut avoir une AUC élevée, mais cela peut masquer des performances médiocres à des seuils spécifiques
2 - Choix du seuil : La courbe ROC permet de choisir un seuil en fonction des priorités (maximiser TPR, minimiser FPR, etc.). Crucial quand le coût d'une erreur est important (médecine, où les faux négatifs sont plus coûteux que les faux positifs).
3 - Interprétation du modèle : la courbe ROC permet d'identifier les points où le modèle a un comportement instable ou des performances contre-intuitives

Question : LOG-REG - What is the purpose of the GINI coefficient? 
Answer  : To quantify the inequalities in a population.

Question : LOG-REG - What should be done with models having an AUC of less than 0.5?
Answer  : Exclude them immediately.

Question : DEC-TREE - What are the two types of problems addressed by Decision Trees?
Answer  : Regression and classification

Question : DEC-TREE - What are the three types of elements in a decision tree?
Answer  : Root, branches, and leaves

Question : DEC-TREE - What is the purpose of the division criterion in decision tree construction?
Answer  : To select the best division for minimizing heterogeneity

Question : DEC-TREE - Which heterogeneity function is used for qualitative variables in decision trees?
Answer  : Gini concentration

Question : DEC-TREE - What is one of the stopping criteria in decision tree construction?
Answer  : Maximum depth of the tree

Question : DEC-TREE - Which parameter can we manipulate to reduce overfitting in a decision tree?
Answer  : Maximum tree depth

Question : DEC-TREE - Which hyperparameters can be adjusted to control the bias/variance trade-off?
Answer  : Minimum samples per leaf and maximum depth

Question : DEC-TREE - What is one advantage of using dummy variables for qualitative variables in decision trees?
Answer  : Enhanced variable selection capability

Question : DEC-TREE - How does the hierarchical structure of decision trees affect error propagation? 
Answer  : Errors are propagated to all child nodes

Question : DEC-TREE - What is a characteristic of the result of a decision tree in regression?
Answer  : It generates stepped function predictions

Question : RND-FOREST - What is the purpose of Random Forest?
Answer  : To reduce overfitting and improve algorithm robustness

Question : RND-FOREST - What does bagging in Random Forest stand for?
Answer  : Bootstrap aggregating

Question : RND-FOREST - What is bootstrapping in Random Forest?
Answer  : A process of artificially increasing the number of observations in a dataset

Question : RND-FOREST - How are multiple models aggregated in Random Forest?
Answer  : By averaging their predictions

Question : RND-FOREST - What is the main advantage of the Random Forest algorithm?
Answer  : It helps reduce the variance of individual decision trees

Question : RND-FOREST - How does Random Forest introduce randomness? (there are 2 answers here)
Answer  : By randomly selecting observations for each tree + By randomly selecting explanatory variables for each tree

Question : RND-FOREST - What does Mean Decrease Accuracy measure in feature importance?
Answer  : The difference in accuracy between pre-switching and post-switching validation errors

Question : RND-FOREST - How is variable importance calculated in decision trees?
Answer  : By summing the decreases in heterogeneity at each node where the variable is used

Question : RND-FOREST - What advantage does variable importance in Random Forests offer?
Answer  : It captures non-linear dependencies between variables and the target

Question : RND-FOREST - How is variable importance calculated in Random Forests?
Answer  : By averaging the variable importances calculated for each tree

Question : SVM - What is the primary objective of Support Vector Machines (SVM)?
Answer  : To find a hyperplane that maximizes the margin between classes

Question : SVM - Which of the following represents a hyperplane?
Answer  : A vector subspace of dimension d-1 in the original space

Question : SVM - How do Support Vector Machines (SVM) handle non-linear data? (3 answers)
Answer  : By applying polynomial transformations to the data + By increasing the dimensionality of the observation space + By using radial kernel functions for mapping data

Question : SVM - What is the purpose of the C parameter in SVM models?
Answer  : To penalize errors on the training data

Question : SVM - Which parameter controls the influence area of each observation in a radial kernel SVM model?
Answer  : Gamma parameter

Question : SVM - What is the effect of increasing the C parameter in SVM models?
Answer  : Higher variance and lower bias models

Question : SVM - How does the gamma parameter influence the decision boundaries in SVM models?
Answer  : It defines the size of influence areas for observations

Question : SVM - What happens when the gamma parameter is too large in a radial kernel SVM model? (one or more correct answers)
Answer  : The model becomes highly sensitive to individual observations + The model is more likely to overfit and have high variance + The decision boundary fits the data too closely, leading to overfitting

Question : BOOST-ADABOOST - Boosting?
Answer  : Surtout en classification mais il existe AdaBoostRegressor() en sklearn. Une méthode d'ensemble qui combine plusieurs modèles faibles pour créer un modèle fort. L'idée est de corriger les erreurs des modèles précédents en se concentrant sur les observations mal classifiées ou mal prédites. ADABoost se concentre particulièrement sur l'ajustement des poids des observations en fonction des erreurs de prédiction.
Étapes principales du Boosting :
	• Initialisation : Commencer avec un modèle faible (par exemple, un arbre de décision simple).
	• Itération : À chaque étape, ajuster les poids des observations en fonction de leurs erreurs, de sorte que les observations mal prédites aient plus d'importance dans le modèle suivant.
	• Combinaison : Les modèles sont combinés selon une pondération basée sur leurs performances pour obtenir le modèle final.
Code AdaBoost:
	base_estimator = DecisionTreeClassifier(max_depth=1)
	adaboost = AdaBoostClassifier(base_estimator=base_estimator, n_estimators=50, learning_rate=1.0, random_state=42)
	adaboost.fit(X_train, y_train)
	y_pred = adaboost.predict(X_test)
	accuracy = accuracy_score(y_test, y_pred)

Question : BOOST-ADABOOST - What is the main goal of boosting algorithms?
Answer  : To improve the performance of a simple supervised Machine Learning model.

Question : BOOST-ADABOOST - How does the AdaBoost algorithm assign weights to observations?
Answer  : Equal weights at initialisation then inversely proportional to the prediction correctness.

Question : BOOST-ADABOOST - How are models agregated at the end of boosting?
Answer  : Predictions are a weigthed average of all model's predictions inversely proportional to models' error.

Question : BOOST-ADABOOST - What is the weak learning condition in AdaBoost?
Answer  : The model must perform better than random chance.

Question : BOOST-ADABOOST - What is a potential drawback of AdaBoost?
Answer  : It may overfit the training data if too many iterations are performed.

Question : BOOST-ADABOOST - What is the margin in AdaBoost?
Answer  : The sum of weights of models that predict the obervation well minus the wieghts of models that don't.

Question : BOOST-ADABOOST - How does the margin affect the generalization error in AdaBoost?
Answer  : Higher margins lead to lower generalization error.

Question : BOOST-ADABOOST - What is the main difference between AdaBoost and gradient boosting?
Answer  : Adaboost updates observation weights to adapt models, while gradient boosting tries to predict the residuals of the previous model.

Question : VOTING-STACKING - What is "Voting classifier and stacking" all about?
Answer  : Voting / Averaging

Question : VOTING-STACKING - In ensemble learning, when is the voting classifier used?
Answer  : In classification problems with qualitative target variables.

Question : VOTING-STACKING - What is the mathematical representation of the voting classifier's prediction?
Answer  : argmax (S1 * (y_hat1 = m) + S2 * (y_hat2 = m) S3 * (y_hat3 = m))

Question : VOTING-STACKING - In ensemble learning, when is averaging used? 
Answer  : In regression problems with quantitative target variables.

Question : VOTING-STACKING - What is the mathematical representation of the averaging prediction?
Answer  : 1/(S1+S2+S3) * (S1 * y_hat1 + S2 * y_hat2 S3 * y_hat3)

Question : VOTING-STACKING - What is the purpose of stacking in ensemble learning?
Answer  : To increase the stability of models and improve their performance.

Question : VOTING-STACKING - How is the training dataset divided in stacking?
Answer  : It is divided into two parts (part 1 and part 2).

Question : VOTING-STACKING - What does the verbose parameter control in the stacking function?
Answer  : The level of information returned when the command is executed.

Question : MODEL-EVAL - Is "Optimize the computational performance of models" a common goal in machine learning?
Answer  : No

Question : MODEL-EVAL - What does the term "i.i.d." stand for in the context of data?
Answer  : Independent and Identically Distributed

Question : MODEL-EVAL - What is the evaluation metric commonly used for classification problems?
Answer  : Accuracy

Question : MODEL-EVAL - What does the bias of a model represent? 
Answer  : The deviation of the model's prediction from the true mean of the target variable

Question : MODEL-EVAL - How do we call the validation method consisting in splitting the data between a training and test set?
Answer  : Holdout validation

Question : MODEL-EVAL - What is the main flaw of the holdout method in evaluating a model's performance?
Answer  : It pessimistically biases the the model evaluation

Question : MODEL-EVAL - How can confidence intervals be calculated for the accuracy of a model?
Answer  : Using the Normal approximation formula for proportions

Question : MODEL-EVAL - What are the three components of a model's error?
Answer  : Bias, variance, and noise

Question : MODEL-EVAL - What does a low variance in a model indicate?
Answer  : The model's outputs vary slowly with the model's input

Question : TIME-SERIES - Which of the following is the definition of a time series?
Answer  : An ordered sequence of values of a variable at equally spaced time intervals

Question : TIME-SERIES - What is the primary difference between time series analysis and classic machine learning methods?
Answer  : Time series analysis accounts for autocorrelation, trend, and seasonal variation

Question : TIME-SERIES - Which of the following is NOT an application of time series analysis?
Answer  : Image recognition (Economic forecasting, Stock market analysis, Yield projections)

Question : TIME-SERIES - Which method is used for reducing or canceling the effect of random variation in time series data?
Answer  : Exponential smoothing, Single moving average, Triple exponential smoothing, Double exponential smoothing

Question : TIME-SERIES - What is the purpose of averaging techniques in time series analysis?
Answer  : To estimate the general trend of the series

Question : TIME-SERIES - Which method calculates the mean of successive smaller sets of past data?
Answer  : Single moving average

Question : TIME-SERIES - What is the main drawback of moving averages in time series analysis?
Answer  : Moving averages are always lagged compared to the actual time series

Question : TIME-SERIES - What does exponential smoothing assign exponentially decreasing weights to? 
Answer  : Older observations

Question : TIME-SERIES - What is the purpose of the triple exponential smoothing method?
Answer  : To forecast future values in a time series with trend and seasonality

Question : TIME-SERIES - What are the three equations associated with triple exponential smoothing (Holt-Winters method)?
Answer  : Overall smoothing, trend smoothing, and seasonal smoothing

