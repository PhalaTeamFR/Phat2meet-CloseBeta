#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

use pink_extension as pink;


#[pink::contract(env=PinkEnvironment)]
mod phat2meet {    
    use super::pink;
    use ink::env::{debug_println, account_id};
    use ink::prelude::{
        format,
        string::{String, ToString},
        vec,
        vec::Vec,
    };  

    use serde::{Deserialize, Serialize};
    use pink_json;
    use pink_s3 as s3;
    
    use pink::{PinkEnvironment};
    use pink::chain_extension::{signing};

    use scale::{Decode, Encode};

    use core::fmt;

    // Management of date format
    use chrono::{Datelike, Timelike, TimeZone, Utc, NaiveDateTime, DateTime, Duration};

    // To encrypt/decrypt HTTP payloads
    use aes_gcm_siv::aead::{Aead, KeyInit, Nonce};
    use aes_gcm_siv::Aes256GcmSiv;
    use cipher::{consts::{U12, U32}, generic_array::GenericArray};

    /// Type alias for the contract's result type.
    pub type Result<T> = core::result::Result<T, Error>;


    #[derive(Debug, Eq, PartialEq, Encode, Decode, Copy, Clone, Serialize, Deserialize)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct SlotDay {
        pub start: i64,
        pub end: i64,
    }

    #[derive(Debug, Eq, PartialEq, Encode, Decode, Clone, Serialize, Deserialize)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub struct SlotHour {
        pub start: String,
        pub end: String,
    }

    #[derive(Debug, Eq, PartialEq, Encode, Decode, Clone, Serialize, Deserialize)]
    #[cfg_attr(not(feature = "std"), no_std)]
    pub struct Meeting {
                    meeting_id: u32,
                    name: String,
                    description:String,
                    owner: String,
                    result_date: u64,
                    slots_ranges: Vec<SlotDay>,
                    hour_ranges: Vec<SlotHour>,
                    meeting_state: MeetingState,
                    slot_state: SlotsState,
                    participants: Vec<MeetingMember>,
                    created_by: String,
                    creation_date: u64,
    }


    #[derive( Debug, Serialize, Deserialize, Encode, Decode, Clone, )]
    #[cfg_attr(not(feature = "std"), no_std)]
    pub struct JsonMeeting {
                    name: String,
                    description:String,
                    result_date: u64,
                    slots_ranges: Vec<SlotDay>,
                    hour_ranges: Vec<SlotHour>,
                    created_by: String
    }


    #[derive( Debug, Serialize, Deserialize, Eq, PartialEq, Encode, Decode, Clone, )]
    struct MeetingMember {
            meeting_id: u32,
            user_id: String,
            user_name:String,
            slots_selection: Vec<SlotDay>,
            creation_date:u64,
    }

    #[derive( Debug, Serialize, Deserialize, Eq, PartialEq, Encode, Decode, Clone, )]
    struct PeopleData {
            meeting_id: u32,
            user_id: String,
            user_name:String,
            availibility: Vec<String>
    }

   /// Defines the storage of your contract.
    /// All the fields will be encrypted and stored on-chain.
    #[ink(storage)]
    // #[derive(SpreadAllocate)]
    pub struct Phat2meet {
            /// Stores a single `bool` value on the storage.
            last_meeting_id: u32,
            admin: AccountId,
            s3_access: Option<String>,
            s3_secret: Option<String>,
            s3_bucket: String,
            platform: String,
            region: String,
            // meetings : Mapping<u32, (u64, AccountId)>
            modif_date: u64
    }
 
    #[derive( Debug, PartialEq, Eq, Encode, Decode, Serialize, Deserialize, Clone, Copy, )]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum MeetingState {
            MeetingInputOpen,
            MeetingInputClosed,
            ConsolidationDone,
            CorrectionOpen,
            CorrectionClosed,
            MeetingCancelled,
            MeetingClosed,
            MeetingCreated,
    }
    impl Default for MeetingState {
        fn default() -> Self {
            MeetingState::MeetingClosed
        }
    }

    impl fmt::Display for MeetingState  {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{:?}", self)
        }
    }



    #[derive(Debug, PartialEq, Eq, Encode, Decode, Clone, Copy, Serialize, Deserialize)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum SlotsState {
        SlotsOpen,
        SlotsUpdated,
        SlotsClosed,
        SlotInvalid,
        SlotValid,
        
    }
    impl Default for SlotsState {
        fn default() -> Self {
            SlotsState::SlotsClosed
        }
    }

    impl fmt::Display for SlotsState  {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{:?}", self)
        }
    }

    #[derive(Encode, Decode, Debug, PartialEq, Eq, Clone, )]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
                ExistingMeeting,
                MissingStartDate,
                MissingMeetingId,
                MissingDescription,
                StartDateNotValid,
                MeetingError(String),
                UserHasAlreadySelectedSameSlot,
                InvalidSlot(String),
                OnlyOwner, 
                NoPermissions,
                MeetingClosed,
                RegistrationClosed,
                RequestFailed(u16),
                EncryptionFailed,
                DecryptionFailed,
                PlatformNotFound,
                CredentialsNotSealed,
                AlreadyInitialized,
                DbRequestFailed,
                NoPathFound,
                S3ConnectionFailed,
                InvalidNumber,
                S3DeletionFailed,
                InvalidJsonFormat(String),
                InvalidTokenString,
                HourOutOfRange(String),
                DayOutofRange(String),
                InvalidDaySlot(String),
                InvalidHourSlot(String),
    }

    #[ink(event)]
    pub struct MeetingCreated {
        name: String,
        meeting_id: u32,
        meeting_owner: AccountId,
        meeting_start_date: u64,
        meeting_result_date: u64,
        date_creation: u64,
    }

    #[ink(event)]
    pub struct SlotsConsolidated {
        meeting_id: u32,
        meeting_quorum: u32,
        consolidation_date: u64,
    }

    #[ink(event)]
    pub struct MeetingRegistrationOpened {
        meeting_id: u32,
        date: u64,
    }

    #[ink(event)]
    pub struct MeetingRegistrationClosed {
        meeting_id: u32,
        date: u64,
    }

    #[ink(event)]
    pub struct MeetingCancelled {
        meeting_id: u32,
        date: u64,
    }

    #[ink(event)]
    pub struct SlotSelected{
        meeting_id: u32,
        meeting_member: AccountId,
        selection_date: u64,
    }

    #[ink(event)]
    pub struct ParamUpdated{
        update_date: u64,
    }

    impl Phat2meet {
        /// Constructor that initializes the
        #[ink(constructor)]
        pub fn new() -> Self {
            Self{
                admin: Self::env().caller(),
                s3_access: Some("jx6rdinb6btci4zvpynj3a6xe6iq".to_string()),
                s3_secret: Some("j2yt3ghh3gglyyeg2luw2qtolowkyrfomugcze76hddso6ksbxgyo".to_string()),
                s3_bucket: "phat2meet".to_string(),
                platform: "storj".to_string(),
                region: "us-east-1".to_string(),
                last_meeting_id: 0,
                modif_date: Self::env().block_timestamp()
            }
        }

        /// Initialize the S3 storage parameters
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

        /// Get the S3 infos
        #[ink(message)]
        pub fn get_platform_info(&self) -> (String, String, String, AccountId) {
            (self.platform.to_string(), 
             self.region.to_string(),
             self.s3_bucket.to_string(),
             self.admin)
        }

        /////////////////////////////////////////////////////////////////
        // Section : MANAGEMENT OF THE S3 API connection
        /////////////////////////////////////////////////////////////////

        ///  HTTP GETs and Decrypts the data from the specified storage platform as String.
        #[ink(message)]
        pub fn get_object_str(&self, object_key: String) -> Result<String> {

            let host = if self.platform == "s3" {
                String::from(format!("s3.{}.amazonaws.com", self.s3_bucket))
                
            } else if self.platform == "4everland" {
                String::from("endpoint.4everland.co")
            } else if self.platform == "storj" {
                String::from("gateway.storjshare.io".to_string())
            } else if self.platform == "filebase" {
                String::from("s3.filebase.com".to_string())
            } else {
                return Err(Error::PlatformNotFound);
            };

            let region = self.region.clone();
            let s3_access = self.s3_access.clone().unwrap();
            let s3_secret = self.s3_secret.clone().unwrap();
            // let binding = self.region.clone();

            // Create a S3 instance
            let s3 = s3::S3::new(&host.as_str(), &region.as_str(), &s3_access.as_str(), &s3_secret.as_str())
                            .unwrap();
                            // .virtual_host_mode(); // virtual host mode is required for newly created AWS S3 buckets.

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

        }

        /// Encrypts and HTTP PUTs the data to the specified storage platform as byte stream.
        #[ink(message)]
        pub fn put_object_str(&self, object_key: String, payload: String) -> Result<()> {

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

            let host = if self.platform == "s3" {
                String::from(format!("s3.{}.amazonaws.com", self.s3_bucket))
                
            } else if self.platform == "4everland" {
                String::from("endpoint.4everland.co")
            } else if self.platform == "storj" {
                String::from("gateway.storjshare.io".to_string())
            } else if self.platform == "filebase" {
                String::from("s3.filebase.com".to_string())
            } else {
                return Err(Error::PlatformNotFound);
            };

            let region = self.region.clone();
            let s3_access = self.s3_access.clone().unwrap();
            let s3_secret = self.s3_secret.clone().unwrap();
            // let binding = self.region.clone();

            // Create a S3 instance
            let s3 = s3::S3::new(&host.as_str(), &region.as_str(), &s3_access.as_str(), &s3_secret.as_str())
                            .unwrap();
                            // .virtual_host_mode(); // virtual host mode is required for newly created AWS S3 buckets.

             let Ok(_put_response) = s3.put(&self.s3_bucket.as_str(), &object_key.as_str(), &encrypted_bytes) else {return Err(Error::S3ConnectionFailed);};

            Ok(())

        }

        /////////////////////////////////////////////////////////////////
        // Section : MANAGEMENT OF MEETING CREATION
        /////////////////////////////////////////////////////////////////
        
        /// Create a new meeting  
        #[ink(message)]
        pub fn create_meeting(&mut self, json_meeting_file: String) -> Result<(MeetingState, String)> {
             debug_println!(" ----- json string ----- :\n {:?}", json_meeting_file);
             if let Ok(result) = pink_json::de::from_str::<JsonMeeting>(&json_meeting_file.trim()) {
                debug_println!(" ----- Json to Meeting ----- :\n ");


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
                
                // let copy_slots: Vec<_> = result.slots_ranges.iter().map(|x| x.clone()).collect();
                let copy_hours: Vec<_> =  result.hour_ranges.iter().map(|x| x.clone()).collect();
                // let owner Self::env().account_id();

                let new_meeting = Meeting {
                                    meeting_id: self.last_meeting_id + 1,
                                    name: result.name,
                                    description:result.description,
                                    // owner: Self::env().caller(),
                                    owner: account_id_to_string(Self::env().caller()),
                                    result_date: result.result_date,
                                    slots_ranges: self.convert_slots_ranges(result.slots_ranges),
                                    hour_ranges: copy_hours,
                                    meeting_state: MeetingState::MeetingCreated,
                                    slot_state: SlotsState::SlotsOpen,
                                    participants: Vec::new(),
                                    created_by: result.created_by,
                                    creation_date: Self::env().block_timestamp()
                };
                self.insert_meeting(&new_meeting);
                Self::env().emit_event(MeetingCreated {
                    name: new_meeting.name,
                    meeting_id: new_meeting.meeting_id,
                    meeting_owner: Self::env().caller(),
                    meeting_start_date: new_meeting.creation_date,
                    meeting_result_date: new_meeting.result_date,
                    date_creation: Self::env().block_timestamp(),
                });
                self.last_meeting_id = new_meeting.meeting_id;
                debug_println!(" ----- Meeting owner ----- :\n {}", new_meeting.created_by);
                Ok((MeetingState::MeetingCreated, new_meeting.meeting_id.to_string()))
             } else {
                debug_println!(" ----- Error in json formating ----- :\n ");
                return Err(Error::InvalidJsonFormat("Meeting JSON".to_string()));
            }
        }
        
        fn insert_meeting(&mut self, meeting: &Meeting) -> Result<()>  {
            // Serialize it to a JSON string.
            // debug_println!(" ----- Insert Meeting ----- :\n {:?}", meeting);
            debug_println!(" ----- Insert Meeting ----- : ");
            let meeting_string:String = pink_json::ser::to_string(&meeting).unwrap_or_default();
            // debug_println!(" ////////// Meeting String//////////:\n {:?}", meeting_string);
            let obj_key = format!("{}{}", "txn_create_".to_string(), meeting.meeting_id);
            debug_println!(" ////////// Meeting object //////////:\n {:?}", obj_key);
            // self.meetings_descr.insert(meeting_id, name);
            let Ok(_put_response) =  self.put_object_str(obj_key, meeting_string) else {return Err(Error::S3ConnectionFailed);};
            debug_println!(" ============== Meeting Saved ============== ");
            Ok(())
            
        }

        fn update_put_meeting(&mut self, meeting: &Meeting, obj_value: String) -> Result<()> {
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

        /// Get the last meeting id
        #[ink(message)]
        pub fn get_last_meeting_id(&self) -> u32 {
            self.last_meeting_id
        }

        /// Get the last meeting created infos
        #[ink(message)]
        pub fn get_last_meeting_created(&self) -> Result<String> {
            let obj_key = format!("{}{}", "txn_create_".to_string(), self.last_meeting_id);
           if let Ok(meeting_str) = self.get_object_str(obj_key){
              Ok(meeting_str)
           } else {
              return Err(Error::MeetingError("Meeting not found".to_string()));
           }
        }


        /// Get the meeting
        #[ink(message)]
        pub fn get_meeting(&self, meeting_id:u32) -> Result<String> {
            let obj_key = format!("{}{}", "txn_create_".to_string(), &meeting_id);
           if let Ok(meeting_str) = self.get_object_str(obj_key){
              Ok(meeting_str)
           } else {
              return Err(Error::MeetingError("S3 extraction".to_string()));
           }
        }

        /////////////////////////////////////////////////////////////////
        // Section : MANAGEMENT OF SLOTS MANAGEMENT
        /////////////////////////////////////////////////////////////////

        /// Add the slots of a user in the meeting
        #[ink(message)]
        pub fn add_slots(&mut self, json_slots_file: String) -> Result<SlotsState> {
            // debug_println!(" ----- JSON Slots----- :\n {:?}", json_slots_file);
            debug_println!(" ----- JSON Slots----- :");
            if let Ok(people_data) = pink_json::de::from_str::<PeopleData>(&json_slots_file.trim()) {


                let meeting_slots = self.convert_people_data(&people_data, 15);

                debug_println!(" ////////// MeetingMember Struct CREATED ////////// ");
                // debug_println!(" ////////// MeetingMember Struct //////////:\n {:?}", meeting_slots);

                let id_meeting = format!("{}{}", "txn_create_".to_string(), &meeting_slots.meeting_id);

                let meeting_key =  id_meeting.clone();
                if let Ok(meeting_str) = self.get_object_str(meeting_key) {

                    if let Ok(mut meeting) = pink_json::de::from_str::<Meeting>(&meeting_str) {

                        debug_println!(" ////////// Meeting extracted ////////// ");

                        let slots_selection:Vec<SlotDay> = meeting_slots.slots_selection.iter().map(|x| x.clone()).collect();
                        let hours_allowed: Vec<SlotHour> = meeting.hour_ranges.iter().map(|z| z.clone()).collect();
                        let days_allowed:Vec<SlotDay> = meeting.slots_ranges.iter().map(|y| y.clone()).collect();
        
                        // If the slots selection is closed or the meeting is closed ===>  reject the addition
                        if meeting.slot_state == SlotsState::SlotsClosed || meeting.meeting_state == MeetingState::MeetingCancelled ||
                            meeting.meeting_state == MeetingState::MeetingInputClosed || meeting.meeting_state == MeetingState::CorrectionClosed || 
                            meeting.meeting_state == MeetingState::MeetingClosed{
                            return Err(Error::RegistrationClosed);
                        }
        
                        // Check if the slots are included in the allowed ranges
                        if let Ok(_slot_status) = self.check_slots(slots_selection, hours_allowed, days_allowed) {
                            debug_println!(" ////////// Slot state OK ////////// ");
                            let participants: Vec<MeetingMember> = meeting.participants.iter().map(|x| x.clone()).collect();
                            let participants_ids: Vec<String> = meeting.participants.iter().map(|x| x.clone().user_id).collect();

                            let nb_participants = participants.len();
                            let new_user_id = meeting_slots.clone().user_id;
                            let slots = meeting_slots.clone(); 

                            let mut meeting_participants = Vec::new();
                            debug_println!(" ////////// CHECKS SEQUENCE OK ////////// ");
                            if nb_participants > 0 {

                                if participants_ids.iter().any(|i| i.clone()== new_user_id) {
                                    for participant in participants {
                                        if participant.user_id == meeting_slots.user_id {
                                            // meeting_participants.retain(|value| *value == participant);
                                            meeting_participants.push(slots.clone());                                           
                                        } else {
                                            meeting_participants.push(participant.clone());
                                        }
                                    }
                                } else {
                                    for participant in participants {
                                            meeting_participants.push(participant.clone());
                                    }
                                    meeting_participants.push(slots.clone());  
                                }

                                
                            } else {
                                if meeting_participants.clone().len() <= 1 {
                                    meeting_participants.push(slots);
                                }
                            }


                            debug_println!(" ////////// PARTICIPANTS check OK ////////// {:?}", &meeting_participants.clone());
                            meeting.participants = meeting_participants.iter().map(|x| x.clone()).collect();
                            meeting.slot_state = SlotsState::SlotsOpen; 

                            debug_println!(" ////////// Meeting updated with participants //////////");

                            self.update_put_meeting(&meeting, id_meeting);


                            Ok(SlotsState::SlotsUpdated)
        
                        } else {
                            return Err(Error::MeetingError("At least one selected slot is not allowed".to_string()));
                        }                    
                    } else {
                        return Err(Error::MeetingError("Slots S3 extraction".to_string()));
                    }

                } else {
                    return Err(Error::MeetingError("Meeting do not exist".to_string()));
                }
                    
            } else {
                return Err(Error::InvalidJsonFormat("Slots JSON".to_string()));
            }
        }

        fn convert_people_data (&mut self, people_data:  &PeopleData, default_range: i32) -> MeetingMember{

            let people_availibility: Vec<String> = people_data.availibility.iter().map(|y| y.clone()).collect();

            let meeting_slots = MeetingMember {
                meeting_id: people_data.meeting_id,
                user_id: people_data.clone().user_id,
                user_name: people_data.clone().user_name,
                slots_selection: self.converts_slots(people_availibility, default_range),
                creation_date: Self::env().block_timestamp()
            };    
            meeting_slots
        }

        fn converts_slots(&mut self, args: Vec<String>, default_range: i32) -> Vec<SlotDay> {
            let counter = args.len();
            let mut i: usize = 0;
            let mut slots: Vec<SlotDay> = Vec::new();
            
            while i < counter{
                
                let start_element;
                let mut end_element;
        
                if i < counter - 1 {
                    start_element = args.get(i);
                    end_element = args.get(i+1);
                } else {
                    start_element = args.get(i);
                    end_element = args.get(i);
                }
                
                let hour_start = &start_element.clone().unwrap()[..4];
                let day_start = &start_element.clone().unwrap()[5..];
                let mut hour_end = &end_element.clone().unwrap()[..4];
                let mut day_end = &end_element.clone().unwrap()[5..]; 
        
                let mut change: bool = false;
                let mut k:i32=1;
        
                while !change && i < counter  {
                   
                    change = true;

                    let next_hour:i32 = if hour_end.clone().ends_with("00") {
                                            hour_end.clone().parse::<i32>().unwrap() - (k * default_range + 40)
                                        } else { 
                                            hour_end.clone().parse::<i32>().unwrap() - k * default_range
                                        };

                    if hour_start.clone().parse::<i32>().unwrap() - next_hour == 0 
                        && day_start.clone() == day_end.clone() {
                       change = false;
                       k += 1;
                       i += 1;
                       if i == counter - 1  {
                           end_element = args.get(i);            
                       } else {
                           end_element = args.get(i + 1); 
                       }
                       hour_end = &end_element.clone().unwrap()[..4];
                       day_end = &end_element.clone().unwrap()[5..];            
                       
                    } else {
                    
                        if i < counter {
                            end_element = args.get(i);          
        
                            hour_end = &end_element.clone().unwrap()[..4];
                            day_end = &end_element.clone().unwrap()[5..]; 
                            
                            let mut final_hour: String = format!("{}", hour_end.parse::<i32>().unwrap() + default_range);
                            
                            if final_hour.len() == 3 {
                                final_hour = format!("0{}", final_hour);
                            }
                            let start_date_time = self.from_str_to_timepstamp(hour_start, day_start);
                            let end_date_time = self.from_str_to_timepstamp( &final_hour.clone(), day_end);
        
                            slots.push(SlotDay{
                                start: start_date_time.into(), 
                                end: end_date_time.into()});
                            change = true;
                            i += 1;
                        }
                    }
                }
            }
            slots
        }

        fn check_slots(
            &self, 
            selection_slots: Vec<SlotDay>, 
            hours_range: Vec<SlotHour>, 
            days_range: Vec<SlotDay>,
        ) -> Result<SlotsState> {
            debug_println!(" ----- CHECK START ----- :");
            for item in selection_slots {
                let selection_timestamp_start = item.start;
                let start_tmp_micros: i64 = format!("{}000", item.start).parse::<i64>().unwrap();
                let dt_start = Utc.timestamp_millis_opt(start_tmp_micros.into()).unwrap();

                let hour_start = dt_start.hour();
                let minute_start = dt_start.minute();
                let hour_min_start = &hour_start * 100 + &minute_start;

                let selection_timestamp_end = item.end;
                let end_tmp_micros: i64 = format!("{}000", item.end).parse::<i64>().unwrap();
                let dt_end = Utc.timestamp_millis_opt(end_tmp_micros.into()).unwrap();

                let hour_end = dt_end.hour();
                let minute_end = dt_end.minute();
                let hour_min_end = &hour_end * 100 + &minute_end;

                let mut slot_status: bool = false;
                let mut hour_status: bool = false;

                debug_println!(":::::----- START DAYS -----:::::");
                for day_range in days_range.clone() {
                    if (selection_timestamp_start >= day_range.start.into() && selection_timestamp_start <= day_range.end.into() 
                        && selection_timestamp_end >= day_range.start.into() && selection_timestamp_end <= day_range.end.into()) && !slot_status {
                            slot_status = true;
                        debug_println!(":::::----- Days range found -----:::::");
                    }                      
                }
                debug_println!(":::::----- START HOURS -----:::::");
                for hour_range in hours_range.clone() {
                    debug_println!(":::::----- {:#?} ----- {} / {}:::::", hour_range.clone(), hour_min_start, hour_min_end);
                    if (hour_min_start >= hour_range.start.parse::<u32>().unwrap() && hour_min_start <= hour_range.end.parse::<u32>().unwrap() &&
                        hour_min_end >= hour_range.start.parse::<u32>().unwrap() && hour_min_end <= hour_range.end.parse::<u32>().unwrap()) && !hour_status {
                        debug_println!(":::::----- Hours range found -----:::::");
                        hour_status = true;
                    } 
                }

                if !slot_status {
                    return Err(Error::DayOutofRange(format!("{:#?} slot out of days range", item)))
                }

                if !hour_status {
                    return Err(Error::HourOutOfRange(format!("{:#?} slots out of hours range", item)))
                }                
            }

            Ok(SlotsState::SlotValid)
        }



        fn from_str_to_timepstamp (&mut self, hour_fmt: &str, date_fmt: &str) -> i64 {
            let dt_hour = hour_fmt[..2].parse::<u32>().unwrap();
            let dt_min = hour_fmt[2..].parse::<u32>().unwrap();
            let dt_day = date_fmt[..2].parse::<u32>().unwrap();
            let dt_month = date_fmt[2..4].parse::<u32>().unwrap();
            let dt_year = date_fmt[4..].parse::<i32>().unwrap();
            
            let date_time;
            
            if dt_min == 60 {
                if dt_hour != 23 {
                    date_time = Utc.with_ymd_and_hms(dt_year, dt_month, dt_day, dt_hour + 1, 0, 0).unwrap();
                } else {
                    date_time = Utc.with_ymd_and_hms(dt_year, dt_month, dt_day, 0, 0, 0).unwrap()  + Duration::days(1);
                }
                
            } else {
                date_time = Utc.with_ymd_and_hms(dt_year, dt_month, dt_day, dt_hour, dt_min, 0).unwrap()
            }
            date_time.timestamp()
        }


        fn convert_slots_ranges (&mut self,  days_range: Vec<SlotDay>) -> Vec<SlotDay> {
            let mut new_slots = Vec::new();
            for day_range in days_range.clone() {

                let start_tmp_micros: i64 = format!("{}000", day_range.start).parse::<i64>().unwrap();
                let end_tmp_micros: i64 = format!("{}000", day_range.end).parse::<i64>().unwrap();
                let dt_start = Utc.timestamp_millis_opt(start_tmp_micros.into()).unwrap();
                let dt_end = Utc.timestamp_millis_opt(end_tmp_micros.into()).unwrap();

                let item_slot = SlotDay {
                    start: Utc.with_ymd_and_hms(dt_start.year(), dt_start.month(), dt_start.day(), 0, 0, 0).unwrap().timestamp(), 
                    end: Utc.with_ymd_and_hms(dt_end.year(), dt_end.month(), dt_end.day(), 23, 59, 0).unwrap().timestamp()
                };
                new_slots.push(item_slot)
            }

            new_slots


        }

    }
     
    fn account_id_to_string(account_id: AccountId) -> String {
        let bytes = account_id.encode();
        String::from_utf8_lossy(bs58::encode(&bytes).into_string().as_ref()).to_string()
        
    }


}
