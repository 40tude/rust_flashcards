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



<!-- https://app.jedha.co/course/distributed-computing-rdd-ft/01-intro-to-big-data-quiz -->

Question : BIG-DATA - Introduction to Big Data - What are the three main characteristics of big data according to Gartner's 3Vs?
Answer  : Volume, variety, velocity

Question : BIG-DATA - Introduction to Big Data - What was the purpose of Google's MapReduce?
Answer  : Solving problems on large datasets using the MapReduce framework

Question : BIG-DATA - Introduction to Big Data - What are the requirements for distributed file systems?
Answer  : Schemaless, durability, handling component failure, automatic rebalancing

Question : BIG-DATA - Introduction to Big Data - What is the main difference between vertical scaling and horizontal scaling?
Answer  : Vertical scaling is limited by Moore's Law, while horizontal scaling allows the usage of commodity hardware.

Question : BIG-DATA - Introduction to Big Data - When do we need distributed processing?
Answer  : When data won't fit in the memory of a single machine and computing can be parallelized

Question : BIG-DATA - Introduction to Big Data - Why is distributed computing considered hard?
Answer  : It involves managing failures and ensuring fault tolerance

Question : BIG-DATA - Introduction to Big Data - What are the three steps in the MapReduce process?
Answer  : Map, Shuffle, Reduce

Question : BIG-DATA - Introduction to Big Data - What is Apache Hadoop?
Answer  : An open-source implementation of the MapReduce paradigm











<!-- https://app.jedha.co/course/distributed-computing-rdd-ft/02-intro-to-spark-quiz -->


Question : BIG-DATA - Introduction to Spark - Which of the following is an advantage of Apache Spark over other distributed computing frameworks (Hadoop) ?
Answer  : 

Faster through In-Memory computation + Simpler (high-level APIs) and eager execution




Question : BIG-DATA - Introduction to Spark - What is Apache Spark primarily written in?
Answer  : 

Scala

Question : BIG-DATA - Introduction to Spark - Transformations vs Actions
Answer  : 

* Une **transformation** retourne un RDD (Resilient Distributed Dataset) ou un DataFrame (mais de manière paresseuse)
* Une **action** ne retourne pas un RDD (respectivement un DataFrame), mais force l'exécution des transformations pour produire un résultat final.

Exemples : 
* De transformations : map(), filter(), flatMap(), union()
* D'actions          : collect(), count(), saveAsTextFile(), reduce(), show()



Question : BIG-DATA - Introduction to Spark - Spark DataFrame vs Pandas DataFrame ?
Answer : 

Dans les 2 cas on parle de données tabulaires (!=RDD qui peut accepter du texte). Pandas pour des analyses de données locales sur des jeux de données qui tiennent en mémoire, avec des opérations rapides et un code simple. Spark lorsque vous travaillez avec des volumes de données massifs qui nécessitent un traitement distribué, ou lorsque vous avez besoin d'exécuter des tâches de traitement des données sur un cluster pour des performances optimisées.

```python
# ----------------------------------
import pandas as pd

# Création d'un DataFrame Pandas
data = {"Name": ["Alice", "Bob", "Charlie"], "Age": [25, 30, 35]}
df_pandas = pd.DataFrame(data)

# Filtrer les données
filtered_df_pandas = df_pandas[df_pandas["Age"] > 30]
print(filtered_df_pandas)


# ----------------------------------
from pyspark.sql import SparkSession

# Initialisation de SparkSession
spark = SparkSession.builder.appName("DataFrame Example").getOrCreate()

# Création d'un DataFrame Spark
data = [("Alice", 25), ("Bob", 30), ("Charlie", 35)]
columns = ["Name", "Age"]
df_spark = spark.createDataFrame(data, schema=columns)

# Filtrer les données
filtered_df_spark = df_spark.filter(df_spark.Age > 30)
filtered_df_spark.show()
```

Questions : BIG-DATA - Introduction to Spark - Que pouvez-vous dire à propos de l'optimisation des DataFrames Spark?
Answer   : 
	• Ils bénéficient d'un optimiseur de requêtes avancé appelé Catalyst, qui génère un plan d'exécution optimisé pour les transformations et les actions appliquées. 
	• Utilisent également le moteur Tungsten, qui optimise l'utilisation de la mémoire et du CPU. 
	• Les opérations sont généralement plus lentes sur de petits jeux de données par rapport à Pandas en raison de la surcharge de gestion de la distribution, mais elles sont bien plus efficaces sur de gros volumes de données.

