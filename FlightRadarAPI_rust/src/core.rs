use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, ACCEPT_ENCODING, ACCEPT_LANGUAGE, CACHE_CONTROL, CONTENT_TYPE, ORIGIN, REFERER, USER_AGENT};

struct Core;

impl Core {
    // Base URLs.
    const API_FLIGHTRADAR_BASE_URL: &'static str = "https://api.flightradar24.com/common/v1";
    const CDN_FLIGHTRADAR_BASE_URL: &'static str = "https://cdn.flightradar24.com";
    const FLIGHTRADAR_BASE_URL: &'static str = "https://www.flightradar24.com";
    const DATA_LIVE_BASE_URL: &'static str = "https://data-live.flightradar24.com";
    const DATA_CLOUD_BASE_URL: &'static str = "https://data-cloud.flightradar24.com";

    // User login URLs
    fn user_login_url() -> String {
        format!("{}/user/login", Self::FLIGHTRADAR_BASE_URL)
    }
    fn user_logout_url() -> String {
        format!("{}/user/logout", Self::FLIGHTRADAR_BASE_URL)
    }

    // Search data URL
    // Format may be wrong
    fn search_data_url(query: &str, limit: usize) -> String {
        format!("{}/v1/search/web/find?query={}&limit={}", Self::FLIGHTRADAR_BASE_URL, query, limit)
    }

    // Flights data URLs
    // Format may be wrong
    fn real_time_flight_tracker_data_url() -> &'static str {
        &format!("{}/zones/fcgi/feed.js", Self::DATA_CLOUD_BASE_URL)
    }
    fn flight_data_url(flight: &str) -> String {
        format!("{}/clickhandler/?flight={}", Self::DATA_LIVE_BASE_URL, flight)
    }

    // Historical data URL
    // Format may be wrong
    fn historical_data_url(flight: &str, file: &str, history: bool) -> String {
        format!("{}/download/?flight={}&file={}&trailLimit=0&history={}", Self::FLIGHTRADAR_BASE_URL, flight, file, history)
    }

    // Airports data URLs
    fn api_airport_data_url() -> &'static str {
        &format!("{}/airport.json", Self::API_FLIGHTRADAR_BASE_URL)
    }
    fn airport_data_url(airport: &str) -> String {
        format!("{}/airports/traffic-stats/?airport={}", Self::FLIGHTRADAR_BASE_URL, airport)
    }
    fn airports_data_url() -> &'static str {
        &format!("{}/_json/airports.php", Self::FLIGHTRADAR_BASE_URL)
    }

    // Airline data URL
    fn airlines_data_url() -> &'static str {
        &format!("{}/_json/airlines.php", Self::FLIGHTRADAR_BASE_URL)
    }

    //Zones data URL
    fn zones_data_url() -> &'static str {
        &format!("{}/js/zones.js.php", Self::FLIGHTRADAR_BASE_URL)
    }

    // Weather data URL
    fn volcanic_eruption_data_url() -> &'static str {
        &format!("{}/weather/volcanic", Self::FLIGHTRADAR_BASE_URL)
    }

    // Most tracked URL
    fn most_tracked_url() -> &'static str {
        &format!("{}/flights/most-tracked", Self::FLIGHTRADAR_BASE_URL)
    }

    // Airport_disruption URL
    fn airport_disruption_url() -> &'static str {
        &format!("{}/webapi/v1/airport-disruptions", Self::FLIGHTRADAR_BASE_URL)
    }

    // Bookmarks URL
    fn bookmarks_url() -> &'static str {
        &format!("{}/webapi/v1/bookmarks", Self::FLIGHTRADAR_BASE_URL)
    }

    // Country flag image URL
    fn country_flag_url(country: &str) -> String {
        format!("{}/static/images/data/flags-small/{}.svg", Self::FLIGHTRADAR_BASE_URL, country)
    }

    // Airline logo image URL
    fn airline_logo_url(iata: &str, icao: &str) -> String {
        format!("{}/assets/airlines/logotypes/{}_{}.png", Self::CDN_FLIGHTRADAR_BASE_URL, iata, icao)
    }
    fn alternative_airline_logo_url(icao: &str) -> String {
        format!("{}/static/images/data/operators/{}_logo0.png", Self::FLIGHTRADAR_BASE_URL, icao)
    }

    // Http headers
    fn default_headers() -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT_ENCODING, HeaderValue::from_static("gzip, br"));
        headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("pt-BR,pt;q=0.9,en-US;q=0.8,en;q=0.7"));
        headers.insert(CACHE_CONTROL, HeaderValue::from_static("max-age=0"));
        headers.insert(ORIGIN, HeaderValue::from_static("https://www.flightradar24.com"));
        headers.insert(REFERER, HeaderValue::from_static("https://www.flightradar24.com/"));
        headers.insert("sec-fetch-dest", HeaderValue::from_static("empty"));
        headers.insert("sec-fetch-mode", HeaderValue::from_static("cors"));
        headers.insert("sec-fetch-site", HeaderValue::from_static("same-site"));
        headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (Windows NT 6.1) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/87.0.4280.88 Safari/537.36"));
        headers
    }
    
    fn json_headers() -> HeaderMap {
        let mut headers = default_headers();
        headers.insert(ACCEPT, HeaderValue::from_static("application/json"));
        headers
    }
    
    fn image_headers() -> HeaderMap {
        let mut headers = default_headers();
        headers.insert(ACCEPT, HeaderValue::from_static("image/gif, image/jpg, image/jpeg, image/png"));
        headers
    }


}