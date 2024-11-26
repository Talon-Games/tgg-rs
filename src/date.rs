use std::time::Duration;

pub fn format_timestamp(timestamp: u32) -> String {
    let duration_since_epoch = Duration::from_secs(timestamp as u64);

    // Total days since the UNIX epoch (1970-01-01)
    let days_since_epoch = duration_since_epoch.as_secs() / 86400;

    // Approximate the year
    let mut year = 1970;
    let mut remaining_days = days_since_epoch;

    while remaining_days >= if is_leap_year(year) { 366 } else { 365 } {
        remaining_days -= if is_leap_year(year) { 366 } else { 365 };
        year += 1;
    }

    let month_lengths = [
        31,                                       // January
        if is_leap_year(year) { 29 } else { 28 }, // February
        31,                                       // March
        30,                                       // April
        31,                                       // May
        30,                                       // June
        31,                                       // July
        31,                                       // August
        30,                                       // September
        31,                                       // October
        30,                                       // November
        31,                                       // December
    ];

    let mut month = 1;
    for &days_in_month in &month_lengths {
        if remaining_days < days_in_month {
            break;
        }
        remaining_days -= days_in_month;
        month += 1;
    }

    let day = remaining_days + 1; // Convert 0-based to 1-based day

    let month_name = match month {
        1 => "January",
        2 => "February",
        3 => "March",
        4 => "April",
        5 => "May",
        6 => "June",
        7 => "July",
        8 => "August",
        9 => "September",
        10 => "October",
        11 => "November",
        12 => "December",
        _ => "Month",
    };

    format!("{}, {:02}, {}", month_name, day, year).to_string()
}

fn is_leap_year(year: u32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}
