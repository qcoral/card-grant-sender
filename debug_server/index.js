const express = require('express')
const app = express()
const port = 3000

app.post('/', (req, res) => {
  res.send('Request Received')
  console.log(req)
})

app.listen(port, () => {
  console.log(`Example app listening on port ${port}`)
})
