// use crate::test::source::TestSource;

mod source;
//
// #[test]
// pub fn test() {
//     I18N_MANAGER.write().unwrap().load_source(TestSource);
//     let english = Locale::new_from_locale_identifier("en");
//     let vietnamese = Locale::new_from_locale_identifier("vi");
//
//     assert_eq!(
//         tr!("TEST_STRING_ONE",
//         #locale=&english)
//         .to_string()
//         .as_str(),
//         "Hello World!"
//     );
//
//     assert_eq!(
//         tr!("TEST_STRING_ONE",
//         #locale=&vietnamese)
//             .to_string()
//             .as_str(),
//         "Xin chào cả thế giới!"
//     )
// }
