const fs = require('fs')

const readFile = path => {
  const start = process.hrtime.bigint()
  const arr = fs.readFileSync(path).toString().split('\n')
  const end = process.hrtime.bigint()
  return [arr, (end - start) / 1000000n]
}

const readFileIntoSet = path => {
  const start = process.hrtime.bigint()
  const arr = fs.readFileSync(path).toString().split('\n')
  const set = new Set(arr)
  const end = process.hrtime.bigint()
  return [set, (end - start) / 1000000n]
}

const writeFile = (path, data) => {
  const start = process.hrtime.bigint()
  fs.writeFileSync(path, data)
  const end = process.hrtime.bigint()
  return (end - start) / 1000000n
}

void async function () {
  const startTotal = process.hrtime.bigint()

  const [database, dbElapsed] = readFile('database_dump.csv')
  const [mixpanel, mpElapsed] = readFileIntoSet('mixpanel_dump.csv')
  const mixpanelFullSize = mixpanel.size

  const startFormat = process.hrtime.bigint()
  const data = []
  for (const x of database) {
    if (x.length === 73) {
      const [userId, cardId] = x.split(';')
      if (mixpanel.has(userId)) {
        mixpanel.delete(userId)
        data.push(`${userId};${cardId}`)
      }
    }
  }
  const endFormat = process.hrtime.bigint()

  const formattedData = data.join("\n")
  const writeElapsed = writeFile('output.csv', formattedData)

  const endTotal = process.hrtime.bigint()
  console.log('Time reading file database_dump.csv:', dbElapsed)
  console.log('Time reading file mixpanel_dump.csv:', mpElapsed)
  console.log('Database file rows:', database.length)
  console.log('Mixpanel file rows:', mixpanelFullSize)
  console.log('Time formatting file:', (endFormat - startFormat) / 1000000n)
  console.log('Time writing file output.csv:', writeElapsed)
  console.log('Total time used in milliseconds:', (endTotal - startTotal) / 1000000n)
}()
    .catch((error) => console.error(error))