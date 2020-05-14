# Dojo reconnaissance faciale

## Avant de commencer

Avant de commencer, vous devez vous assurer que la bibliothèque C++ `dlib` soit installée.
Pour cela, après s'être assuré que vous avez installé [Homebrew](https://brew.sh/index_fr) sur votre machine, exécutez simplement :

```shell script
    $ brew install dlib
```

## Contenu du dojo

Votre travail va se résumer à combler les trous présents dans le fichier `src/main.rs`.

Vous pouvez prendre une photo de vous et la ranger dans le dossier `data/` sous le formalisme `nom-prenom.jpg`. Le programme
est écrit pour aller directement chercher et indexer les photos qui s'y trouvent.

Vous recevrez un script python qu'il suffira de placer dans le dossier `data/` et d'exécuter pour récupérer tous les visages de BeTomorrow !
(d'ailleurs c'est normal niveau sécurité ça ?)

## Exécuter votre programme

Avec Rust le mode Debug et le mode Release sont vraiment très différents. Dans notre cas le mode Debug nous permettra de 
débugger très facilement et rapidement, mais les temps de calculs seront infiniment longs.

Pour exécuter votre programme en mode debug, la ligne de commande suivante suffit :

```shell script
    $ cargo run
``` 

Lorsque vous aurez terminé de débugger votre programme et que vous serez prêts pour obtenir des performances qui dépassent
l'entendement, compilez et exécutez en mode release :

```shell script
    $ cargo run --release
```

## Troubleshooting

Pour ceux qui n'utilisent pas CLion mais plutôt VSCode, MacOS est un peu pénible au niveau des permissions.
OpenCV a besoin de sa persmission pour lancer la caméra, mais ne semble pas la demander automatiquement via la commande `cargo run` ni le shell VSCode.
Il vous vaudra mieux alors ouvrir un shell dédié, naviguer jusqu'à votre dossier, puis :

Lancer un 
```shell script
    $ cargo build --release
```

Suivi d'un 
```shell script
    $ ./target/release/dojo_rust
```

Pour lancer le programme. Merci Apple ♥.

> That's all folks !