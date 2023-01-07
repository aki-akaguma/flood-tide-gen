mod basic {
    use flood_tide_gen::MetaType;
    use flood_tide_gen::OptStr;
    use flood_tide_gen::Pasc;
    //
    #[test]
    fn size_of() {
        #[cfg(target_pointer_width = "64")]
        {
            assert_eq!(std::mem::size_of::<OptStr>(), 184);
            assert_eq!(std::mem::size_of::<MetaType>(), 32);
        }
        #[cfg(target_pointer_width = "32")]
        {
            assert_eq!(std::mem::size_of::<OptStr>(), 96);
            assert_eq!(std::mem::size_of::<MetaType>(), 16);
        }
        assert_eq!(std::mem::size_of::<Pasc>(), 1);
    }
}
