# InventarWerk

## Anforderungen
- Verschiede Inventare
  - Teil-Stufen: privat (geteilt nur mit dir), geteilt (m r/w), public
- Inventar verwaltet Items
- Jedes Inventar hat Geld
- Mathe in Betragsmengen-Feldern
- DM Notes
- Account System (Discord)
- Item presets speichern
- Jedes Item hat: Name, Wert, text

## Structure
```mermaid
classDiagram
    class Inventar {
        uuid: string
        name: string
        owner: string
        money: number
        reader: string[]
        writer: string[]
    }

    class ItemPreset {
        uuid: string
        name: string
        price: number
        text: string
        creator: string
        itemType: string
    }

    class Item {
        name: string
        uuid: string
        presetReference: string
        amount: number
        dmNote: string
    }

    Inventar --> Item
```

## Schnittstellen
### /inventar/all
Get
### /inventar?uuid=""
Get
### /itemPreset?uuid=""
Get
### /inventar?name=""
Put
### /inventar/addPrest?presetUuid="",amount=""
Put
### /inventar/addNew?name="",amount=""
Put
### /inventar/delete?uuid=""
Delete
### /inventar/money?amount=""
Patch
### /inventar/modifyAmount?uuid="",amount=""
Patch
### /inventar/share
Patch
machts public
### /inventar/share?userid="",write=""
Patch
### /account/get
Get
return all accounts
### /note/add?uuid="",note=""
Patch
### /itemPreset/modify?uuid="",name="",price="",text=""
Patch
all optional
### /itemPreset/delete?uuid=""
Delete
### /lastChanges
Get
time stamp of last changes in millis
### /itemPreset/all
Get
### /account/isDm
Get


# Backend
## Prerequisites
install libsqlite3-dev
```
sudo apt update
sudo apt install libsqlite3-dev
```

set `DATABASE_URL=` in .env in backend/
change migrations path in diesel.toml in backend/

## Database Structure
```mermaid

erDiagram
    inventory {
        text uuid PK
        text owneruuid FK
        integer money
        text name
    }
    inventory 1+--1 user: "owned by/owns"
    inventory 1--0+ item: "contains/contained by"
    inventoryReadersList {
        integer id PK
        text useruuid FK
        text inventoryuuid FK
    }
    inventoryReadersList 1+--1+ inventory: "reads/read by"
    inventoryReadersList 1+--1+ user: "contains/contained by"
    inventoryWritersList {
        integer id PK
        text useruuid FK
        text inventoryuuid FK
    }
    inventoryWritersList 1+--1+ inventory: "writes/written by"
    inventoryWritersList 1+--1+ user: "contains/contained by"
    item {
        text uuid PK
        text name
        text presetReference FK
        integer amount
        text dmnote
        text inventoryuuid FK
    }
    itempreset {
        text uuid PK
        text name
        integer price
        text text
        text creator
        text itemType
    }
    itempreset 1--0+ item: "creates/created by"
    user {
        text uuid PK
        text name
        boolean dm
    }
```
