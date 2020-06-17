// Model to the data gathered from the API will only include the temperature (in kelvin)
// and the name of the ciry in the specified coordinates. 

#[derive(Deserialize)]
pub struct Weather {

pub  name: String,

#[serde(rename = "main")]
pub condition : Condition,

}

#[derive(Deserialize)]
pub struct Condition {

pub temp:f32    

}

#[derive(Copy,Clone)]
pub struct Coords{

    pub  lat : f32,
    pub  lon : f32,
}



#[cfg(test)]
mod test{

    use serde_json as json;

    static DATA: &'static str = include_str!("../response.json"); 

    #[test]
    fn deserialize() {
        json::from_str::<super::Weather>(DATA).unwrap();
    }

}




