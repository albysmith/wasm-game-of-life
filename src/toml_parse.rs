#[derive(Deserialize, Serialize)]
pub struct Toml {
    pub start: Option<StartData>,
    pub Sectors: Option<Vec<SectorData>>,
    pub Ships: Option<Vec<ShipData>>,
    pub Stations: Option<Vec<StationData>>,
    pub Trade: Option<Vec<OfferData>>,
}
#[derive(Deserialize, Serialize)]
pub struct StartData {
    pub _id: Option<String>,
    pub r#type: Option<String>,
    pub time: Option<f32>,
    pub deadships: Option<i32>,
    pub deadstations: Option<i32>,
    pub iteration: Option<i32>,
}
// #[derive(Deserialize, Serialize)]
// pub struct FactionData {
//     pub Sectors: Option<Vec<SectorData>>,
//     pub Ships: Option<Vec<ShipData>>,
//     pub Stations: Option<Vec<StationData>>,
// }
#[derive(Deserialize, Serialize)]
pub struct SectorData {
    pub _id: Option<String>,
    pub r#type: Option<String>,
    pub owner: Option<String>,
    pub sector: Option<String>,
    pub r#macro: Option<String>,
    pub iscontested: Option<bool>,
    pub averagedistancetolockbox: Option<f32>,
    pub typicallockboxmacro: Option<String>,
}
#[derive(Deserialize, Serialize)]
pub struct ShipData {
    pub _id: Option<String>,
    pub r#type: Option<String>,
    pub owner: Option<String>,
    pub ship: Option<String>,
    pub r#macro: Option<String>,
    pub job: Option<String>,
    pub canbeclaimed: Option<bool>,
    pub issupplyship: Option<bool>,
    pub iscapitalship: Option<bool>,
    pub averageprice: Option<i32>,
    pub sector: Option<String>,
    pub trueowner: Option<String>,
    pub isdeployedconstructionvessel: Option<bool>,
    pub isknown: Option<bool>,
    pub hullpercentage: Option<f32>,
    pub shieldpercentage: Option<f32>,
    pub maxhull: Option<f32>,
    pub maxshield: Option<f32>,
    pub isformationleader: Option<bool>,
    pub value: Option<i64>,
    pub order: Option<String>,
    pub primarypurpose: Option<String>,
    pub pilot: Option<String>,
    pub boarding: Option<i32>,
    pub engineering: Option<i32>,
    pub management: Option<i32>,
    pub morale: Option<i32>,
    pub piloting: Option<i32>,
}
#[derive(Deserialize, Serialize)]
pub struct StationData {
    pub _id: Option<String>,
    pub r#type: Option<String>,
    pub owner: Option<String>,
    pub station: Option<String>,
    pub sector: Option<String>,
    pub istradestation: Option<bool>,
    pub isshipyard: Option<bool>,
    pub iswharf: Option<bool>,
    pub isequipmentdock: Option<bool>,
    pub isdefencestation: Option<bool>,
    pub ispiratebase: Option<bool>,
    pub isheadquarters: Option<bool>,
    pub isfactionheadquarters: Option<bool>,
    pub isplannedshipyard: Option<bool>,
    pub isplannedwharf: Option<bool>,
    pub isplannedequipmentdock: Option<bool>,
    pub isplanneddefencestation: Option<bool>,
}
#[derive(Deserialize, Serialize)]
pub struct OfferData {
    pub _id: Option<String>,
    pub r#type: Option<String>,
    pub offer: Option<String>,
    pub buyer: Option<String>,
    pub seller: Option<String>,
    pub owner: Option<String>,
    pub ware: Option<String>,
    pub amount: Option<i32>,
    pub desiredamount: Option<i32>,
    pub offeramount: Option<i32>,
    pub minamount: Option<i32>,
    pub transferredamount: Option<i32>,
    pub destroyedamount: Option<i32>,
    pub volume: Option<i32>,
    pub price: Option<i64>,
    pub unitprice: Option<i32>,
    pub iswareexchange: Option<bool>,
    pub isshiptoship: Option<bool>,
    pub isbuyerpassive: Option<bool>,
    pub issellerpassive: Option<bool>,
}