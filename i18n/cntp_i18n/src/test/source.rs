// pub struct TestSource;
//
// impl I18nSource for TestSource {
//     fn lookup(&self, locale: &Locale, id: &str, lookup_crate: &str) -> Option<&I18nEntry> {
//         match locale.messages.first().unwrap().as_str() {
//             "en" => match id {
//                 "TEST_STRING_ONE" => Some(&I18nEntry::Entry(I18nStringEntry {
//                     entry: I18nString::Borrowed("Hello World!"),
//                 })),
//                 _ => None,
//             },
//             "vi" => match id {
//                 "TEST_STRING_ONE" => Some(&I18nEntry::Entry(I18nStringEntry {
//                     entry: I18nString::Borrowed("Xin chào cả thế giới!"),
//                 })),
//                 _ => None,
//             },
//             _ => None,
//         }
//     }
// }
