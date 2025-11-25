# First, read the README.md file
# I can't do it for you :-)

# Load environment variables FIRST
from dotenv import load_dotenv
load_dotenv()


import os
import re
import inspect
import logging
import sqlite3
from pathlib import Path
from markdown import markdown
from werkzeug.wrappers import Response
from typing import List, Dict, Tuple, Optional
from flask import Flask, render_template, session, request, redirect, url_for

# ----------------------------------------------------------------------
k_DB_Path = "./flashcards.db"
k_QAFolder = "./static/md"
k_PNGFolder = "./static/png"


# Global logger
logging.basicConfig(level=logging.INFO)
# logging.basicConfig(
#     level=logging.INFO,
#     format='%(asctime)s [%(levelname)s] %(name)s: %(message)s',
#     datefmt='%Y-%m-%d %H:%M:%S'
# )
g_logger = logging.getLogger("app_logger")


# ----------------------------------------------------------------------
# Parse the markdown files and convert to HTML
def parse_markdown_to_html(markdown_text: str) -> List[Dict[str, str]]:
    """Parse a markdown text, convert the question-answer pairs to HTML.

    Args:
        markdown_text (str): The raw markdown text.

    Returns:
        List[Dict[str, str]]: A list of dictionaries with questions and answers in HTML format.
    """

    # app.logger.info(f"{inspect.stack()[0][3]}()")
    g_logger.info(f"{inspect.stack()[0][3]}()")

    markdown_text = re.sub(r"<!--.*?-->", "", markdown_text, flags=re.DOTALL)
    pattern = re.compile(r"Question\s*:\s*(.*?)\nAnswer\s*:\s*(.*?)(?=\nQuestion|\Z)", re.DOTALL)
    matches = pattern.findall(markdown_text)
    return [
        {
            "question_html": markdown(
                "###Question :\n" + match[0].strip(), extensions=["extra", "codehilite", "sane_lists"]
            ),
            "answer_html": markdown(
                "###Answer :\n" + match[1].strip(), extensions=["extra", "codehilite", "sane_lists"]
            ),
        }
        for match in matches
    ]


# ----------------------------------------------------------------------
def load_qa_files(directory: str) -> List[Dict[str, str]]:
    """Load and parse markdown files from the specified directory, converting them to HTML.

    Args:
        directory (str): The directory containing markdown files.

    Returns:
        List[Dict[str, str]]: A list of question-answer pairs in HTML format extracted from markdown files.
    """

    # app.logger.info(f"{inspect.stack()[0][3]}()")
    g_logger.info(f"{inspect.stack()[0][3]}()")

    qa_pairs = []
    # qa_files = [file for file in Path(directory).iterdir() if file.is_file()]
    # Now it get all .md files no matter the directory organization underneath the ./md parent directory
    qa_files = [file for file in Path(directory).rglob("*.md") if file.is_file()]

    for qa_file in qa_files:
        try:
            with qa_file.open("r", encoding="utf-8") as f:
                markdown_text = f.read()
                qa_pairs.extend(parse_markdown_to_html(markdown_text))
        except Exception as e:
            print(f"Error reading file {qa_file.name}: {e}")
    return qa_pairs


# ----------------------------------------------------------------------
def create_fts() -> None:

    g_logger.info(f"{inspect.stack()[0][3]}()")

    with sqlite3.connect(k_DB_Path) as conn:
        cursor = conn.cursor()
        cursor.execute(
            """
            CREATE VIRTUAL TABLE IF NOT EXISTS flashcards_fts
            USING fts5(id, question_html, answer_html);
        """
        )

        cursor.execute(
            """
            INSERT INTO flashcards_fts(id, question_html, answer_html)
            SELECT id, question_html, answer_html FROM flashcards;
        """
        )
        conn.commit()

    return


