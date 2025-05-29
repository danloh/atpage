//! An event and RSVP management application.
//! lexicons: 
//! https://tangled.sh/@smokesignal.events/smokesignal/blob/main/src/atproto/lexicon/events_smokesignal_calendar_event.rs

use serde::{Deserialize, Serialize};

use crate::{
	helper::utils::{str_to_timestamp, ts_to_dt},
	resources::{
		atpage::AtRecord,
		record::{list_record, uri_parts},
	},
};

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct SmokeSignalListResp {
	pub records: Vec<SmokeSignalResp>,
	pub cursor: Option<String>,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct SmokeSignalResp {
	pub uri: String,
	pub cid: String,
	pub value: SmokeSignalValue,
}

#[derive(Clone, Default, Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct SmokeSignalValue {
	#[serde(rename = "$type")]
	pub kind: String,
	pub name: String,
	pub text: Option<String>,
	pub startsAt: Option<String>,
	pub endsAt: Option<String>,
	pub createdAt: String,
}

// TODO, leverage cursor for pagination
pub async fn get_smokesignal_records(
	did: &str,
	serv: &str,
	cur: Option<String>,
) -> (Vec<AtRecord>, Option<String>) {
	match list_record("events.smokesignal.calendar.event", serv, &did, cur).await {
		Ok(raw_data) => {
			// gloo::console::log!("smokesignal records: ", format!("{:?}", raw_data));
			match serde_json::from_str::<SmokeSignalListResp>(&raw_data) {
				Ok(data_res) => {
					let cursor = data_res.cursor;
					let entrylist: Vec<SmokeSignalResp> = data_res.records;
					let mut at_records: Vec<AtRecord> = Vec::new();
					for entry in entrylist {
						let uri = entry.uri;
						let (did, _col, rkey) = uri_parts(&uri);
						let link = format!("https://smokesignal.events/{did}/{rkey}");
						let value = entry.value;
						
						let mut schedule = String::new();
						if let Some(starts_at) = value.startsAt {
							if !starts_at.trim().is_empty() {
								schedule = ts_to_dt(str_to_timestamp(&starts_at) / 1000);
								if let Some(ends_at) = value.endsAt {
									if !ends_at.trim().is_empty() {
										schedule.push_str(
											&format!(" - {} UTC", ts_to_dt(str_to_timestamp(&ends_at) / 1000))
										);
									}
								}
							}
						}

						let rec = AtRecord {
							kind: String::from("smokesignal"),
							title: format!("Event - {}", value.name),
							link,
							content: format!("{} \n\n {}", schedule, value.text.unwrap_or_else(|| value.name)),
							images: vec![],
							timestamp: str_to_timestamp(&value.createdAt) / 1000,
						};
						at_records.push(rec);
					}

					// gloo::console::log!("smokesignal res: ", format!("{:?}", at_records));

					return (at_records, cursor);
				}
				Err(e) => {
					gloo::console::log!("get smokesignal records error: ", format!("{:?}", e));
					return (Default::default(), None);
				}
			}
		}
		Err(e) => {
			gloo::console::log!("get smokesignal records error: ", format!("{:?}", e));
			return (Default::default(), None);
		}
	}
}