Question : BIG-DATA - Introduction to Spark - What does PySpark refer to?
Answer  : 

Spark's Python API

Question : BIG-DATA - Introduction to Spark - Which type of operations in Spark are stored in the execution plan but not immediately executed?
Answer  : 

Transformations

Question : BIG-DATA - Introduction to Spark - Which type of operation in Spark trigger the execution of transformations?
Answer  : 

Actions

Question : BIG-DATA - Introduction to Spark - What does lazy execution mean in the context of Spark?
Answer  : 

Spark delays the execution of transformations until an action is called

Question : BIG-DATA - Introduction to Spark - What is one of the challenges of debugging PySpark?
Answer  : 

Lazy evaluation can be difficult to debug + Debugging distributed systems is hard + Debugging mixed languages is hard


<!-- 
############################################################
## 
############################################################ 
-->
Question : BIG-DATA - Introduction to Spark - What is Spark SQL ? 
Answer  : 

* It brings native support for SQL to Spark. 
* It streamlines the process of querying data stored both in RDDs ((Resilient Distributed Dataset) , Spark’s distributed datasets) and in external sources. 
* Spark SQL blurs the lines between RDDs and relational tables. 
* Unifying these abstractions makes it easy for developers to intermix SQL commands querying external data with complex analytics, all within in a single application.








<!-- https://app.jedha.co/course/distributed-computing-rdd-ft/03-intro-to-pyspark-quiz -->

Question : BIG-DATA - Introduction to Spark - What is the primary access point to the Spark framework that allows you to use RDDs (Resilient Distributed Dataset) ?
Answer  : 

Spark Context


Question : BIG-DATA - Introduction to PySpark -  What is an RDD (Resilient Distributed Dataset) in Spark?
Answer  : 

Un RDD est l'abstraction de base de Spark. Il représente une collection distribuée d'objets immutables, répartie à travers les nœuds d'un cluster.  



Question : BIG-DATA - Introduction to Spark - Which method is used to create an RDD (Resilient Distributed Dataset) by parallelizing an existing collection?
Answer  : 

`sc.parallelize(...)`



Question : BIG-DATA - Introduction to Spark - What is the purpose of lazy evaluation in Spark?
Answer  : 

Spark executes all applied transformations when an action is called



Question : BIG-DATA - Introduction to Spark - Which action is used to retrieve the first few elements of an RDD (Resilient Distributed Dataset) ?
Answer  : 

`.take(num)`



Question : BIG-DATA - Introduction to Spark - What type of operations in Spark trigger the execution of transformations?
Answer  : 

Actions



Question : BIG-DATA - Introduction to Spark - Which language is Apache Spark primarily written in? 
Answer  : 

Scala



Question : BIG-DATA - Introduction to Spark - What does PySpark refer to?
Answer  : 

Spark's Python API



Question : BIG-DATA - Introduction to Spark - Which component is the slowest when comparing CPU, Memory, Disk, and Network?
Answer  : 

Network


Question : BIG-DATA - Introduction to Spark - What is one of the challenges of debugging PySpark?
Answer  : 

* Debugging distributed systems is hard
* Debugging mixed languages is hard
* Lazy evaluation can be difficult to debug




<!-- Question : SPARK - What is a DataFrame in Spark?
Answer  : A distributed collection of data grouped into named columns. A DataFrame is equivalent to a relational table in SQL. Ils ne sont PAS schema-less. Tabulares.spark.createDataFrame(spark.rdd ou pandas.dataframe) -->





<!-- https://app.jedha.co/course/distributed-computing-rdd-ft/03-intro-to-pyspark-quiz -->

Question : BIG-DATA - Introduction to PySpark - What is the primary access point to the Spark framework that allows you to use RDDs (resilient distributed dataset)?
Answer  : Spark Context


Question : BIG-DATA - Introduction to PySpark - What is an RDD (Resilient Distributed Dataset) in Apache Spark?
Answer  : A collection of objects


Question : BIG-DATA - Introduction to PySpark - Which method is used to create an RDD by parallelizing an existing collection?
Answer  : 

``sc.parallelize(...)``


Question : BIG-DATA - Introduction to PySpark - What is the purpose of lazy evaluation in Spark?
Answer  : Spark executes all applied transformations when an action is called

Question : BIG-DATA - Introduction to PySpark - Which action is used to retrieve the first few elements of an RDD (Resilient Distributed Dataset) ?
Answer  : 

