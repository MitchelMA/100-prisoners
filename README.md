# 100 Prisoners problem
## Het probleem
De *100 Prisoners problem* gaat over dat 100 gevangenen vrij kunnen komen op de volgende manier:  
Er is een kamer met 100 dozen, in elke doos zit een strookje met daarop een nummer. Op iedere doos staat een nummer optellend van 1 naar 100, hetzelfde gaat op voor de strookjes. Een gevangene mag 50 dozen open, als de gevangene in één van deze 50 dozen zijn nummer vindt, is het de gevangene gelukt en mag deze verder nadat alles weer netjes is opgeruimt. Lukt dit alle 100 gevangenen, dan zijn ze vrij. Lukt het ook maar eentje niet, krijgen ze allemaal de doodstraf.  

## De oplossing
volgens standaard kansberekening, is de kans dat dit lukt (1/2)^100. Dit is een ongelofelijk klein getal (7.888609052x10^-31).  
Maar er is een simpele, maar effectieve manier om toch de kans te vergroten (naar ~31%, zelfs):  
De gevangene kiest als eerst de doos met zijn nummer erop, in deze doos zit een strookje met een ander nummer. De gevangene gaat naar de doos met daarop het nummer wat op het strookje stond.  
Dit herhaalt de gevangene net zolang totdat deze zijn eigen nummer in een doos vindt. Op deze manier creëer je een soort "ketting" van verwijzingen. Omdat je als gevangene de doos kiest met jouw nummer erop, is er hoe dan ook een vorige doos in deze ketting. Op deze manier is alleen de vraag: "hoelang is deze ketting?"  
Ik weet niet de precieze wiskunde erachter en waarom het nou ~31% is, maar ik heb hier wel de uitwerking die bewijst dat het ~31% is.  
  
## Verwijzing
Het idee om dit programma te maken kwam uit de volgende [video](https://www.youtube.com/watch?v=iSNsgj1OCLA). In deze video staat meer informatie op een betere manier dan ik ooit zou kunnen, dus ik raad je aan hem te kijken als het je intereseert.