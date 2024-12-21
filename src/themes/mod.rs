use egui::{Color32, Style};
use strum::IntoEnumIterator;

pub mod base16_3024;
pub mod base16_apathy;
pub mod base16_apprentice;
pub mod base16_ashes;
pub mod base16_atelier_cave;
pub mod base16_atelier_cave_light;
pub mod base16_atelier_dune;
pub mod base16_atelier_dune_light;
pub mod base16_atelier_estuary;
pub mod base16_atelier_estuary_light;
pub mod base16_atelier_forest;
pub mod base16_atelier_forest_light;
pub mod base16_atelier_heath;
pub mod base16_atelier_heath_light;
pub mod base16_atelier_lakeside;
pub mod base16_atelier_lakeside_light;
pub mod base16_atelier_plateau;
pub mod base16_atelier_plateau_light;
pub mod base16_atelier_savanna;
pub mod base16_atelier_savanna_light;
pub mod base16_atelier_seaside;
pub mod base16_atelier_seaside_light;
pub mod base16_atelier_sulphurpool;
pub mod base16_atelier_sulphurpool_light;
pub mod base16_atlas;
pub mod base16_ayu_dark;
pub mod base16_ayu_light;
pub mod base16_ayu_mirage;
pub mod base16_aztec;
pub mod base16_bespin;
pub mod base16_black_metal;
pub mod base16_black_metal_bathory;
pub mod base16_black_metal_burzum;
pub mod base16_black_metal_dark_funeral;
pub mod base16_black_metal_gorgoroth;
pub mod base16_black_metal_immortal;
pub mod base16_black_metal_khold;
pub mod base16_black_metal_marduk;
pub mod base16_black_metal_mayhem;
pub mod base16_black_metal_nile;
pub mod base16_black_metal_venom;
pub mod base16_blueforest;
pub mod base16_blueish;
pub mod base16_brewer;
pub mod base16_bright;
pub mod base16_brogrammer;
pub mod base16_brushtrees;
pub mod base16_brushtrees_dark;
pub mod base16_caroline;
pub mod base16_catppuccin_frappe;
pub mod base16_catppuccin_latte;
pub mod base16_catppuccin_macchiato;
pub mod base16_catppuccin_mocha;
pub mod base16_chalk;
pub mod base16_circus;
pub mod base16_classic_dark;
pub mod base16_classic_light;
pub mod base16_codeschool;
pub mod base16_colors;
pub mod base16_cupcake;
pub mod base16_cupertino;
pub mod base16_da_one_black;
pub mod base16_da_one_gray;
pub mod base16_da_one_ocean;
pub mod base16_da_one_paper;
pub mod base16_da_one_sea;
pub mod base16_da_one_white;
pub mod base16_danqing;
pub mod base16_danqing_light;
pub mod base16_darcula;
pub mod base16_darkmoss;
pub mod base16_darktooth;
pub mod base16_darkviolet;
pub mod base16_decaf;
pub mod base16_default_dark;
pub mod base16_default_light;
pub mod base16_dirtysea;
pub mod base16_dracula;
pub mod base16_edge_dark;
pub mod base16_edge_light;
pub mod base16_eighties;
pub mod base16_embers;
pub mod base16_embers_light;
pub mod base16_emil;
pub mod base16_equilibrium_dark;
pub mod base16_equilibrium_gray_dark;
pub mod base16_equilibrium_gray_light;
pub mod base16_equilibrium_light;
pub mod base16_eris;
pub mod base16_espresso;
pub mod base16_eva;
pub mod base16_eva_dim;
pub mod base16_evenok_dark;
pub mod base16_everforest;
pub mod base16_everforest_dark_hard;
pub mod base16_flat;
pub mod base16_framer;
pub mod base16_fruit_soda;
pub mod base16_gigavolt;
pub mod base16_github;
pub mod base16_google_dark;
pub mod base16_google_light;
pub mod base16_gotham;
pub mod base16_grayscale_dark;
pub mod base16_grayscale_light;
pub mod base16_greenscreen;
pub mod base16_gruber;
pub mod base16_gruvbox_dark_hard;
pub mod base16_gruvbox_dark_medium;
pub mod base16_gruvbox_dark_pale;
pub mod base16_gruvbox_dark_soft;
pub mod base16_gruvbox_light_hard;
pub mod base16_gruvbox_light_medium;
pub mod base16_gruvbox_light_soft;
pub mod base16_gruvbox_material_dark_hard;
pub mod base16_gruvbox_material_dark_medium;
pub mod base16_gruvbox_material_dark_soft;
pub mod base16_gruvbox_material_light_hard;
pub mod base16_gruvbox_material_light_medium;
pub mod base16_gruvbox_material_light_soft;
pub mod base16_hardcore;
pub mod base16_harmonic16_dark;
pub mod base16_harmonic16_light;
pub mod base16_heetch;
pub mod base16_heetch_light;
pub mod base16_helios;
pub mod base16_hopscotch;
pub mod base16_horizon_dark;
pub mod base16_horizon_light;
pub mod base16_horizon_terminal_dark;
pub mod base16_horizon_terminal_light;
pub mod base16_humanoid_dark;
pub mod base16_humanoid_light;
pub mod base16_ia_dark;
pub mod base16_ia_light;
pub mod base16_icy;
pub mod base16_irblack;
pub mod base16_isotope;
pub mod base16_jabuti;
pub mod base16_kanagawa;
pub mod base16_katy;
pub mod base16_kimber;
pub mod base16_lime;
pub mod base16_macintosh;
pub mod base16_marrakesh;
pub mod base16_materia;
pub mod base16_material;
pub mod base16_material_darker;
pub mod base16_material_lighter;
pub mod base16_material_palenight;
pub mod base16_material_vivid;
pub mod base16_measured_dark;
pub mod base16_measured_light;
pub mod base16_mellow_purple;
pub mod base16_mexico_light;
pub mod base16_mocha;
pub mod base16_monokai;
pub mod base16_moonlight;
pub mod base16_mountain;
pub mod base16_nebula;
pub mod base16_nord;
pub mod base16_nord_light;
pub mod base16_nova;
pub mod base16_ocean;
pub mod base16_oceanicnext;
pub mod base16_one_light;
pub mod base16_onedark;
pub mod base16_onedark_dark;
pub mod base16_outrun_dark;
pub mod base16_oxocarbon_dark;
pub mod base16_oxocarbon_light;
pub mod base16_pandora;
pub mod base16_papercolor_dark;
pub mod base16_papercolor_light;
pub mod base16_paraiso;
pub mod base16_pasque;
pub mod base16_phd;
pub mod base16_pico;
pub mod base16_pinky;
pub mod base16_pop;
pub mod base16_porple;
pub mod base16_precious_dark_eleven;
pub mod base16_precious_dark_fifteen;
pub mod base16_precious_light_warm;
pub mod base16_precious_light_white;
pub mod base16_primer_dark;
pub mod base16_primer_dark_dimmed;
pub mod base16_primer_light;
pub mod base16_purpledream;
pub mod base16_qualia;
pub mod base16_railscasts;
pub mod base16_rebecca;
pub mod base16_rose_pine;
pub mod base16_rose_pine_dawn;
pub mod base16_rose_pine_moon;
pub mod base16_saga;
pub mod base16_sagelight;
pub mod base16_sakura;
pub mod base16_sandcastle;
pub mod base16_selenized_black;
pub mod base16_selenized_dark;
pub mod base16_selenized_light;
pub mod base16_selenized_white;
pub mod base16_seti;
pub mod base16_shades_of_purple;
pub mod base16_shadesmear_dark;
pub mod base16_shadesmear_light;
pub mod base16_shapeshifter;
pub mod base16_silk_dark;
pub mod base16_silk_light;
pub mod base16_snazzy;
pub mod base16_solarflare;
pub mod base16_solarflare_light;
pub mod base16_solarized_dark;
pub mod base16_solarized_light;
pub mod base16_spaceduck;
pub mod base16_spacemacs;
pub mod base16_sparky;
pub mod base16_standardized_dark;
pub mod base16_standardized_light;
pub mod base16_stella;
pub mod base16_still_alive;
pub mod base16_summercamp;
pub mod base16_summerfruit_dark;
pub mod base16_summerfruit_light;
pub mod base16_synth_midnight_dark;
pub mod base16_synth_midnight_light;
pub mod base16_tango;
pub mod base16_tarot;
pub mod base16_tender;
pub mod base16_terracotta;
pub mod base16_terracotta_dark;
pub mod base16_tokyo_city_dark;
pub mod base16_tokyo_city_light;
pub mod base16_tokyo_city_terminal_dark;
pub mod base16_tokyo_city_terminal_light;
pub mod base16_tokyo_night_dark;
pub mod base16_tokyo_night_light;
pub mod base16_tokyo_night_moon;
pub mod base16_tokyo_night_storm;
pub mod base16_tokyo_night_terminal_dark;
pub mod base16_tokyo_night_terminal_light;
pub mod base16_tokyo_night_terminal_storm;
pub mod base16_tokyodark;
pub mod base16_tokyodark_terminal;
pub mod base16_tomorrow;
pub mod base16_tomorrow_night;
pub mod base16_tomorrow_night_eighties;
pub mod base16_tube;
pub mod base16_twilight;
pub mod base16_unikitty_dark;
pub mod base16_unikitty_light;
pub mod base16_unikitty_reversible;
pub mod base16_uwunicorn;
pub mod base16_vesper;
pub mod base16_vice;
pub mod base16_vulcan;
pub mod base16_windows_10;
pub mod base16_windows_10_light;
pub mod base16_windows_95;
pub mod base16_windows_95_light;
pub mod base16_windows_highcontrast;
pub mod base16_windows_highcontrast_light;
pub mod base16_windows_nt;
pub mod base16_windows_nt_light;
pub mod base16_woodland;
pub mod base16_xcode_dusk;
pub mod base16_zenbones;
pub mod base16_zenburn;