``.take(num)``


Question : BIG-DATA - Introduction to PySpark - What type of operations in Spark trigger the execution of transformations?
Answer  : Actions


Question : BIG-DATA - Introduction to PySpark - Which language is Apache Spark primarily written in?
Answer  : Scala


Question : BIG-DATA - Introduction to PySpark - What does PySpark refer to?
Answer  : Spark's Python API



Question : BIG-DATA - Introduction to PySpark - Which component is the slowest when comparing CPU, Memory, Disk, and Network?
Answer  : Network



Question : BIG-DATA - Introduction to PySpark - What is one of the challenges of debugging PySpark?
Answer  : 

* Debugging distributed systems is hard
* Debugging mixed languages is hard
* Lazy evaluation can be difficult to debug










<!-- https://app.jedha.co/course/distributed-computing-dataframes-ft/01-spark-stack-quiz -->


Question : BIG-DATA - Spark's stack - What is the main functionality of Spark Core?
Answer  : Task dispatching and scheduling

Question : BIG-DATA - Spark's stack - Which component of Spark is used for handling structured data and running queries? 
Answer  : Spark SQL

Question : BIG-DATA - Spark's stack - Which component of Spark is used for graph data structures?
Answer  : GraphX

Question : BIG-DATA - Spark's stack - What is the primary API for MLlib?
Answer  : DataFrame-based API

Question : BIG-DATA - Spark's stack - Which component of Spark is used for handling continuous inflow of data? 
Answer  : Spark Streaming

Question : PYSPARK - What is the name of Spark's optimizer for query execution?
Answer  : Catalyst optimizer

Question : BIG-DATA - Spark's stack - Which component of Spark provides a unified analytics system? 
Answer  : Spark Core

Question : BIG-DATA - Spark's stack - What is the term used for processing continuous data streams in Spark?
Answer  : Stream processing









<!-- https://app.jedha.co/course/distributed-computing-dataframes-ft/02-hands-on-intro-pyspark-quiz -->



Question : BIG-DATA - Hands on introduction to PySpark - What is a DataFrame in PySpark?
Answer  : A distributed collection of data grouped into named columns

Question : BIG-DATA - Hands on introduction to PySpark - How are Spark DataFrames different from SQL tables and pandas DataFrames?
Answer  : Spark DataFrames have richer optimizations

Question : BIG-DATA - Hands on introduction to PySpark - What are the ways to create a Spark DataFrame?
Answer  : 

* csv
* parquet
* panda df
* RDD

Question : BIG-DATA - Hands on introduction to PySpark - Which action displays the first 20 values of a DataFrame?
Answer  : 

``.show()``


Question : BIG-DATA - Hands on introduction to PySpark - What does the ``.filter()`` method do in PySpark?
Answer  : Selects rows based on a condition.


Question : BIG-DATA - Hands on introduction to PySpark - Which method is used to select columns in a Spark DataFrame?
Answer  : 

* ``.select()`` 
* ``.withColumn()`` sert à ajouter une nouvelle colonne ou à remplacer une colonne existante dans le DataFrame

```python
df = spark.createDataFrame([(1, 'Alice'), (2, 'Bob')], ["id", "name"])
df.select("id").show()

from pyspark.sql.functions import col
df = spark.createDataFrame([(1, 'Alice'), (2, 'Bob')], ["id", "name"])
df.withColumn("id_squared", col("id") ** 2).show()
```


Question : BIG-DATA - Hands on introduction to PySpark - What does the ``.limit()`` transformation do?
Answer  : Limits the DataFrame to a specified number of rows.

Question : BIG-DATA - Hands on introduction to PySpark - How can you drop duplicate rows in a DataFrame?
Answer  : 

``.dropDuplicates()``



Question : BIG-DATA - Hands on introduction to PySpark - How can you chain multiple operations together in a DataFrame?
Answer  : Use methods one after the other by addind .methode1().method2()...















<!-- https://app.jedha.co/course/data-warehousing-ft/01-intro-data-warehousing-quiz -->

Question : BIG-DATA - Introduction to Data Warehousing - What is the main difference between a Data Warehouse and a Data Lake?
Answer  : Data Warehouse stores data for analytics purposes, while Data Lake stores raw data for future usage.

Question : BIG-DATA - Introduction to Data Warehousing - What is a key difference between a Data Warehouse and traditional databases?
Answer  : Data Warehouses are optimized for column-based analysis.

