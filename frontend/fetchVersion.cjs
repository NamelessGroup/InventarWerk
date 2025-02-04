const { writeFileSync } = require('fs')

const version = require('child_process').execSync('git rev-parse HEAD').toString().trim()
const versionFilePath = 'src/utils/version.ts'
const versionFileContent = `export const version = '${version.substring(0,6)}'`
writeFileSync(versionFilePath, versionFileContent)