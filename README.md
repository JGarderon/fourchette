# fourchette

Fourchette ton process'... Commande permettant de lancer des sous-processus et de récupérer la valeur booléenne finale. Une idée d'usage : l'*entrypoint* d'un conteneur par exemple ! 

### Arguments pris en charge 

- ":print" : afficher un texte (préfixer par ">>> ") et retourne *true*
- ":bash" : raccourci pour lancer une commande via Bash 
- ":cmd" : exécute un sous-processus, géré dans un fil indépendant 
- ":env" : surcharge localement, pour la ou les commandes suivantes, une valeur de l'environnement 

### Limites actuelles 

- pas de stdin pour les commandes à exécuter (pas de *pipe* !)
- un seul *stdout* / *stderr* 

### A venir : 

- prise en charge de stdin pour fixer des valeurs et de nouvelles commandes à exécuter 

### Exemple simple 

	13:09 julien@julieng ~/fourchette(main)* $ cargo run :print coucou 
	    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
	     Running `target/debug/initrust ':print' coucou`
	>>> coucou 

### Exemple sans erreur 

	13:09 julien@julieng ~/fourchette(main)* $ cargo run :env toto=tata :bash echo \$toto :env toto=titi :cmd bash -c "echo \$toto" ; echo "$?"
	    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
	     Running `target/debug/initrust ':env' toto=tata ':bash' echo '$toto' ':env' toto=titi ':cmd' bash -c 'echo $toto'`
	titi
	tata
	0

### Exemple avec erreur 

	13:09 julien@julieng ~/fourchette(main)* $ cargo run :env toto=tata :bash echo \$toto :env toto=titi :cmd bash -c "dsdqsd" ; echo "$?"
	    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
	     Running `target/debug/initrust ':env' toto=tata ':bash' echo '$toto' ':env' toto=titi ':cmd' bash -c dsdqsd`
	tata
	bash: dsdqsd : commande introuvable
	1


