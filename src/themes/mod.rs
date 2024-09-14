use egui::{Color32, Style};

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

#[derive(Copy, Clone, Debug, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
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
            Base16::_3024 => base16_3024::default_style(),
            Base16::Apathy => base16_apathy::default_style(),
            Base16::Apprentice => base16_apprentice::default_style(),
            Base16::Ashes => base16_ashes::default_style(),
            Base16::AtelierCaveLight => base16_atelier_cave_light::default_style(),
            Base16::AtelierCave => base16_atelier_cave::default_style(),
            Base16::AtelierDuneLight => base16_atelier_dune_light::default_style(),
            Base16::AtelierDune => base16_atelier_dune::default_style(),
            Base16::AtelierEstuaryLight => base16_atelier_estuary_light::default_style(),
            Base16::AtelierEstuary => base16_atelier_estuary::default_style(),
            Base16::AtelierForestLight => base16_atelier_forest_light::default_style(),
            Base16::AtelierForest => base16_atelier_forest::default_style(),
            Base16::AtelierHeathLight => base16_atelier_heath_light::default_style(),
            Base16::AtelierHeath => base16_atelier_heath::default_style(),
            Base16::AtelierLakesideLight => base16_atelier_lakeside_light::default_style(),
            Base16::AtelierLakeside => base16_atelier_lakeside::default_style(),
            Base16::AtelierPlateauLight => base16_atelier_plateau_light::default_style(),
            Base16::AtelierPlateau => base16_atelier_plateau::default_style(),
            Base16::AtelierSavannaLight => base16_atelier_savanna_light::default_style(),
            Base16::AtelierSavanna => base16_atelier_savanna::default_style(),
            Base16::AtelierSeasideLight => base16_atelier_seaside_light::default_style(),
            Base16::AtelierSeaside => base16_atelier_seaside::default_style(),
            Base16::AtelierSulphurpoolLight => base16_atelier_sulphurpool_light::default_style(),
            Base16::AtelierSulphurpool => base16_atelier_sulphurpool::default_style(),
            Base16::Atlas => base16_atlas::default_style(),
            Base16::AyuDark => base16_ayu_dark::default_style(),
            Base16::AyuLight => base16_ayu_light::default_style(),
            Base16::AyuMirage => base16_ayu_mirage::default_style(),
            Base16::Aztec => base16_aztec::default_style(),
            Base16::Bespin => base16_bespin::default_style(),
            Base16::BlackMetalBathory => base16_black_metal_bathory::default_style(),
            Base16::BlackMetalBurzum => base16_black_metal_burzum::default_style(),
            Base16::BlackMetalDarkFuneral => base16_black_metal_dark_funeral::default_style(),
            Base16::BlackMetalGorgoroth => base16_black_metal_gorgoroth::default_style(),
            Base16::BlackMetalImmortal => base16_black_metal_immortal::default_style(),
            Base16::BlackMetalKhold => base16_black_metal_khold::default_style(),
            Base16::BlackMetalMarduk => base16_black_metal_marduk::default_style(),
            Base16::BlackMetalMayhem => base16_black_metal_mayhem::default_style(),
            Base16::BlackMetalNile => base16_black_metal_nile::default_style(),
            Base16::BlackMetalVenom => base16_black_metal_venom::default_style(),
            Base16::BlackMetal => base16_black_metal::default_style(),
            Base16::Blueforest => base16_blueforest::default_style(),
            Base16::Blueish => base16_blueish::default_style(),
            Base16::Brewer => base16_brewer::default_style(),
            Base16::Bright => base16_bright::default_style(),
            Base16::Brogrammer => base16_brogrammer::default_style(),
            Base16::BrushtreesDark => base16_brushtrees_dark::default_style(),
            Base16::Brushtrees => base16_brushtrees::default_style(),
            Base16::Caroline => base16_caroline::default_style(),
            Base16::CatppuccinFrappe => base16_catppuccin_frappe::default_style(),
            Base16::CatppuccinLatte => base16_catppuccin_latte::default_style(),
            Base16::CatppuccinMacchiato => base16_catppuccin_macchiato::default_style(),
            Base16::CatppuccinMocha => base16_catppuccin_mocha::default_style(),
            Base16::Chalk => base16_chalk::default_style(),
            Base16::Circus => base16_circus::default_style(),
            Base16::ClassicDark => base16_classic_dark::default_style(),
            Base16::ClassicLight => base16_classic_light::default_style(),
            Base16::Codeschool => base16_codeschool::default_style(),
            Base16::Colors => base16_colors::default_style(),
            Base16::Cupcake => base16_cupcake::default_style(),
            Base16::Cupertino => base16_cupertino::default_style(),
            Base16::DaOneBlack => base16_da_one_black::default_style(),
            Base16::DaOneGray => base16_da_one_gray::default_style(),
            Base16::DaOneOcean => base16_da_one_ocean::default_style(),
            Base16::DaOnePaper => base16_da_one_paper::default_style(),
            Base16::DaOneSea => base16_da_one_sea::default_style(),
            Base16::DaOneWhite => base16_da_one_white::default_style(),
            Base16::DanqingLight => base16_danqing_light::default_style(),
            Base16::Danqing => base16_danqing::default_style(),
            Base16::Darcula => base16_darcula::default_style(),
            Base16::Darkmoss => base16_darkmoss::default_style(),
            Base16::Darktooth => base16_darktooth::default_style(),
            Base16::Darkviolet => base16_darkviolet::default_style(),
            Base16::Decaf => base16_decaf::default_style(),
            Base16::DefaultDark => base16_default_dark::default_style(),
            Base16::DefaultLight => base16_default_light::default_style(),
            Base16::Dirtysea => base16_dirtysea::default_style(),
            Base16::Dracula => base16_dracula::default_style(),
            Base16::EdgeDark => base16_edge_dark::default_style(),
            Base16::EdgeLight => base16_edge_light::default_style(),
            Base16::Eighties => base16_eighties::default_style(),
            Base16::EmbersLight => base16_embers_light::default_style(),
            Base16::Embers => base16_embers::default_style(),
            Base16::Emil => base16_emil::default_style(),
            Base16::EquilibriumDark => base16_equilibrium_dark::default_style(),
            Base16::EquilibriumGrayDark => base16_equilibrium_gray_dark::default_style(),
            Base16::EquilibriumGrayLight => base16_equilibrium_gray_light::default_style(),
            Base16::EquilibriumLight => base16_equilibrium_light::default_style(),
            Base16::Eris => base16_eris::default_style(),
            Base16::Espresso => base16_espresso::default_style(),
            Base16::EvaDim => base16_eva_dim::default_style(),
            Base16::Eva => base16_eva::default_style(),
            Base16::EvenokDark => base16_evenok_dark::default_style(),
            Base16::EverforestDarkHard => base16_everforest_dark_hard::default_style(),
            Base16::Everforest => base16_everforest::default_style(),
            Base16::Flat => base16_flat::default_style(),
            Base16::Framer => base16_framer::default_style(),
            Base16::FruitSoda => base16_fruit_soda::default_style(),
            Base16::Gigavolt => base16_gigavolt::default_style(),
            Base16::Github => base16_github::default_style(),
            Base16::GoogleDark => base16_google_dark::default_style(),
            Base16::GoogleLight => base16_google_light::default_style(),
            Base16::Gotham => base16_gotham::default_style(),
            Base16::GrayscaleDark => base16_grayscale_dark::default_style(),
            Base16::GrayscaleLight => base16_grayscale_light::default_style(),
            Base16::Greenscreen => base16_greenscreen::default_style(),
            Base16::Gruber => base16_gruber::default_style(),
            Base16::GruvboxDarkHard => base16_gruvbox_dark_hard::default_style(),
            Base16::GruvboxDarkMedium => base16_gruvbox_dark_medium::default_style(),
            Base16::GruvboxDarkPale => base16_gruvbox_dark_pale::default_style(),
            Base16::GruvboxDarkSoft => base16_gruvbox_dark_soft::default_style(),
            Base16::GruvboxLightHard => base16_gruvbox_light_hard::default_style(),
            Base16::GruvboxLightMedium => base16_gruvbox_light_medium::default_style(),
            Base16::GruvboxLightSoft => base16_gruvbox_light_soft::default_style(),
            Base16::GruvboxMaterialDarkHard => base16_gruvbox_material_dark_hard::default_style(),
            Base16::GruvboxMaterialDarkMedium => {
                base16_gruvbox_material_dark_medium::default_style()
            }
            Base16::GruvboxMaterialDarkSoft => base16_gruvbox_material_dark_soft::default_style(),
            Base16::GruvboxMaterialLightHard => base16_gruvbox_material_light_hard::default_style(),
            Base16::GruvboxMaterialLightMedium => {
                base16_gruvbox_material_light_medium::default_style()
            }
            Base16::GruvboxMaterialLightSoft => base16_gruvbox_material_light_soft::default_style(),
            Base16::Hardcore => base16_hardcore::default_style(),
            Base16::Harmonic16Dark => base16_harmonic16_dark::default_style(),
            Base16::Harmonic16Light => base16_harmonic16_light::default_style(),
            Base16::HeetchLight => base16_heetch_light::default_style(),
            Base16::Heetch => base16_heetch::default_style(),
            Base16::Helios => base16_helios::default_style(),
            Base16::Hopscotch => base16_hopscotch::default_style(),
            Base16::HorizonDark => base16_horizon_dark::default_style(),
            Base16::HorizonLight => base16_horizon_light::default_style(),
            Base16::HorizonTerminalDark => base16_horizon_terminal_dark::default_style(),
            Base16::HorizonTerminalLight => base16_horizon_terminal_light::default_style(),
            Base16::HumanoidDark => base16_humanoid_dark::default_style(),
            Base16::HumanoidLight => base16_humanoid_light::default_style(),
            Base16::IaDark => base16_ia_dark::default_style(),
            Base16::IaLight => base16_ia_light::default_style(),
            Base16::Icy => base16_icy::default_style(),
            Base16::Irblack => base16_irblack::default_style(),
            Base16::Isotope => base16_isotope::default_style(),
            Base16::Jabuti => base16_jabuti::default_style(),
            Base16::Kanagawa => base16_kanagawa::default_style(),
            Base16::Katy => base16_katy::default_style(),
            Base16::Kimber => base16_kimber::default_style(),
            Base16::Lime => base16_lime::default_style(),
            Base16::Macintosh => base16_macintosh::default_style(),
            Base16::Marrakesh => base16_marrakesh::default_style(),
            Base16::Materia => base16_materia::default_style(),
            Base16::MaterialDarker => base16_material_darker::default_style(),
            Base16::MaterialLighter => base16_material_lighter::default_style(),
            Base16::MaterialPalenight => base16_material_palenight::default_style(),
            Base16::MaterialVivid => base16_material_vivid::default_style(),
            Base16::Material => base16_material::default_style(),
            Base16::MeasuredDark => base16_measured_dark::default_style(),
            Base16::MeasuredLight => base16_measured_light::default_style(),
            Base16::MellowPurple => base16_mellow_purple::default_style(),
            Base16::MexicoLight => base16_mexico_light::default_style(),
            Base16::Mocha => base16_mocha::default_style(),
            Base16::Monokai => base16_monokai::default_style(),
            Base16::Moonlight => base16_moonlight::default_style(),
            Base16::Mountain => base16_mountain::default_style(),
            Base16::Nebula => base16_nebula::default_style(),
            Base16::NordLight => base16_nord_light::default_style(),
            Base16::Nord => base16_nord::default_style(),
            Base16::Nova => base16_nova::default_style(),
            Base16::Ocean => base16_ocean::default_style(),
            Base16::Oceanicnext => base16_oceanicnext::default_style(),
            Base16::OneLight => base16_one_light::default_style(),
            Base16::OnedarkDark => base16_onedark_dark::default_style(),
            Base16::Onedark => base16_onedark::default_style(),
            Base16::OutrunDark => base16_outrun_dark::default_style(),
            Base16::OxocarbonDark => base16_oxocarbon_dark::default_style(),
            Base16::OxocarbonLight => base16_oxocarbon_light::default_style(),
            Base16::Pandora => base16_pandora::default_style(),
            Base16::PapercolorDark => base16_papercolor_dark::default_style(),
            Base16::PapercolorLight => base16_papercolor_light::default_style(),
            Base16::Paraiso => base16_paraiso::default_style(),
            Base16::Pasque => base16_pasque::default_style(),
            Base16::Phd => base16_phd::default_style(),
            Base16::Pico => base16_pico::default_style(),
            Base16::Pinky => base16_pinky::default_style(),
            Base16::Pop => base16_pop::default_style(),
            Base16::Porple => base16_porple::default_style(),
            Base16::PreciousDarkEleven => base16_precious_dark_eleven::default_style(),
            Base16::PreciousDarkFifteen => base16_precious_dark_fifteen::default_style(),
            Base16::PreciousLightWarm => base16_precious_light_warm::default_style(),
            Base16::PreciousLightWhite => base16_precious_light_white::default_style(),
            Base16::PrimerDarkDimmed => base16_primer_dark_dimmed::default_style(),
            Base16::PrimerDark => base16_primer_dark::default_style(),
            Base16::PrimerLight => base16_primer_light::default_style(),
            Base16::Purpledream => base16_purpledream::default_style(),
            Base16::Qualia => base16_qualia::default_style(),
            Base16::Railscasts => base16_railscasts::default_style(),
            Base16::Rebecca => base16_rebecca::default_style(),
            Base16::RosePineDawn => base16_rose_pine_dawn::default_style(),
            Base16::RosePineMoon => base16_rose_pine_moon::default_style(),
            Base16::RosePine => base16_rose_pine::default_style(),
            Base16::Saga => base16_saga::default_style(),
            Base16::Sagelight => base16_sagelight::default_style(),
            Base16::Sakura => base16_sakura::default_style(),
            Base16::Sandcastle => base16_sandcastle::default_style(),
            Base16::SelenizedBlack => base16_selenized_black::default_style(),
            Base16::SelenizedDark => base16_selenized_dark::default_style(),
            Base16::SelenizedLight => base16_selenized_light::default_style(),
            Base16::SelenizedWhite => base16_selenized_white::default_style(),
            Base16::Seti => base16_seti::default_style(),
            Base16::ShadesOfPurple => base16_shades_of_purple::default_style(),
            Base16::ShadesmearDark => base16_shadesmear_dark::default_style(),
            Base16::ShadesmearLight => base16_shadesmear_light::default_style(),
            Base16::Shapeshifter => base16_shapeshifter::default_style(),
            Base16::SilkDark => base16_silk_dark::default_style(),
            Base16::SilkLight => base16_silk_light::default_style(),
            Base16::Snazzy => base16_snazzy::default_style(),
            Base16::SolarflareLight => base16_solarflare_light::default_style(),
            Base16::Solarflare => base16_solarflare::default_style(),
            Base16::SolarizedDark => base16_solarized_dark::default_style(),
            Base16::SolarizedLight => base16_solarized_light::default_style(),
            Base16::Spaceduck => base16_spaceduck::default_style(),
            Base16::Spacemacs => base16_spacemacs::default_style(),
            Base16::Sparky => base16_sparky::default_style(),
            Base16::StandardizedDark => base16_standardized_dark::default_style(),
            Base16::StandardizedLight => base16_standardized_light::default_style(),
            Base16::Stella => base16_stella::default_style(),
            Base16::StillAlive => base16_still_alive::default_style(),
            Base16::Summercamp => base16_summercamp::default_style(),
            Base16::SummerfruitDark => base16_summerfruit_dark::default_style(),
            Base16::SummerfruitLight => base16_summerfruit_light::default_style(),
            Base16::SynthMidnightDark => base16_synth_midnight_dark::default_style(),
            Base16::SynthMidnightLight => base16_synth_midnight_light::default_style(),
            Base16::Tango => base16_tango::default_style(),
            Base16::Tarot => base16_tarot::default_style(),
            Base16::Tender => base16_tender::default_style(),
            Base16::TerracottaDark => base16_terracotta_dark::default_style(),
            Base16::Terracotta => base16_terracotta::default_style(),
            Base16::TokyoCityDark => base16_tokyo_city_dark::default_style(),
            Base16::TokyoCityLight => base16_tokyo_city_light::default_style(),
            Base16::TokyoCityTerminalDark => base16_tokyo_city_terminal_dark::default_style(),
            Base16::TokyoCityTerminalLight => base16_tokyo_city_terminal_light::default_style(),
            Base16::TokyoNightDark => base16_tokyo_night_dark::default_style(),
            Base16::TokyoNightLight => base16_tokyo_night_light::default_style(),
            Base16::TokyoNightMoon => base16_tokyo_night_moon::default_style(),
            Base16::TokyoNightStorm => base16_tokyo_night_storm::default_style(),
            Base16::TokyoNightTerminalDark => base16_tokyo_night_terminal_dark::default_style(),
            Base16::TokyoNightTerminalLight => base16_tokyo_night_terminal_light::default_style(),
            Base16::TokyoNightTerminalStorm => base16_tokyo_night_terminal_storm::default_style(),
            Base16::TokyodarkTerminal => base16_tokyodark_terminal::default_style(),
            Base16::Tokyodark => base16_tokyodark::default_style(),
            Base16::TomorrowNightEighties => base16_tomorrow_night_eighties::default_style(),
            Base16::TomorrowNight => base16_tomorrow_night::default_style(),
            Base16::Tomorrow => base16_tomorrow::default_style(),
            Base16::Tube => base16_tube::default_style(),
            Base16::Twilight => base16_twilight::default_style(),
            Base16::UnikittyDark => base16_unikitty_dark::default_style(),
            Base16::UnikittyLight => base16_unikitty_light::default_style(),
            Base16::UnikittyReversible => base16_unikitty_reversible::default_style(),
            Base16::Uwunicorn => base16_uwunicorn::default_style(),
            Base16::Vesper => base16_vesper::default_style(),
            Base16::Vice => base16_vice::default_style(),
            Base16::Vulcan => base16_vulcan::default_style(),
            Base16::Windows10Light => base16_windows_10_light::default_style(),
            Base16::Windows10 => base16_windows_10::default_style(),
            Base16::Windows95Light => base16_windows_95_light::default_style(),
            Base16::Windows95 => base16_windows_95::default_style(),
            Base16::WindowsHighcontrastLight => base16_windows_highcontrast_light::default_style(),
            Base16::WindowsHighcontrast => base16_windows_highcontrast::default_style(),
            Base16::WindowsNtLight => base16_windows_nt_light::default_style(),
            Base16::WindowsNt => base16_windows_nt::default_style(),
            Base16::Woodland => base16_woodland::default_style(),
            Base16::XcodeDusk => base16_xcode_dusk::default_style(),
            Base16::Zenbones => base16_zenbones::default_style(),
            Base16::Zenburn => base16_zenburn::default_style(),
        }
    }

    pub fn background(self) -> Color32 {
        match self {
            Base16::_3024 => base16_3024::BASE_01,
            Base16::Apathy => base16_apathy::BASE_01,
            Base16::Apprentice => base16_apprentice::BASE_01,
            Base16::Ashes => base16_ashes::BASE_01,
            Base16::AtelierCaveLight => base16_atelier_cave_light::BASE_01,
            Base16::AtelierCave => base16_atelier_cave::BASE_01,
            Base16::AtelierDuneLight => base16_atelier_dune_light::BASE_01,
            Base16::AtelierDune => base16_atelier_dune::BASE_01,
            Base16::AtelierEstuaryLight => base16_atelier_estuary_light::BASE_01,
            Base16::AtelierEstuary => base16_atelier_estuary::BASE_01,
            Base16::AtelierForestLight => base16_atelier_forest_light::BASE_01,
            Base16::AtelierForest => base16_atelier_forest::BASE_01,
            Base16::AtelierHeathLight => base16_atelier_heath_light::BASE_01,
            Base16::AtelierHeath => base16_atelier_heath::BASE_01,
            Base16::AtelierLakesideLight => base16_atelier_lakeside_light::BASE_01,
            Base16::AtelierLakeside => base16_atelier_lakeside::BASE_01,
            Base16::AtelierPlateauLight => base16_atelier_plateau_light::BASE_01,
            Base16::AtelierPlateau => base16_atelier_plateau::BASE_01,
            Base16::AtelierSavannaLight => base16_atelier_savanna_light::BASE_01,
            Base16::AtelierSavanna => base16_atelier_savanna::BASE_01,
            Base16::AtelierSeasideLight => base16_atelier_seaside_light::BASE_01,
            Base16::AtelierSeaside => base16_atelier_seaside::BASE_01,
            Base16::AtelierSulphurpoolLight => base16_atelier_sulphurpool_light::BASE_01,
            Base16::AtelierSulphurpool => base16_atelier_sulphurpool::BASE_01,
            Base16::Atlas => base16_atlas::BASE_01,
            Base16::AyuDark => base16_ayu_dark::BASE_01,
            Base16::AyuLight => base16_ayu_light::BASE_01,
            Base16::AyuMirage => base16_ayu_mirage::BASE_01,
            Base16::Aztec => base16_aztec::BASE_01,
            Base16::Bespin => base16_bespin::BASE_01,
            Base16::BlackMetalBathory => base16_black_metal_bathory::BASE_01,
            Base16::BlackMetalBurzum => base16_black_metal_burzum::BASE_01,
            Base16::BlackMetalDarkFuneral => base16_black_metal_dark_funeral::BASE_01,
            Base16::BlackMetalGorgoroth => base16_black_metal_gorgoroth::BASE_01,
            Base16::BlackMetalImmortal => base16_black_metal_immortal::BASE_01,
            Base16::BlackMetalKhold => base16_black_metal_khold::BASE_01,
            Base16::BlackMetalMarduk => base16_black_metal_marduk::BASE_01,
            Base16::BlackMetalMayhem => base16_black_metal_mayhem::BASE_01,
            Base16::BlackMetalNile => base16_black_metal_nile::BASE_01,
            Base16::BlackMetalVenom => base16_black_metal_venom::BASE_01,
            Base16::BlackMetal => base16_black_metal::BASE_01,
            Base16::Blueforest => base16_blueforest::BASE_01,
            Base16::Blueish => base16_blueish::BASE_01,
            Base16::Brewer => base16_brewer::BASE_01,
            Base16::Bright => base16_bright::BASE_01,
            Base16::Brogrammer => base16_brogrammer::BASE_01,
            Base16::BrushtreesDark => base16_brushtrees_dark::BASE_01,
            Base16::Brushtrees => base16_brushtrees::BASE_01,
            Base16::Caroline => base16_caroline::BASE_01,
            Base16::CatppuccinFrappe => base16_catppuccin_frappe::BASE_01,
            Base16::CatppuccinLatte => base16_catppuccin_latte::BASE_01,
            Base16::CatppuccinMacchiato => base16_catppuccin_macchiato::BASE_01,
            Base16::CatppuccinMocha => base16_catppuccin_mocha::BASE_01,
            Base16::Chalk => base16_chalk::BASE_01,
            Base16::Circus => base16_circus::BASE_01,
            Base16::ClassicDark => base16_classic_dark::BASE_01,
            Base16::ClassicLight => base16_classic_light::BASE_01,
            Base16::Codeschool => base16_codeschool::BASE_01,
            Base16::Colors => base16_colors::BASE_01,
            Base16::Cupcake => base16_cupcake::BASE_01,
            Base16::Cupertino => base16_cupertino::BASE_01,
            Base16::DaOneBlack => base16_da_one_black::BASE_01,
            Base16::DaOneGray => base16_da_one_gray::BASE_01,
            Base16::DaOneOcean => base16_da_one_ocean::BASE_01,
            Base16::DaOnePaper => base16_da_one_paper::BASE_01,
            Base16::DaOneSea => base16_da_one_sea::BASE_01,
            Base16::DaOneWhite => base16_da_one_white::BASE_01,
            Base16::DanqingLight => base16_danqing_light::BASE_01,
            Base16::Danqing => base16_danqing::BASE_01,
            Base16::Darcula => base16_darcula::BASE_01,
            Base16::Darkmoss => base16_darkmoss::BASE_01,
            Base16::Darktooth => base16_darktooth::BASE_01,
            Base16::Darkviolet => base16_darkviolet::BASE_01,
            Base16::Decaf => base16_decaf::BASE_01,
            Base16::DefaultDark => base16_default_dark::BASE_01,
            Base16::DefaultLight => base16_default_light::BASE_01,
            Base16::Dirtysea => base16_dirtysea::BASE_01,
            Base16::Dracula => base16_dracula::BASE_01,
            Base16::EdgeDark => base16_edge_dark::BASE_01,
            Base16::EdgeLight => base16_edge_light::BASE_01,
            Base16::Eighties => base16_eighties::BASE_01,
            Base16::EmbersLight => base16_embers_light::BASE_01,
            Base16::Embers => base16_embers::BASE_01,
            Base16::Emil => base16_emil::BASE_01,
            Base16::EquilibriumDark => base16_equilibrium_dark::BASE_01,
            Base16::EquilibriumGrayDark => base16_equilibrium_gray_dark::BASE_01,
            Base16::EquilibriumGrayLight => base16_equilibrium_gray_light::BASE_01,
            Base16::EquilibriumLight => base16_equilibrium_light::BASE_01,
            Base16::Eris => base16_eris::BASE_01,
            Base16::Espresso => base16_espresso::BASE_01,
            Base16::EvaDim => base16_eva_dim::BASE_01,
            Base16::Eva => base16_eva::BASE_01,
            Base16::EvenokDark => base16_evenok_dark::BASE_01,
            Base16::EverforestDarkHard => base16_everforest_dark_hard::BASE_01,
            Base16::Everforest => base16_everforest::BASE_01,
            Base16::Flat => base16_flat::BASE_01,
            Base16::Framer => base16_framer::BASE_01,
            Base16::FruitSoda => base16_fruit_soda::BASE_01,
            Base16::Gigavolt => base16_gigavolt::BASE_01,
            Base16::Github => base16_github::BASE_01,
            Base16::GoogleDark => base16_google_dark::BASE_01,
            Base16::GoogleLight => base16_google_light::BASE_01,
            Base16::Gotham => base16_gotham::BASE_01,
            Base16::GrayscaleDark => base16_grayscale_dark::BASE_01,
            Base16::GrayscaleLight => base16_grayscale_light::BASE_01,
            Base16::Greenscreen => base16_greenscreen::BASE_01,
            Base16::Gruber => base16_gruber::BASE_01,
            Base16::GruvboxDarkHard => base16_gruvbox_dark_hard::BASE_01,
            Base16::GruvboxDarkMedium => base16_gruvbox_dark_medium::BASE_01,
            Base16::GruvboxDarkPale => base16_gruvbox_dark_pale::BASE_01,
            Base16::GruvboxDarkSoft => base16_gruvbox_dark_soft::BASE_01,
            Base16::GruvboxLightHard => base16_gruvbox_light_hard::BASE_01,
            Base16::GruvboxLightMedium => base16_gruvbox_light_medium::BASE_01,
            Base16::GruvboxLightSoft => base16_gruvbox_light_soft::BASE_01,
            Base16::GruvboxMaterialDarkHard => base16_gruvbox_material_dark_hard::BASE_01,
            Base16::GruvboxMaterialDarkMedium => base16_gruvbox_material_dark_medium::BASE_01,
            Base16::GruvboxMaterialDarkSoft => base16_gruvbox_material_dark_soft::BASE_01,
            Base16::GruvboxMaterialLightHard => base16_gruvbox_material_light_hard::BASE_01,
            Base16::GruvboxMaterialLightMedium => base16_gruvbox_material_light_medium::BASE_01,
            Base16::GruvboxMaterialLightSoft => base16_gruvbox_material_light_soft::BASE_01,
            Base16::Hardcore => base16_hardcore::BASE_01,
            Base16::Harmonic16Dark => base16_harmonic16_dark::BASE_01,
            Base16::Harmonic16Light => base16_harmonic16_light::BASE_01,
            Base16::HeetchLight => base16_heetch_light::BASE_01,
            Base16::Heetch => base16_heetch::BASE_01,
            Base16::Helios => base16_helios::BASE_01,
            Base16::Hopscotch => base16_hopscotch::BASE_01,
            Base16::HorizonDark => base16_horizon_dark::BASE_01,
            Base16::HorizonLight => base16_horizon_light::BASE_01,
            Base16::HorizonTerminalDark => base16_horizon_terminal_dark::BASE_01,
            Base16::HorizonTerminalLight => base16_horizon_terminal_light::BASE_01,
            Base16::HumanoidDark => base16_humanoid_dark::BASE_01,
            Base16::HumanoidLight => base16_humanoid_light::BASE_01,
            Base16::IaDark => base16_ia_dark::BASE_01,
            Base16::IaLight => base16_ia_light::BASE_01,
            Base16::Icy => base16_icy::BASE_01,
            Base16::Irblack => base16_irblack::BASE_01,
            Base16::Isotope => base16_isotope::BASE_01,
            Base16::Jabuti => base16_jabuti::BASE_01,
            Base16::Kanagawa => base16_kanagawa::BASE_01,
            Base16::Katy => base16_katy::BASE_01,
            Base16::Kimber => base16_kimber::BASE_01,
            Base16::Lime => base16_lime::BASE_01,
            Base16::Macintosh => base16_macintosh::BASE_01,
            Base16::Marrakesh => base16_marrakesh::BASE_01,
            Base16::Materia => base16_materia::BASE_01,
            Base16::MaterialDarker => base16_material_darker::BASE_01,
            Base16::MaterialLighter => base16_material_lighter::BASE_01,
            Base16::MaterialPalenight => base16_material_palenight::BASE_01,
            Base16::MaterialVivid => base16_material_vivid::BASE_01,
            Base16::Material => base16_material::BASE_01,
            Base16::MeasuredDark => base16_measured_dark::BASE_01,
            Base16::MeasuredLight => base16_measured_light::BASE_01,
            Base16::MellowPurple => base16_mellow_purple::BASE_01,
            Base16::MexicoLight => base16_mexico_light::BASE_01,
            Base16::Mocha => base16_mocha::BASE_01,
            Base16::Monokai => base16_monokai::BASE_01,
            Base16::Moonlight => base16_moonlight::BASE_01,
            Base16::Mountain => base16_mountain::BASE_01,
            Base16::Nebula => base16_nebula::BASE_01,
            Base16::NordLight => base16_nord_light::BASE_01,
            Base16::Nord => base16_nord::BASE_01,
            Base16::Nova => base16_nova::BASE_01,
            Base16::Ocean => base16_ocean::BASE_01,
            Base16::Oceanicnext => base16_oceanicnext::BASE_01,
            Base16::OneLight => base16_one_light::BASE_01,
            Base16::OnedarkDark => base16_onedark_dark::BASE_01,
            Base16::Onedark => base16_onedark::BASE_01,
            Base16::OutrunDark => base16_outrun_dark::BASE_01,
            Base16::OxocarbonDark => base16_oxocarbon_dark::BASE_01,
            Base16::OxocarbonLight => base16_oxocarbon_light::BASE_01,
            Base16::Pandora => base16_pandora::BASE_01,
            Base16::PapercolorDark => base16_papercolor_dark::BASE_01,
            Base16::PapercolorLight => base16_papercolor_light::BASE_01,
            Base16::Paraiso => base16_paraiso::BASE_01,
            Base16::Pasque => base16_pasque::BASE_01,
            Base16::Phd => base16_phd::BASE_01,
            Base16::Pico => base16_pico::BASE_01,
            Base16::Pinky => base16_pinky::BASE_01,
            Base16::Pop => base16_pop::BASE_01,
            Base16::Porple => base16_porple::BASE_01,
            Base16::PreciousDarkEleven => base16_precious_dark_eleven::BASE_01,
            Base16::PreciousDarkFifteen => base16_precious_dark_fifteen::BASE_01,
            Base16::PreciousLightWarm => base16_precious_light_warm::BASE_01,
            Base16::PreciousLightWhite => base16_precious_light_white::BASE_01,
            Base16::PrimerDarkDimmed => base16_primer_dark_dimmed::BASE_01,
            Base16::PrimerDark => base16_primer_dark::BASE_01,
            Base16::PrimerLight => base16_primer_light::BASE_01,
            Base16::Purpledream => base16_purpledream::BASE_01,
            Base16::Qualia => base16_qualia::BASE_01,
            Base16::Railscasts => base16_railscasts::BASE_01,
            Base16::Rebecca => base16_rebecca::BASE_01,
            Base16::RosePineDawn => base16_rose_pine_dawn::BASE_01,
            Base16::RosePineMoon => base16_rose_pine_moon::BASE_01,
            Base16::RosePine => base16_rose_pine::BASE_01,
            Base16::Saga => base16_saga::BASE_01,
            Base16::Sagelight => base16_sagelight::BASE_01,
            Base16::Sakura => base16_sakura::BASE_01,
            Base16::Sandcastle => base16_sandcastle::BASE_01,
            Base16::SelenizedBlack => base16_selenized_black::BASE_01,
            Base16::SelenizedDark => base16_selenized_dark::BASE_01,
            Base16::SelenizedLight => base16_selenized_light::BASE_01,
            Base16::SelenizedWhite => base16_selenized_white::BASE_01,
            Base16::Seti => base16_seti::BASE_01,
            Base16::ShadesOfPurple => base16_shades_of_purple::BASE_01,
            Base16::ShadesmearDark => base16_shadesmear_dark::BASE_01,
            Base16::ShadesmearLight => base16_shadesmear_light::BASE_01,
            Base16::Shapeshifter => base16_shapeshifter::BASE_01,
            Base16::SilkDark => base16_silk_dark::BASE_01,
            Base16::SilkLight => base16_silk_light::BASE_01,
            Base16::Snazzy => base16_snazzy::BASE_01,
            Base16::SolarflareLight => base16_solarflare_light::BASE_01,
            Base16::Solarflare => base16_solarflare::BASE_01,
            Base16::SolarizedDark => base16_solarized_dark::BASE_01,
            Base16::SolarizedLight => base16_solarized_light::BASE_01,
            Base16::Spaceduck => base16_spaceduck::BASE_01,
            Base16::Spacemacs => base16_spacemacs::BASE_01,
            Base16::Sparky => base16_sparky::BASE_01,
            Base16::StandardizedDark => base16_standardized_dark::BASE_01,
            Base16::StandardizedLight => base16_standardized_light::BASE_01,
            Base16::Stella => base16_stella::BASE_01,
            Base16::StillAlive => base16_still_alive::BASE_01,
            Base16::Summercamp => base16_summercamp::BASE_01,
            Base16::SummerfruitDark => base16_summerfruit_dark::BASE_01,
            Base16::SummerfruitLight => base16_summerfruit_light::BASE_01,
            Base16::SynthMidnightDark => base16_synth_midnight_dark::BASE_01,
            Base16::SynthMidnightLight => base16_synth_midnight_light::BASE_01,
            Base16::Tango => base16_tango::BASE_01,
            Base16::Tarot => base16_tarot::BASE_01,
            Base16::Tender => base16_tender::BASE_01,
            Base16::TerracottaDark => base16_terracotta_dark::BASE_01,
            Base16::Terracotta => base16_terracotta::BASE_01,
            Base16::TokyoCityDark => base16_tokyo_city_dark::BASE_01,
            Base16::TokyoCityLight => base16_tokyo_city_light::BASE_01,
            Base16::TokyoCityTerminalDark => base16_tokyo_city_terminal_dark::BASE_01,
            Base16::TokyoCityTerminalLight => base16_tokyo_city_terminal_light::BASE_01,
            Base16::TokyoNightDark => base16_tokyo_night_dark::BASE_01,
            Base16::TokyoNightLight => base16_tokyo_night_light::BASE_01,
            Base16::TokyoNightMoon => base16_tokyo_night_moon::BASE_01,
            Base16::TokyoNightStorm => base16_tokyo_night_storm::BASE_01,
            Base16::TokyoNightTerminalDark => base16_tokyo_night_terminal_dark::BASE_01,
            Base16::TokyoNightTerminalLight => base16_tokyo_night_terminal_light::BASE_01,
            Base16::TokyoNightTerminalStorm => base16_tokyo_night_terminal_storm::BASE_01,
            Base16::TokyodarkTerminal => base16_tokyodark_terminal::BASE_01,
            Base16::Tokyodark => base16_tokyodark::BASE_01,
            Base16::TomorrowNightEighties => base16_tomorrow_night_eighties::BASE_01,
            Base16::TomorrowNight => base16_tomorrow_night::BASE_01,
            Base16::Tomorrow => base16_tomorrow::BASE_01,
            Base16::Tube => base16_tube::BASE_01,
            Base16::Twilight => base16_twilight::BASE_01,
            Base16::UnikittyDark => base16_unikitty_dark::BASE_01,
            Base16::UnikittyLight => base16_unikitty_light::BASE_01,
            Base16::UnikittyReversible => base16_unikitty_reversible::BASE_01,
            Base16::Uwunicorn => base16_uwunicorn::BASE_01,
            Base16::Vesper => base16_vesper::BASE_01,
            Base16::Vice => base16_vice::BASE_01,
            Base16::Vulcan => base16_vulcan::BASE_01,
            Base16::Windows10Light => base16_windows_10_light::BASE_01,
            Base16::Windows10 => base16_windows_10::BASE_01,
            Base16::Windows95Light => base16_windows_95_light::BASE_01,
            Base16::Windows95 => base16_windows_95::BASE_01,
            Base16::WindowsHighcontrastLight => base16_windows_highcontrast_light::BASE_01,
            Base16::WindowsHighcontrast => base16_windows_highcontrast::BASE_01,
            Base16::WindowsNtLight => base16_windows_nt_light::BASE_01,
            Base16::WindowsNt => base16_windows_nt::BASE_01,
            Base16::Woodland => base16_woodland::BASE_01,
            Base16::XcodeDusk => base16_xcode_dusk::BASE_01,
            Base16::Zenbones => base16_zenbones::BASE_01,
            Base16::Zenburn => base16_zenburn::BASE_01,
        }
    }
}

macro_rules! generate_base_color_function {
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

generate_base_color_function!(base00, BASE_00);
generate_base_color_function!(base01, BASE_01);
generate_base_color_function!(base02, BASE_02);
generate_base_color_function!(base03, BASE_03);
generate_base_color_function!(base04, BASE_04);
generate_base_color_function!(base05, BASE_05);
generate_base_color_function!(base06, BASE_06);
generate_base_color_function!(base07, BASE_07);
generate_base_color_function!(base08, BASE_08);
generate_base_color_function!(base09, BASE_09);
generate_base_color_function!(base0a, BASE_0A);
generate_base_color_function!(base0b, BASE_0B);
generate_base_color_function!(base0c, BASE_0C);
generate_base_color_function!(base0d, BASE_0D);
generate_base_color_function!(base0e, BASE_0E);
generate_base_color_function!(base0f, BASE_0F);
