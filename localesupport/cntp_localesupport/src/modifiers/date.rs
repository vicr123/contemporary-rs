//! Date and time formatting modifier.
//!
//! This module provides the [`Date`] modifier for formatting dates and times
//! according to locale conventions.

use icu::{
    calendar::Iso,
    datetime::{
        DateTimeFormatter,
        fieldsets::{
            D, DE, DET, DT, E, ET, M, MD, MDE, MDET, MDT, T, Y, YM, YMD, YMDE, YMDET, YMDT,
            builder::{DateFields, FieldSetBuilder},
            enums::{
                CalendarPeriodFieldSet, CompositeDateTimeFieldSet, DateAndTimeFieldSet,
                DateFieldSet, TimeFieldSet,
            },
        },
        options::{Alignment, Length, SubsecondDigits, TimePrecision, YearStyle},
    },
    time::{DateTime, ZonedDateTime, zone::UtcOffset},
};

use crate::{Locale, modifiers::StringModifier};

/// A string modifier that formats dates and times according to locale conventions.
///
/// The `Date` modifier can format various input types (strings, timestamps, `chrono` types)
/// into locale-appropriate date/time strings.
///
/// # Input types
///
/// The modifier accepts several input types:
///
/// - `&str` - An ISO 8601 date string (e.g., `"2024-01-15T10:30:00"`)
/// - `i8`, `i16`, `i32`, `i64`, `i128`, `isize` - Unix timestamp in milliseconds
/// - `f32`, `f64` - Unix timestamp in seconds (converted to milliseconds)
/// - `chrono::DateTime<T>` - When the `chrono` feature is enabled
/// - `chrono::NaiveDateTime` - When the `chrono` feature is enabled
///
/// # Format Specifiers
///
/// ## Simple Field Sets
///
/// Use a single string argument to specify which date/time components to include:
///
/// | Format    | Components                      | Example Output                  |
/// |-----------|---------------------------------|---------------------------------|
/// | `"D"`     | Day                             | "15"                            |
/// | `"DE"`    | Day, Weekday                    | "Mon 15"                        |
/// | `"DT"`    | Day, Time                       | "15, 10:30 AM"                  |
/// | `"DET"`   | Day, Weekday, Time              | "Mon 15, 10:30 AM"              |
/// | `"E"`     | Weekday                         | "Monday"                        |
/// | `"ET"`    | Weekday, Time                   | "Monday, 10:30 AM"              |
/// | `"M"`     | Month                           | "January"                       |
/// | `"MD"`    | Month, Day                      | "Jan 15"                        |
/// | `"MDE"`   | Month, Day, Weekday             | "Mon, Jan 15"                   |
/// | `"MDT"`   | Month, Day, Time                | "Jan 15, 10:30 AM"              |
/// | `"MDET"`  | Month, Day, Weekday, Time       | "Mon, Jan 15, 10:30 AM"         |
/// | `"T"`     | Time                            | "10:30 AM"                      |
/// | `"Y"`     | Year                            | "2024"                          |
/// | `"YM"`    | Year, Month                     | "January 2024"                  |
/// | `"YMD"`   | Year, Month, Day                | "Jan 15, 2024" (default)        |
/// | `"YMDE"`  | Year, Month, Day, Weekday       | "Mon, Jan 15, 2024"             |
/// | `"YMDT"`  | Year, Month, Day, Time          | "Jan 15, 2024, 10:30 AM"        |
/// | `"YMDET"` | Year, Month, Day, Weekday, Time | "Mon, Jan 15, 2024, 10:30 AM"   |
///
/// ## Length option
///
/// Add `length` to control verbosity:
///
/// - `length = "short"` - Abbreviated (e.g., "1/15/24")
/// - `length = "medium"` - Default (e.g., "Jan 15, 2024")
/// - `length = "long"` - Full names (e.g., "January 15, 2024")
///
/// ## Complex field sets
///
/// For more control, use named arguments:
///
/// - `date` - Date fields: `"D"`, `"MD"`, `"YMD"`, `"DE"`, `"MDE"`, `"YMDE"`, `"E"`, `"M"`, `"YM"`, `"Y"`
/// - `time` - Time precision: `"hour"`, `"minute"`, `"second"`, `"millisecond"`
/// - `align` - Column alignment: `"column"`, `"none"`
/// - `year` - Year style: `"full"`, `"with_era"`, `"auto"`
/// - `length` - Overall length: `"short"`, `"medium"`, `"long"`
///
/// # Examples
///
/// ```rust,ignore
/// // Default format (YMD, medium length)
/// tr!("EVENT_DATE", "Event on {{date}}.", date:Date = timestamp);
///
/// // Short year-month-day
/// tr!("CREATED", "Created {{date}}.", date:Date("YMD", length="short") = created_at);
///
/// // Time only
/// tr!("ALARM", "Alarm set for {{time}}.", time:Date("T") = alarm_time);
///
/// // Full date with weekday
/// tr!("MEETING", "Meeting on {{date}}.", date:Date("YMDE", length="long") = meeting_date);
///
/// // Complex: date and time with custom options
/// tr!(
///     "APPOINTMENT",
///     "Your appointment is {{datetime}}.",
///     datetime:Date(date="YMD", time="minute", length="medium") = appointment
/// );
/// ```
///
/// # Locale awareness
///
/// Output is automatically formatted according to the user's locale:
///
/// - **en-US**: "January 15, 2024"
/// - **de-DE**: "15. Januar 2024"
/// - **ja-JP**: "2024年1月15日"
/// - **ar-SA**: "١٥ يناير ٢٠٢٤"
pub struct Date;

