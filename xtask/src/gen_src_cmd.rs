use flood_tide_gen::{FixupType, MetaType, Pasc};

pub fn do_gen_src() -> anyhow::Result<()> {
    flood_tide_gen::do_gen_src(
        Pasc::Void,
        "xtask/src/aki-image-view-gtk-cmd.txt",
        Some("src/conf/cmd.help.rs.txt"),
        Some("src/conf/cmd.match.rs.txt"),
        |opt_str| {
            let tup = match opt_str.lon_or_sho() {
                "config" => (true, false, opt_str.meta_type.clone()),
                //
                "X" => (false, true, MetaType::Other("opt_uc_x_param".into())),
                _ => return None,
            };
            Some(FixupType::from_tuple(tup))
        },
    )
}