Question : BIG-DATA - Introduction to Data Warehousing - What is Redshift?
Answer  : A cloud-based data warehousing solution provided by AWS. Cluster (AWS RDS is SQL database)

Question : BIG-DATA - Introduction to Data Warehousing - How can you write data to Redshift from a PySpark DataFrame?
Answer  : Using the ``df.write.jdbc()`` method with the Redshift URL and table name.

Question : BIG-DATA - Introduction to Data Warehousing - What is the mode option used for when writing data to Redshift?
Answer  : Determining whether to overwrite, append, raise an error, or ignore if the table already exists in Redshift.

Question : BIG-DATA - Introduction to Data Warehousing - How can you read data from Redshift into a PySpark DataFrame?
Answer  : Using the ``df.read.jdbc()`` method with the Redshift URL and table name.









<!-- https://app.jedha.co/course/data-warehousing-ft/02-tidy-data-nested-schemas-quiz -->

Question : BIG-DATA - Tidy data and nested schemas - What is the purpose of data tidying in the context of analyzing datasets?
Answer  : To structure datasets for easier analysis

Question : BIG-DATA - Tidy data and nested schemas - Which principles are associated with tidy data in the context of relational databases?
Answer  : Each variable forms a column, each observation forms a row, and each type of observational unit forms a table

Question : BIG-DATA - Tidy data and nested schemas - What is the purpose of the ``F.size()`` function in Spark SQL?
Answer  : To calculate the number of elements in an array type column

Question : BIG-DATA - Tidy data and nested schemas - What does the ``F.explode()`` function do in Spark SQL? 
Answer  : It replicates rows based on the elements in an array type column. If list then explode! 

Question : BIG-DATA - Tidy data and nested schemas - Which method is used to group data by specific columns in a DataFrame?
Answer  : ``.groupBy()``

Question : BIG-DATA - Tidy data and nested schemas - What is the purpose of the ``.collect_list()`` transformation in Spark SQL?
Answer  : It creates an array of values from a column.

Question : BIG-DATA - Tidy data and nested schemas - How can you access nested fields in a DataFrame using Spark SQL?
Answer  : Using the ``.getField()`` method.

Question : BIG-DATA - Tidy data and nested schemas - What is the purpose of the ``.agg()`` method in Spark SQL?
Answer  : To aggregate data using functions like ``.sum()`` or ``.avg()``.

Question : BIG-DATA - Tidy data and nested schemas - How can you unnest a deeply nested schema in a DataFrame?
Answer  : using ``.explode()`` and ``.getField()`` until the schema is flatenned.

Question : BIG-DATA - Tidy data and nested schemas - What is the benefit of tidying up a nested schema before performing data analysis? (one or more correct answers)
Answer  : 

* It enables easier execution of SQL queries 
* It improves data visualization














<!-- https://app.jedha.co/course/introduction-to-spark-sql-ft/01-aggregates-quiz -->

Question : BIG-DATA - Aggregates - Which module contains PySpark's SQL functions?
Answer  : pyspark.sql.functions

Question : BIG-DATA - Aggregates - What is the purpose of the agg function in PySpark?
Answer  : It performs aggregation operations on specified columns.

Question : BIG-DATA - Aggregates - Which function is used to calculate the mean (average) of a column in PySpark?
Answer  : ``mean()``

Question : BIG-DATA - Aggregates - What is the purpose of the groupBy function in PySpark?
Answer  : It groups the DataFrame by specified columns for aggregation.

Question : BIG-DATA - Aggregates - Which function is used to calculate the sum of a column in PySpark?
Answer  : ``sum()``

Question : BIG-DATA - Aggregates - What does the ``count()`` function in PySpark do?
Answer  : It counts the number of rows in a DataFrame.

Question : BIG-DATA - Aggregates - How can you calculate multiple aggregations in PySpark?
Answer  : By using the ``agg()`` function with a single dict mapping column names to aggregate functions.

Question : BIG-DATA - Aggregates - What is the purpose of the pivot function in PySpark?
Answer  : It creates a pivot table with rows representing customer IDs and columns representing quantities.

Question : BIG-DATA - Aggregates - Which function is used to convert character strings to timestamp type in PySpark?
Answer  : ``to_timestamp()``




<!-- TODO -->
<!-- https://app.jedha.co/course/introduction-to-spark-sql-ft/quiz-fs-m05-v1 -->



