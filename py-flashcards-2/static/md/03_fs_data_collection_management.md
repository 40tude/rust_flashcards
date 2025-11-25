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

<!-- https://app.jedha.co/path/full-stack-full-time -->

Question : Data Collection - HTTP - What is the purpose of the HTTP protocol?
Answer  : 

To transfer data and instructions over the Internet



Question : Data Collection - HTTP - What does an HTTP request consist of? 
Answer  : 

Method, URL, and protocol version



Question : Data Collection - HTTP - url et route ?
Answer  : 

Dans https://api.github.com/zen 

* L'**url** c'est https://api.github.com
* La **route** c'est zen  



Question : Data Collection - HTTP - Which HTTP method is used to request a resource at a specified URL?
Answer  : 

``GET``



Question : Data Collection - HTTP - What does an HTTP response contain?
Answer  : 

* Status code
* Headers
* Body




Question : Data Collection - HTTP - What does a status code of 404 indicate?
Answer  : 

Resource is no longer available at the requested location



Question : Data Collection - HTTP - Which response header provides information about the type of content in the body of the response?
Answer  : 

Content-Type



Question : Data Collection - HTTP - Which status code indicates a client error?
Answer  : 

``401``



Question : Data Collection - HTTP - What is the difference between a ``GET`` and a ``POST`` method for HTTP requests?
Answer  : 

* The **POST** method lets you send data to the web server
* While the **GET** method only gathers data from the web server without sending any



<!-- https://app.jedha.co/course/http-and-apis-ft/02-apis-quiz -->

Question : Data Collection - API - Which HTTP method is used to retrieve data from an API?
Answer  : 

``GET``



Question : Data Collection - API - What does REST stand for in REST API?
Answer  : 

**Representational State Transfer**. Décrit une architecture où : 