/// Internal enum for date format length options.
enum DateLength {
    /// Abbreviated format (e.g., "1/15/24").
    Short,
    /// Default format (e.g., "Jan 15, 2024").
    Medium,
    /// Full format (e.g., "January 15, 2024").
    Long,
}

impl Date {
    fn make_basic_field_set(string: &str, length: Option<DateLength>) -> CompositeDateTimeFieldSet {
        match string {
            "D" => CompositeDateTimeFieldSet::Date(DateFieldSet::D(
                match length.unwrap_or(DateLength::Medium) {
                    DateLength::Short => D::short(),
                    DateLength::Medium => D::medium(),
                    DateLength::Long => D::long(),
                },
            )),
            "DE" => CompositeDateTimeFieldSet::Date(DateFieldSet::DE(
                match length.unwrap_or(DateLength::Medium) {
                    DateLength::Short => DE::short(),
                    DateLength::Medium => DE::medium(),
                    DateLength::Long => DE::long(),
                },
            )),
            "DET" => CompositeDateTimeFieldSet::DateTime(DateAndTimeFieldSet::DET(
                match length.unwrap_or(DateLength::Medium) {
                    DateLength::Short => DET::short(),
                    DateLength::Medium => DET::medium(),
                    DateLength::Long => DET::long(),
                },
            )),
            "DT" => CompositeDateTimeFieldSet::DateTime(DateAndTimeFieldSet::DT(
                match length.unwrap_or(DateLength::Medium) {
                    DateLength::Short => DT::short(),
                    DateLength::Medium => DT::medium(),
                    DateLength::Long => DT::long(),
                },
            )),
            "E" => CompositeDateTimeFieldSet::Date(DateFieldSet::E(
                match length.unwrap_or(DateLength::Medium) {
                    DateLength::Short => E::short(),
                    DateLength::Medium => E::medium(),
                    DateLength::Long => E::long(),
                },
            )),
            "ET" => CompositeDateTimeFieldSet::DateTime(DateAndTimeFieldSet::ET(
                match length.unwrap_or(DateLength::Medium) {
                    DateLength::Short => ET::short(),
                    DateLength::Medium => ET::medium(),
                    DateLength::Long => ET::long(),
                },
            )),
            "M" => CompositeDateTimeFieldSet::CalendarPeriod(CalendarPeriodFieldSet::M(
                match length.unwrap_or(DateLength::Medium) {
                    DateLength::Short => M::short(),
                    DateLength::Medium => M::medium(),
                    DateLength::Long => M::long(),
                },
            )),
            "MD" => CompositeDateTimeFieldSet::Date(DateFieldSet::MD(
                match length.unwrap_or(DateLength::Medium) {
                    DateLength::Short => MD::short(),
                    DateLength::Medium => MD::medium(),
                    DateLength::Long => MD::long(),
                },
            )),
            "MDE" => CompositeDateTimeFieldSet::Date(DateFieldSet::MDE(
                match length.unwrap_or(DateLength::Medium) {
                    DateLength::Short => MDE::short(),
                    DateLength::Medium => MDE::medium(),
                    DateLength::Long => MDE::long(),
                },
            )),
            "MDET" => CompositeDateTimeFieldSet::DateTime(DateAndTimeFieldSet::MDET(match length
                .unwrap_or(DateLength::Medium)
            {
                DateLength::Short => MDET::short(),
                DateLength::Medium => MDET::medium(),
                DateLength::Long => MDET::long(),
            })),
            "MDT" => CompositeDateTimeFieldSet::DateTime(DateAndTimeFieldSet::MDT(
                match length.unwrap_or(DateLength::Medium) {
                    DateLength::Short => MDT::short(),
                    DateLength::Medium => MDT::medium(),
                    DateLength::Long => MDT::long(),
                },
            )),
            "T" => CompositeDateTimeFieldSet::Time(TimeFieldSet::T(
                match length.unwrap_or(DateLength::Medium) {
                    DateLength::Short => T::short(),
                    DateLength::Medium => T::medium(),
                    DateLength::Long => T::long(),
                },
            )),
            "Y" => CompositeDateTimeFieldSet::CalendarPeriod(CalendarPeriodFieldSet::Y(
                match length.unwrap_or(DateLength::Medium) {
                    DateLength::Short => Y::short(),
                    DateLength::Medium => Y::medium(),
                    DateLength::Long => Y::long(),
                },
            )),
            "YM" => {
                CompositeDateTimeFieldSet::CalendarPeriod(CalendarPeriodFieldSet::YM(match length
                    .unwrap_or(DateLength::Medium)
                {
                    DateLength::Short => YM::short(),
                    DateLength::Medium => YM::medium(),
                    DateLength::Long => YM::long(),
                }))
            }
            "YMD" => CompositeDateTimeFieldSet::Date(DateFieldSet::YMD(
                match length.unwrap_or(DateLength::Medium) {
                    DateLength::Short => YMD::short(),
                    DateLength::Medium => YMD::medium(),
                    DateLength::Long => YMD::long(),
                },
            )),
            "YMDE" => CompositeDateTimeFieldSet::Date(DateFieldSet::YMDE(
                match length.unwrap_or(DateLength::Medium) {
                    DateLength::Short => YMDE::short(),
                    DateLength::Medium => YMDE::medium(),
                    DateLength::Long => YMDE::long(),
                },
            )),
            "YMDET" => CompositeDateTimeFieldSet::DateTime(DateAndTimeFieldSet::YMDET(
                match length.unwrap_or(DateLength::Medium) {
                    DateLength::Short => YMDET::short(),
                    DateLength::Medium => YMDET::medium(),
                    DateLength::Long => YMDET::long(),
                },
            )),
            "YMDT" => CompositeDateTimeFieldSet::DateTime(DateAndTimeFieldSet::YMDT(match length
                .unwrap_or(DateLength::Medium)
            {
                DateLength::Short => YMDT::short(),
                DateLength::Medium => YMDT::medium(),
                DateLength::Long => YMDT::long(),
            })),
            _ => panic!("Invalid date format: {string:?}"),
        }
    }

