# BotMonitor  


Monitor permet de surveiller un service en effectuant des appels http à intervalle régulier sur celui-ci.

Si le service ne répond pas le message d'erreur est transmis dans une room webex.

~ Mise en place entre 5 & 10Min

## Création du bot Webex

Lien vers la documentation pour la création du bot : [ici](https://developer.webex.com/docs/bots) (5min si vous connaissez votre pwd webex)

À la création du bot un token vous sera fournis, il faut le conserver. Si vous le perdez il faut générer un nouveau token ce qui entraîne l'annulation de l'ancien.


## Récuperation de la room_id
* Création de la room via votre client webex
* [Cliquez ici](https://developer.webex.com/docs/api/v1/rooms/list-rooms) pour vous rendre sur l'api webex (il faut être log).
* Cliquez sur "Run"
* Trouver la room qui vous intéresse dans le json de retour pour récupérer le champ "id"


## Ajout du bot dans la room
Sur le client webex :
* Sélectionner la room
* Allez dans l'onglet "Personnes"
* Clic sur "Ajouter des personnes"
* Ajout du bot (via son nom) pour qu'il ait l'autorisation d'envoyer des messages.


##Configuration de BotMonitor

Créer un fichier .json et y placer la configuration suivante :

Exemple pour deux services:
```json
[
  {
    "name" : "Service1",
    "url": "https://monurl.fr/isalive",
    "bot_token": "YWYxMjM1MTItSDFRFS00ZWVlLWEyMzktNmZjNzAxYmYxYzMzNjIyMDM3NWMtNzg3_RFG_gfgZEfd-471f-934c-50faa59de0db",
    "room_id": "Y2lzYQSDGzL1JPT00GFGFHEYzQ2YTAtYWE0Yi0xMWVhLWE5YTEtMjc5YTNhYjY4M2Vk",
    "schedule": "1/60 * * * * *"
  },
  {
    "name" : "Service2",
    "url": "https://mondomain.com/isalive",
    "bot_token": "YWYxMjM1MTItNWRlYS00ZWVlLWEyMzktNmZjNzAxYmYxYzMzNjIyMDM3NWMtNzg3_PF84_ca98695d-c6fd-471f-934c-50faa59de0db",
    "room_id": "Y2lzY29zcGFyazovL3VzL1JPT00vOTE4YzQ2YTAtYWE0Yi0xMWSDFGDFYTNhYjY4M2Vk",
    "schedule": "1/60 * * * * *"
  }
]
```

##Release 
Les release ce situe dans release/

## Start

Windows
```bash
$ bot_monitor.exe  config.json
```

Linux
```bash
$ bot_monitor  config.json
```

## Notification

Un message est envoyé lorsque : 
* Changement d'état du service
* Toutes les 5 minutes si le service n'est pas up
