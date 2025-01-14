import { ErrorHandler } from "@/errorHandling/ErrorHandler"
import type { ItemPreset } from "@/model/ItemPreset"
import { DatabaseHandler } from "@/store/DatabaseHandler"
import axios from "axios"
import { isInterfaceDeclaration } from "typescript"

interface ItemJSON {
    name: string,
    source?: string,
    page?: number,
    reqAttune?: string,
    weight?: number,
    entries: Entry[],
    value?: number,
    type?: string
}
interface ItemListJSON {
    item: ItemJSON[]
}
type Entry = string | ComplexEntry
interface ComplexEntry {
    type: LineType,
    name?: string,
    entries?: Entry[],
    caption?: string,
    colLabels?: string[],
    rows?: string[][],
    items?: string[]
}
  
const typeTranslator: Record<string,string> = {
    "undefined": "Other",
    "$A|DMG": "Treasure (art object)",
    "$A|XDMG": "Treasure (art object)",
    "$C": "Treasure (coinage)",
    "$G|DMG": "Treasure (gemstone)",
    "$G|XDMG": "Treasure (gemstone)",
    "A": "Ammunition",
    "AIR|DMG": "Vehicle (air)",
    "AIR|XPHB": "Vehicle (air)",
    "AT": "Artisan's tools",
    "AT|XPHB": "Artisan's tools",
    "EXP|DMG": "Explosive",
    "EXP|XDMG": "Explosive",
    "FD": "Food and drink",
    "FD|XPHB": "Food and drink",
    "G": "Adventuring gear",
    "GS": "Gaming set",
    "GS|XPHB": "Gaming set",
    "G|XPHB": "Adventuring gear",
    "HA": "Heavy Armor",
    "HA|XPHB": "Heavy Armor",
    "IDG|TDCSR": "Illegal drug",
    "INS": "Instrument",
    "LA": "Light Armor",
    "LA|XPHB": "Light Armor",
    "M": "Martial weapom",
    "MA": "Medium Armor",
    "MA|XPHB": "Medium Armor",
    "MNT": "Mount",
    "MNT|XPHB": "Mount",
    "M|XPHB": "Martial Weapon",
    "OTH": "Other",
    "P": "Potion",
    "P|XPHB": "Potion",
    "R": "Ranged weapon",
    "RD|DMG": "Rod",
    "RD|XDMG": "Rod",
    "RG|DMG": "Ring",
    "RG|XDMG": "Ring",
    "S": "Shield",
    "SCF": "Spellcasting Focus",
    "SCF|XPHB": "Spellcasting Focus",
    "SC|DMG": "Scroll",
    "SC|XPHB": "Scroll",
    "SHP": "Vehicle (water)",
    "SHP|XPHB": "Vehucle (water)",
    "SPC|AAG": "Vehicle (space)",
    "S|XPHB": "Shield",
    "T": "Tools",
    "TAH": "Tack and harness",
    "TAH|XPHB": "Tack and harness",
    "TB|XDMG": "Trade Bar",
    "TG": "Trade good",
    "TG|XDMG": "Trade good",
    "T|XPHB": "Tools",
    "VEH": "Vehicle (land)",
    "VEH|XPHB": "Vehicle (land)",
    "WD|DMG": "Wand",
    "WD|XDMG": "Wand",
}

type LineType = "entries" | "inset" | "list" | "section" | "table" | "quote";

const descriptionTranslator: Record<LineType, Function> = {
    entries: entryParser, //"Ythryn Mythallar",
    inset: insetParser, //"Will of the Talon (Dormant)",
    list: listParser, //"Ythryn Mythallar",
    section: sectionParser, //"Keystone of Creation",
    table: tableParser, //"Xen'drik Trinket"
    quote: quoteParser, //Iggwilv's Cauldron
}

function quoteParser(entry: ComplexEntry) {
    let lines: Array<string> = []
    for (const line of entry.entries??"") {
        if (typeof line === "string") {
            lines.push(line)
        } else {
            const complexEntry:ComplexEntry = line
            lines.push(descriptionTranslator[line.type](complexEntry))
        }
    }
    lines[0] = `*"${lines[0]}`
    lines[lines.length - 1] = `${lines[lines.length - 1]}"*`
    return lines.join("\n")
}