* les interactions avec des ressources web passent par des échanges de représentations de ces ressources. On ne manipule pas les ressources mais leur repésentation.
* l'état de l'application (les données) est transféré à chaque requête de manière stateless (pas de session en mémoire d'une requête à une autre)



Question : Data Collection - API - Which Python library can be used to interact with APIs?
Answer  : 

``requests``





Question : Data Collection - API - How can you add parameters to a ``GET`` request?
Answer  : 

Use the params parameter

#### Code snippet 

```python
my_params = {
  "q" : "paris",
  "countrycodes" : "fr",
  "format":"json",
}
response = requests.get(url, params=my_params) 
```



Question : Data Collection - API - Which HTTP method is used to send data to an API?
Answer  : 

``POST``



Question : Data Collection - API - How can you access the content of a response as plain text in requests library?
Answer  : 

``response.text``




Question : Data Collection - API - How can you retrieve binary content, such as an image, from an API response?
Answer  : 

``response.content``








<!-- https://app.jedha.co/course/http-and-apis-ft/03-async-programming-quiz -->
<!-- TODO -->










<!-- https://app.jedha.co/course/web-scraping-ft/01-html-css-quiz -->
<!-- 
What is the purpose of web scraping?
Which elements are used to create headings in HTML?
How can you separate different sections of a website in HTML?
What elements are used to create lists in HTML?
Which element is used to add images in HTML?
What does the CSS acronym stand for?
How can you apply CSS styles to an HTML element?
What is the recommended way to add extensive CSS styling to a website?
How can you view the source code of a webpage in Google Chrome?
-->

Question : Data Collection - HTML-CSS - Which CSS selector is used to select an element by its class?
Answer  : 

``.class``



<!-- https://app.jedha.co/course/web-scraping-ft/02-scrapy-basics-quiz -->
<!-- 
-->

Question : Data Collection - SCRAPY - What is Scrapy used for?
Answer  : 

Parsing HTML pages, Scraping websites automatically, Running multiple crawlers simultaneously



Question : Data Collection - SCRAPY - What does the parse() method in a Scrapy spider do?
Answer  : 

It defines the callback function for processing the response

#### Code snippet 

```python
def parse(self, response):
    hotel = response.xpath(
    "/html/body/div[4]/div/div[2]/div/div[2]/div[2]/div/div/div[1]/div/div[1]/div/h3/a/div[1]/text()" 
    ).get()
    # TODO : convertir utf8 en texte ou supporter utf8

    url = response.xpath(
    "/html/body/div[4]/div/div[2]/div/div[2]/]/div[1]/div/div[1]/div/h3/a"
    ).attrib["href"]

    processed_data = {
    "hotel" : hotel,
    "url" : url,
    }
    yield processed_data

```


Question : Data Collection - SCRAPY - How can you avoid being banned from websites when using Scrapy?
Answer  : 

1. Use a different IP address for each request
1. Slow down the crawling speed
1. Randomize the order of requests

Question : Data Collection - SCRAPY - Which setting is used to specify the user agent in Scrapy?
Answer  : 

``USER_AGENT``

#### Code snippet 

```python
process = CrawlerProcess(
    settings={
        "USER_AGENT": "Chrome/97.0",
        "LOG_LEVEL": logging.INFO,
        "FEEDS": {
            current_dir + "/" + filename: {"format": "json"},
        },
    }
)
```



Question : Data Collection - SCRAPY - What does the ``LOG_LEVEL`` setting control in Scrapy?
Answer  : 

The level of logs displayed

#### Code snippet 

```python
process = CrawlerProcess(
    settings={
        "USER_AGENT": "Chrome/97.0",
        "LOG_LEVEL": logging.INFO,
        "FEEDS": {
            current_dir + "/" + filename: {"format": "json"},
        },
    }
)
```

Question : Data Collection - SCRAPY - What is the purpose of the ``CrawlerProcess`` in Scrapy?
Answer  : 

1. It runs the spider and saves the results
1. It sets up the user agent for scraping




Question : Data Collection - SCRAPY - How can you save the results of a Scrapy spider in a JSON file?
Answer  : 

Specify the output file using the ``FEEDS`` setting

#### Code snippet 

```python
process = CrawlerProcess(
    settings={
        "USER_AGENT": "Chrome/97.0",
        "LOG_LEVEL": logging.INFO,
        "FEEDS": {
            current_dir + "/" + filename: {"format": "json"},
        },
    }
)
```





<!-- https://app.jedha.co/course/web-scraping-ft/03-scrapy-advanced-quiz -->

Question : Data Collection - SCRAPY - What are callbacks used for in Scrapy?
Answer  : 

To perform tasks that are independent of the code itself



Question : Data Collection - SCRAPY - How can you navigate the web and follow links using Scrapy? 
Answer  : 

By using the `.follow()` method and providing the XPath of the link

#### Code snippet 

```python
class ArticleSpider(scrapy.Spider):
    name = "article_spider"
    start_urls = ['https://example-blog.com']  

    def parse(self, response):
        article_links = response.css('a.article-link::attr(href)').getall()
        
        for link in article_links:
            yield response.follow(link, callback=self.parse_article)

    def parse_article(self, response):
        title = response.css('h1::text').get()
        author = response.css('span.author::text').get()
        content = response.css('div.content').get()

        yield {
            'title': title,
            'author': author,
            'content': content
        }

```


Question : Data Collection - SCRAPY - How can you authenticate on a website using Scrapy?
Answer  : 

By using the ``.from_response()`` method and sending a ``POST`` request with the login data.

#### Code snippet 

```python
# Parse function for login
def parse(self, response):
    # FormRequest used to login
    return scrapy.FormRequest.from_response(
        response,
        formdata={"username": "john", "password": "secret"},
        # Function to be called once logged in
        callback=self.after_login,  # after_login() is another callback
    )

```


Question : Data Collection - SCRAPY - What is the purpose of Scrapy projects?
Answer  : 

To configure the scraping process and manage settings.



Question : Data Collection - SCRAPY - How can you enable AutoThrottle in Scrapy?
Answer  : 

By uncommenting the appropriate lines in the settings.py file.

```python
# settings.py

BOT_NAME = 'myproject'

SPIDER_MODULES = ['myproject.spiders']
NEWSPIDER_MODULE = 'myproject.spiders'

# Enable AutoThrottle
AUTOTHROTTLE_ENABLED = True
AUTOTHROTTLE_START_DELAY = 5
AUTOTHROTTLE_MAX_DELAY = 60
AUTOTHROTTLE_TARGET_CONCURRENCY = 1.0
AUTOTHROTTLE_DEBUG = False

# Other settings ...
```



Question : Data Collection - SCRAPY - What is the purpose of the Scrapy ``AutoThrottle`` extension?
Answer  : 

To automatically adjust Scrapy to the optimum crawling speed and avoid exceeding requests limitations.



Question : Data Collection - SCRAPY - How can you rotate user agents in Scrapy?
Answer  : 

By installing the ``scrapy-user-agents`` library and configuring the ``settings.py`` file.





Question : Data Collection - SCRAPY - What is the purpose of rotating IP addresses in Scrapy?
Answer  : 

To bypass website bans and avoid detection.


Question : Data Collection - SCRAPY - How can you specify a list of rotating proxies in Scrapy?
Answer  : 

By installing the ``scrapy-rotating-proxies`` library and configuring the ``settings.py`` file.




Question : Data Collection - SCRAPY - Which command is used to start a Scrapy spider contained in a project?
Answer  : 

```bash
scrapy crawl spider_name
```

Question : Data Collection - SCRAPY - Which Scrapy commands allows you to start a new Scrapy project?
Answer  : 

```bash
scrapy startproject
```


Question : Data Collection - SCRAPY - What is the purpose of the ``.follow()`` method in Scrapy?
Answer  : 

To navigate to the next page in a pagination sequence.

#### Code snippet 

```python
class ArticleSpider(scrapy.Spider):
    name = "article_spider"
    start_urls = ['https://example-blog.com']  

    def parse(self, response):
        article_links = response.css('a.article-link::attr(href)').getall()
        
        for link in article_links:
            yield response.follow(link, callback=self.parse_article)

    def parse_article(self, response):
        title = response.css('h1::text').get()
        author = response.css('span.author::text').get()
        content = response.css('div.content').get()

        yield {
            'title': title,
            'author': author,
            'content': content
        }

```








































<!-- https://app.jedha.co/course/etl-processes-ft/02-etl-processes-quiz -->

Question : Data Collection - ETL - What does ETL stand for?
Answer  : **Extract Transform Load**

Question : Data Collection - ETL - What is the purpose of an ETL process?
Answer  : To clean and load data into a database

Question : Data Collection - ETL - In the context of ETL, what is the role of Extract?
Answer  : To gather data from various sources

Question : Data Collection - ETL - Which storage system is commonly used as a datalake?
Answer  : AWS S3

Question : Data Collection - ETL - What is the purpose of transforming data in an ETL process?
Answer  : To clean and validate data

Question : Data Collection - ETL - Why is an ETL process useful for business intelligence (BI)?
Answer  : To gather data in one place for analysis

Question : Data Collection - ETL - What is a Data Warehouse?
Answer  : A database specifically optimized for analytics

Question : Data Collection - ETL - What does ETL process ensure for data in a company?
Answer  : Data accuracy and validity

Question : Data Collection - ETL - When would a company greatly benefit from implementing an ETL process?
Answer  : When it has multiple data sources

Question : Data Collection - ETL - What is the primary purpose of a transactional database and a data warehouse, respectively?
Answer  : 
* A **transactional database** is primarily used for day-to-day operational tasks
* A **data warehouse** is primarily used for historical data analysis








<!-- https://app.jedha.co/course/etl-processes-ft/03-sql-reminder-quiz -->

Question : Data Collection - SQL reminder - What is a relational database?
Answer  : A database consisting of 2-dimensional tables

Question : Data Collection - SQL reminder - What is a DBMS?
Answer  : Database Management System. Examples of cloud DBMS include Amazon RedShift, Google BigQuery, Microsoft SQL Database, IBM Db2, Oracle Database, MongoDB, MariaDB, Azure Synapse Analytics...

Question : Data Collection - SQL reminder - What does a schema represent in a database?
Answer  : The structure of the database

Question : Data Collection - SQL reminder - What is a line in a table also known as?
Answer  : Record

Question : Data Collection - SQL reminder - What does NULL represent in SQL?
Answer  : Unknown or missing values


Question : Data Collection - SQL reminder - Which SQL command is used to create tables?
Answer   : `CREATE TABLE`

#### Code snippet 

```sql
CREATE TABLE IF NOT EXISTS flashcards (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    question_html TEXT NOT NULL,
    answer_html TEXT NOT NULL
);
```


Question : Data Collection - SQL reminder - Which SQL command is used to insert values into a table?
Answer  : `INSERT INTO`

#### Code snippet 

```sql
INSERT INTO Customers (CustomerName, ContactName, Address, City, PostalCode, Country)
VALUES ('Cardinal', 'Tom B. Erichsen', 'Skagen 21', 'Stavanger', '4006', 'Norway');
```



Question : Data Collection - SQL reminder - Which SQL command is used to select specific columns from a table?
Answer  : `SELECT`

#### Code snippet 

```sql
SELECT id, question_html, answer_html FROM flashcards ORDER BY RANDOM() LIMIT 1;
```



Question : Data Collection - SQL reminder - Which SQL command is used to join two tables?
Answer  : `INNER JOIN`

#### Code snippet 

```sql
SELECT ProductID, ProductName, CategoryName
FROM Products
INNER JOIN Categories ON Products.CategoryID = Categories.CategoryID;
```




Question : SQL-ALCHEMY - What is SQLAlchemy?
Answer  : Python library for manipulating databases

Question : SQL-ALCHEMY - Which layer of SQLAlchemy allows you to communicate with databases and create flexible models?
Answer  : ORM

Question : SQL-ALCHEMY - What is the purpose of the ``__repr__`` method in SQLAlchemy? 
Answer  : It formats the output of an object

Question : SQL-ALCHEMY - How do you persist values in a database using SQLAlchemy?
Answer  : By calling the ``commit()`` method

#### Code snippet 

```python
session = Session()
session.add(ed_user)
session.add(al_user)
session.commit()

```


Question : SQL-ALCHEMY - How can you query data from a database using SQLAlchemy?
Answer  : By using the ``query()`` function

```python
from sqlalchemy.orm import sessionmaker
from sqlalchemy import create_engine, text

engine = create_engine(f"postgresql+psycopg2://{key.USERNAME}:{key.PASSWORD}@{key.HOSTNAME}/{key.DBNAME}", echo=True)
Session = sessionmaker(bind=engine)
session = Session()
user = session.query(User) # User is a class wich encapsulate the tabel "users"
user.all() # output all

statement = text("SELECT * FROM users where name=:name")
session.query(User).from_statement(statement).params(name="Zoubida").all()

```
