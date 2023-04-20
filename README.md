# The ClosedBeta of PhalaNetwork

<a href="https://wiki.phala.network/en-us/build/general/intro/"><img align="right" width="320" style="margin-left: 20px" src="https://user-images.githubusercontent.com/16654460/233195645-1819d837-75b9-4406-affb-172236f187d6.png"></a>

Phat Contract Closed Beta is launched before the release of [Phat Contract](https://wiki.phala.network/en-us/build/general/intro/) on Phala mainnet. The aim of it is to monitor the experience of developers, identify any potential issues, and then address them in order to improve the quality of our service.

Both Phala's partner projects and individual developers are involved in this test. The contents to experience in the Closed Beta is listed in our [wiki](https://wiki.phala.network/en-us/build/general/closed-beta/). Individual developers need to apply for the test first by filling in this [form](https://forms.gle/sbCDHfu7t9SNvytB7).

Phala proposes a Closed Beta prize pool to encourage the engagement,  and this repository is for individual developers to submit their projects to win the rewards.

# PhalaTeamFR

<a href="https://linktr.ee/phalafr"><img align="right" width="160" alt="Capture d’écran 2023-01-17 093826" src="https://user-images.githubusercontent.com/16654460/233196636-bbcc60be-f6fe-4565-b9ed-474582d2e5ab.png">

The PhalaTeamFr is the french team organized by the french ambassadors for PhalaNetwork.
Our community is spans across various platforms where anyone can follow, discuss, train or share about PhalaNetwork.
We are present on [Telegram](https://t.me/phala_french), [Discord](https://discord.io/PhalaTeamFr), [Twitter](https://twitter.com/phala_fr), [Youtube](https://www.youtube.com/@phalafr?sub_confirmation=1), [Twitch](https://www.twitch.tv/phalanetworkfr), [github](https://github.com/PhalaTeamFR) and [medium](https://medium.com/phala-fran%C3%A7ais).
  
During the launch of the ClosedBeta for the PhatContract, the french ambassadors purpose formed a team to represent our community in this challenge. That was really ambitious because nobody in participants had knowledges in Blockchain developement and with the language Rust. To organize our project and show our progress to the french community, each wednesday at 21h since the 8th february we oraganized 11 online events on the fr discord.

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
- ### The constructor.
At the implementation of the Phat2Meet PhatContract some variables are declared onChain like the S3 connexion informations :
        
  ```
  #[ink(constructor)]
  pub fn default() -> Self {
            Self{
                admin: Self::env().caller(),
                s3_access: Some("YOUR_ACCESS_key".to_string()),
                s3_secret: Some("YOUR_SECRET_KEY".to_string()),
                s3_bucket: "phat2meet".to_string(),
                platform: "storj".to_string(),
                region: "us-east-1".to_string(),
                last_meeting_id: 0,
                modif_date: Self::env().block_timestamp()
            }
}
 
```
The setting of s3 connexion is apply by the function pub fn init_param()

```
#[ink(message)]
pub fn init_param(
            &mut self,
            s3_secret_key: String,
            s3_access_key: String,
            s3_platform: String,
            s3_region: String,
            s3_bucket: String
        ) -> Result<String> {
            if Self::env().caller() != self.admin {
                return Err(Error::NoPermissions);
            }
            self.s3_access = Some(s3_access_key);
            self.s3_secret = Some(s3_secret_key);
            self.platform  = s3_platform;
            self.region = s3_region;
            self.s3_bucket = s3_bucket;
        
            Self::env().emit_event(ParamUpdated {
                update_date: Self::env().block_timestamp(),
            });
            Ok("Secret keys updated".to_string())
  }
```  

  - ### The management of the s3 storage.
  Our datas which are stored in the S3 storage are mananged by the Phatconct thanks to 2 functions (Put & Get).
```#[ink(message)]
        pub fn put_object_str(&self, object_key: String, payload: String) -> Result<()> {
        }
```
The Put function reiceve 2 parameters Object and payload.
The Object is the name of the file in the S3 will content datas transmited in the payload variable. The payload datas will be received as string in a json format.
Before to be sent datas are encrypted.
```
  // Generate key and nonce
            let key_bytes: Vec<u8> = signing::derive_sr25519_key(object_key.as_bytes())[..32].to_vec();
            let key: &GenericArray<u8, U32> = GenericArray::from_slice(&key_bytes);
            let nonce_bytes: Vec<u8> = self.s3_access
                                            .as_ref()
                                            .unwrap()
                                            .as_bytes()[..12]
                                            .to_vec();
            let nonce: &GenericArray<u8, U12> = Nonce::<Aes256GcmSiv>::from_slice(&nonce_bytes);

            // Encrypt payload
            let cipher = Aes256GcmSiv::new(key.into());
            let encrypted_bytes: Vec<u8> = cipher.encrypt(nonce, payload.as_ref()).unwrap();
  ```
At the reception of datas from the S3, it will be decrypted and restitute to the computer as string in a json format too.

```
  let Ok(get_response) = s3.get(&self.s3_bucket.as_str(), &object_key.as_str()) else {return Err(Error::S3ConnectionFailed);};

            let key_bytes: Vec<u8> = signing::derive_sr25519_key(object_key.as_bytes())[..32].to_vec();
            let key: &GenericArray<u8, U32> = GenericArray::from_slice(&key_bytes);
            let nonce_bytes: Vec<u8> = self.s3_access
                                                        .as_ref()
                                                        .unwrap()
                                                        .as_bytes()[..12]
                                                        .to_vec();
            let nonce: &GenericArray<u8, U12> = Nonce::<Aes256GcmSiv>::from_slice(&nonce_bytes);   

            // Decrypt payload
            let cipher = Aes256GcmSiv::new(key.into());
            let decrypted_byte = cipher.decrypt(&nonce, &*get_response)
                                       .or(Err(Error::DecryptionFailed)).unwrap();
            // debug_println!("Decrypted result: {:?}", decrypted_byte);

            Ok(format!("{}", String::from_utf8_lossy(&decrypted_byte))) 
  ```  
- ### The management of our meetings.
The first function is `/// Create a new meeting`.
At the creation of a new meeting we set a collection of slots that define periodes when users can apply theire availabilities.
```
for item in result.slots_ranges.clone() {
                    if item.start >= item.end {
                        return Err(Error::InvalidDaySlot("Start should be higher than end date".to_string()));
                    }
                } 

                for item in result.hour_ranges.clone() {
                    if item.start >= item.end {
                        return Err(Error::InvalidHourSlot("Start hour should be higher than end hour".to_string()));
                    }
                }
                
                let copy_slots: Vec<_> = result.slots_ranges.iter().map(|x| x.clone()).collect();
                let copy_hours: Vec<_> =  result.hour_ranges.iter().map(|x| x.clone()).collect();

                let new_meeting = Meeting {
                                    meeting_id: self.last_meeting_id + 1,
                                    name: result.name,
                                    description:result.description,
                                    owner: format!("{:?}", Self::env().caller()),
                                    result_date: result.result_date,
                                    slots_ranges: copy_slots,
                                    hour_ranges: copy_hours,
                                    meeting_state: MeetingState::MeetingCreated,
                                    slot_state: SlotsState::SlotsOpen,
                                    participants: Vec::new(),
                                    created_by: result.created_by,
                                    creation_date: Self::env().block_timestamp()
                };
                self.insert_meeting(&new_meeting);
  ....
}
```    
This new meeting is recorded in the S3 storage and can be updated : 
```
        fn insert_meeting(&mut self, meeting: &Meeting)  {
            // Serialize it to a JSON string.
            // debug_println!(" ----- Insert Meeting ----- :\n {:?}", meeting);
            debug_println!(" ----- Insert Meeting ----- : ");
            let meeting_string:String = pink_json::ser::to_string(&meeting).unwrap_or_default();
            // debug_println!(" ////////// Meeting String//////////:\n {:?}", meeting_string);
            let obj_key = format!("{}{}", "txn_create_".to_string(), meeting.meeting_id);
            debug_println!(" ////////// Meeting object //////////:\n {:?}", obj_key);
            // self.meetings_descr.insert(meeting_id, name);
            self.put_object_str(obj_key, meeting_string)
                .or(Err(Error::S3ConnectionFailed));
            debug_println!(" ============== Meeting Saved ============== ");
        }


        fn update_meeting(&mut self, meeting: &Meeting, obj_value: String) -> Result<()> {
            // Serialize it to a JSON string.
            debug_println!(" ----- Update Meeting ----- :");
            // debug_println!(" ----- Update Meeting ----- :\n {:?}", meeting);
            let meeting_string:String = pink_json::ser::to_string(&meeting).unwrap_or_default();
            debug_println!(" ----- From Struct to Meeting ----- :");
            // debug_println!(" ////////// Meeting String //////////:\n {:?}", meeting_string);
            let Ok(_result1) = self.put_object_str(obj_value, meeting_string.clone()) else {return Err(Error::S3ConnectionFailed);};
                                // .or(Err(Error::S3ConnectionFailed));
            
            debug_println!(" ============== Meeting Updated ============== \n {:#?}", meeting_string);
            Ok(())
        }
``` 
  - ### The management of the slots.
In the same way we recorded a new meeting we recorded avalabilities for an event from users who fill the timeboard.
This function is the most complex of our Phat2Meet Phat contract and it seems to me that it is not very useful to put it here, so much it is long and deserves that you go to see it here :    
