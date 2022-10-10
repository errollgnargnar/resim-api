const express = require('express')
const { exec } = require("child_process");

const app = express()
app.use(express.json());
const port = 4444

let accounts = [];
let packages = [];

exec("bash ../shscripts/index.sh reset", (err, stdout,stderr) => {
  if (err) {
    console.log(`error: ${err.message}`);
    return;
  }
  if (stderr) {
    console.log(`stderr: ${stderr}`);
    return;
  }
  let newAccnt = stdout.split(' ');
  accounts.push(newAccnt);
  accounts[accounts.length-1][2] = accounts[accounts.length-1][2].replace('\n','');
})

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
    let newAccnt = stdout.split(' ');
    accounts.push(newAccnt);
    accounts[accounts.length-1][2] = accounts[accounts.length-1][2].replace('\n','');
    res.send(accounts);
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
    let newAccnt = stdout.split(' ');
    accounts.push(newAccnt);
    accounts[accounts.length-1][2] = accounts[accounts.length-1][2].replace('\n','');
    res.send(accounts);
  })
});

app.get('/newtreasury', (req, res) => {
  exec(`bash ../shscripts/index.sh new_treasury ${packages[0][0]}`, (err, stdout, stderr) => {
    packages[0].push(stdout);
    packages[0][packages[0].length-1] = packages[0][packages[0].length-1].replace('\n','');
    res.send(packages);
  })
});

app.get('/getpackages', (req,res) => {
  res.send(packages);
})

app.listen(port, () => {
  console.log(`Example app listening on port ${port}`)
})