    fn make_complex_field_set<'a>(
        date_fields: Option<&'a str>,
        time_precision: Option<&'a str>,
        alignment: Option<&'a str>,
        year_style: Option<&'a str>,
        length: Option<&'a str>,
    ) -> CompositeDateTimeFieldSet {
        let mut builder = FieldSetBuilder::new();
        builder.date_fields = match date_fields {
            Some("D") => Some(DateFields::D),
            Some("MD") => Some(DateFields::MD),
            Some("YMD") => Some(DateFields::YMD),
            Some("DE") => Some(DateFields::DE),
            Some("MDE") => Some(DateFields::MDE),
            Some("YMDE") => Some(DateFields::YMDE),
            Some("E") => Some(DateFields::E),
            Some("M") => Some(DateFields::M),
            Some("YM") => Some(DateFields::YM),
            Some("Y") => Some(DateFields::Y),
            None => None,
            _ => panic!("Invalid date fields: {date_fields:?}"),
        };

        builder.time_precision = match time_precision {
            Some("hour") => Some(TimePrecision::Hour),
            Some("minute") => Some(TimePrecision::Minute),
            Some("second") => Some(TimePrecision::Second),
            Some("millisecond") => Some(TimePrecision::Subsecond(SubsecondDigits::S3)),
            Some("microsecond") => Some(TimePrecision::Subsecond(SubsecondDigits::S3)),
            Some("nanosecond") => Some(TimePrecision::Subsecond(SubsecondDigits::S3)),
            None => None,
            _ => panic!("Invalid time precision: {time_precision:?}"),
        };

        builder.alignment = match alignment {
            Some("column") => Some(Alignment::Column),
            Some("none") => Some(Alignment::Auto),
            None => Some(Alignment::Auto),
            _ => panic!("Invalid alignment: {alignment:?}"),
        };

        builder.year_style = match year_style {
            Some("full") => Some(YearStyle::Full),
            Some("with_era") => Some(YearStyle::WithEra),
            Some("auto") => Some(YearStyle::Auto),
            None => None,
            _ => panic!("Invalid year style: {year_style:?}"),
        };

        builder.length = match length {
            Some("short") => Some(Length::Short),
            Some("medium") => Some(Length::Medium),
            Some("long") => Some(Length::Long),
            None => None,
            _ => panic!("Invalid length: {length:?}"),
        };

        builder
            .build_composite_datetime()
            .expect("Failed to build composite field set")
    }

    fn make_field_set<'a>(
        variables: &'a [super::ModifierVariable<'a>],
    ) -> CompositeDateTimeFieldSet {
        if variables.is_empty() {
            CompositeDateTimeFieldSet::Date(DateFieldSet::YMD(YMD::medium()))
        } else if let Some((None, string)) = variables.first() {
            let length = variables
                .iter()
                .find(|v| v.0 == Some("length"))
                .map(|v| match v.1 {
                    "short" => DateLength::Short,
                    "medium" => DateLength::Medium,
                    "long" => DateLength::Long,
                    _ => panic!("Invalid length: {:?}", v.1),
                });

            Date::make_basic_field_set(string, length)
        } else {
            let date_fields = variables.iter().find(|v| v.0 == Some("date")).map(|v| v.1);
            let time_precision = variables.iter().find(|v| v.0 == Some("time")).map(|v| v.1);
            let alignment = variables.iter().find(|v| v.0 == Some("align")).map(|v| v.1);
            let year_style = variables.iter().find(|v| v.0 == Some("year")).map(|v| v.1);
            let length = variables
                .iter()
                .find(|v| v.0 == Some("length"))
                .map(|v| v.1);

            Date::make_complex_field_set(date_fields, time_precision, alignment, year_style, length)
        }
    }

    fn make_date_string(
        locale: &Locale,
        input: DateTime<Iso>,
        variables: CompositeDateTimeFieldSet,
    ) -> String {
        let dtf = DateTimeFormatter::try_new(locale.time_icu.clone().into(), variables).unwrap();

        dtf.format(&input).to_string()
    }
}