# ----------------------------------------------------------------------
def load_png_files(directory: str) -> List[Dict[str, str]]:
    # """Load and parse markdown files from the specified directory, converting them to HTML.

    # Args:
    #     directory (str): The directory containing markdown files.

    # Returns:
    #     List[Dict[str, str]]: A list of question-answer pairs in HTML format extracted from markdown files.
    # """

    # app.logger.info(f"{inspect.stack()[0][3]}()")
    g_logger.info(f"{inspect.stack()[0][3]}()")

    # png_pairs = []
    # qa_files = [file for file in Path(directory).iterdir() if file.is_file()]
    # It now get all .md files no matter the directory organization underneath the ./md parent directory
    # qa_files = [file for file in Path(directory).rglob("*.md") if file.is_file()]
    # g_logger.info(qa_files)

    images = [file for file in Path(directory).rglob("*.png") if file.is_file()]
    # images = [Path(*file.parts[2:]) for file in images]
    # images = [str(file).replace("\\", "/") for file in images]

    return [
        {
            "question_html": markdown("###Question :\n", extensions=["extra", "codehilite", "sane_lists"]),
            "answer_html": markdown(
                "###Answer :\n" + f"<img src='{img}' class='img-fluid' alt='Random Image'>",
                extensions=["extra", "codehilite", "sane_lists"],
            ),
        }
        for img in images
    ]


# ----------------------------------------------------------------------
def create_db() -> None:
    """Create the SQLite database and populate it with questions and answers in HTML format."""

    g_logger.info(f"{inspect.stack()[0][3]}()")

    with sqlite3.connect(k_DB_Path) as conn:
        cursor = conn.cursor()
        cursor.execute(
            """
            CREATE TABLE IF NOT EXISTS flashcards (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                question_html TEXT NOT NULL,
                answer_html TEXT NOT NULL
            )
        """
        )

        qa_pairs = load_qa_files(k_QAFolder)  # Parse markdown and convert to HTML before storing
        for qa in qa_pairs:
            cursor.execute(
                "INSERT INTO flashcards (question_html, answer_html) VALUES (?, ?)",
                (qa["question_html"], qa["answer_html"]),
            )
        conn.commit()

    # once the text based cards are in the database, creates full text search database
    create_fts()

    # load the png based cards in the database
    with sqlite3.connect(k_DB_Path) as conn:
        png_pairs = load_png_files(k_PNGFolder)

        cursor = conn.cursor()
        for png in png_pairs:
            cursor.execute(
                "INSERT INTO flashcards (question_html, answer_html) VALUES (?, ?)",
                (png["question_html"], png["answer_html"]),
            )
        conn.commit()

    return


# ----------------------------------------------------------------------
def get_count() -> int:

    g_logger.info(f"{inspect.stack()[0][3]}()")

    with sqlite3.connect(k_DB_Path) as conn:
        cursor = conn.cursor()
        cursor.execute(
            """
            SELECT COUNT(*) FROM flashcards ;
            """
        )
        count = cursor.fetchone()[0]
        return count


# ----------------------------------------------------------------------
# SQLite database setup
def init_db() -> None:
    """Initialize the SQLite database, creating it if it doesn't exist."""

    # app.logger.info(f"{inspect.stack()[0][3]}()")
    g_logger.info(f"{inspect.stack()[0][3]}()")

    if not os.path.exists(k_DB_Path):
        create_db()

    # create full text search database
    # create_fts()


# ----------------------------------------------------------------------
def get_random_flashcard(exclude_ids: List[int]) -> Optional[Tuple[int, str, str]]:
    """Get a random flashcard from the database, excluding the ones already seen.

    Args:
        exclude_ids (List[int]): List of flashcard IDs to exclude from selection.

    Returns:
        Optional[Tuple[int, str, str]]: A tuple containing the flashcard ID, question HTML, and answer HTML, or None if no flashcards remain.
    """

    # app.logger.info(f"{inspect.stack()[0][3]}()")
    g_logger.info(f"{inspect.stack()[0][3]}()")

    with sqlite3.connect(k_DB_Path) as conn:
        cursor = conn.cursor()
        # Build query
        if exclude_ids:
            query = "SELECT id, question_html, answer_html FROM flashcards WHERE id NOT IN ({seq}) ORDER BY RANDOM() LIMIT 1".format(
                seq=",".join(["?"] * len(exclude_ids))
            )
            # exclude_ids will fill the ?
            cursor.execute(query, exclude_ids)
        else:
            cursor.execute("SELECT id, question_html, answer_html FROM flashcards ORDER BY RANDOM() LIMIT 1")

        # Fetch the result
        return cursor.fetchone()


# ----------------------------------------------------------------------
# def get_random_searched_flashcard(
#     exclude_searched_ids: List[int], keywords: List[str]
# ) -> Optional[Tuple[int, str, str]]:

