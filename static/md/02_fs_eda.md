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

<!-- https://app.jedha.co/course/numpy-linear-algebra-functions-ft/01-numpy-linear-algebra-quiz -->

Question : EDA - Numpy and linear algebra - What is the result of matrix multiplication in NumPy?
Answer  : 

A new matrix where each element is the dot product of rows and columns



Question : EDA - Numpy and linear algebra - How are eigenvalues and eigenvectors of a matrix in NumPy computed?
Answer  : 

#### Code snippet 

```python
...
cov_mat = X.T.dot(X)
eig_val, eig_vec = np.linalg.eig(cov_mat)
eig_val, eig_vec

```



Question : EDA - Numpy and linear algebra - What does the "@" symbol represent in NumPy in linear algebra context?
Answer  : 

Matrix multiplication

#### Code snippet 

```python
A = np.random.randint(1,10,size = (3,3))
B = np.random.randint(1,10,size = (3,2))
C = A@B

```



Question : EDA - Numpy and linear algebra - What is slicing in NumPy?
Answer  : 

A way to access a portion of an array or matrix

#### Code snippet 

```python
a[start:stop]      # items start through stop-1
a[start:]          # items start through the rest of the array
a[:stop]           # items from the beginning through stop-1
a[:]               # a copy of the whole array
a[start:stop:step] # start through not past stop, by step
a[-1]              # last item in the array
a[-2:]             # last two items in the array
a[:-2]             # everything except the last two items
a[::-1]            # all items in the array, reversed
a[1::-1]           # the first two items, reversed
a[:-3:-1]          # the last two items, reversed
a[-3::-1]          # everything except the last two items, reversed

tableau_1D = [10, 20, 30, 40, 50, 60, 70, 80, 90]
sous_tableau_1 = tableau_1D[2:5]  # [30, 40, 50]
sous_tableau_2 = tableau_1D[::2]  # [10, 30, 50, 70, 90]

matrice = [
    [1, 2, 3],
    [4, 5, 6],
    [7, 8, 9]
]

sous_matrice_1 = [ligne[:2] for ligne in matrice[:2]]  # [[1, 2], [4, 5]]
sous_matrice_2 = [ligne[1] for ligne in matrice]       # [2, 5, 8]

```



<!-- 
############################################################
## 
############################################################ 
-->
Question : EDA - Numpy and linear algebra - Quelle est l'importance de l'analyse exploratoire des données (EDA) dans le processus de modélisation en science des données ?
Answer : 

* L'analyse exploratoire des données permet de comprendre la structure, les modèles et les relations présentes dans les données avant de construire des modèles. 
* Cela inclut l'examen 
    * des distributions
    * des corrélations
    * des valeurs manquantes
    * des outliers
* C'est ce qui guide les décisions sur le nettoyage des données et le choix des modèles. 
* Penser à parler aussi des distributions qui sont **skewed** d'un côté ou de l'autre (1.5 IQR vs 3 sigma pour virer les outliers). 
* Parler aussi de l'importance du déséquilibre dans la target.







<!-- https://app.jedha.co/course/numpy-linear-algebra-functions-ft/02-numpy-and-functions-quiz -->

Question : EDA - Numpy and functions - Which NumPy function is used to compute the square root of each element in an array?
Answer   : 

#### Code snippet 

```python
np.sqrt()
```


Question : EDA - Numpy and functions - How do you generate a sample of random values from a standard normal distribution in NumPy?
Answer   : 

#### Code snippet 

```python
np.random.standard_normal()
```

Question : EDA - Numpy and functions - What is the result of np.log(np.exp(1)) in NumPy?
Answer   : 

#### Code snippet 

```python
1 = np.log(np.exp(1))
```

Question : EDA - Numpy and functions - Which function is used to compute the trigonometric sine for each element of an array?
Answer   : 

#### Code snippet 

```python
np.sin()
```


Question : EDA - Numpy and functions - What does ``np.random.seed(0)`` do in NumPy?
Answer   : 

* Initializes the random number generator with seed 0, ensuring reproducibility
* Everybody know that instead you should do `np.random.seed(42)`








<!-- https://app.jedha.co/course/numpy-linear-algebra-functions-ft/03-manipulate-arrays-with-numpy-quiz -->



Question : EDA - Manipulate arrays with Numpy - How do you create an array of zeros in NumPy?
Answer  : 

#### Code snippet 

```python
mat_a = np.zeros([2, 2], dtype = int)
mat_a
```



Question : EDA - Manipulate arrays with Numpy - How do you change the shape of an existing NumPy array?
Answer  : 

#### Code snippet 

