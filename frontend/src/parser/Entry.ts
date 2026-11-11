export type LineType = 'entries' | 'inset' | 'list' | 'section' | 'table' | 'quote'
export type Entry = string | ComplexEntry
export interface ComplexEntry {
  type: LineType
  name?: string
  entries?: Entry[]
  caption?: string
  colLabels?: string[]
  rows?: string[][]
  items?: string[]
}

const descriptionTranslator: Record<LineType, (entry: ComplexEntry, section?: boolean) => string> =
  {
    entries: entryParser, //"Ythryn Mythallar",
    inset: insetParser, //"Will of the Talon (Dormant)",
    list: listParser, //"Ythryn Mythallar",
    section: sectionParser, //"Keystone of Creation",
    table: tableParser, //"Xen'drik Trinket"
    quote: quoteParser //Iggwilv's Cauldron
  }


  function quoteParser(entry: ComplexEntry) {
    const lines: Array<string> = []
    for (const line of entry.entries ?? '') {
      if (typeof line === 'string') {
        lines.push(line)
      } else {
        const complexEntry: ComplexEntry = line
        lines.push(descriptionTranslator[line.type](complexEntry))
      }
    }
    lines[0] = `*"${lines[0]}`
    lines[lines.length - 1] = `${lines[lines.length - 1]}"*`
    return lines.join('\n')
  }

  function entryParser(entry: ComplexEntry, section?: boolean) {
    const lines: Array<string> = []
    //Should at least have one string line
    for (const line of entry.entries ?? '') {
      if (typeof line === 'string') {
        lines.push(line)
      } else {
        const complexEntry: ComplexEntry = line
        lines.push(descriptionTranslator[line.type](complexEntry))
      }
    }
    if (lines.length == 0) lines.push('')
    if ((section ?? false) == true && entry.name) {
      lines.unshift(`## ${entry.name}`)
    } else {
      lines[0] = `${entry.name ? '**' + entry.name + '**. ' : ''}${lines[0]}`
    }
    return lines.join('\n\n')
  }

  function insetParser(entry: ComplexEntry) {
    const lines: Array<string> = ['---', `**${entry.name ?? ''}**`]
    for (const line of entry.entries ?? '') {
      if (typeof line === 'string') {
        lines.push(line)
      } else {
        const complexEntry: ComplexEntry = line
        lines.push(descriptionTranslator[line.type](complexEntry))
      }
    }
    lines.push('---')
    return lines.join('\n\n')
  }

  function listParser(entry: ComplexEntry) {
    const lines: Array<string> = []
    for (const line of entry.items ?? '') {
      lines.push(`- ${line}`)
    }
    return lines.join('\n')
  }

  function sectionParser(entry: ComplexEntry) {
    const lines: Array<string> = []
    for (const line of entry.entries ?? '') {
      if (typeof line === 'string') {
        lines.push(line)
      } else {
        const complexEntry: ComplexEntry = line
        if (line.type == 'entries') lines.push(descriptionTranslator[line.type](complexEntry, true))
        else lines.push(descriptionTranslator[line.type](complexEntry))
      }
    }
    return lines.join('\n\n')
  }

  function tableParser(entry: ComplexEntry) {
    const lines: Array<string> = [
      ` *${entry.caption ?? ''}*`,
      `|${entry.colLabels?.join('|')}|`,
      `|${entry.colLabels?.map(() => '---').join('|')}|`
    ]
    for (const row of entry.rows ?? []) {
      lines.push(`|${row.join('|')}|`)
    }
    return lines.join('\n')
  }
