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
        description: string
        creator: string
        itemType: string
    }

    class Item {
        name: string
        uuid: string
        presetReference: string
        amount: number
        dmNote: string
        description: string
    }

    Inventar --> Item
```

## Schnittstellen
### Inventar
#### /inventar/all
Get
#### /inventar?inventory_uuid=""
Get
#### /inventar?inventory_uuid="",name=""
Put
#### /inventar/addPreset?inventory_uuid="",preset_uuid="",amount=""
Put
#### /inventar/addNew?name="",amount=""
Put
#### /inventar/money?amount=""
Patch
#### /inventar/share?uuid=""
Patch
machts public
#### /inventar/share?uuid="",read="",write=""
Patch
#### /inventar/delete?uuid=""
Delete
### Item
#### /item/edit?uuid="",name="",amount="",description=""
Patch
name, amount and description are optional
### ItemPreset
#### /itemPreset?uuid=""
Get
#### /itemPreset/modify?uuid="",name="",price="",text=""
Patch
all optional
#### /itemPreset/delete?uuid=""
Delete
#### /itemPreset/all
Get
Response: List of {name: string, itemType:string}
### Account
#### /account/get
Get
return all accounts
#### /account/isDm?uuid=""
Get
### Note
#### /note/add?uuid="",note=""
Patch
### last Changes
#### /lastChanges?timestamp=""
Get
time stamp of last fetch in millis
Response: {uuid: string, type: 'create'|'patch'|'delete'}[]


# Backend
##
required fields in .enc in backend/
```
DATABASE_URL=
DISCORD_CLIENT_ID=
DISCORD_CLIENT_SECRET=
DISCORD_REDIRECT_URI=
```
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
    inventory 1+--1 user: "owned by/owns"
    
    inventory {
        text uuid PK
        text owner_uuid FK
        integer money
        text name
    }

    inventory_reader 1+--1+ inventory: "reads/read by"
    inventory_reader 1+--1+ user: "contains/contained by"

    inventory_reader {
        text user_uuid PK
        text inventory_uuid PK
    }
    
    inventory_writer 1+--1+ inventory: "writes/written by"
    inventory_writer 1+--1+ user: "contains/contained by"

    inventory_writer {
        text user_uuid PK
        text inventory_uuid PK
    }


    item_preset {
        text uuid PK
        text name
        integer price
        text description
        text creator
        text itemType
    }
    user {
        text uuid PK
        text name
        boolean dm
    }
    inventory_item 1+--1+ inventory: ""
    inventory_item 1+--1+ item_preset: ""
    inventory_item {
        text inventory_uuid PK
        text itempreset_uuid PK
        text dm_note
        integer amount
    }
```
