<img alt="Phat2Meet Contract description" src="https://gateway.pinata.cloud/ipfs/QmeF9GiNWR8jMWoGw6iLbTVoKymK34Stn22ewo3nh5P66V">

The Phat2Meet Contract is based on has been developed on ``ink! 4.1.0.``

The compiler of the contract was ``rustc 1.68.2``
For the json de/serialization, the crates ``pink-json version "0.4.0"`` has been used.

For the data encryption, the following package have been used
``aes-gcm-siv version = "0.11.1"``
``cipher version = "0.4.3"``
``bs58  version = "0.4"``

For the S3 storage, ``pink-s3 version = "0.4.1"`` have been used.

## Phat Contract pre-requirements

<a href="https://wiki.phala.network/en-us/build/stateless/setup/">Phat Contract Environment Setup</a>

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


## How it's implemented 
- ### The constructor.

At the implementation of the Phat2Meet PhatContract some variables are declared onChain like the S3 connexion informations :
        
  ```rust
  #[ink(constructor)]
  pub fn new() -> Self {
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

```rust
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

```rust
#[ink(message)]
        pub fn put_object_str(&self, object_key: String, payload: String) -> Result<()> {
        }
```

The Put function reiceve 2 parameters Object and payload.
The Object is the name of the file in the S3 will content datas transmited in the payload variable. The payload datas will be received as string in a json format.
Before to be sent datas are encrypted.

```rust
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

```rust
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

```rust
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
```    

This new meeting is recorded in the S3 storage and can be updated : 

```rust
  fn insert_meeting(&mut self, meeting: &Meeting)  {
    // Serialize it to a JSON string.            
    let meeting_string:String = pink_json::ser::to_string(&meeting).unwrap_or_default();            
    let obj_key = format!("{}{}", "txn_create_".to_string(), meeting.meeting_id);          
    self.put_object_str(obj_key, meeting_string)
        .or(Err(Error::S3ConnectionFailed));          
  }
``` 