```python
vec = np.arange(0, 100)
vec2 = vec.reshape(4, 5, 5)
vec2
reshape()
```



Question : EDA - Manipulate arrays with Numpy - What is a NumPy mask?
Answer  : 

An array of boolean values indicating where a condition is met

#### Code snippet 

```python
array = np.array([15, 25, 35, 45, 55, 65, 75, 85, 95])
mask = array > 50
filtered_array = array[mask]
print(filtered_array)

array[array <= 45] = -1
print(array)
```













<!-- https://app.jedha.co/course/pandas-statistics-ft/01-stats-basics-quiz -->

Question : EDA - STAT101 - What is an example of discrete quantitative data?
Answer  : 

Nb people in a room




Question : EDA - STAT101 - Which measure of central tendency is the most affected by outliers?
Answer  : Mean

<!-- TODO: Add code or picture -->



Question : EDA - STAT101 - What does a Z-score measure?
Answer  : 
 
* the deviation of a value from the mean 
* measured in standard deviation

Let's go back to the formula... 

* If ``Value`` and ``µ`` are speeds
* The difference between ``Value`` and ``µ`` is also in km/h 
* So it is not easy to know if it's significant or not
    * 10 km/h difference when driving at 130 km/h vs 1_000 km/h difference when running at ½ the speed of light
* But the standard deviation is also in km/h
* So, when you divide by the standard deviation there's no unit 
* Now, we know how far we are from the mean "standard deviations wise" (3 standard deviations, 0.2 standard deviation ...). 




Question : STAT101 - Which sampling method should be avoided due to potential bias?
Answer  : 

Convenience Sampling (vs Random Sampling Stratified Sampling  Cluster Sampling)





<!-- https://app.jedha.co/course/pandas-statistics-ft/data-manipulation-with-pandas-quiz -->

Question : EDA - PANDAS - How do you read a CSV file using Pandas?
Answer  : 

#### Code snippet 

```python
from pathlib import Path
k_Current_dir = Path.cwd()
k_AssetsDir   = "assets"                       
k_Filename    = "cities.csv"                   
df_cities = pd.read_csv(k_Current_dir/k.AssetsDir/k.Filename, nrows = 2)
```



Question : EDA - PANDAS - What are the two primary data structures in Pandas?
Answer  : 

DataFrames and Series

#### Code snippet 

```python
data_series = pd.Series([10, 20, 30, 40, 50], index=['a', 'b', 'c', 'd', 'e'])
print(data_series)

data_dict = {
    'Name': ['Alice', 'Bob', 'Charlie', 'David', 'Eva'],
    'Age': [25, 30, 35, 40, 45],
    'City': ['New York', 'Los Angeles', 'Chicago', 'Houston', 'Phoenix']
}
dataframe = pd.DataFrame(data_dict)
print(dataframe)
```



Question : EDA - PANDAS - In Pandas, how can you handle missing data?
Answer  : 

By replacing missing values with mean or median

#### Code snippet 

```python
df = pd.DataFrame({
  'A': [1, None, 4], 
  'B': [4, 5, 6]
})
median = df["A"].median()  
df["A"].fillna(median, inplace=True)
```



Question : EDA - PANDAS - What does the "merge" function do in Pandas?
Answer  : 

It combines data from two different DataFrames

#### Code snippet 

```python
result_merge = pd.merge(df1, df2, on='Id')
```






<!-- https://app.jedha.co/course/pandas-statistics-ft/dealing-with-datetimes-and-strings-quiz -->




Question : EDA - PANDAS datetime and strings - How do you convert a string to a datetime object in Pandas?
Answer  : 

#### Code snippet 

```python
my_datetime   = pd.to_datetime() 
your_datetime = pd.to_datetime(df['dates'], format='%Y-%m-%d')
…
```



Question : EDA - PANDAS datetime and strings - What is the purpose of the .dt accessor in Pandas?
Answer  : 

To access datetime properties and methods on Series

#### Code snippet 

```python
dates = pd.Series(pd.date_range("2024-01-01", periods=5, freq="12D)) 
date.dt.day
```



Question : EDA - PANDAS datetime and strings - How can you extract the year from a datetime object in Pandas?
Answer  : 

#### Code snippet 

```python
datetime_object.year
```



Question : EDA - PANDAS datetime and strings - What does the Timedelta type represent in Pandas?
Answer  : 

The difference between two datetime values

#### Code snippet 

