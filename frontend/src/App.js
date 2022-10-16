import { useEffect, useState } from "react";
import Blockie from "./Components/Blockie";
import axios from "axios";

function App() {

  const [ledgerState, setLedgerState] = useState([]);
  const [loaded, setLoaded] = useState(false);

  const callLedgerState = async () => {
    console.log('cookies');

    axios.get(`http://localhost:4444/showledger`)
    .then(function (response) {
      // handle success
      console.log(response.data.split('\n'));
      setLoaded(true);
      setLedgerState(response.data.split('\n'));

    })
    .catch(function (error) {
      // handle error
      console.log(error);
    })
    .finally(function () {
      // always executed
      console.log('show ledger server call complete')
    });
  } 

  useEffect(() => {
    callLedgerState();
  }, []);

  const ledgerMap = ledgerState.map((thing, i) => {
    return (
      <Blockie key={i} data={thing} />
    )
  })

  const styles = {
    typicalCenter: {
      border:'1px solid gray', borderRadius:"13px", padding: "2px", textAlign:"center"
    }
  }

  return (
    <div className="App">
      <div style={styles.typicalCenter}>
        { !loaded ? (
          <div>Loading...</div>
        ) : <div>Load successful!</div> }
        <button onClick={callLedgerState}>Load Ledger State</button>
      </div>
      <div>
        {ledgerMap}
      </div>
    </div>
  );
}

export default App;