#[derive(
    Copy,
    Clone,
    Debug,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
    strum_macros::EnumIter,
    strum_macros::AsRefStr,
    strum_macros::EnumString,
)]
pub enum Base16 {
    #[serde(rename = "3024")]
    _3024,
    Apathy,
    Apprentice,
    Ashes,
    AtelierCaveLight,
    AtelierCave,
    AtelierDuneLight,
    AtelierDune,
    AtelierEstuaryLight,
    AtelierEstuary,
    AtelierForestLight,
    AtelierForest,
    AtelierHeathLight,
    AtelierHeath,
    AtelierLakesideLight,
    AtelierLakeside,
    AtelierPlateauLight,
    AtelierPlateau,
    AtelierSavannaLight,
    AtelierSavanna,
    AtelierSeasideLight,
    AtelierSeaside,
    AtelierSulphurpoolLight,
    AtelierSulphurpool,
    Atlas,
    AyuDark,
    AyuLight,
    AyuMirage,
    Aztec,
    Bespin,
    BlackMetalBathory,
    BlackMetalBurzum,
    BlackMetalDarkFuneral,
    BlackMetalGorgoroth,
    BlackMetalImmortal,
    BlackMetalKhold,
    BlackMetalMarduk,
    BlackMetalMayhem,
    BlackMetalNile,
    BlackMetalVenom,
    BlackMetal,
    Blueforest,
    Blueish,
    Brewer,
    Bright,
    Brogrammer,
    BrushtreesDark,
    Brushtrees,
    Caroline,
    CatppuccinFrappe,
    CatppuccinLatte,
    CatppuccinMacchiato,
    CatppuccinMocha,
    Chalk,
    Circus,
    ClassicDark,
    ClassicLight,
    Codeschool,
    Colors,
    Cupcake,
    Cupertino,
    DaOneBlack,
    DaOneGray,
    DaOneOcean,
    DaOnePaper,
    DaOneSea,
    DaOneWhite,
    DanqingLight,
    Danqing,
    Darcula,
    Darkmoss,
    Darktooth,
    Darkviolet,
    Decaf,
    DefaultDark,
    DefaultLight,
    Dirtysea,
    Dracula,
    EdgeDark,
    EdgeLight,
    Eighties,
    EmbersLight,
    Embers,
    Emil,
    EquilibriumDark,
    EquilibriumGrayDark,
    EquilibriumGrayLight,
    EquilibriumLight,
    Eris,
    Espresso,
    EvaDim,
    Eva,
    EvenokDark,
    EverforestDarkHard,
    Everforest,
    Flat,
    Framer,
    FruitSoda,
    Gigavolt,
    Github,
    GoogleDark,
    GoogleLight,
    Gotham,
    GrayscaleDark,
    GrayscaleLight,
    Greenscreen,
    Gruber,
    GruvboxDarkHard,
    GruvboxDarkMedium,
    GruvboxDarkPale,
    GruvboxDarkSoft,
    GruvboxLightHard,
    GruvboxLightMedium,
    GruvboxLightSoft,
    GruvboxMaterialDarkHard,
    GruvboxMaterialDarkMedium,
    GruvboxMaterialDarkSoft,
    GruvboxMaterialLightHard,
    GruvboxMaterialLightMedium,
    GruvboxMaterialLightSoft,
    Hardcore,
    Harmonic16Dark,
    Harmonic16Light,
    HeetchLight,
    Heetch,
    Helios,
    Hopscotch,
    HorizonDark,
    HorizonLight,
    HorizonTerminalDark,
    HorizonTerminalLight,
    HumanoidDark,
    HumanoidLight,
    IaDark,
    IaLight,
    Icy,
    Irblack,
    Isotope,
    Jabuti,
    Kanagawa,
    Katy,
    Kimber,
    Lime,
    Macintosh,
    Marrakesh,
    Materia,
    MaterialDarker,
    MaterialLighter,
    MaterialPalenight,
    MaterialVivid,
    Material,
    MeasuredDark,
    MeasuredLight,
    MellowPurple,
    MexicoLight,
    Mocha,
    Monokai,
    Moonlight,
    Mountain,
    Nebula,
    NordLight,
    Nord,
    Nova,
    Ocean,
    Oceanicnext,
    OneLight,
    OnedarkDark,
    Onedark,
    OutrunDark,
    OxocarbonDark,
    OxocarbonLight,
    Pandora,
    PapercolorDark,
    PapercolorLight,
    Paraiso,
    Pasque,
    Phd,
    Pico,
    Pinky,
    Pop,
    Porple,
    PreciousDarkEleven,
    PreciousDarkFifteen,
    PreciousLightWarm,
    PreciousLightWhite,
    PrimerDarkDimmed,
    PrimerDark,
    PrimerLight,
    Purpledream,
    Qualia,
    Railscasts,
    Rebecca,
    RosePineDawn,
    RosePineMoon,
    RosePine,
    Saga,
    Sagelight,
    Sakura,
    Sandcastle,
    SelenizedBlack,
    SelenizedDark,
    SelenizedLight,
    SelenizedWhite,
    Seti,
    ShadesOfPurple,
    ShadesmearDark,
    ShadesmearLight,
    Shapeshifter,
    SilkDark,
    SilkLight,
    Snazzy,
    SolarflareLight,
    Solarflare,
    SolarizedDark,
    SolarizedLight,
    Spaceduck,
    Spacemacs,
    Sparky,
    StandardizedDark,
    StandardizedLight,
    Stella,
    StillAlive,
    Summercamp,
    SummerfruitDark,
    SummerfruitLight,
    SynthMidnightDark,
    SynthMidnightLight,
    Tango,
    Tarot,
    Tender,
    TerracottaDark,
    Terracotta,
    TokyoCityDark,
    TokyoCityLight,
    TokyoCityTerminalDark,
    TokyoCityTerminalLight,
    TokyoNightDark,
    TokyoNightLight,
    TokyoNightMoon,
    TokyoNightStorm,
    TokyoNightTerminalDark,
    TokyoNightTerminalLight,
    TokyoNightTerminalStorm,
    TokyodarkTerminal,
    Tokyodark,
    TomorrowNightEighties,
    TomorrowNight,
    Tomorrow,
    Tube,
    Twilight,
    UnikittyDark,
    UnikittyLight,
    UnikittyReversible,
    Uwunicorn,
    Vesper,
    Vice,
    Vulcan,
    Windows10Light,
    Windows10,
    Windows95Light,
    Windows95,
    WindowsHighcontrastLight,
    WindowsHighcontrast,
    WindowsNtLight,
    WindowsNt,
    Woodland,
    XcodeDusk,
    Zenbones,
    Zenburn,
}

