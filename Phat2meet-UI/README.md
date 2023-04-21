<img alt="Phat2Meet UI description" src="https://gateway.pinata.cloud/ipfs/Qmd3cHAdWaPsVuj3zi6KeSe4wtTBodPLGEGyNTFBc6dZfH">

In this project, we used React and Vite for development. We created a user interface with Chakra UI, integrated the Phala SDK and Polkadot API to add functionality and facilitate Web3 integration. We also managed dates and times with Dayjs and adopted Jotai for state management. 


```json
"dependencies": {
    "@chakra-ui/react": "^2.5.0",
    "@phala/sdk": "^0.4.0-nightly-20230321",
    "@polkadot/api": "^10.1.3",
    "@polkadot/api-contract": "^10.1.3",
    "@polkadot/wasm-crypto": "^7.0.3",
    "@polkadot/extension-dapp": "^0.44.6",
    "dayjs": "^1.11.7",
    "jotai": "^2.0.3",
  },
  "devDependencies": {
    "@types/react": "^18.0.27",
    "vite": "^4.1.0"
  }
```

# Setup

## Clone of our repository on GitHub

You must start by cloning our Github repository [Phat2meet-ClosedBeta](https://github.com/PhalaTeamFR/Phat2meet-ClosedBeta).

***SSH clone:***

```
$ git clone git@github.com:PhalaTeamFR/Phat2meet-ClosedBeta.git
$ cd Phat2meet-ClosedBeta
```

***or HTTPS clone:*** 

```
$ git clone https://github.com/PhalaTeamFR/Phat2meet-ClosedBeta.git
$ cd Phat2meet-ClosedBeta
```

<!-- Installation UI -->

## Install the dependencies for Phat2Meet UI

```
$ cd Phat2meet-ClosedBeta/Phat2meet-UI/
$ yarn install
```

**Congratulations !** Clone and the dependencies are well installed


## Launch the local server

To launch the site locally type the following command:

```
$ yarn dev
```

```js
 VITE v4.1.1  ready in 522 ms

  ➜  Local:   http://localhost:5173/
  ➜  Network: use --host to expose
  ➜  press h to show help
```

/!\ The *localhost:5173* address is specific to my local configuration. Please refer to your local network configuration
