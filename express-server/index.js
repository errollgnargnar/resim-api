const express = require('express')
var cors = require('cors')
const { exec } = require("child_process");

const app = express()
app.use(express.json());
app.use(cors());

const port = 4444;

let accounts = [];
let packages = [];

/// reset simulator on npm start
exec("bash ../shscripts/index.sh reset", (err, stdout,stderr) => {
  if (err) {
    console.log(`error: ${err.message}`);
    return;
  }
  if (stderr) {
    console.log(`stderr: ${stderr}`);
    return;
  }
  let accntParse = stdout.split(' ');
  let newAccnt = { accnt: accntParse[0], pub: accntParse[1], priv: accntParse[2].replace('\n','')}
  accounts.push(newAccnt);
})

/////// GENERIC/INDEX ENDPOINTS
app.get('/reset', (req,res) => {
  accounts = [];
  packages = [];
  exec("bash ../shscripts/index.sh reset", (err, stdout,stderr) => {
    if (err) {
      console.log(`error: ${err.message}`);
      return;
    }
    if (stderr) {
      console.log(`stderr: ${stderr}`);
      return;
    }
    let accntParse = stdout.split(' ');
    let newAccnt = { accnt: accntParse[0], pub: accntParse[1], priv: accntParse[2].replace('\n','')};
    accounts.push(newAccnt);
    console.log(`Resim simulator reset successful \n`);
    console.log(accounts);
    res.send(`Resim simulator reset successful \n`);
  })
});

/*
POST publish example:
curl -X POST localhost:3000/publish -H "Content-Type: application/json" -d '{"dir": "~/radix_dev/the_a_team/dao/scrypto/", "packageName": "ace treasury"}'
*/
app.post('/publish', (req, res) => {
  const dir = req.body.dir;
  const packageName = req.body.packageName;
  const fullCmd = `bash ../shscripts/index.sh publish ${dir} "${packageName}"`;
  exec(fullCmd, (err, stdout, stderr) => {
    packages.push(stdout.split(','));
    packages[packages.length-1][1] = packages[packages.length-1][1].replace('\n','');
    res.send(packages);
  });
  // res.sendStatus(200);
});

app.get('/getaccounts', (req, res) => {
  console.log(accounts);
  res.send(accounts);
})

app.get('/newaccount', (req,res) => {
  exec("bash ../shscripts/index.sh newaccnt", (err, stdout,stderr) => {
    if (err) {
      console.log(`error: ${err.message}`);
      return;
    }
    if (stderr) {
      console.log(`stderr: ${stderr}`);
      return;
    }
    let accntParse = stdout.split(' ');
    let newAccnt = { accnt: accntParse[0], pub: accntParse[1], priv: accntParse[2].replace('\n','')};
    accounts.push(newAccnt);
    console.log(accounts);
    res.send(accounts);
  })
});

app.get('/showledger', (req, res, next) => {
  console.log(`server pinged by ${req.ip}`);
  
  exec(`bash ../shscripts/index.sh showledger`, (err, stdout, stderr) => {
    if (err) {
      console.log(`error: ${err.message}`);
      res.send(`error: ${err.message}`);
      return;
    }
    if (stderr) {
      console.log(`stderr: ${stderr}`);
      res.send(`stderr: ${stderr}`);
      return;
    }
    console.log('Current Ledger State');
    console.log(stdout);
    res.send(stdout);
  })
})

// packages are blueprints/smart-contracts right?
app.get('/getpackages', (req,res) => {
  res.send(packages);
})

/////// CONTRACT SPECIFIC ENDPOINTS /////////
app.get('/newtreasury', (req, res) => {
  exec(`bash ../shscripts/contract_specific.sh new_treasury ${packages[0][0]}`, (err, stdout, stderr) => {
    packages[0].push(stdout);
    packages[0][packages[0].length-1] = packages[0][packages[0].length-1].replace('\n','');
    res.send(packages);
  })
});

///// listen....
app.listen(port, () => {
  console.log(`Example app listening on port ${port}`)
})