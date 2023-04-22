<div>
<div align="center">
  <img alt="Welcome Phat2Meet" src="https://gateway.pinata.cloud/ipfs/QmbG7B88PD33AYTnSVMEdsKK3RaZUGc942cxQCvbG5rHc7">
  <br>
  <br>
  </div>
    <div align="center">
    <a href="#about-phat">About Phat2Meet</a> 
      |
    <a href="#letsgo">Let's start</a>
      |
    <a href="#process">Process</a>
      |
    <a href="#about-team">About Teams</a>
  </div>
  <br>

  <img alt="View Phat2Meet" src="https://gateway.pinata.cloud/ipfs/QmTzKKk9ekJHrpayJ9AANqVqSYzwXMp6e5t7BVJQPohGrk">
</div>

----

<div>
  <img id="about-phat" alt="About Phat2Meet" src="https://gateway.pinata.cloud/ipfs/QmRd5ov9pknwimq9kFxpfsWRG5qs1MAzB7tdtAUemTgLvj">

  <a href="https://wiki.phala.network/en-us/build/general/intro/"><img align="right" width="320" style="margin-left: 20px" src="https://user-images.githubusercontent.com/16654460/233195645-1819d837-75b9-4406-affb-172236f187d6.png"></a>


  <p><b>Phat2Meet</b> is an online platform built on web3 technology, similar to <a href="https://www.when2meet.com/">when2meet</a>, aimed at facilitating the management of participant availability for an event.

  The main objective of this project is to preserve the confidentiality of each participant's availability while providing a simple and practical solution for event organizers. By using web3 technology, we are able to ensure the confidentiality of participant data while providing a smooth and user-friendly experience.

  The Phat2Meet platform allows users to select the time slots during which they are available for an event while protecting their confidentiality. Event organizers can easily check the availability of participants and choose the most appropriate time slot for the event.

  Phat2Meet uses the Phat Contract to ensure data confidentiality and data encrypted and stored and S3 Storage using pink dedicated crate Contract. For different use cases, the owner will always have access to the data, but others may or may not have access depending on the specified settings. For example : 
  - Use case 1: only the owner will have access to all the availibility and the user will only have access to his own availibility, 
  - Use case 2: the data will only be visible by invited contributors, 
  - Use case 3: All the contributors will have access to all availibility.

  We have developed a POC with use case 3 to demonstrate the power and flexibility of the Phat Contract.
 </p>

  <p> <b>More information:</b> </p>
    <ul>
    <li>Website - <a href="https://phat2meet-closedbeta-ui.netlify.app/">PHAT2MEET</a></li>
    <li>GitHub Repositories:</li>
    <ul>
      <li>Phat2Meet - <a href="https://github.com/PhalaTeamFR/Phat2meet-ClosedBeta">Phat2meet-ClosedBeta</a></li>
      <li>Front - <a href="https://github.com/PhalaTeamFR/Phat2meet-ClosedBeta/tree/main/Phat2meet-UI">Phat2Meet - UI</a></li>
      <li>Contracts - <a href="https://github.com/PhalaTeamFR/Phat2meet-ClosedBeta/tree/main/Phat2meet-Contract">Phat2Meet - Contracts</a></li>
    </ul>
  </ul>
</div>

----

<img id="letsgo" alt="Let's Go" src="https://gateway.pinata.cloud/ipfs/QmbjUT4rVNGkUaD14HCG1zaEGGdAD52qdyhbd8LZqzpj4x">

<!-- Pre Requierment -->

## Phat Contract pre-requirements

<a href="https://wiki.phala.network/en-us/build/stateless/setup/">Phat Contract Environment Setup</a>

## Clone repository on GitHub

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

<!-- Build Contract -->

## Setup Phat Contract

```
$ cd Phat2meet-ClosedBeta/Phat2meet-contracts/
$ cargo contract build
```

Once the build is finished, go to the ``./target/ink`` folder and get the following files: 

- ``phat2meet.contract`` (To initialize our contract with the Phala Phat Contract UI)
- ``phat2meet.json`` (To interconnect our contract with the Phat2Meet UI)

Once the files are saved, go to the <a href="https://phat-cb.phala.network/">Phala PhatContract UI</a> to initialize your contract


