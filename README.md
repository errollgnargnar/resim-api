## About

Express server api interacts with Scrypto/Radix blockchain simulator. The simulator requires CLI commands to interact with the private local blockchain. This project contains an Express server which sets up end points to trigger bash scripts to interact with the simulator. The Scrypto smart contract was created by a partner in ACE DAO and has not been pushed to this repo for his own intellectual property. There is however, all functions within the bash scripts which execute standard Radix/Scrypto simulator commands. 

## FrontEnd
React pulls data from Express Server Endpoints (API) to create rendered components.  React front end shows ledger state where the component's background color will be based on data type returned (black or yellow - divider or address, respectively), and a Blockie Icon with the returned hash/value as its seed. Next task would be to add a webhook where the front end will automatically update with changes to the ledger

## How to Run
- Close this repo - cd into repo
- cd into express directory and run 
`npm install`, then run `npm start`
- open a new terminal and cd into 
the frontend directory and run `npm install` and `npm start`
- open browser and go to 'localhost:3000'
- have fun!