impl StringModifier<&str> for Date {
    fn transform<'a>(
        &self,
        locale: &Locale,
        input: &str,
        variables: &'a [super::ModifierVariable<'a>],
    ) -> String {
        let fset = Date::make_field_set(variables);
        let Ok(date) = DateTime::try_from_str(input, Iso) else {
            return "Invalid Date".to_string();
        };

        Date::make_date_string(locale, date, fset)
    }
}

#[cfg(feature = "chrono")]
impl<T: chrono::TimeZone> StringModifier<&chrono::DateTime<T>> for Date {
    fn transform<'a>(
        &self,
        locale: &Locale,
        input: &chrono::DateTime<T>,
        variables: &'a [super::ModifierVariable<'a>],
    ) -> String {
        use chrono::Offset;
        use icu::time::{ZonedDateTime, zone::UtcOffset};

        let fset = Date::make_field_set(variables);
        let offset = UtcOffset::try_from_seconds(input.offset().fix().local_minus_utc()).unwrap();
        let epoch = input.timestamp_millis();

        let zdt =
            ZonedDateTime::<Iso, UtcOffset>::from_epoch_milliseconds_and_utc_offset(epoch, offset);

        let date = DateTime {
            date: zdt.date,
            time: zdt.time,
        };

        Date::make_date_string(locale, date, fset)
    }
}