impl Base16 {
    pub fn style(self) -> Style {
        match self {
            Base16::_3024 => base16_3024::style(),
            Base16::Apathy => base16_apathy::style(),
            Base16::Apprentice => base16_apprentice::style(),
            Base16::Ashes => base16_ashes::style(),
            Base16::AtelierCaveLight => base16_atelier_cave_light::style(),
            Base16::AtelierCave => base16_atelier_cave::style(),
            Base16::AtelierDuneLight => base16_atelier_dune_light::style(),
            Base16::AtelierDune => base16_atelier_dune::style(),
            Base16::AtelierEstuaryLight => base16_atelier_estuary_light::style(),
            Base16::AtelierEstuary => base16_atelier_estuary::style(),
            Base16::AtelierForestLight => base16_atelier_forest_light::style(),
            Base16::AtelierForest => base16_atelier_forest::style(),
            Base16::AtelierHeathLight => base16_atelier_heath_light::style(),
            Base16::AtelierHeath => base16_atelier_heath::style(),
            Base16::AtelierLakesideLight => base16_atelier_lakeside_light::style(),
            Base16::AtelierLakeside => base16_atelier_lakeside::style(),
            Base16::AtelierPlateauLight => base16_atelier_plateau_light::style(),
            Base16::AtelierPlateau => base16_atelier_plateau::style(),
            Base16::AtelierSavannaLight => base16_atelier_savanna_light::style(),
            Base16::AtelierSavanna => base16_atelier_savanna::style(),
            Base16::AtelierSeasideLight => base16_atelier_seaside_light::style(),
            Base16::AtelierSeaside => base16_atelier_seaside::style(),
            Base16::AtelierSulphurpoolLight => base16_atelier_sulphurpool_light::style(),
            Base16::AtelierSulphurpool => base16_atelier_sulphurpool::style(),
            Base16::Atlas => base16_atlas::style(),
            Base16::AyuDark => base16_ayu_dark::style(),
            Base16::AyuLight => base16_ayu_light::style(),
            Base16::AyuMirage => base16_ayu_mirage::style(),
            Base16::Aztec => base16_aztec::style(),
            Base16::Bespin => base16_bespin::style(),
            Base16::BlackMetalBathory => base16_black_metal_bathory::style(),
            Base16::BlackMetalBurzum => base16_black_metal_burzum::style(),
            Base16::BlackMetalDarkFuneral => base16_black_metal_dark_funeral::style(),
            Base16::BlackMetalGorgoroth => base16_black_metal_gorgoroth::style(),
            Base16::BlackMetalImmortal => base16_black_metal_immortal::style(),
            Base16::BlackMetalKhold => base16_black_metal_khold::style(),
            Base16::BlackMetalMarduk => base16_black_metal_marduk::style(),
            Base16::BlackMetalMayhem => base16_black_metal_mayhem::style(),
            Base16::BlackMetalNile => base16_black_metal_nile::style(),
            Base16::BlackMetalVenom => base16_black_metal_venom::style(),
            Base16::BlackMetal => base16_black_metal::style(),
            Base16::Blueforest => base16_blueforest::style(),
            Base16::Blueish => base16_blueish::style(),
            Base16::Brewer => base16_brewer::style(),
            Base16::Bright => base16_bright::style(),
            Base16::Brogrammer => base16_brogrammer::style(),
            Base16::BrushtreesDark => base16_brushtrees_dark::style(),
            Base16::Brushtrees => base16_brushtrees::style(),
            Base16::Caroline => base16_caroline::style(),
            Base16::CatppuccinFrappe => base16_catppuccin_frappe::style(),
            Base16::CatppuccinLatte => base16_catppuccin_latte::style(),
            Base16::CatppuccinMacchiato => base16_catppuccin_macchiato::style(),
            Base16::CatppuccinMocha => base16_catppuccin_mocha::style(),
            Base16::Chalk => base16_chalk::style(),
            Base16::Circus => base16_circus::style(),
            Base16::ClassicDark => base16_classic_dark::style(),
            Base16::ClassicLight => base16_classic_light::style(),
            Base16::Codeschool => base16_codeschool::style(),
            Base16::Colors => base16_colors::style(),
            Base16::Cupcake => base16_cupcake::style(),
            Base16::Cupertino => base16_cupertino::style(),
            Base16::DaOneBlack => base16_da_one_black::style(),
            Base16::DaOneGray => base16_da_one_gray::style(),
            Base16::DaOneOcean => base16_da_one_ocean::style(),
            Base16::DaOnePaper => base16_da_one_paper::style(),
            Base16::DaOneSea => base16_da_one_sea::style(),
            Base16::DaOneWhite => base16_da_one_white::style(),
            Base16::DanqingLight => base16_danqing_light::style(),
            Base16::Danqing => base16_danqing::style(),
            Base16::Darcula => base16_darcula::style(),
            Base16::Darkmoss => base16_darkmoss::style(),
            Base16::Darktooth => base16_darktooth::style(),
            Base16::Darkviolet => base16_darkviolet::style(),
            Base16::Decaf => base16_decaf::style(),
            Base16::DefaultDark => base16_default_dark::style(),
            Base16::DefaultLight => base16_default_light::style(),
            Base16::Dirtysea => base16_dirtysea::style(),
            Base16::Dracula => base16_dracula::style(),
            Base16::EdgeDark => base16_edge_dark::style(),
            Base16::EdgeLight => base16_edge_light::style(),
            Base16::Eighties => base16_eighties::style(),
            Base16::EmbersLight => base16_embers_light::style(),
            Base16::Embers => base16_embers::style(),
            Base16::Emil => base16_emil::style(),
            Base16::EquilibriumDark => base16_equilibrium_dark::style(),
            Base16::EquilibriumGrayDark => base16_equilibrium_gray_dark::style(),
            Base16::EquilibriumGrayLight => base16_equilibrium_gray_light::style(),
            Base16::EquilibriumLight => base16_equilibrium_light::style(),
            Base16::Eris => base16_eris::style(),
            Base16::Espresso => base16_espresso::style(),
            Base16::EvaDim => base16_eva_dim::style(),
            Base16::Eva => base16_eva::style(),
            Base16::EvenokDark => base16_evenok_dark::style(),
            Base16::EverforestDarkHard => base16_everforest_dark_hard::style(),
            Base16::Everforest => base16_everforest::style(),
            Base16::Flat => base16_flat::style(),
            Base16::Framer => base16_framer::style(),
            Base16::FruitSoda => base16_fruit_soda::style(),
            Base16::Gigavolt => base16_gigavolt::style(),
            Base16::Github => base16_github::style(),
            Base16::GoogleDark => base16_google_dark::style(),
            Base16::GoogleLight => base16_google_light::style(),
            Base16::Gotham => base16_gotham::style(),
            Base16::GrayscaleDark => base16_grayscale_dark::style(),
            Base16::GrayscaleLight => base16_grayscale_light::style(),
            Base16::Greenscreen => base16_greenscreen::style(),
            Base16::Gruber => base16_gruber::style(),
            Base16::GruvboxDarkHard => base16_gruvbox_dark_hard::style(),
            Base16::GruvboxDarkMedium => base16_gruvbox_dark_medium::style(),
            Base16::GruvboxDarkPale => base16_gruvbox_dark_pale::style(),
            Base16::GruvboxDarkSoft => base16_gruvbox_dark_soft::style(),
            Base16::GruvboxLightHard => base16_gruvbox_light_hard::style(),
            Base16::GruvboxLightMedium => base16_gruvbox_light_medium::style(),
            Base16::GruvboxLightSoft => base16_gruvbox_light_soft::style(),
            Base16::GruvboxMaterialDarkHard => base16_gruvbox_material_dark_hard::style(),
            Base16::GruvboxMaterialDarkMedium => base16_gruvbox_material_dark_medium::style(),
            Base16::GruvboxMaterialDarkSoft => base16_gruvbox_material_dark_soft::style(),
            Base16::GruvboxMaterialLightHard => base16_gruvbox_material_light_hard::style(),
            Base16::GruvboxMaterialLightMedium => base16_gruvbox_material_light_medium::style(),
            Base16::GruvboxMaterialLightSoft => base16_gruvbox_material_light_soft::style(),
            Base16::Hardcore => base16_hardcore::style(),
            Base16::Harmonic16Dark => base16_harmonic16_dark::style(),
            Base16::Harmonic16Light => base16_harmonic16_light::style(),
            Base16::HeetchLight => base16_heetch_light::style(),
            Base16::Heetch => base16_heetch::style(),
            Base16::Helios => base16_helios::style(),
            Base16::Hopscotch => base16_hopscotch::style(),
            Base16::HorizonDark => base16_horizon_dark::style(),
            Base16::HorizonLight => base16_horizon_light::style(),
            Base16::HorizonTerminalDark => base16_horizon_terminal_dark::style(),
            Base16::HorizonTerminalLight => base16_horizon_terminal_light::style(),
            Base16::HumanoidDark => base16_humanoid_dark::style(),
            Base16::HumanoidLight => base16_humanoid_light::style(),
            Base16::IaDark => base16_ia_dark::style(),
            Base16::IaLight => base16_ia_light::style(),
            Base16::Icy => base16_icy::style(),
            Base16::Irblack => base16_irblack::style(),
            Base16::Isotope => base16_isotope::style(),
            Base16::Jabuti => base16_jabuti::style(),
            Base16::Kanagawa => base16_kanagawa::style(),
            Base16::Katy => base16_katy::style(),
            Base16::Kimber => base16_kimber::style(),
            Base16::Lime => base16_lime::style(),
            Base16::Macintosh => base16_macintosh::style(),
            Base16::Marrakesh => base16_marrakesh::style(),
            Base16::Materia => base16_materia::style(),
            Base16::MaterialDarker => base16_material_darker::style(),
            Base16::MaterialLighter => base16_material_lighter::style(),
            Base16::MaterialPalenight => base16_material_palenight::style(),
            Base16::MaterialVivid => base16_material_vivid::style(),
            Base16::Material => base16_material::style(),
            Base16::MeasuredDark => base16_measured_dark::style(),
            Base16::MeasuredLight => base16_measured_light::style(),
            Base16::MellowPurple => base16_mellow_purple::style(),
            Base16::MexicoLight => base16_mexico_light::style(),
            Base16::Mocha => base16_mocha::style(),
            Base16::Monokai => base16_monokai::style(),
            Base16::Moonlight => base16_moonlight::style(),
            Base16::Mountain => base16_mountain::style(),
            Base16::Nebula => base16_nebula::style(),
            Base16::NordLight => base16_nord_light::style(),
            Base16::Nord => base16_nord::style(),
            Base16::Nova => base16_nova::style(),
            Base16::Ocean => base16_ocean::style(),
            Base16::Oceanicnext => base16_oceanicnext::style(),
            Base16::OneLight => base16_one_light::style(),
            Base16::OnedarkDark => base16_onedark_dark::style(),
            Base16::Onedark => base16_onedark::style(),
            Base16::OutrunDark => base16_outrun_dark::style(),
            Base16::OxocarbonDark => base16_oxocarbon_dark::style(),
            Base16::OxocarbonLight => base16_oxocarbon_light::style(),
            Base16::Pandora => base16_pandora::style(),
            Base16::PapercolorDark => base16_papercolor_dark::style(),
            Base16::PapercolorLight => base16_papercolor_light::style(),
            Base16::Paraiso => base16_paraiso::style(),
            Base16::Pasque => base16_pasque::style(),
            Base16::Phd => base16_phd::style(),
            Base16::Pico => base16_pico::style(),
            Base16::Pinky => base16_pinky::style(),
            Base16::Pop => base16_pop::style(),
            Base16::Porple => base16_porple::style(),
            Base16::PreciousDarkEleven => base16_precious_dark_eleven::style(),
            Base16::PreciousDarkFifteen => base16_precious_dark_fifteen::style(),
            Base16::PreciousLightWarm => base16_precious_light_warm::style(),
            Base16::PreciousLightWhite => base16_precious_light_white::style(),
            Base16::PrimerDarkDimmed => base16_primer_dark_dimmed::style(),
            Base16::PrimerDark => base16_primer_dark::style(),
            Base16::PrimerLight => base16_primer_light::style(),
            Base16::Purpledream => base16_purpledream::style(),
            Base16::Qualia => base16_qualia::style(),
            Base16::Railscasts => base16_railscasts::style(),
            Base16::Rebecca => base16_rebecca::style(),
            Base16::RosePineDawn => base16_rose_pine_dawn::style(),
            Base16::RosePineMoon => base16_rose_pine_moon::style(),
            Base16::RosePine => base16_rose_pine::style(),
            Base16::Saga => base16_saga::style(),
            Base16::Sagelight => base16_sagelight::style(),
            Base16::Sakura => base16_sakura::style(),
            Base16::Sandcastle => base16_sandcastle::style(),
            Base16::SelenizedBlack => base16_selenized_black::style(),
            Base16::SelenizedDark => base16_selenized_dark::style(),
            Base16::SelenizedLight => base16_selenized_light::style(),
            Base16::SelenizedWhite => base16_selenized_white::style(),
            Base16::Seti => base16_seti::style(),
            Base16::ShadesOfPurple => base16_shades_of_purple::style(),
            Base16::ShadesmearDark => base16_shadesmear_dark::style(),
            Base16::ShadesmearLight => base16_shadesmear_light::style(),
            Base16::Shapeshifter => base16_shapeshifter::style(),
            Base16::SilkDark => base16_silk_dark::style(),
            Base16::SilkLight => base16_silk_light::style(),
            Base16::Snazzy => base16_snazzy::style(),
            Base16::SolarflareLight => base16_solarflare_light::style(),
            Base16::Solarflare => base16_solarflare::style(),
            Base16::SolarizedDark => base16_solarized_dark::style(),
            Base16::SolarizedLight => base16_solarized_light::style(),
            Base16::Spaceduck => base16_spaceduck::style(),
            Base16::Spacemacs => base16_spacemacs::style(),
            Base16::Sparky => base16_sparky::style(),
            Base16::StandardizedDark => base16_standardized_dark::style(),
            Base16::StandardizedLight => base16_standardized_light::style(),
            Base16::Stella => base16_stella::style(),
            Base16::StillAlive => base16_still_alive::style(),
            Base16::Summercamp => base16_summercamp::style(),
            Base16::SummerfruitDark => base16_summerfruit_dark::style(),
            Base16::SummerfruitLight => base16_summerfruit_light::style(),
            Base16::SynthMidnightDark => base16_synth_midnight_dark::style(),
            Base16::SynthMidnightLight => base16_synth_midnight_light::style(),
            Base16::Tango => base16_tango::style(),
            Base16::Tarot => base16_tarot::style(),
            Base16::Tender => base16_tender::style(),
            Base16::TerracottaDark => base16_terracotta_dark::style(),
            Base16::Terracotta => base16_terracotta::style(),
            Base16::TokyoCityDark => base16_tokyo_city_dark::style(),
            Base16::TokyoCityLight => base16_tokyo_city_light::style(),
            Base16::TokyoCityTerminalDark => base16_tokyo_city_terminal_dark::style(),
            Base16::TokyoCityTerminalLight => base16_tokyo_city_terminal_light::style(),
            Base16::TokyoNightDark => base16_tokyo_night_dark::style(),
            Base16::TokyoNightLight => base16_tokyo_night_light::style(),
            Base16::TokyoNightMoon => base16_tokyo_night_moon::style(),
            Base16::TokyoNightStorm => base16_tokyo_night_storm::style(),
            Base16::TokyoNightTerminalDark => base16_tokyo_night_terminal_dark::style(),
            Base16::TokyoNightTerminalLight => base16_tokyo_night_terminal_light::style(),
            Base16::TokyoNightTerminalStorm => base16_tokyo_night_terminal_storm::style(),
            Base16::TokyodarkTerminal => base16_tokyodark_terminal::style(),
            Base16::Tokyodark => base16_tokyodark::style(),
            Base16::TomorrowNightEighties => base16_tomorrow_night_eighties::style(),
            Base16::TomorrowNight => base16_tomorrow_night::style(),
            Base16::Tomorrow => base16_tomorrow::style(),
            Base16::Tube => base16_tube::style(),
            Base16::Twilight => base16_twilight::style(),
            Base16::UnikittyDark => base16_unikitty_dark::style(),
            Base16::UnikittyLight => base16_unikitty_light::style(),
            Base16::UnikittyReversible => base16_unikitty_reversible::style(),
            Base16::Uwunicorn => base16_uwunicorn::style(),
            Base16::Vesper => base16_vesper::style(),
            Base16::Vice => base16_vice::style(),
            Base16::Vulcan => base16_vulcan::style(),
            Base16::Windows10Light => base16_windows_10_light::style(),
            Base16::Windows10 => base16_windows_10::style(),
            Base16::Windows95Light => base16_windows_95_light::style(),
            Base16::Windows95 => base16_windows_95::style(),
            Base16::WindowsHighcontrastLight => base16_windows_highcontrast_light::style(),
            Base16::WindowsHighcontrast => base16_windows_highcontrast::style(),
            Base16::WindowsNtLight => base16_windows_nt_light::style(),
            Base16::WindowsNt => base16_windows_nt::style(),
            Base16::Woodland => base16_woodland::style(),
            Base16::XcodeDusk => base16_xcode_dusk::style(),
            Base16::Zenbones => base16_zenbones::style(),
            Base16::Zenburn => base16_zenburn::style(),
        }
    }
}

