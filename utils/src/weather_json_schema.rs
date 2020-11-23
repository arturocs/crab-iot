use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub city: City,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct City {
    pub lang: String,
    pub city_name: String,
    pub city_latitude: String,
    pub city_longitude: String,
    pub city_id: i64,
    pub is_capital: bool,
    pub station_name: String,
    #[serde(rename = "tourismURL")]
    pub tourism_url: String,
    pub tourism_board_name: String,
    pub is_dep: bool,
    pub time_zone: String,
    #[serde(rename = "isDST")]
    pub is_dst: String,
    pub member: Member,
    pub forecast: Forecast,
    pub climate: Climate,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Member {
    pub mem_id: i64,
    pub mem_name: String,
    pub short_mem_name: String,
    pub url: String,
    pub org_name: String,
    pub logo: String,
    pub ra: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Forecast {
    pub issue_date: String,
    pub time_zone: String,
    pub forecast_day: Vec<ForecastDay>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForecastDay {
    pub forecast_date: String,
    pub wxdesc: String,
    pub weather: String,
    pub min_temp: String,
    pub max_temp: String,
    pub min_temp_f: String,
    pub max_temp_f: String,
    pub weather_icon: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Climate {
    pub raintype: String,
    pub raindef: String,
    pub rainunit: String,
    pub datab: i64,
    pub datae: i64,
    pub tempb: String,
    pub tempe: String,
    pub rdayb: String,
    pub rdaye: String,
    pub rainfallb: String,
    pub rainfalle: String,
    pub climatefromclino: String,
    pub climate_month: Vec<ClimateMonth>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClimateMonth {
    pub month: i64,
    pub max_temp: String,
    pub min_temp: String,
    pub mean_temp: Option<String>,
    pub max_temp_f: String,
    pub min_temp_f: String,
    pub mean_temp_f: Option<String>,
    pub raindays: String,
    pub rainfall: String,
    pub climate_from_mem_date: String,
}