function entryParser(entry: ComplexEntry) {
    let lines: Array<string> = []
    //Should at least have one string line
    for (const line of entry.entries??"") {
        if (typeof line === "string") {
            lines.push(line)
        } else {
            const complexEntry:ComplexEntry = line
            lines.push(descriptionTranslator[line.type](complexEntry))
        }
    }
    if (lines.length == 0) lines.push("")
    lines[0] = `**${entry.name??""}**${lines[0]}`
    return lines.join("\n\n")
}

function insetParser(entry: ComplexEntry) {
    let lines: Array<string> = [
        "---",
        `**${entry.name??""}**`
    ]
    for (const line of entry.entries??"") {
        if (typeof line === "string") {
            lines.push(line)
        } else {
            const complexEntry:ComplexEntry = line
            console.log("line.type:", line.type);
            lines.push(descriptionTranslator[line.type](complexEntry))
        }
    }
    lines.push("---")
    return lines.join("\n\n")
    
}

function listParser(entry: ComplexEntry) {
    let lines: Array<string> = []
    for (const line of entry.items??"") {
        lines.push(`- ${line}`)
    }
    return lines.join("\n")
}

function sectionParser(entry: ComplexEntry) {
    let lines: Array<string> = [
        `## ${entry.name??""}`
    ]
    for (const line of entry.entries??"") {
        if (typeof line === "string") {
            lines.push(line)
        } else {
            const complexEntry:ComplexEntry = line
            lines.push(descriptionTranslator[line.type](complexEntry))
        }
    }
    return lines.join("\n\n")
}

function tableParser(entry: ComplexEntry) {
    let lines: Array<string> = [
        ` *${entry.caption??""}*`,
        `|${entry.colLabels?.join("|")}|`,
        `|${entry.colLabels?.map((x) => "---").join("|")}|`,
    ]
    for (const row of entry.rows ?? []) {
        lines.push(`|${row.join("|")}|`)
    }
    return lines.join("\n\n")
}


export async function parseItem(itemList: ItemListJSON) {
    const parsedItemList: Array<ItemPreset> = []

    for (const x  of itemList.item) {
        const parsedItem: ItemPreset = {
        name: "",
        uuid: "",
        description: "",
        price: 0,
        creator: "public-import",
        itemType: "",
        weight: 0,
        }
        parsedItem.name = x.name
        parsedItem.price = x.value?? 0
        parsedItem.weight = x.weight?? 0
        parsedItem.itemType = typeTranslator[x.type??"undefined"] ?? (() => {console.log(`Missing Type: ${x.type}`)})
        let lines: Array<String> = []
        for (const line of x.entries??"") {
            if (typeof line === "string") {
                lines.push(line)
            } else {
                const complexEntry:ComplexEntry = line
                lines.push(descriptionTranslator[line.type](complexEntry))
            }
        }
        parsedItem.description = lines.join("\n\n")
        parsedItemList.push(parsedItem)

    }
    for (const item of parsedItemList) {
        pushPresetToServer(item)
        await (new Promise( resolve => setTimeout(resolve, 50) ));
    }
    
}


async function pushPresetToServer(itemPreset: ItemPreset) {
    let params = new URLSearchParams({
        "creator": itemPreset.creator,
        "description": itemPreset.description,
        "item_type": itemPreset.itemType,
        "name": itemPreset.name,
        "price": "" + itemPreset.price,
        "weight": "" + itemPreset.weight
    })
    const response = await axios.put<unknown>(DatabaseHandler.BASE_URL + 'itemPreset/addExtern', {}, {
        params,
        withCredentials: true
      }).then((response) => response).catch((error) => error.response)
    if (response && response.status >= 200 && response.status < 300) {
      return
    } else {
      ErrorHandler.getInstance().registerError(
        new Error(
          `Could put extern itemPreset ${itemPreset.name} to Server due to: ${response.status} ${response.statusText}`
        )
      )
    }
}