## Contract initialization

 <p><b>/!\ To use the <a href="https://phat-cb.phala.network/">Phala PhatContract UI</a></b>, you need to have Phala Test tokens. If you don't have any, you can get them on the Phala Test Network by following the instructions in the <a href="https://wiki.phala.network/en-us/build/getting-started/deploy-contract/#claim-test-tokens">official PhalaNetwork documentation</a> or ask directly on the <a href="https://discord.gg/phala">Discord</a>.
  </p>

<img alt="Step 1 and 2 contract" src="https://gateway.pinata.cloud/ipfs/QmPcCGjw3tTSvfov7515rGNPXAFTsFjQqCL9YqsjHhj35F">

- **Step 1:** Connect to the Phala PhatContract UI with your wallet 
- **Step 2:** Click on the **"Upload"** button 

<img alt="Step 3 contract" src="https://gateway.pinata.cloud/ipfs/QmcndFCaNAsN5tPSt4PVBrsDekJy1ane4A3igJ4RXSARt4">

- **Step 3:** Add the file ``phat2meet.contract`` retrieved earlier and click on the button **"Submit"**, sign with your wallet.
 
<img alt="Step 4 and 5 contract" src="https://gateway.pinata.cloud/ipfs/QmP2D9ux4Y5NaHGuU1Fjn8BXBQUF3a7WtGWQc7VAJF4JU7">

- **Step 4:** On the main Dashboard, click on **"Init_param"** for initialize your S3 information.
- **Step 5:** Add your S3 login information in the corresponding fields and click on **"Run"**.

<img alt="Step 6 and 7 contract" src="https://gateway.pinata.cloud/ipfs/QmVHhJn2kHyEbq3PUuq623hYNyH2YDtK877RSwZ82fgZLq">

- **Step 6:** On the main Dashboard, click on **"create_meeting"** for create a new meeting.
- **Step 7:** Fill in the field **json_meeting_file** according to the JSON format below and click on **"Run"**

```json
{
  "name": "Create Meeting #1",  
  "description": "TestValue",  
  "result_date": 1679498180,  
  "slots_ranges": [
    {
      "start": 1681084999,
      "end": 1681257599
    },{
      "start": 1681344000, 
      "end": 1681948799
      }
    ],  
  "hour_ranges": 
    [        
      {
          "start": "1400", 
          "end": "1900"
      }
    ],  
  "created_by": "Change"
}
```

**Congratulations !** Your meeting is created

<!-- Instantiate Phat Contract -->

## Instantiate Phat Contract in Phat2Meet UI

- **Step1:** After getting the above ``phat2meet.json`` file, you will have to add it in the ``./Phat2meet-ClosedBeta/Phat2meet-ui/src/contract/`` folder

<br>

- **Step2:** You will have to modify the contract address in the file ``./Phat2meet-ClosedBeta/Phat2meet-ui/src/context/ContractCall.jsx`` line 84 

```javascript
      const contractId = "0xe7a02339c4dc2b233e3d81c06fd30c21a3bfe0f03972513ccd9ee2db508425b4"
```

**Congratulations !** Your Phat contract adresse as updated in Phat2Meet UI

<!-- Installation UI -->

## Install the dependencies in Phat2Meet UI

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

----

