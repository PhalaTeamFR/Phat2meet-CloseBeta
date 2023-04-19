# The ClosedBeta of PhalaNetwork

<a href="https://wiki.phala.network/en-us/build/general/intro/"><img align="right" width="320" style="margin-left: 20px" src="https://user-images.githubusercontent.com/16654460/233195645-1819d837-75b9-4406-affb-172236f187d6.png"></a>

Phat Contract Closed Beta is launched before the release of [Phat Contract](https://wiki.phala.network/en-us/build/general/intro/) on Phala mainnet. The aim of it is to monitor the experience of developers, identify any potential issues, and then address them in order to improve the quality of our service.

Both Phala's partner projects and individual developers are involved in this test. The contents to experience in the Closed Beta is listed in our [wiki](https://wiki.phala.network/en-us/build/general/closed-beta/). Individual developers need to apply for the test first by filling in this [form](https://forms.gle/sbCDHfu7t9SNvytB7).

Phala proposes a Closed Beta prize pool to encourage the engagement,  and this repository is for individual developers to submit their projects to win the rewards.

# PhalaTeamFR

<a href="https://linktr.ee/phalafr"><img align="right" width="160" alt="Capture d’écran 2023-01-17 093826" src="https://user-images.githubusercontent.com/16654460/233196636-bbcc60be-f6fe-4565-b9ed-474582d2e5ab.png">

The PhalaTeamFr is the french team organized by the french ambassadors for PhalaNetwork.
Our community is organised around several place where anyone who want to understand the power of the project can follow, discuss, train or share about it.
We are present on [Telegram](https://t.me/phala_french), [Discord](https://discord.io/PhalaTeamFr), [Twitter](https://twitter.com/phala_fr), [Youtube](https://www.youtube.com/@phalafr?sub_confirmation=1), [Twitch](https://www.twitch.tv/phalanetworkfr), [github](https://github.com/PhalaTeamFR) and [medium](https://medium.com/phala-fran%C3%A7ais).
  
At the launch of the ClosedBeta for the PhatContract, french ambassadors purpose to constitute a team to represent our community in this challenge. That was really ambitious at the beguinning because nobody in participants had knowledges in Blockchain developement and with the language Rust. To organize our project and show our progress to the french community, each wednesday at 21h since the 8th february we oraganized 11 online events on the fr discord.

# The Phat2meet

The idea was to rebuild a web2 tool used to know the avalabilities of participants to an event.
https://www.when2meet.com/.

"Nobody need to know than i'm available each monday at 1pm"

To build the Phat2meet we split the team in 2 to build the FrontEnd & the BackEnd.

Our FrontEnd is devellop in React and integrates the sdk from @polkadot/extension-dapp, the polkadot API and the phalaNetwork SDK.
Thanks @ricardo-eth for the incredible work you delivered and @arnobase for your help about the Phala SDK integration.

For the BackEnd, the 3 first weeks was complicated. As begginers with Rust & ink! we began to study the Phat-Hello.
We learn how to install an rust environement with cargo, compile a PhatContract and import it in the [closebeta UI](phat-cb.phala.network)
After that we created our first PhatContact to write and read a data onChain and then in a S3 storage.
Indeed, for our project we would to store our datas in a Simple Storage Service to try the Http request ✅, the gain in speed of off-chain processing ✅. but also, to make an idea by ourselves concerning the security and the privacy ✅.
This period need from us a long time to fight against dependencies and noobs errors. And the passage from the Ink3! to Ink4! don't facilitate our game.
once these first steps have been taken in large part and thanks to @Janemake our Phat2Meet project could start to build.

## How it work 
- ### The implementation