```python
time_delta = pd.Timedelta(days=5, hours=4, minutes=30)
start_date = pd.Timestamp('2023-10-01 12:00:00')
new_date = start_date + time_delta

date_series = pd.Series([
    pd.Timestamp('2023-10-01 09:00:00'),
    pd.Timestamp('2023-10-02 10:30:00'),
    pd.Timestamp('2023-10-03 12:00:00')
])

delta = pd.Timedelta(days=1, hours=2)
new_dates_series = date_series + delta
print(new_dates_series)
```













<!--  https://app.jedha.co/course/introduction-data-visualization-ft/01-introduction-to-data-visualization-quiz -->

Question : EDA - Data Visualization - What is the primary purpose of data visualization?
Answer  : 

To make complex data easy to understand

Question : EDA - Data Visualization - Which of the following is a key principle in creating effective data visualizations?
Answer  : 

Keeping a consistent style and scaling

Question : EDA - Data Visualization - How does color affect data visualization?
Answer  : 

Color can help highlight key information

Question : EDA - Data Visualization - What does a good data visualization aim to achieve? 
Answer  : 

Convey complex information in an easy-to-understand format

Question : EDA - Data Visualization - What is an important aspect to consider when choosing a color scale for your visualization?
Answer  : 

How the colors can accurately represent and differentiate the data




<!-- https://app.jedha.co/course/introduction-data-visualization-ft/different-charts-plotly-express-quiz -->

Question : EDA - Different charts and plotly express - What is Plotly Express primarily used for?
Answer  : 

Generating a variety of static and interactive charts

Question : EDA - Different charts and plotly express - How do you display a chart in a Jupyter notebook using Plotly Express?
Answer  : 

Using the ``fig.show()`` method after creating the chart

Question : EDA - Different charts and plotly express - What type of data visualization can be created with Plotly Express?
Answer  : 

A wide range of chart types, including line, bar, scatter, and more

Question : EDA - Different charts and plotly express - Is it possible to customize the appearance of charts in Plotly Express?
Answer  : 

Yes, including aspects like color, layout, and annotations

Question : EDA - Different charts and plotly express - What is a primary advantage of using Plotly Express for data visualization?
Answer  : 

It offers a simple syntax for quickly creating a variety of charts




<!-- https://app.jedha.co/course/interactive-graphs-ft/quiz-fs-m03-v1 -->

Question : EDA - Interactive Graphs - What is pandas and its purpose?
Answer   : 

It's a python library that enables you to conduct exploratory data analysis, analyse variables' distributions, check for missing values etc.

Question : EDA - Interactive Graphs - What are the two main object types from pandas?
Answer   : 

``pandas.Series`` and ``pandas.DataFrame``



Question : EDA - Interactive Graphs - What method would you use if you had to quickly compute usual descriptive statistics on the DataFrame "df" taking into account both numerical and non-numerical features?

Answer   :

#### Code snippet 

```python
df.describe()
```



Question : EDA - Interactive Graphs - What is numpy, and what's its purpose?
Answer   : 

It's a python library that let's you compute mathematical operations in a quick and easy manner




Question : EDA - Interactive Graphs - What is the main object type in numpy?
Answer   :

#### Code snippet 

```python
numpy.data
numpy.vector
numpy.array
numpy.matrix
```



Question : EDA - Interactive Graphs - It's possible to easily convert pandas.DataFrame objects into numpy objects, what attributes of the pandas class  can be used for this?

Answer   :

#### Code snippet 

```python
numpy.data
numpy.vector
numpy.array
numpy.matrix
```



Question : EDA - Interactive Graphs - What is the seaborn method you would use to plot the distribution of a variable?
Answer   : 

`displot()` but it is deprecated. Use this instead.

#### Code snippet 

```python
fig, ax = plt.subplots(figsize=(8,5))
sns.histplot(tips['total_bill'], kde=True, stat="density", kde_kws=dict(cut=3), alpha=.4, edgecolor=(1, 1, 1, .4)) 
ax.set_title("Total Bill Distribution")
ax.set_xlabel("Total Bill ($)")
```




Question : EDA - Interactive Graphs - What is the purpose of using this code fragment before running some visualization code?

#### Code snippet 

```python
import matplotlib.pyplot as plt
fig, ax = plt.subplots(nrow=3,ncols=3)
```

Answer   : 

It displays 9 figures in the same output, organized on 3 rows and 3 columns







Question : EDA - Interactive Graphs - What is the module that lets you make easy interactive graph in plotly and which one is the more complicated version of the module?
Answer   :

* easy plotly.express 
* hard plotly.graph_objects




Question : EDA - Interactive Graphs - What is the plotly method that lets you visualize your data points as dots in a two-dimensional space?
Answer   : 

`px.scatter()`






