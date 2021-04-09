mod basic {
    use flood_tide_gen::MetaType;
    use flood_tide_gen::OptStr;
    use flood_tide_gen::SrcHelpFlags;
    //
    #[test]
    fn size_of() {
        assert_eq!(std::mem::size_of::<OptStr>(), 184);
        assert_eq!(std::mem::size_of::<MetaType>(), 32);
        assert_eq!(std::mem::size_of::<SrcHelpFlags>(), 8);
    }
}
