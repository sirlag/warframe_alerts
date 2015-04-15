//extern crate time;

//use time::Tm;
use std::fmt;

#[allow(dead_code)]
struct Alert {
    guid : String,
    planet : String,
    node : String,
    alert_type : Event,
    enemy : Faction,
    description : Option<String>,
    rewards : Vec<String>,
//    End : Option<Tm>,
}

impl fmt::Display for Alert{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Guid : {}\n{} - {}\nFaction : {:?}\nType - {:?}", self.guid, self.node, self.planet, self.enemy, self.alert_type)
    }  
}

//An enum for mission typing, to allow for better use of "Match"
//Other Signifies that the the returned alert type was either in error, or produced from abnormal
//Alert types
#[allow(dead_code)]
#[derive(Debug)]
enum Event {
    Alert,
    Outbreak,
    Invasion,
    Other,
}

//An enum for faction typing, once again, Other is used in the event there is a failed parse, or
//Otherwise abnormal faction (I.E. I didn't update the library for the Sentiants
#[allow(dead_code)]
#[derive(Debug)]
enum Faction {
    Grineer,
    Corpus,
    Infested,
    Other,
}


//The Time format string to use with rust time lib
//%a, %d %b %G %T %z

fn main(){
    let alert = Alert {guid :"123456".to_string(), planet : "Earth".to_string(), node : "New York".to_string(), 
                       alert_type : Event::Alert, enemy : Faction::Corpus, description : None, rewards : vec![]};
    println!("Compile! {}", alert);   
}