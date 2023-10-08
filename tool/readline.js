const fs = require('fs')
const path = require('path')

const text = fs.readFileSync(path.join(__dirname, 'input.rs')).toString()

fs.writeFileSync(
    path.join(__dirname, 'output.txt'),
    text.split("\n")
        .map(line => {

            return `"${line.replace(/"/g, '\\"')}"`
        })
        .join(",\r\n")
)