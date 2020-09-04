# BotMonitor  

Read this in other language: [French](https://github.com/modship/BotMonitor/blob/master/README.fr.md)

BotMonitor can monitor service by making http calls at regular intervals on it.

If the service does not respond the error message is transmitted in a webex room.

~ Setting up between 5 and 10Min

## Create webex bot

Link to the documentation for creating the bot : [here](https://developer.webex.com/docs/bots)

When creating the bot a token will be provided to you, you must keep it. If you lose it you have to generate a new token which causes the old token to be cancelled.


## Get the room_id
* Creating the room with your webex client
* [Here](https://developer.webex.com/docs/api/v1/rooms/list-rooms) to go to the webex api.
* Clic on "Run"
* Find the room your room in the response json to retrieve the field "id"


## Add the bot into your room
On the webex client :
* Select the room
* Go to the "People" tab
* Click on "Add People"
* Add the bot (with his name) to allow it to send messages.


## BotMonitor Configuration

Create a json file and place the following configuration :

Example with two service :
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

## Release 
Release are in /release folder

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

Message is sent when : 
* Change in service status
* Every 5 minutes if service is not up