macro_rules! generate_color_fn {
    ($func_name:ident, $base_color:ident) => {
        impl Base16 {
            pub fn $func_name(self) -> Color32 {
                match self {
                    Base16::_3024 => base16_3024::$base_color,
                    Base16::Apathy => base16_apathy::$base_color,
                    Base16::Apprentice => base16_apprentice::$base_color,
                    Base16::Ashes => base16_ashes::$base_color,
                    Base16::AtelierCaveLight => base16_atelier_cave_light::$base_color,
                    Base16::AtelierCave => base16_atelier_cave::$base_color,
                    Base16::AtelierDuneLight => base16_atelier_dune_light::$base_color,
                    Base16::AtelierDune => base16_atelier_dune::$base_color,
                    Base16::AtelierEstuaryLight => base16_atelier_estuary_light::$base_color,
                    Base16::AtelierEstuary => base16_atelier_estuary::$base_color,
                    Base16::AtelierForestLight => base16_atelier_forest_light::$base_color,
                    Base16::AtelierForest => base16_atelier_forest::$base_color,
                    Base16::AtelierHeathLight => base16_atelier_heath_light::$base_color,
                    Base16::AtelierHeath => base16_atelier_heath::$base_color,
                    Base16::AtelierLakesideLight => base16_atelier_lakeside_light::$base_color,
                    Base16::AtelierLakeside => base16_atelier_lakeside::$base_color,
                    Base16::AtelierPlateauLight => base16_atelier_plateau_light::$base_color,
                    Base16::AtelierPlateau => base16_atelier_plateau::$base_color,
                    Base16::AtelierSavannaLight => base16_atelier_savanna_light::$base_color,
                    Base16::AtelierSavanna => base16_atelier_savanna::$base_color,
                    Base16::AtelierSeasideLight => base16_atelier_seaside_light::$base_color,
                    Base16::AtelierSeaside => base16_atelier_seaside::$base_color,
                    Base16::AtelierSulphurpoolLight => {
                        base16_atelier_sulphurpool_light::$base_color
                    }
                    Base16::AtelierSulphurpool => base16_atelier_sulphurpool::$base_color,
                    Base16::Atlas => base16_atlas::$base_color,
                    Base16::AyuDark => base16_ayu_dark::$base_color,
                    Base16::AyuLight => base16_ayu_light::$base_color,
                    Base16::AyuMirage => base16_ayu_mirage::$base_color,
                    Base16::Aztec => base16_aztec::$base_color,
                    Base16::Bespin => base16_bespin::$base_color,
                    Base16::BlackMetalBathory => base16_black_metal_bathory::$base_color,
                    Base16::BlackMetalBurzum => base16_black_metal_burzum::$base_color,
                    Base16::BlackMetalDarkFuneral => base16_black_metal_dark_funeral::$base_color,
                    Base16::BlackMetalGorgoroth => base16_black_metal_gorgoroth::$base_color,
                    Base16::BlackMetalImmortal => base16_black_metal_immortal::$base_color,
                    Base16::BlackMetalKhold => base16_black_metal_khold::$base_color,
                    Base16::BlackMetalMarduk => base16_black_metal_marduk::$base_color,
                    Base16::BlackMetalMayhem => base16_black_metal_mayhem::$base_color,
                    Base16::BlackMetalNile => base16_black_metal_nile::$base_color,
                    Base16::BlackMetalVenom => base16_black_metal_venom::$base_color,
                    Base16::BlackMetal => base16_black_metal::$base_color,
                    Base16::Blueforest => base16_blueforest::$base_color,
                    Base16::Blueish => base16_blueish::$base_color,
                    Base16::Brewer => base16_brewer::$base_color,
                    Base16::Bright => base16_bright::$base_color,
                    Base16::Brogrammer => base16_brogrammer::$base_color,
                    Base16::BrushtreesDark => base16_brushtrees_dark::$base_color,
                    Base16::Brushtrees => base16_brushtrees::$base_color,
                    Base16::Caroline => base16_caroline::$base_color,
                    Base16::CatppuccinFrappe => base16_catppuccin_frappe::$base_color,
                    Base16::CatppuccinLatte => base16_catppuccin_latte::$base_color,
                    Base16::CatppuccinMacchiato => base16_catppuccin_macchiato::$base_color,
                    Base16::CatppuccinMocha => base16_catppuccin_mocha::$base_color,
                    Base16::Chalk => base16_chalk::$base_color,
                    Base16::Circus => base16_circus::$base_color,
                    Base16::ClassicDark => base16_classic_dark::$base_color,
                    Base16::ClassicLight => base16_classic_light::$base_color,
                    Base16::Codeschool => base16_codeschool::$base_color,
                    Base16::Colors => base16_colors::$base_color,
                    Base16::Cupcake => base16_cupcake::$base_color,
                    Base16::Cupertino => base16_cupertino::$base_color,
                    Base16::DaOneBlack => base16_da_one_black::$base_color,
                    Base16::DaOneGray => base16_da_one_gray::$base_color,
                    Base16::DaOneOcean => base16_da_one_ocean::$base_color,
                    Base16::DaOnePaper => base16_da_one_paper::$base_color,
                    Base16::DaOneSea => base16_da_one_sea::$base_color,
                    Base16::DaOneWhite => base16_da_one_white::$base_color,
                    Base16::DanqingLight => base16_danqing_light::$base_color,
                    Base16::Danqing => base16_danqing::$base_color,
                    Base16::Darcula => base16_darcula::$base_color,
                    Base16::Darkmoss => base16_darkmoss::$base_color,
                    Base16::Darktooth => base16_darktooth::$base_color,
                    Base16::Darkviolet => base16_darkviolet::$base_color,
                    Base16::Decaf => base16_decaf::$base_color,
                    Base16::DefaultDark => base16_default_dark::$base_color,
                    Base16::DefaultLight => base16_default_light::$base_color,
                    Base16::Dirtysea => base16_dirtysea::$base_color,
                    Base16::Dracula => base16_dracula::$base_color,
                    Base16::EdgeDark => base16_edge_dark::$base_color,
                    Base16::EdgeLight => base16_edge_light::$base_color,
                    Base16::Eighties => base16_eighties::$base_color,
                    Base16::EmbersLight => base16_embers_light::$base_color,
                    Base16::Embers => base16_embers::$base_color,
                    Base16::Emil => base16_emil::$base_color,
                    Base16::EquilibriumDark => base16_equilibrium_dark::$base_color,
                    Base16::EquilibriumGrayDark => base16_equilibrium_gray_dark::$base_color,
                    Base16::EquilibriumGrayLight => base16_equilibrium_gray_light::$base_color,
                    Base16::EquilibriumLight => base16_equilibrium_light::$base_color,
                    Base16::Eris => base16_eris::$base_color,
                    Base16::Espresso => base16_espresso::$base_color,
                    Base16::EvaDim => base16_eva_dim::$base_color,
                    Base16::Eva => base16_eva::$base_color,
                    Base16::EvenokDark => base16_evenok_dark::$base_color,
                    Base16::EverforestDarkHard => base16_everforest_dark_hard::$base_color,
                    Base16::Everforest => base16_everforest::$base_color,
                    Base16::Flat => base16_flat::$base_color,
                    Base16::Framer => base16_framer::$base_color,
                    Base16::FruitSoda => base16_fruit_soda::$base_color,
                    Base16::Gigavolt => base16_gigavolt::$base_color,
                    Base16::Github => base16_github::$base_color,
                    Base16::GoogleDark => base16_google_dark::$base_color,
                    Base16::GoogleLight => base16_google_light::$base_color,
                    Base16::Gotham => base16_gotham::$base_color,
                    Base16::GrayscaleDark => base16_grayscale_dark::$base_color,
                    Base16::GrayscaleLight => base16_grayscale_light::$base_color,
                    Base16::Greenscreen => base16_greenscreen::$base_color,
                    Base16::Gruber => base16_gruber::$base_color,
                    Base16::GruvboxDarkHard => base16_gruvbox_dark_hard::$base_color,
                    Base16::GruvboxDarkMedium => base16_gruvbox_dark_medium::$base_color,
                    Base16::GruvboxDarkPale => base16_gruvbox_dark_pale::$base_color,
                    Base16::GruvboxDarkSoft => base16_gruvbox_dark_soft::$base_color,
                    Base16::GruvboxLightHard => base16_gruvbox_light_hard::$base_color,
                    Base16::GruvboxLightMedium => base16_gruvbox_light_medium::$base_color,
                    Base16::GruvboxLightSoft => base16_gruvbox_light_soft::$base_color,
                    Base16::GruvboxMaterialDarkHard => {
                        base16_gruvbox_material_dark_hard::$base_color
                    }
                    Base16::GruvboxMaterialDarkMedium => {
                        base16_gruvbox_material_dark_medium::$base_color
                    }
                    Base16::GruvboxMaterialDarkSoft => {
                        base16_gruvbox_material_dark_soft::$base_color
                    }
                    Base16::GruvboxMaterialLightHard => {
                        base16_gruvbox_material_light_hard::$base_color
                    }
                    Base16::GruvboxMaterialLightMedium => {
                        base16_gruvbox_material_light_medium::$base_color
                    }
                    Base16::GruvboxMaterialLightSoft => {
                        base16_gruvbox_material_light_soft::$base_color
                    }
                    Base16::Hardcore => base16_hardcore::$base_color,
                    Base16::Harmonic16Dark => base16_harmonic16_dark::$base_color,
                    Base16::Harmonic16Light => base16_harmonic16_light::$base_color,
                    Base16::HeetchLight => base16_heetch_light::$base_color,
                    Base16::Heetch => base16_heetch::$base_color,
                    Base16::Helios => base16_helios::$base_color,
                    Base16::Hopscotch => base16_hopscotch::$base_color,
                    Base16::HorizonDark => base16_horizon_dark::$base_color,
                    Base16::HorizonLight => base16_horizon_light::$base_color,
                    Base16::HorizonTerminalDark => base16_horizon_terminal_dark::$base_color,
                    Base16::HorizonTerminalLight => base16_horizon_terminal_light::$base_color,
                    Base16::HumanoidDark => base16_humanoid_dark::$base_color,
                    Base16::HumanoidLight => base16_humanoid_light::$base_color,
                    Base16::IaDark => base16_ia_dark::$base_color,
                    Base16::IaLight => base16_ia_light::$base_color,
                    Base16::Icy => base16_icy::$base_color,
                    Base16::Irblack => base16_irblack::$base_color,
                    Base16::Isotope => base16_isotope::$base_color,
                    Base16::Jabuti => base16_jabuti::$base_color,
                    Base16::Kanagawa => base16_kanagawa::$base_color,
                    Base16::Katy => base16_katy::$base_color,
                    Base16::Kimber => base16_kimber::$base_color,
                    Base16::Lime => base16_lime::$base_color,
                    Base16::Macintosh => base16_macintosh::$base_color,
                    Base16::Marrakesh => base16_marrakesh::$base_color,
                    Base16::Materia => base16_materia::$base_color,
                    Base16::MaterialDarker => base16_material_darker::$base_color,
                    Base16::MaterialLighter => base16_material_lighter::$base_color,
                    Base16::MaterialPalenight => base16_material_palenight::$base_color,
                    Base16::MaterialVivid => base16_material_vivid::$base_color,
                    Base16::Material => base16_material::$base_color,
                    Base16::MeasuredDark => base16_measured_dark::$base_color,
                    Base16::MeasuredLight => base16_measured_light::$base_color,
                    Base16::MellowPurple => base16_mellow_purple::$base_color,
                    Base16::MexicoLight => base16_mexico_light::$base_color,
                    Base16::Mocha => base16_mocha::$base_color,
                    Base16::Monokai => base16_monokai::$base_color,
                    Base16::Moonlight => base16_moonlight::$base_color,
                    Base16::Mountain => base16_mountain::$base_color,
                    Base16::Nebula => base16_nebula::$base_color,
                    Base16::NordLight => base16_nord_light::$base_color,
                    Base16::Nord => base16_nord::$base_color,
                    Base16::Nova => base16_nova::$base_color,
                    Base16::Ocean => base16_ocean::$base_color,
                    Base16::Oceanicnext => base16_oceanicnext::$base_color,
                    Base16::OneLight => base16_one_light::$base_color,
                    Base16::OnedarkDark => base16_onedark_dark::$base_color,
                    Base16::Onedark => base16_onedark::$base_color,
                    Base16::OutrunDark => base16_outrun_dark::$base_color,
                    Base16::OxocarbonDark => base16_oxocarbon_dark::$base_color,
                    Base16::OxocarbonLight => base16_oxocarbon_light::$base_color,
                    Base16::Pandora => base16_pandora::$base_color,
                    Base16::PapercolorDark => base16_papercolor_dark::$base_color,
                    Base16::PapercolorLight => base16_papercolor_light::$base_color,
                    Base16::Paraiso => base16_paraiso::$base_color,
                    Base16::Pasque => base16_pasque::$base_color,
                    Base16::Phd => base16_phd::$base_color,
                    Base16::Pico => base16_pico::$base_color,
                    Base16::Pinky => base16_pinky::$base_color,
                    Base16::Pop => base16_pop::$base_color,
                    Base16::Porple => base16_porple::$base_color,
                    Base16::PreciousDarkEleven => base16_precious_dark_eleven::$base_color,
                    Base16::PreciousDarkFifteen => base16_precious_dark_fifteen::$base_color,
                    Base16::PreciousLightWarm => base16_precious_light_warm::$base_color,
                    Base16::PreciousLightWhite => base16_precious_light_white::$base_color,
                    Base16::PrimerDarkDimmed => base16_primer_dark_dimmed::$base_color,
                    Base16::PrimerDark => base16_primer_dark::$base_color,
                    Base16::PrimerLight => base16_primer_light::$base_color,
                    Base16::Purpledream => base16_purpledream::$base_color,
                    Base16::Qualia => base16_qualia::$base_color,
                    Base16::Railscasts => base16_railscasts::$base_color,
                    Base16::Rebecca => base16_rebecca::$base_color,
                    Base16::RosePineDawn => base16_rose_pine_dawn::$base_color,
                    Base16::RosePineMoon => base16_rose_pine_moon::$base_color,
                    Base16::RosePine => base16_rose_pine::$base_color,
                    Base16::Saga => base16_saga::$base_color,
                    Base16::Sagelight => base16_sagelight::$base_color,
                    Base16::Sakura => base16_sakura::$base_color,
                    Base16::Sandcastle => base16_sandcastle::$base_color,
                    Base16::SelenizedBlack => base16_selenized_black::$base_color,
                    Base16::SelenizedDark => base16_selenized_dark::$base_color,
                    Base16::SelenizedLight => base16_selenized_light::$base_color,
                    Base16::SelenizedWhite => base16_selenized_white::$base_color,
                    Base16::Seti => base16_seti::$base_color,
                    Base16::ShadesOfPurple => base16_shades_of_purple::$base_color,
                    Base16::ShadesmearDark => base16_shadesmear_dark::$base_color,
                    Base16::ShadesmearLight => base16_shadesmear_light::$base_color,
                    Base16::Shapeshifter => base16_shapeshifter::$base_color,
                    Base16::SilkDark => base16_silk_dark::$base_color,
                    Base16::SilkLight => base16_silk_light::$base_color,
                    Base16::Snazzy => base16_snazzy::$base_color,
                    Base16::SolarflareLight => base16_solarflare_light::$base_color,
                    Base16::Solarflare => base16_solarflare::$base_color,
                    Base16::SolarizedDark => base16_solarized_dark::$base_color,
                    Base16::SolarizedLight => base16_solarized_light::$base_color,
                    Base16::Spaceduck => base16_spaceduck::$base_color,
                    Base16::Spacemacs => base16_spacemacs::$base_color,
                    Base16::Sparky => base16_sparky::$base_color,
                    Base16::StandardizedDark => base16_standardized_dark::$base_color,
                    Base16::StandardizedLight => base16_standardized_light::$base_color,
                    Base16::Stella => base16_stella::$base_color,
                    Base16::StillAlive => base16_still_alive::$base_color,
                    Base16::Summercamp => base16_summercamp::$base_color,
                    Base16::SummerfruitDark => base16_summerfruit_dark::$base_color,
                    Base16::SummerfruitLight => base16_summerfruit_light::$base_color,
                    Base16::SynthMidnightDark => base16_synth_midnight_dark::$base_color,
                    Base16::SynthMidnightLight => base16_synth_midnight_light::$base_color,
                    Base16::Tango => base16_tango::$base_color,
                    Base16::Tarot => base16_tarot::$base_color,
                    Base16::Tender => base16_tender::$base_color,
                    Base16::TerracottaDark => base16_terracotta_dark::$base_color,
                    Base16::Terracotta => base16_terracotta::$base_color,
                    Base16::TokyoCityDark => base16_tokyo_city_dark::$base_color,
                    Base16::TokyoCityLight => base16_tokyo_city_light::$base_color,
                    Base16::TokyoCityTerminalDark => base16_tokyo_city_terminal_dark::$base_color,
                    Base16::TokyoCityTerminalLight => base16_tokyo_city_terminal_light::$base_color,
                    Base16::TokyoNightDark => base16_tokyo_night_dark::$base_color,
                    Base16::TokyoNightLight => base16_tokyo_night_light::$base_color,
                    Base16::TokyoNightMoon => base16_tokyo_night_moon::$base_color,
                    Base16::TokyoNightStorm => base16_tokyo_night_storm::$base_color,
                    Base16::TokyoNightTerminalDark => base16_tokyo_night_terminal_dark::$base_color,
                    Base16::TokyoNightTerminalLight => {
                        base16_tokyo_night_terminal_light::$base_color
                    }
                    Base16::TokyoNightTerminalStorm => {
                        base16_tokyo_night_terminal_storm::$base_color
                    }
                    Base16::TokyodarkTerminal => base16_tokyodark_terminal::$base_color,
                    Base16::Tokyodark => base16_tokyodark::$base_color,
                    Base16::TomorrowNightEighties => base16_tomorrow_night_eighties::$base_color,
                    Base16::TomorrowNight => base16_tomorrow_night::$base_color,
                    Base16::Tomorrow => base16_tomorrow::$base_color,
                    Base16::Tube => base16_tube::$base_color,
                    Base16::Twilight => base16_twilight::$base_color,
                    Base16::UnikittyDark => base16_unikitty_dark::$base_color,
                    Base16::UnikittyLight => base16_unikitty_light::$base_color,
                    Base16::UnikittyReversible => base16_unikitty_reversible::$base_color,
                    Base16::Uwunicorn => base16_uwunicorn::$base_color,
                    Base16::Vesper => base16_vesper::$base_color,
                    Base16::Vice => base16_vice::$base_color,
                    Base16::Vulcan => base16_vulcan::$base_color,
                    Base16::Windows10Light => base16_windows_10_light::$base_color,
                    Base16::Windows10 => base16_windows_10::$base_color,
                    Base16::Windows95Light => base16_windows_95_light::$base_color,
                    Base16::Windows95 => base16_windows_95::$base_color,
                    Base16::WindowsHighcontrastLight => {
                        base16_windows_highcontrast_light::$base_color
                    }
                    Base16::WindowsHighcontrast => base16_windows_highcontrast::$base_color,
                    Base16::WindowsNtLight => base16_windows_nt_light::$base_color,
                    Base16::WindowsNt => base16_windows_nt::$base_color,
                    Base16::Woodland => base16_woodland::$base_color,
                    Base16::XcodeDusk => base16_xcode_dusk::$base_color,
                    Base16::Zenbones => base16_zenbones::$base_color,
                    Base16::Zenburn => base16_zenburn::$base_color,
                }
            }
        }
    };
}

generate_color_fn!(background, BACKGROUND);
generate_color_fn!(shadow, SHADOW);
generate_color_fn!(transparent, TRANSPARENT);
generate_color_fn!(base00, BASE_00);
generate_color_fn!(base01, BASE_01);
generate_color_fn!(base02, BASE_02);
generate_color_fn!(base03, BASE_03);
generate_color_fn!(base04, BASE_04);
generate_color_fn!(base05, BASE_05);
generate_color_fn!(base06, BASE_06);
generate_color_fn!(base07, BASE_07);
generate_color_fn!(base08, BASE_08);
generate_color_fn!(base09, BASE_09);
generate_color_fn!(base0a, BASE_0A);
generate_color_fn!(base0b, BASE_0B);
generate_color_fn!(base0c, BASE_0C);
generate_color_fn!(base0d, BASE_0D);
generate_color_fn!(base0e, BASE_0E);
generate_color_fn!(base0f, BASE_0F);
