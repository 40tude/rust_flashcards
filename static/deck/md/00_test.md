<!--
############################################################
##
############################################################
-->
Question : My category - Maths - The F1 score is an harmonic mean. Why?
Answer   :

#### From Default Deck

* Think of the “harmonious” average
* It is highest when the values are identical
* F1: we are looking for the compromise between Recall & Precision

<p align="center">
<img src="static/deck/md/assets/harmonic.png" alt="harmonic" width="600"/>
</p>

* Note that we are not saying that the car drives for one hour at 40 km/h and then for one hour at 60 km/h
* In that case, it would have driven for 2 hours and covered 100 km
    * The arithmetic mean value of the speed would then be 50 km/h
* No, no, here we are saying that it travels half the distance at 40 km/h and then the other half at 60 km/h
* We wonder what constant speed it would have to travel at to cover the same distance in the same time.

#### Reasoning:
* The car travels 40 km in 1 hour
* So it travels $\frac{D}{2}$ in $\frac{D}{2\cdot40}$ hours (it's a rule of three, it's still manageable...)
* Similarly, it travels $\frac{D}{2}$ in $\frac{D}{2\cdot60}$ hours
* The total distance is $D$
* The total travel time is: $t = \frac{D}{2\cdot40} + \frac{D}{2\cdot60}$
* So the average speed is: $V = \frac{d}{t} = \frac{D}{\frac{D}{2\cdot40} + \frac{D}{2\cdot60}} = \frac{2}{\frac{1}{40} + \frac{1}{60}}$



<!--
############################################################
##
############################################################
-->
Question : Culture - Star Wars - I love you
Answer   :

#### From Default Deck

I know


<!--
############################################################
##
############################################################
-->
Question: Culture - The Hitchhiker's Guide to the Galaxy - The Answer to the Ultimate Question of Life, the Universe, and Everything is?
Answer  :

42


<!--
############################################################
##
############################################################
-->
Question: Frequentist or Bayesian?
Answer  :

Bayesian!

<!--
Above, category and subcategory are missing.
A WARN is displayed be the database is rebuilt. The card is displayed with no category/subcategory
-->