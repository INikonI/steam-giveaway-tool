use crate::steam::SteamUser;

pub const CIS_COUNTRIES: &[&str] = &[
    "AM", // Армения
    "AZ", // Азербайджан
    "BY", // Беларусь
    "KZ", // Казахстан
    "KG", // Кыргызстан
    "MD", // Молдова
    "RU", // Россия
    "TJ", // Таджикистан
    "UA", // Украина
    "UZ", // Узбекистан
    "TM", // Туркменистан
    "GE", // Грузия
];

pub const EU_COUNTRIES: &[&str] = &[
    "AT", // Австрия
    "BE", // Бельгия
    "BG", // Болгария
    "HR", // Хорватия
    "CY", // Кипр
    "CZ", // Чехия
    "DK", // Дания
    "EE", // Эстония
    "FI", // Финляндия
    "FR", // Франция
    "DE", // Германия
    "GR", // Греция
    "HU", // Венгрия
    "IE", // Ирландия
    "IT", // Италия
    "LV", // Латвия
    "LT", // Литва
    "LU", // Люксембург
    "MT", // Мальта
    "NL", // Нидерланды
    "PL", // Польша
    "PT", // Португалия
    "RO", // Румыния
    "SK", // Словакия
    "SI", // Словения
    "ES", // Испания
    "SE", // Швеция
];

#[derive(Default)]
pub enum RegionFilter {
    #[default]
    Available,
    Include,
    Exclude,
}

#[derive(Default)]
pub struct RegionsAndCountriesFilter {
    pub available_countries: Vec<String>,
    pub include_countries: Vec<String>,
    pub exclude_countries: Vec<String>,

    pub unknown: RegionFilter,
    pub cis: RegionFilter,
    pub eu: RegionFilter,
}

pub fn apply_region_filters(friends: &mut Vec<SteamUser>, filters: &RegionsAndCountriesFilter) {
    let include_unknown = matches!(filters.unknown, RegionFilter::Include);
    let include_cis = matches!(filters.cis, RegionFilter::Include);
    let include_eu = matches!(filters.eu, RegionFilter::Include);
    let include_countries = !filters.include_countries.is_empty();

    if include_countries || include_unknown || include_cis || include_eu {
        friends.retain(|friend| {
            friend.country_code.as_ref().map_or_else(
                || include_unknown,
                |friend_country| {
                    if include_cis
                        && CIS_COUNTRIES
                            .iter()
                            .any(|country| country == friend_country)
                    {
                        return true;
                    }
                    if include_eu && EU_COUNTRIES.iter().any(|country| country == friend_country) {
                        return true;
                    }
                    if include_countries
                        && filters
                            .include_countries
                            .iter()
                            .any(|region_or_country| region_or_country == friend_country)
                    {
                        return true;
                    }
                    false
                },
            )
        });
    }

    let exclude_unknown = matches!(filters.unknown, RegionFilter::Exclude);
    let exclude_cis = matches!(filters.cis, RegionFilter::Exclude);
    let exclude_eu = matches!(filters.eu, RegionFilter::Exclude);
    let exclude_countries = !filters.exclude_countries.is_empty();

    if exclude_countries || exclude_unknown || exclude_cis || exclude_eu {
        friends.retain(|friend| {
            !friend.country_code.as_ref().map_or_else(
                || exclude_unknown,
                |friend_country| {
                    if exclude_cis
                        && CIS_COUNTRIES
                            .iter()
                            .any(|country| country == friend_country)
                    {
                        return true;
                    }
                    if exclude_eu && EU_COUNTRIES.iter().any(|country| country == friend_country) {
                        return true;
                    }
                    if exclude_countries
                        && filters
                            .exclude_countries
                            .iter()
                            .any(|region_or_country| region_or_country == friend_country)
                    {
                        return true;
                    }
                    false
                },
            )
        });
    }
}