#     with sqlite3.connect(k_DB_Path) as conn:
#         cursor = conn.cursor()
#         if exclude_searched_ids:
#             query = "SELECT id, question_html, answer_html FROM flashcards_fts WHERE flashcards_fts MATCH '{kwds}' AND id NOT IN ({seq}) ORDER BY RANDOM() LIMIT 1".format(
#                 seq=",".join(["?"] * len(exclude_searched_ids)), kwds=" AND ".join(keywords)
#             )
#             cursor.execute(query, exclude_searched_ids)
#         else:
#             cursor.execute(
#                 "SELECT id, question_html, answer_html FROM flashcards_fts WHERE flashcards_fts MATCH '{kwds}' ORDER BY RANDOM() LIMIT 1".format(
#                     kwds=" AND ".join(keywords)
#                 )
#             )

#         # Fetch the result
#         return cursor.fetchone()


# ----------------------------------------------------------------------
def get_random_searched_flashcard(
    exclude_searched_ids: List[int], keywords: List[str]
) -> Optional[Tuple[Optional[Tuple[int, str, str]], int]]:

    g_logger.info(f"{inspect.stack()[0][3]}()")

    with sqlite3.connect(k_DB_Path) as conn:
        cursor = conn.cursor()

        # WHERE clause with keywords and exclusions
        where_clause = "flashcards_fts MATCH '{kwds}'".format(kwds=" AND ".join(keywords))
        if exclude_searched_ids:
            where_clause += " AND id NOT IN ({seq})".format(seq=",".join(["?"] * len(exclude_searched_ids)))

        # Get total number of corresponding records
        count_query = "SELECT COUNT(*) FROM flashcards_fts WHERE " + where_clause
        cursor.execute(count_query, exclude_searched_ids if exclude_searched_ids else [])
        total_count = cursor.fetchone()[0]

        # Get a random record
        random_query = (
            "SELECT id, question_html, answer_html FROM flashcards_fts WHERE "
            + where_clause
            + " ORDER BY RANDOM() LIMIT 1"
        )
        cursor.execute(random_query, exclude_searched_ids if exclude_searched_ids else [])
        random_flashcard = cursor.fetchone()

        if random_flashcard:
            return (random_flashcard, total_count)
        else:
            return (None, 0)


