const fs = require('fs')
const path = require('path')

const text = fs.readFileSync(path.join(__dirname, 'input.rs')).toString()

fs.writeFileSync(path.join(__dirname, 'output.txt'), text.split("\r\n").map(line=>`"${line}"`).join(",\r\n"))