#[cfg(feature = "chrono")]
impl StringModifier<&chrono::NaiveDateTime> for Date {
    fn transform<'a>(
        &self,
        locale: &Locale,
        input: &chrono::NaiveDateTime,
        variables: &'a [super::ModifierVariable<'a>],
    ) -> String {
        let fset = Date::make_field_set(variables);
        let offset = UtcOffset::try_from_seconds(0).unwrap();
        let epoch = input.and_utc().timestamp_millis();

        let zdt =
            ZonedDateTime::<Iso, UtcOffset>::from_epoch_milliseconds_and_utc_offset(epoch, offset);

        let date = DateTime {
            date: zdt.date,
            time: zdt.time,
        };

        Date::make_date_string(locale, date, fset)
    }
}

macro_rules! float_impl_date {
    ($t:ty) => {
        impl StringModifier<&$t> for Date {
            fn transform<'a>(
                &self,
                locale: &Locale,
                input: &$t,
                variables: &'a [super::ModifierVariable<'a>],
            ) -> String {
                let fset = Date::make_field_set(variables);
                let offset = UtcOffset::try_from_seconds(0).unwrap();
                let epoch = (*input as f64 * 1000_f64).trunc() as i64;

                let zdt = ZonedDateTime::<Iso, UtcOffset>::from_epoch_milliseconds_and_utc_offset(
                    epoch, offset,
                );

                let date = DateTime {
                    date: zdt.date,
                    time: zdt.time,
                };

                Date::make_date_string(locale, date, fset)
            }
        }
    };
}

float_impl_date!(f32);
float_impl_date!(f64);

macro_rules! int_impl_date {
    ($t:ty) => {
        impl StringModifier<&$t> for Date {
            fn transform<'a>(
                &self,
                locale: &Locale,
                input: &$t,
                variables: &'a [super::ModifierVariable<'a>],
            ) -> String {
                let fset = Date::make_field_set(variables);
                let offset = UtcOffset::try_from_seconds(0).unwrap();
                let epoch = *input as i64;

                let zdt = ZonedDateTime::<Iso, UtcOffset>::from_epoch_milliseconds_and_utc_offset(
                    epoch, offset,
                );

                let date = DateTime {
                    date: zdt.date,
                    time: zdt.time,
                };

                Date::make_date_string(locale, date, fset)
            }
        }
    };
}

int_impl_date!(i8);
int_impl_date!(i16);
int_impl_date!(i32);
int_impl_date!(i64);
int_impl_date!(i128);
int_impl_date!(isize);