# ----------------------------------------------------------------------
# create_app() function is the entry point which configure the Flask app before it runs
# double check the content of Procfile file
def create_app() -> Flask:

    # logging.basicConfig(level=logging.INFO)

    app = Flask(__name__)

    app.logger.info(f"{inspect.stack()[0][3]}()")
    # If you run the app locally you must run ./secrets.ps1 first (see above)
    # In production on Heroku FLASHCARDS_SECRET_KEY must have been set manually (see readme.md)
    # Without session key, Flask does not allow the app to set or access the session dictionary
    # app.secret_key = os.environ.get("FLASHCARDS_SECRET_KEY")
    app.secret_key = os.environ["FLASHCARDS_SECRET_KEY"]

    # app.config["SESSION_COOKIE_HTTPONLY"] = True  # Empêche l'accès au cookie via JavaScript
    # app.config["SESSION_COOKIE_SECURE"] = False  # Assure que le cookie est accessible sur HTTP
    # app.config["SESSION_PERMANENT"] = False  # Les sessions ne sont pas permanentes

    with app.app_context():
        init_db()  # Initialise la base de données quand l'application est créée

    # Route must be defined inside create_app() otherwise "app" is not yet defined
    # ----------------------------------------------------------------------
    # Flask routes
    @app.route("/")
    def index() -> str:
        """Main route to display a random question and answer.

        Returns:
            str: Rendered HTML with the question and answer or a message if no questions remain.
        """

        # app.logger.info(f"{inspect.stack()[0][3]}()")
        g_logger.info(f"{inspect.stack()[0][3]}()")

        # Initialize session for unseen and seen question IDs
        if "seen_ids" not in session:
            session["seen_ids"] = []

        # Get the total number of cards once per session
        if "nb_cards" not in session:
            session["nb_cards"] = get_count()
            g_logger.info(f"Total number of cards : {session["nb_cards"]}")

        # Check if all cards have been seen
        if len(session["seen_ids"]) >= session["nb_cards"]:
            # Reset the seen cards list since all cards have been seen
            session["seen_ids"] = []
            g_logger.info("All cards seen, resetting seen_ids.")

        # Fetch a random flashcard from the database
        flashcard = get_random_flashcard(session["seen_ids"])

        if flashcard:
            current_QA = {"id": flashcard[0], "question_html": flashcard[1], "answer_html": flashcard[2]}
            # session["seen_ids"].append(current_QA["id"])  # Add this question to seen list
            seen_id_list = session["seen_ids"]
            seen_id_list.append(current_QA["id"])
            # ! I have lost too much time on this !!!!!
            # ! This reassignment operation is crucial because it forces Flask to re-serialize and save the updated session in the underlying storage
            # ! Flask should overload the "="" operator !!!!!!!!!!!!
            session["seen_ids"] = seen_id_list

            return render_template(
                "index.html",
                Q_html=current_QA["question_html"],
                A_html=current_QA["answer_html"],
                nb_cards=session["nb_cards"],
            )
        else:
            return render_template("index.html", Q_html="No more questions.", A_html="", nb_cards=session["nb_cards"])

    # ----------------------------------------------------------------------
    @app.route("/search", methods=["GET", "POST"])
    def search() -> str:

        g_logger.info(f"{inspect.stack()[0][3]}()")

        # Form is submitted
        if request.method == "POST":
            # Create a clean session of searched_ids for unseen and seen searched question ids
            session["searched_ids"] = []
            session["keywords"] = request.form["keywords"].split()  # Séparer les mots par espace
            # return render_template("search_results.html")
            return redirect(url_for("search_results"))

        # otherwise if GET then display the search page and the form
        return render_template("search.html")

    # ----------------------------------------------------------------------
    @app.route("/search_results")
    def search_results() -> str:

        g_logger.info(f"{inspect.stack()[0][3]}()")

        # Retrieve from the database a flashcard corresponding to the search criteria (keywords)
        flashcard, nb_cards_in_search = get_random_searched_flashcard(session["searched_ids"], session["keywords"])

        if flashcard:
            current_QA = {"id": flashcard[0], "question_html": flashcard[1], "answer_html": flashcard[2]}
            session["searched_ids"].append(current_QA["id"])  # Add this question to seen list
            return render_template(
                "search_results.html",
                Q_html=current_QA["question_html"],
                A_html=current_QA["answer_html"],
                nb_cards=nb_cards_in_search,
            )
        else:
            return render_template(
                "search_results.html", Q_html="No cards in search results.", A_html="", nb_cards=nb_cards_in_search
            )

    # ----------------------------------------------------------------------
    @app.route("/next")
    def next() -> Response:
        """Route to go to the next question.

        Returns:
            str: Redirect to the index route.
        """

        # app.logger.info(f"{inspect.stack()[0][3]}()")
        g_logger.info(f"{inspect.stack()[0][3]}()")
        return redirect(url_for("index"))

    # ----------------------------------------------------------------------
    # For debug ???
    @app.route("/reset_session")
    def reset_session() -> str:
        session.clear()
        return "Session cleared."

    return app


# ----------------------------------------------------------------------
if __name__ == "__main__":

    # DONE : it seems that locally, in debug mode the application starts twice...
    # Uncomment the print() below to see what happen
    # print(f"XXXXXXXXXXXXXXXXXXXXXXXXXXXXXX")

    # In debug mode, Flask uses an automatic reloader called Werkzeug.
    # This reloader automatically restarts the application whenever it detects a change in the source code.
    # This way, modifications are taken into account without having to restart the application manually.
    # This reloader creates two processes:
    #   - The first process starts the Flask server, then launches the reloader.
    #   - The reloader then restarts the application in a second process to enable hot reloading of the code.
    # This double startup results in the double display of print(f “XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX”)

    # In debug mode, we want to delete the database the very first time
    # That is, when WERKZEUG_RUN_MAIN is still “”.

    if os.environ.get("WERKZEUG_RUN_MAIN") == None:
        # if os.path.exists(k_DB_Path):
        # os.remove(k_DB_Path)
        db_path = Path(k_DB_Path)
        if db_path.exists():
            db_path.unlink()

    app = create_app()
    # app.logger.info("main()")
    g_logger.info("main()")
    app.run(debug=True)
