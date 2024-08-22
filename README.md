### Exercice Complémentaire : Ajout des Touches "F" et "G" pour Afficher et Cacher les Mines

#### Objectif

Ajouter deux fonctionnalités au jeu :

1. **Touche "F"** : Permettre de révéler toutes les mines sur la grille sans terminer le jeu.
2. **Touche "G"** : Permettre de cacher à nouveau toutes les mines révélées avec la touche "F".

#### Instructions

1. **Gestion de la Touche "F"** :
   - Ajoutez la gestion de l'événement pour la touche "F" dans la boucle d'événements du jeu. Lorsque cette touche est pressée, toutes les mines doivent être révélées sur la grille sans que le jeu ne se termine.

2. **Gestion de la Touche "G"** :
   - Ajoutez la gestion de l'événement pour la touche "G". Lorsque cette touche est pressée, toutes les mines précédemment révélées doivent être cachées à nouveau, sans affecter les autres cases ni terminer le jeu.

3. **Création de la Fonction de Masquage des Mines** :
   - Implémentez une fonction `hide_all_mine` qui parcourt la grille et remet toutes les cases contenant des mines à l'état "hidden" si elles sont révélées.

4. **Testez la Fonctionnalité** :
   - Testez ces fonctionnalités en démarrant une partie, en appuyant sur la touche "F" pour révéler les mines, puis sur la touche "G" pour les cacher à nouveau.

#### Critères de Réussite

- La touche "F" doit révéler toutes les mines sans terminer le jeu.
- La touche "G" doit cacher toutes les mines précédemment révélées sans terminer le jeu.
- Le jeu doit continuer normalement après avoir utilisé les touches "F" et "G".

#### Scénarios à Tester

- **Appui sur la touche "F"** : Toutes les mines doivent être révélées, mais le jeu doit continuer.
- **Appui sur la touche "G"** : Toutes les mines révélées doivent être cachées à nouveau, mais le jeu doit continuer.
- **Jeu normal** : Le joueur doit pouvoir continuer à jouer normalement après avoir révélé et caché les mines.

Bonne chance !