<div>
  <img id="process" alt="Process Phat2Meet" src="https://gateway.pinata.cloud/ipfs/QmeH1HD1J8VvtzgAi7hrWgdFh6zHLxEbboP25RKgoPdZHA">

  <h3>The creation of the Meeting is generated only by <a href="https://phat-cb.phala.network/">the Phala PhatContract UI</a> (Under development)</h3>
  <br>
  <img alt="" src="https://gateway.pinata.cloud/ipfs/QmSk9goaB7b8mns4bUeP9LLFpo9QDoSehr4C1TtvB866nH">

  <p><b>/!\ To use the Phat2Meet platform</b>, you need to have Phala Test tokens. If you don't have any, you can get them on the Phala Test Network by following the instructions in the <a href="https://wiki.phala.network/en-us/build/getting-started/deploy-contract/#claim-test-tokens">official PhalaNetwork documentation</a> or ask directly on the <a href="https://discord.gg/phala">Discord</a>.
  </p>


  <img alt="Step 1 and 2" src="https://gateway.pinata.cloud/ipfs/QmX8b9n5nHTc3gKt8VCD6FxQtFd8VqkgRxRbmrjwd5U3Vx">

  ### Step 1: Wallet Login -> Select a wallet
  
  <p>
  The first step is to connect your wallet to the Phat2Meet platform. To do so, select your preferred wallet from the available options (e.g. Polka.js or Talisman) and log in using your login information
  </p>

  ### Step 2: Set your Username
  
  Once you are logged into your wallet, you can now add your username. Enter the username you would like to use on the platform. Be sure to choose a username that does not contain sensitive personal information, then click <b>"Save"</b>.
  </p>

  <img alt="Step 3" src="https://gateway.pinata.cloud/ipfs/QmVQ4fTfznEgNtANYP2CszuR6b5zAZcZcwNF4APdGsPQZf" >

  ### Step 3: Choose your time slots
  
  The next step is to choose your available time slots for online dating. To do this, click on the <b>"Your Availability"</b> tab and select the hours of the day and days of the week that you are available to meet other members of the community.
  </p>

  <img alt="" src="https://gateway.pinata.cloud/ipfs/QmUNEsoGP9R5Gy7L6TpWnZ9XMUwZf8LwLdzUr69WgMrGS5">
  
  ### Step 4: Check if the slots are good in group
  
  Before sending your transaction, check that the slots you have selected are compatible with the slots of other community members. To do this, click on the <b>"Group availability"</b> tab and check that your slots are displayed
  </p> 
  
  <img alt="Step 5" src="https://gateway.pinata.cloud/ipfs/QmV1x2wAYJn6M4UY5E82a9tk28uyduQSn5A3WvT5wjzZoR">

  ### Step 5: Send Tx

  Once you have verified that the time slots are compatible, you can now send your transaction by clicking the <b>"Send TX"</b> button and sign your transaction with your wallet. 

  <b>/!\ Be sure to verify all information before sending the transaction, as it cannot be undone once it has been sent.</b>
  </p> 
</div>

----

<div>
  <img id="about-team" alt="About Teams" src="https://gateway.pinata.cloud/ipfs/QmQU1vQ1RXZqqVuSqkMFqGLXHN8kmH9o6bGgEmp8476dVp">

     
  <br>
    <p>The PhalaTeamFr is a French team organized by PhalaNetwork ambassadors. This community is active on several platforms and allows its members to follow PhalaNetwork news, discuss, train or share their experiences.</p>

  <img alt="Team Dev Phat2Meet" src="https://gateway.pinata.cloud/ipfs/QmRwpntgfVToNW2TsZTbsFG53RABs78CJyq4SbbmLjLfm8">

  <p>The Phat2Meet project is initiated by PhalaNetwork France ambassadors, in collaboration with a team of passionate community members. Together we have worked to create an innovative tool that will meet the needs of our community.
  </p>

  <p>The challenge was great because no one among the participants had any real knowledge of the Rust language. However, our team was determined to meet this challenge and show the French community that we were up to the task.</p>

  <p>Our team is composed of passionate, determined and committed members. We have worked hard to develop our project. We have put in place a rigorous organization to ensure a steady and efficient progress of our project.</p>

  <p>To organize our project and show our progress to the community, we organized 11 online events on our Discord fr, every Wednesday at 9pm since February 8th. These events have been a success and have allowed our team to present our progress, answer questions from the community and receive valuable feedback to improve our project.
  </p>

  <p>We are proud of what we have accomplished so far and look forward to continuing to work to improve our project and meet the community's expectations.
  </p>

  <p>We would like to thank all the team members for their dedication and hard work, as well as the French community for their constant support.</p>

  <p>We hope our project will inspire others to get into Blockchain and take on ambitious challenges. Stay tuned for more updates on our exciting project!</p>

  <div align="center">
    <a href="https://t.me/phala_french">Telegram</a> | <a href="https://discord.io/PhalaTeamFr">Discord</a> | <a href="https://twitter.com/phala_fr">Twitter</a> | <a href="https://www.youtube.com/@phalafr?sub_confirmation=1">Youtube</a> | <a href="https://www.twitch.tv/phalanetworkfr">Twitch</a> | <a href="https://github.com/PhalaTeamFR">GitHub</a> | <a href="https://medium.com/phala-fran%C3%A7ais">Medium</a></p>
  </div>
</div>
