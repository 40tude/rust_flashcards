## Can Claude Code do 100% of the job?

* The idea - Take an existing working Python project and let Claude Code translate it in Rust
* The Python project acts like a flash cards server
    * You write your cards in markdown. You can include links, images, maths formula, code fragments...
    * You can create cards with image only
    * You can run the app locally. This is a Web server and SQLite database
    * The app is deployed on Heroku - Server, SQL database, application
* This is an experiment. I want to:
    * Learn more about Claude Code
    * See how far I can go with Claude Code
    * Learn by experience
    * Discover what make sense, what does'nt...

## Where are we (Claude and I)
It went so well that I decided to go forward and today, the master plan include 4 major steps.
* [✅] **Step 1:** Translation and deployment at iso configuration
* [✅] **Step 2:** Refactor the database so that it includes categories and subcategories (see Step 4)
* [✅] **Step 3:** Refactor the Q&A cards so that they include a Reveal/Hide button
* [✅] **Step 4:** Refactor the landing page so that the user can select to review only certain categories, subcategories or flash cards with certain keyword (this is already working but this is not great)




## How To

* I'm an happy user of Windows 11, Powershell and VSCode
* I had forgotten how this Python project was made, how to it was running it etc.
* I restarted it, review it quickly and redeploy it on Heroku (few things had changed there)
* Then I use Cargo to create a directory for the Rust project
* I make a copy of the Python directory (delete the `.git` directory) in the Rust project directory
* I'm not 100% sure but at this stage I may have committed the project on Github
* I invoke Claude Code in a VSCode integrated terminal from the root of the Rust project



### /init
* To let Claude create a `Claude.md` file
    * I'm not sure I understand the purpose of this file
    * I need to investigate
* Claude Code reads all the files etc.


### A preparation phase in Plan Mode
* `Shift Tab` in the terminal to switch in **Plan Mode**. This is important.
* Explain what I want to do
* Iterate, iterate...
* The key : ask Claude Code to create a **multi-phase plan** and to save this plan in a markdown document. See `assets/multi_phase_plan.md`
* This is important because with this ressource on the side I can leave Claude Code and then come back and continue (think of time limit issues, number of tokens issues...)

### Execution Mode
* Once the multi-phase plan is OK and saved...
* Check the remaining tokens with `/context`
* If needed leave, come back, start a new instance (no tokens used) ask Claude to read the multi-phase markdown plan and to execute Phase 1 but and to stop at the end.
* I let it ask permission for everything at the beginning because I want to read and follow what it does
* At the end of Phase 1, I commit the changes and push them to GitHub.
    * Later I let Claude commit the changes
    * **TODO:** see how to give instructions regarding commit messages


**Side Note**
I installed and use [`ccusage`](https://ccusage.com/) (see `npx ccusage@latest`).






## What I had to do so far

### For Step 1 - Translating and Deploying on Heroku
* Step 8. Create the project on Heroku etc. Read more in `assets\multi_phase_plan.md`
* Create `.slugignore` file

### For Step 2 - Extract Categories and SubCategories
* Step 1. Make sure all Cards with Q&A use the right template. It was easier and faster that way. Read more in `assets\multi_phase_plan_2.md`


### For Step 3 - Implement Show answer

* Yesterday night I read more about `Claude.md`
* I made an important cleanup in directories, files etc.
* I also `/init` to start Step 3 and 4 with a clean setup
* Then I tuned the `Claude.md` (make sure it use the ms-rust skill, write in English US...)
* I did nothing!
    * I start in Planning Mode
    * Then I switch to Execution Mode
    * I make sure the plan is saved in `assets/` directory
* Read more about Step 3 in `assets\hide_reveal_answer_plan.md`
* In addition *we* fix a bug (double insertion in FTS table)
* Claude wrote the commit message and pushed on origin

### For Step 4 - New landing page
* Commit
* Plan Mode first
* I realized that the app was rebuilding the database on each start so I check and ask to change the behavior
* I provided a drawing of what I want to see and explain the behavior
* Double check few point : `/search` now removed, `landing page == index.html` etc.
* Switch to Execution Mode
* Save the plan : see `assets\landing_page_plan.md`
* I let Claude do the job, write code etc.
* One or 2 bugs
* Let Claude create a commit and push on origin
* Added feature : Tab support in the card so that we can use keyboard only
* Added feature : make sure md and png if missing the app quit and leave an explicit message
* Fixed : Issues with the logic of category/subcategory check boxes
    * This one was touchy and took some time


## Issue rendering picture on cell phone
The problem stems from the interaction between:

1. Viewport meta tag (practice.html:5):
    - `<meta name="viewport" content="width=device-width, initial-scale=1.0">`
2. Galaxy Note 20 resolution: 3088x1440 pixels (very high density)
3. Images with width="433" in the markdown

What happens:

- Your Galaxy Note 20 has a physical resolution of 3088x1440, but a Device Pixel Ratio (DPR) of probably 3.5x (yes, verified)
  - The effective CSS width is ~440px (1440/3.5 ≈ 411px or 3088/3.5 ≈ 882px in landscape)
  - The viewport causes the page to use the entire width of the virtual screen
- Bootstrap .container has responsive max-width, but on mobile it takes up ~100% of the width (minus margins)
  - Images that are 577px physically with width="433" CSS overflow because 433px CSS > container width on some mobile devices

Possible solutions:

Option 1 - Responsive CSS for images (recommended):
Add to default.css:
img {
    max-width: 100%;
    height: auto;
}

  Option 2 - Limit image width in container:
.container img {
      max-width: 100%;
      height: auto;
  }

Option 3 - Use Bootstrap classes in markdown:
<img src="..." class="img-fluid" width="433"/>


## Notes
* I'm totally stressed with the time window. I use `/status` and `npx ccusage@latest` all the time
* Claude Code loves Linux
* **TODO:** Find a way to let it know it is in a Win/Powershell context




## TODO (this is for me)
* Hide answers - Step 3
* Landing page with search options - Step 4
* Review math formula $ vs $$ - See `static\md\07_fs_deep_learning.md` for example.




## This is for me

Release:
* Win = 4.2 MB
* Linux = 5.2 MB (by default debug info were not stripped)

```
git push heroku main
http://localhost:8080/
https://rust-flashcards-ae94334b8997.herokuapp.com/
powershell -Command "Stop-Process -Name rust-flashcards -Force"

heroku run bash -a rust-flashcards

```

