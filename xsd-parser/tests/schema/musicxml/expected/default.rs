#[derive(Debug)]
pub enum AboveBelow {
    Above,
    Below,
}
#[derive(Debug)]
pub struct Accidental {
    pub cautionary: ::core::option::Option<YesNo>,
    pub editorial: ::core::option::Option<YesNo>,
    pub parentheses: ::core::option::Option<YesNo>,
    pub bracket: ::core::option::Option<YesNo>,
    pub size: ::core::option::Option<SymbolSize>,
    pub default_x: ::core::option::Option<::core::primitive::f64>,
    pub default_y: ::core::option::Option<::core::primitive::f64>,
    pub relative_x: ::core::option::Option<::core::primitive::f64>,
    pub relative_y: ::core::option::Option<::core::primitive::f64>,
    pub font_family: ::core::option::Option<::std::string::String>,
    pub font_style: ::core::option::Option<FontStyle>,
    pub font_size: ::core::option::Option<FontSize>,
    pub font_weight: ::core::option::Option<FontWeight>,
    pub color: ::core::option::Option<::std::string::String>,
    pub smufl: ::core::option::Option<::std::string::String>,
    pub content: AccidentalValue,
}
#[derive(Debug)]
pub struct AccidentalMark {
    pub parentheses: ::core::option::Option<YesNo>,
    pub bracket: ::core::option::Option<YesNo>,
    pub size: ::core::option::Option<SymbolSize>,
    pub default_x: ::core::option::Option<::core::primitive::f64>,
    pub default_y: ::core::option::Option<::core::primitive::f64>,
    pub relative_x: ::core::option::Option<::core::primitive::f64>,
    pub relative_y: ::core::option::Option<::core::primitive::f64>,
    pub font_family: ::core::option::Option<::std::string::String>,
    pub font_style: ::core::option::Option<FontStyle>,
    pub font_size: ::core::option::Option<FontSize>,
    pub font_weight: ::core::option::Option<FontWeight>,
    pub color: ::core::option::Option<::std::string::String>,
    pub placement: ::core::option::Option<AboveBelow>,
    pub smufl: ::core::option::Option<::std::string::String>,
    pub id: ::core::option::Option<::std::string::String>,
    pub content: AccidentalValue,
}
#[derive(Debug)]
pub struct AccidentalText {
    pub justify: ::core::option::Option<LeftCenterRight>,
    pub default_x: ::core::option::Option<::core::primitive::f64>,
    pub default_y: ::core::option::Option<::core::primitive::f64>,
    pub relative_x: ::core::option::Option<::core::primitive::f64>,
    pub relative_y: ::core::option::Option<::core::primitive::f64>,
    pub font_family: ::core::option::Option<::std::string::String>,
    pub font_style: ::core::option::Option<FontStyle>,
    pub font_size: ::core::option::Option<FontSize>,
    pub font_weight: ::core::option::Option<FontWeight>,
    pub color: ::core::option::Option<::std::string::String>,
    pub halign: ::core::option::Option<LeftCenterRight>,
    pub valign: ::core::option::Option<Valign>,
    pub underline: ::core::option::Option<::num::BigUint>,
    pub overline: ::core::option::Option<::num::BigUint>,
    pub line_through: ::core::option::Option<::num::BigUint>,
    pub rotation: ::core::option::Option<::core::primitive::f64>,
    pub letter_spacing: ::core::option::Option<NumberOrNormal>,
    pub line_height: ::core::option::Option<NumberOrNormal>,
    pub lang: ::core::option::Option<::std::string::String>,
    pub space: ::core::option::Option<xml::Space>,
    pub dir: ::core::option::Option<TextDirection>,
    pub enclosure: ::core::option::Option<EnclosureShape>,
    pub smufl: ::core::option::Option<::std::string::String>,
    pub content: AccidentalValue,
}
#[derive(Debug)]
pub enum AccidentalValue {
    Sharp,
    Natural,
    Flat,
    DoubleSharp,
    SharpSharp,
    FlatFlat,
    NaturalSharp,
    NaturalFlat,
    QuarterFlat,
    QuarterSharp,
    ThreeQuartersFlat,
    ThreeQuartersSharp,
    SharpDown,
    SharpUp,
    NaturalDown,
    NaturalUp,
    FlatDown,
    FlatUp,
    DoubleSharpDown,
    DoubleSharpUp,
    FlatFlatDown,
    FlatFlatUp,
    ArrowDown,
    ArrowUp,
    TripleSharp,
    TripleFlat,
    SlashQuarterSharp,
    SlashSharp,
    SlashFlat,
    DoubleSlashFlat,
    Sharp1,
    Sharp2,
    Sharp3,
    Sharp5,
    Flat1,
    Flat2,
    Flat3,
    Flat4,
    Sori,
    Koron,
    Other,
}
#[derive(Debug)]
pub struct Accord {
    pub string: ::core::option::Option<::num::BigUint>,
    pub content: AccordContent,
}
#[derive(Debug)]
pub struct AccordContent {
    pub tuning_step: Step,
    pub tuning_alter: ::core::option::Option<::core::primitive::f64>,
    pub tuning_octave: ::num::BigInt,
}
pub type AccordionMiddle = ::num::BigUint;
#[derive(Debug)]
pub struct AccordionRegistration {
    pub default_x: ::core::option::Option<::core::primitive::f64>,
    pub default_y: ::core::option::Option<::core::primitive::f64>,
    pub relative_x: ::core::option::Option<::core::primitive::f64>,
    pub relative_y: ::core::option::Option<::core::primitive::f64>,
    pub font_family: ::core::option::Option<::std::string::String>,
    pub font_style: ::core::option::Option<FontStyle>,
    pub font_size: ::core::option::Option<FontSize>,
    pub font_weight: ::core::option::Option<FontWeight>,
    pub color: ::core::option::Option<::std::string::String>,
    pub halign: ::core::option::Option<LeftCenterRight>,
    pub valign: ::core::option::Option<Valign>,
    pub id: ::core::option::Option<::std::string::String>,
    pub content: AccordionRegistrationContent,
}
#[derive(Debug)]
pub struct AccordionRegistrationContent {
    pub accordion_high: ::core::option::Option<Empty>,
    pub accordion_middle: ::core::option::Option<::num::BigUint>,
    pub accordion_low: ::core::option::Option<Empty>,
}
#[derive(Debug)]
pub struct Appearance {
    pub content: AppearanceContent,
}
#[derive(Debug)]
pub struct AppearanceContent {
    pub line_width: ::std::vec::Vec<LineWidth>,
    pub note_size: ::std::vec::Vec<NoteSize>,
    pub distance: ::std::vec::Vec<Distance>,
    pub glyph: ::std::vec::Vec<Glyph>,
    pub other_appearance: ::std::vec::Vec<OtherAppearance>,
}
#[derive(Debug)]
pub struct Arpeggiate {
    pub number: ::core::option::Option<::num::BigUint>,
    pub direction: ::core::option::Option<UpDown>,
    pub unbroken: ::core::option::Option<YesNo>,
    pub default_x: ::core::option::Option<::core::primitive::f64>,
    pub default_y: ::core::option::Option<::core::primitive::f64>,
    pub relative_x: ::core::option::Option<::core::primitive::f64>,
    pub relative_y: ::core::option::Option<::core::primitive::f64>,
    pub placement: ::core::option::Option<AboveBelow>,
    pub color: ::core::option::Option<::std::string::String>,
    pub id: ::core::option::Option<::std::string::String>,
}
#[derive(Debug)]
pub struct Arrow {
    pub default_x: ::core::option::Option<::core::primitive::f64>,
    pub default_y: ::core::option::Option<::core::primitive::f64>,
    pub relative_x: ::core::option::Option<::core::primitive::f64>,
    pub relative_y: ::core::option::Option<::core::primitive::f64>,
    pub font_family: ::core::option::Option<::std::string::String>,
    pub font_style: ::core::option::Option<FontStyle>,
    pub font_size: ::core::option::Option<FontSize>,
    pub font_weight: ::core::option::Option<FontWeight>,
    pub color: ::core::option::Option<::std::string::String>,
    pub placement: ::core::option::Option<AboveBelow>,
    pub smufl: ::core::option::Option<::std::string::String>,
    pub content: ::std::vec::Vec<ArrowContent>,
}
#[derive(Debug)]
pub enum ArrowContent {
    ArrowDirection(ArrowDirection),
    ArrowStyle(ArrowStyle),
    Arrowhead(Empty),
    CircularArrow(CircularArrow),
}
#[derive(Debug)]
pub enum ArrowDirection {
    Left,
    Up,
    Right,
    Down,
    Northwest,
    Northeast,
    Southeast,
    Southwest,
    LeftRight,
    UpDown,
    NorthwestSoutheast,
    NortheastSouthwest,
    Other,
}
#[derive(Debug)]
pub enum ArrowStyle {
    Single,
    Double,
    Filled,
    Hollow,
    Paired,
    Combined,
    Other,
}
#[derive(Debug)]
pub struct Articulations {
    pub id: ::core::option::Option<::std::string::String>,
    pub content: ::std::vec::Vec<ArticulationsContent>,
}
#[derive(Debug)]
pub enum ArticulationsContent {
    Accent(EmptyPlacement),
    StrongAccent(StrongAccent),
    Staccato(EmptyPlacement),
    Tenuto(EmptyPlacement),
    DetachedLegato(EmptyPlacement),
    Staccatissimo(EmptyPlacement),
    Spiccato(EmptyPlacement),
    Scoop(EmptyLine),
    Plop(EmptyLine),
    Doit(EmptyLine),
    Falloff(EmptyLine),
    BreathMark(BreathMark),
    Caesura(Caesura),
    Stress(EmptyPlacement),
    Unstress(EmptyPlacement),
    SoftAccent(EmptyPlacement),
    OtherArticulation(OtherPlacementText),
}
#[derive(Debug)]
pub struct Assess {
    pub type_: YesNo,
    pub player: ::core::option::Option<::std::string::String>,
    pub time_only: ::core::option::Option<::std::string::String>,
}
#[derive(Debug)]
pub struct Attributes {
    pub content: ::std::vec::Vec<AttributesContent>,
}
#[derive(Debug)]
pub enum AttributesContent {
    Footnote(FormattedText),
    Level(Level),
    Divisions(::core::primitive::f64),
    Key(Key),
    Time(Time),
    Staves(::num::BigUint),
    PartSymbol(PartSymbol),
    Instruments(::num::BigUint),
    Clef(Clef),
    StaffDetails(StaffDetails),
    Transpose(Transpose),
    ForPart(ForPart),
    Directive(AttributesDirectiveElementType),
    MeasureStyle(MeasureStyle),
}
#[derive(Debug)]
pub struct Backup {
    pub content: BackupContent,
}
#[derive(Debug)]
pub struct BackupContent {
    pub duration: ::core::primitive::f64,
    pub footnote: ::core::option::Option<FormattedText>,
    pub level: ::core::option::Option<Level>,
}
#[derive(Debug)]
pub enum BackwardForward {
    Backward,
    Forward,
}
#[derive(Debug)]
pub enum BarStyle {
    Regular,
    Dotted,
    Dashed,
    Heavy,
    LightLight,
    LightHeavy,
    HeavyLight,
    HeavyHeavy,
    Tick,
    Short,
    None,
}
#[derive(Debug)]
pub struct BarStyleColor {
    pub color: ::core::option::Option<::std::string::String>,
    pub content: BarStyle,
}
#[derive(Debug)]
pub struct Barline {
    pub location: RightLeftMiddle,
    pub segno_attr: ::core::option::Option<::std::string::String>,
    pub coda_attr: ::core::option::Option<::std::string::String>,
    pub divisions: ::core::option::Option<::core::primitive::f64>,
    pub id: ::core::option::Option<::std::string::String>,
    pub content: BarlineContent,
}
#[derive(Debug)]
pub struct BarlineContent {
    pub bar_style: ::core::option::Option<BarStyleColor>,
    pub footnote: ::core::option::Option<FormattedText>,
    pub level: ::core::option::Option<Level>,
    pub wavy_line: ::core::option::Option<WavyLine>,
    pub segno: ::core::option::Option<Segno>,
    pub coda: ::core::option::Option<Coda>,
    pub fermata: ::std::vec::Vec<Fermata>,
    pub ending: ::core::option::Option<Ending>,
    pub repeat: ::core::option::Option<Repeat>,
}
#[derive(Debug)]
pub struct Barre {
    pub type_: StartStop,
    pub color: ::core::option::Option<::std::string::String>,
}
#[derive(Debug)]
pub struct Bass {
    pub arrangement: ::core::option::Option<HarmonyArrangement>,
    pub content: BassContent,
}
#[derive(Debug)]
pub struct BassContent {
    pub bass_separator: ::core::option::Option<StyleText>,
    pub bass_step: BassStep,
    pub bass_alter: ::core::option::Option<HarmonyAlter>,
}
#[derive(Debug)]
pub struct BassStep {
    pub text: ::core::option::Option<::std::string::String>,
    pub default_x: ::core::option::Option<::core::primitive::f64>,
    pub default_y: ::core::option::Option<::core::primitive::f64>,
    pub relative_x: ::core::option::Option<::core::primitive::f64>,
    pub relative_y: ::core::option::Option<::core::primitive::f64>,
    pub font_family: ::core::option::Option<::std::string::String>,
    pub font_style: ::core::option::Option<FontStyle>,
    pub font_size: ::core::option::Option<FontSize>,
    pub font_weight: ::core::option::Option<FontWeight>,
    pub color: ::core::option::Option<::std::string::String>,
    pub content: Step,
}
#[derive(Debug)]
pub struct Beam {
    pub number: ::num::BigUint,
    pub repeater: ::core::option::Option<YesNo>,
    pub fan: ::core::option::Option<Fan>,
    pub color: ::core::option::Option<::std::string::String>,
    pub id: ::core::option::Option<::std::string::String>,
    pub content: BeamValue,
}
pub type BeamLevel = ::num::BigUint;
#[derive(Debug)]
pub enum BeamValue {
    Begin,
    Continue,
    End,
    ForwardHook,
    BackwardHook,
}
#[derive(Debug)]
pub struct BeatRepeat {
    pub type_: StartStop,
    pub slashes: ::core::option::Option<::num::BigUint>,
    pub use_dots: ::core::option::Option<YesNo>,
    pub content: ::core::option::Option<BeatRepeatContent>,
}
#[derive(Debug)]
pub struct BeatRepeatContent {
    pub slash_type: ::core::option::Option<NoteTypeValue>,
    pub slash_dot: ::std::vec::Vec<Empty>,
    pub except_voice: ::std::vec::Vec<::std::string::String>,
}
#[derive(Debug)]
pub struct BeatUnitTied {
    pub content: BeatUnitTiedContent,
}
#[derive(Debug)]
pub struct BeatUnitTiedContent {
    pub beat_unit: NoteTypeValue,
    pub beat_unit_dot: ::std::vec::Vec<Empty>,
}
#[derive(Debug)]
pub struct Beater {
    pub tip: ::core::option::Option<TipDirection>,
    pub content: BeaterValue,
}
#[derive(Debug)]
pub enum BeaterValue {
    Bow,
    ChimeHammer,
    Coin,
    DrumStick,
    Finger,
    Fingernail,
    Fist,
    GuiroScraper,
    Hammer,
    Hand,
    JazzStick,
    KnittingNeedle,
    MetalHammer,
    SlideBrushOnGong,
    SnareStick,
    SpoonMallet,
    Superball,
    TriangleBeater,
    TriangleBeaterPlain,
    WireBrush,
}
#[derive(Debug)]
pub struct Bend {
    pub shape: ::core::option::Option<BendShape>,
    pub default_x: ::core::option::Option<::core::primitive::f64>,
    pub default_y: ::core::option::Option<::core::primitive::f64>,
    pub relative_x: ::core::option::Option<::core::primitive::f64>,
    pub relative_y: ::core::option::Option<::core::primitive::f64>,
    pub font_family: ::core::option::Option<::std::string::String>,
    pub font_style: ::core::option::Option<FontStyle>,
    pub font_size: ::core::option::Option<FontSize>,
    pub font_weight: ::core::option::Option<FontWeight>,
    pub color: ::core::option::Option<::std::string::String>,
    pub accelerate: ::core::option::Option<YesNo>,
    pub beats: ::core::option::Option<::core::primitive::f64>,
    pub first_beat: ::core::option::Option<::core::primitive::f64>,
    pub last_beat: ::core::option::Option<::core::primitive::f64>,
    pub content: ::std::vec::Vec<BendContent>,
}
#[derive(Debug)]
pub enum BendContent {
    BendAlter(::core::primitive::f64),
    PreBend(Empty),
    Release(Release),
    WithBar(PlacementText),
}
#[derive(Debug)]
pub enum BendShape {
    Angled,
    Curved,
}
#[derive(Debug)]
pub struct Bookmark {
    pub id: ::std::string::String,
    pub name: ::core::option::Option<::std::string::String>,
    pub element: ::core::option::Option<::std::string::String>,
    pub position: ::core::option::Option<::num::BigUint>,
}
#[derive(Debug)]
pub struct Bracket {
    pub type_: StartStopContinue,
    pub number: ::core::option::Option<::num::BigUint>,
    pub line_end: LineEnd,
    pub end_length: ::core::option::Option<::core::primitive::f64>,
    pub line_type: ::core::option::Option<LineType>,
    pub dash_length: ::core::option::Option<::core::primitive::f64>,
    pub space_length: ::core::option::Option<::core::primitive::f64>,
    pub default_x: ::core::option::Option<::core::primitive::f64>,
    pub default_y: ::core::option::Option<::core::primitive::f64>,
    pub relative_x: ::core::option::Option<::core::primitive::f64>,
    pub relative_y: ::core::option::Option<::core::primitive::f64>,
    pub color: ::core::option::Option<::std::string::String>,
    pub id: ::core::option::Option<::std::string::String>,
}
#[derive(Debug)]
pub struct BreathMark {
    pub default_x: ::core::option::Option<::core::primitive::f64>,
    pub default_y: ::core::option::Option<::core::primitive::f64>,
    pub relative_x: ::core::option::Option<::core::primitive::f64>,
    pub relative_y: ::core::option::Option<::core::primitive::f64>,
    pub font_family: ::core::option::Option<::std::string::String>,
    pub font_style: ::core::option::Option<FontStyle>,
    pub font_size: ::core::option::Option<FontSize>,
    pub font_weight: ::core::option::Option<FontWeight>,
    pub color: ::core::option::Option<::std::string::String>,
    pub placement: ::core::option::Option<AboveBelow>,
    pub content: BreathMarkValue,
}
#[derive(Debug)]
pub enum BreathMarkValue {
    Comma,
    Tick,
    Upbow,
    Salzedo,
}
#[derive(Debug)]
pub struct Caesura {
    pub default_x: ::core::option::Option<::core::primitive::f64>,
    pub default_y: ::core::option::Option<::core::primitive::f64>,
    pub relative_x: ::core::option::Option<::core::primitive::f64>,
    pub relative_y: ::core::option::Option<::core::primitive::f64>,
    pub font_family: ::core::option::Option<::std::string::String>,
    pub font_style: ::core::option::Option<FontStyle>,
    pub font_size: ::core::option::Option<FontSize>,
    pub font_weight: ::core::option::Option<FontWeight>,
    pub color: ::core::option::Option<::std::string::String>,
    pub placement: ::core::option::Option<AboveBelow>,
    pub content: CaesuraValue,
}
#[derive(Debug)]
pub enum CaesuraValue {
    Normal,
    Thick,
    Short,
    Curved,
    Single,
}
#[derive(Debug)]
pub struct Cancel {
    pub location: ::core::option::Option<CancelLocation>,
    pub content: ::num::BigInt,
}
#[derive(Debug)]
pub enum CancelLocation {
    Left,
    Right,
    BeforeBarline,
}
#[derive(Debug)]
pub enum CircularArrow {
    Clockwise,
    Anticlockwise,
}
#[derive(Debug)]
pub struct Clef {
    pub number: ::core::option::Option<::num::BigUint>,
    pub additional: ::core::option::Option<YesNo>,
    pub size: ::core::option::Option<SymbolSize>,
    pub after_barline: ::core::option::Option<YesNo>,
    pub default_x: ::core::option::Option<::core::primitive::f64>,
    pub default_y: ::core::option::Option<::core::primitive::f64>,
    pub relative_x: ::core::option::Option<::core::primitive::f64>,
    pub relative_y: ::core::option::Option<::core::primitive::f64>,
    pub font_family: ::core::option::Option<::std::string::String>,
    pub font_style: ::core::option::Option<FontStyle>,
    pub font_size: ::core::option::Option<FontSize>,
    pub font_weight: ::core::option::Option<FontWeight>,
    pub color: ::core::option::Option<::std::string::String>,
    pub print_object: ::core::option::Option<YesNo>,
    pub id: ::core::option::Option<::std::string::String>,
    pub content: ClefContent,
}
#[derive(Debug)]
pub struct ClefContent {
    pub sign: ClefSign,
    pub line: ::core::option::Option<::num::BigInt>,
    pub clef_octave_change: ::core::option::Option<::num::BigInt>,
}
#[derive(Debug)]
pub enum ClefSign {
    G,
    F,
    C,
    Percussion,
    Tab,
    Jianpu,
    None,
}
#[derive(Debug)]
pub struct Coda {
    pub default_x: ::core::option::Option<::core::primitive::f64>,
    pub default_y: ::core::option::Option<::core::primitive::f64>,
    pub relative_x: ::core::option::Option<::core::primitive::f64>,
    pub relative_y: ::core::option::Option<::core::primitive::f64>,
    pub font_family: ::core::option::Option<::std::string::String>,
    pub font_style: ::core::option::Option<FontStyle>,
    pub font_size: ::core::option::Option<FontSize>,
    pub font_weight: ::core::option::Option<FontWeight>,
    pub color: ::core::option::Option<::std::string::String>,
    pub halign: ::core::option::Option<LeftCenterRight>,
    pub valign: ::core::option::Option<Valign>,
    pub id: ::core::option::Option<::std::string::String>,
    pub smufl: ::core::option::Option<::std::string::String>,
}
pub type Color = ::std::string::String;
pub type CommaSeparatedText = ::std::string::String;
#[derive(Debug)]
pub struct Credit {
    pub page: ::core::option::Option<::num::BigUint>,
    pub id: ::core::option::Option<::std::string::String>,
    pub content: ::std::vec::Vec<CreditContent>,
}
#[derive(Debug)]
pub enum CreditContent {
    CreditType(::std::string::String),
    Link(Link),
    Bookmark(Bookmark),
    CreditImage(Image),
    CreditWords(FormattedTextId),
    CreditSymbol(FormattedSymbolId),
}
#[derive(Debug)]
pub enum CssFontSize {
    XxSmall,
    XSmall,
    Small,
    Medium,
    Large,
    XLarge,
    XxLarge,
}
#[derive(Debug)]
pub struct Dashes {
    pub type_: StartStopContinue,
    pub number: ::core::option::Option<::num::BigUint>,
    pub dash_length: ::core::option::Option<::core::primitive::f64>,
    pub space_length: ::core::option::Option<::core::primitive::f64>,
    pub default_x: ::core::option::Option<::core::primitive::f64>,
    pub default_y: ::core::option::Option<::core::primitive::f64>,
    pub relative_x: ::core::option::Option<::core::primitive::f64>,
    pub relative_y: ::core::option::Option<::core::primitive::f64>,
    pub color: ::core::option::Option<::std::string::String>,
    pub id: ::core::option::Option<::std::string::String>,
}
#[derive(Debug)]
pub struct Defaults {
    pub content: DefaultsContent,
}
#[derive(Debug)]
pub struct DefaultsContent {
    pub scaling: ::core::option::Option<Scaling>,
    pub concert_score: ::core::option::Option<Empty>,
    pub page_layout: ::core::option::Option<PageLayout>,
    pub system_layout: ::core::option::Option<SystemLayout>,
    pub staff_layout: ::std::vec::Vec<StaffLayout>,
    pub appearance: ::core::option::Option<Appearance>,
    pub music_font: ::core::option::Option<EmptyFont>,
    pub word_font: ::core::option::Option<EmptyFont>,
    pub lyric_font: ::std::vec::Vec<LyricFont>,
    pub lyric_language: ::std::vec::Vec<LyricLanguage>,
}
#[derive(Debug)]
pub struct Degree {
    pub print_object: ::core::option::Option<YesNo>,
    pub content: DegreeContent,
}
#[derive(Debug)]
pub struct DegreeContent {
    pub degree_value: DegreeValue,
    pub degree_alter: DegreeAlter,
    pub degree_type: DegreeType,
}
#[derive(Debug)]
pub struct DegreeAlter {
    pub default_x: ::core::option::Option<::core::primitive::f64>,
    pub default_y: ::core::option::Option<::core::primitive::f64>,
    pub relative_x: ::core::option::Option<::core::primitive::f64>,
    pub relative_y: ::core::option::Option<::core::primitive::f64>,
    pub font_family: ::core::option::Option<::std::string::String>,
    pub font_style: ::core::option::Option<FontStyle>,
    pub font_size: ::core::option::Option<FontSize>,
    pub font_weight: ::core::option::Option<FontWeight>,
    pub color: ::core::option::Option<::std::string::String>,
    pub plus_minus: ::core::option::Option<YesNo>,
    pub content: ::core::primitive::f64,
}
#[derive(Debug)]
pub enum DegreeSymbolValue {
    Major,
    Minor,
    Augmented,
    Diminished,
    HalfDiminished,
}
#[derive(Debug)]
pub struct DegreeType {
    pub text: ::core::option::Option<::std::string::String>,
    pub default_x: ::core::option::Option<::core::primitive::f64>,
    pub default_y: ::core::option::Option<::core::primitive::f64>,
    pub relative_x: ::core::option::Option<::core::primitive::f64>,
    pub relative_y: ::core::option::Option<::core::primitive::f64>,
    pub font_family: ::core::option::Option<::std::string::String>,
    pub font_style: ::core::option::Option<FontStyle>,
    pub font_size: ::core::option::Option<FontSize>,
    pub font_weight: ::core::option::Option<FontWeight>,
    pub color: ::core::option::Option<::std::string::String>,
    pub content: DegreeTypeValue,
}
#[derive(Debug)]
pub enum DegreeTypeValue {
    Add,
    Alter,
    Subtract,
}
#[derive(Debug)]
pub struct DegreeValue {
    pub symbol: ::core::option::Option<DegreeSymbolValue>,
    pub text: ::core::option::Option<::std::string::String>,
    pub default_x: ::core::option::Option<::core::primitive::f64>,
    pub default_y: ::core::option::Option<::core::primitive::f64>,
    pub relative_x: ::core::option::Option<::core::primitive::f64>,
    pub relative_y: ::core::option::Option<::core::primitive::f64>,
    pub font_family: ::core::option::Option<::std::string::String>,
    pub font_style: ::core::option::Option<FontStyle>,
    pub font_size: ::core::option::Option<FontSize>,
    pub font_weight: ::core::option::Option<FontWeight>,
    pub color: ::core::option::Option<::std::string::String>,
    pub content: ::num::BigUint,
}
#[derive(Debug)]
pub struct Direction {
    pub placement: ::core::option::Option<AboveBelow>,
    pub directive: ::core::option::Option<YesNo>,
    pub system: ::core::option::Option<SystemRelation>,
    pub id: ::core::option::Option<::std::string::String>,
    pub content: DirectionContent,
}
#[derive(Debug)]
pub struct DirectionContent {
    pub direction_type: ::std::vec::Vec<DirectionType>,
    pub offset: ::core::option::Option<Offset>,
    pub footnote: ::core::option::Option<FormattedText>,
    pub level: ::core::option::Option<Level>,
    pub voice: ::core::option::Option<::std::string::String>,
    pub staff: ::core::option::Option<::num::BigUint>,
    pub sound: ::core::option::Option<Sound>,
    pub listening: ::core::option::Option<Listening>,
}
#[derive(Debug)]
pub struct DirectionType {
    pub id: ::core::option::Option<::std::string::String>,
    pub content: ::std::vec::Vec<DirectionTypeContent>,
}
#[derive(Debug)]
pub enum DirectionTypeContent {
    Rehearsal(FormattedTextId),
    Segno(Segno),
    Coda(Coda),
    Words(FormattedTextId),
    Symbol(FormattedSymbolId),
    Wedge(Wedge),
    Dynamics(Dynamics),
    Dashes(Dashes),
    Bracket(Bracket),
    Pedal(Pedal),
    Metronome(Metronome),
    OctaveShift(OctaveShift),
    HarpPedals(HarpPedals),
    Damp(EmptyPrintStyleAlignId),
    DampAll(EmptyPrintStyleAlignId),
    Eyeglasses(EmptyPrintStyleAlignId),
    StringMute(StringMute),
    Scordatura(Scordatura),
    Image(Image),
    PrincipalVoice(PrincipalVoice),
    Percussion(Percussion),
    AccordionRegistration(AccordionRegistration),
    StaffDivide(StaffDivide),
    OtherDirection(OtherDirection),
}
#[derive(Debug)]
pub struct Distance {
    pub type_: ::std::string::String,
    pub content: ::core::primitive::f64,
}
pub type DistanceType = ::std::string::String;
pub type Divisions = ::core::primitive::f64;
#[derive(Debug)]
pub struct Double {
    pub above: ::core::option::Option<YesNo>,
}
#[derive(Debug)]
pub struct Dynamics {
    pub default_x: ::core::option::Option<::core::primitive::f64>,
    pub default_y: ::core::option::Option<::core::primitive::f64>,
    pub relative_x: ::core::option::Option<::core::primitive::f64>,
    pub relative_y: ::core::option::Option<::core::primitive::f64>,
    pub font_family: ::core::option::Option<::std::string::String>,
    pub font_style: ::core::option::Option<FontStyle>,
    pub font_size: ::core::option::Option<FontSize>,
    pub font_weight: ::core::option::Option<FontWeight>,
    pub color: ::core::option::Option<::std::string::String>,
    pub halign: ::core::option::Option<LeftCenterRight>,
    pub valign: ::core::option::Option<Valign>,
    pub placement: ::core::option::Option<AboveBelow>,
    pub underline: ::core::option::Option<::num::BigUint>,
    pub overline: ::core::option::Option<::num::BigUint>,
    pub line_through: ::core::option::Option<::num::BigUint>,
    pub enclosure: ::core::option::Option<EnclosureShape>,
    pub id: ::core::option::Option<::std::string::String>,
    pub content: ::std::vec::Vec<DynamicsContent>,
}
#[derive(Debug)]
pub enum DynamicsContent {
    P(Empty),
    Pp(Empty),
    Ppp(Empty),
    Pppp(Empty),
    Ppppp(Empty),
    Pppppp(Empty),
    F(Empty),
    Ff(Empty),
    Fff(Empty),
    Ffff(Empty),
    Fffff(Empty),
    Ffffff(Empty),
    Mp(Empty),
    Mf(Empty),
    Sf(Empty),
    Sfp(Empty),
    Sfpp(Empty),
    Fp(Empty),
    Rf(Empty),
    Rfz(Empty),
    Sfz(Empty),
    Sffz(Empty),
    Fz(Empty),
    N(Empty),
    Pf(Empty),
    Sfzp(Empty),
    OtherDynamics(OtherText),
}
#[derive(Debug)]
pub struct Effect {
    pub smufl: ::core::option::Option<::std::string::String>,
    pub content: EffectValue,
}
#[derive(Debug)]
pub enum EffectValue {
    Anvil,
    AutoHorn,
    BirdWhistle,
    Cannon,
    DuckCall,
    GunShot,
    KlaxonHorn,
    LionsRoar,
    LotusFlute,
    Megaphone,
    PoliceWhistle,
    Siren,
    SlideWhistle,
    ThunderSheet,
    WindMachine,
    WindWhistle,
}
#[derive(Debug)]
pub struct Elision {
    pub font_family: ::core::option::Option<::std::string::String>,
    pub font_style: ::core::option::Option<FontStyle>,
    pub font_size: ::core::option::Option<FontSize>,
    pub font_weight: ::core::option::Option<FontWeight>,
    pub color: ::core::option::Option<::std::string::String>,
    pub smufl: ::core::option::Option<::std::string::String>,
    pub content: ::std::string::String,
}
#[derive(Debug)]
pub struct Empty;
#[derive(Debug)]
pub struct EmptyFont {
    pub font_family: ::core::option::Option<::std::string::String>,
    pub font_style: ::core::option::Option<FontStyle>,
    pub font_size: ::core::option::Option<FontSize>,
    pub font_weight: ::core::option::Option<FontWeight>,
}
#[derive(Debug)]
pub struct EmptyLine {
    pub line_shape: ::core::option::Option<LineShape>,
    pub line_type: ::core::option::Option<LineType>,
    pub line_length: ::core::option::Option<LineLength>,
    pub dash_length: ::core::option::Option<::core::primitive::f64>,
    pub space_length: ::core::option::Option<::core::primitive::f64>,
    pub default_x: ::core::option::Option<::core::primitive::f64>,
    pub default_y: ::core::option::Option<::core::primitive::f64>,
    pub relative_x: ::core::option::Option<::core::primitive::f64>,
    pub relative_y: ::core::option::Option<::core::primitive::f64>,
    pub font_family: ::core::option::Option<::std::string::String>,
    pub font_style: ::core::option::Option<FontStyle>,
    pub font_size: ::core::option::Option<FontSize>,
    pub font_weight: ::core::option::Option<FontWeight>,
    pub color: ::core::option::Option<::std::string::String>,
    pub placement: ::core::option::Option<AboveBelow>,
}
#[derive(Debug)]
pub struct EmptyPlacement {
    pub default_x: ::core::option::Option<::core::primitive::f64>,
    pub default_y: ::core::option::Option<::core::primitive::f64>,
    pub relative_x: ::core::option::Option<::core::primitive::f64>,
    pub relative_y: ::core::option::Option<::core::primitive::f64>,
    pub font_family: ::core::option::Option<::std::string::String>,
    pub font_style: ::core::option::Option<FontStyle>,
    pub font_size: ::core::option::Option<FontSize>,
    pub font_weight: ::core::option::Option<FontWeight>,
    pub color: ::core::option::Option<::std::string::String>,
    pub placement: ::core::option::Option<AboveBelow>,
}
#[derive(Debug)]
pub struct EmptyPlacementSmufl {
    pub default_x: ::core::option::Option<::core::primitive::f64>,
    pub default_y: ::core::option::Option<::core::primitive::f64>,
    pub relative_x: ::core::option::Option<::core::primitive::f64>,
    pub relative_y: ::core::option::Option<::core::primitive::f64>,
    pub font_family: ::core::option::Option<::std::string::String>,
    pub font_style: ::core::option::Option<FontStyle>,
    pub font_size: ::core::option::Option<FontSize>,
    pub font_weight: ::core::option::Option<FontWeight>,
    pub color: ::core::option::Option<::std::string::String>,
    pub placement: ::core::option::Option<AboveBelow>,
    pub smufl: ::core::option::Option<::std::string::String>,
}
#[derive(Debug)]
pub struct EmptyPrintObjectStyleAlign {
    pub print_object: ::core::option::Option<YesNo>,
    pub default_x: ::core::option::Option<::core::primitive::f64>,
    pub default_y: ::core::option::Option<::core::primitive::f64>,
    pub relative_x: ::core::option::Option<::core::primitive::f64>,
    pub relative_y: ::core::option::Option<::core::primitive::f64>,
    pub font_family: ::core::option::Option<::std::string::String>,
    pub font_style: ::core::option::Option<FontStyle>,
    pub font_size: ::core::option::Option<FontSize>,
    pub font_weight: ::core::option::Option<FontWeight>,
    pub color: ::core::option::Option<::std::string::String>,
    pub halign: ::core::option::Option<LeftCenterRight>,
    pub valign: ::core::option::Option<Valign>,
}
#[derive(Debug)]
pub struct EmptyPrintStyle {
    pub default_x: ::core::option::Option<::core::primitive::f64>,
    pub default_y: ::core::option::Option<::core::primitive::f64>,
    pub relative_x: ::core::option::Option<::core::primitive::f64>,
    pub relative_y: ::core::option::Option<::core::primitive::f64>,
    pub font_family: ::core::option::Option<::std::string::String>,
    pub font_style: ::core::option::Option<FontStyle>,
    pub font_size: ::core::option::Option<FontSize>,
    pub font_weight: ::core::option::Option<FontWeight>,
    pub color: ::core::option::Option<::std::string::String>,
}
#[derive(Debug)]
pub struct EmptyPrintStyleAlign {
    pub default_x: ::core::option::Option<::core::primitive::f64>,
    pub default_y: ::core::option::Option<::core::primitive::f64>,
    pub relative_x: ::core::option::Option<::core::primitive::f64>,
    pub relative_y: ::core::option::Option<::core::primitive::f64>,
    pub font_family: ::core::option::Option<::std::string::String>,
    pub font_style: ::core::option::Option<FontStyle>,
    pub font_size: ::core::option::Option<FontSize>,
    pub font_weight: ::core::option::Option<FontWeight>,
    pub color: ::core::option::Option<::std::string::String>,
    pub halign: ::core::option::Option<LeftCenterRight>,
    pub valign: ::core::option::Option<Valign>,
}
#[derive(Debug)]
pub struct EmptyPrintStyleAlignId {
    pub default_x: ::core::option::Option<::core::primitive::f64>,
    pub default_y: ::core::option::Option<::core::primitive::f64>,
    pub relative_x: ::core::option::Option<::core::primitive::f64>,
    pub relative_y: ::core::option::Option<::core::primitive::f64>,
    pub font_family: ::core::option::Option<::std::string::String>,
    pub font_style: ::core::option::Option<FontStyle>,
    pub font_size: ::core::option::Option<FontSize>,
    pub font_weight: ::core::option::Option<FontWeight>,
    pub color: ::core::option::Option<::std::string::String>,
    pub halign: ::core::option::Option<LeftCenterRight>,
    pub valign: ::core::option::Option<Valign>,
    pub id: ::core::option::Option<::std::string::String>,
}
#[derive(Debug)]
pub struct EmptyTrillSound {
    pub default_x: ::core::option::Option<::core::primitive::f64>,
    pub default_y: ::core::option::Option<::core::primitive::f64>,
    pub relative_x: ::core::option::Option<::core::primitive::f64>,
    pub relative_y: ::core::option::Option<::core::primitive::f64>,
    pub font_family: ::core::option::Option<::std::string::String>,
    pub font_style: ::core::option::Option<FontStyle>,
    pub font_size: ::core::option::Option<FontSize>,
    pub font_weight: ::core::option::Option<FontWeight>,
    pub color: ::core::option::Option<::std::string::String>,
    pub placement: ::core::option::Option<AboveBelow>,
    pub start_note: ::core::option::Option<StartNote>,
    pub trill_step: ::core::option::Option<TrillStep>,
    pub two_note_turn: ::core::option::Option<TwoNoteTurn>,
    pub accelerate: ::core::option::Option<YesNo>,
    pub beats: ::core::option::Option<::core::primitive::f64>,
    pub second_beat: ::core::option::Option<::core::primitive::f64>,
    pub last_beat: ::core::option::Option<::core::primitive::f64>,
}
#[derive(Debug)]
pub enum EnclosureShape {
    Rectangle,
    Square,
    Oval,
    Circle,
    Bracket,
    InvertedBracket,
    Triangle,
    Diamond,
    Pentagon,
    Hexagon,
    Heptagon,
    Octagon,
    Nonagon,
    Decagon,
    None,
}
#[derive(Debug)]
pub struct Encoding {
    pub content: ::std::vec::Vec<EncodingContent>,
}
#[derive(Debug)]
pub enum EncodingContent {
    EncodingDate(::std::string::String),
    Encoder(TypedText),
    Software(::std::string::String),
    EncodingDescription(::std::string::String),
    Supports(Supports),
}
#[derive(Debug)]
pub struct Ending {
    pub number: ::std::string::String,
    pub type_: StartStopDiscontinue,
    pub print_object: ::core::option::Option<YesNo>,
    pub default_x: ::core::option::Option<::core::primitive::f64>,
    pub default_y: ::core::option::Option<::core::primitive::f64>,
    pub relative_x: ::core::option::Option<::core::primitive::f64>,
    pub relative_y: ::core::option::Option<::core::primitive::f64>,
    pub font_family: ::core::option::Option<::std::string::String>,
    pub font_style: ::core::option::Option<FontStyle>,
    pub font_size: ::core::option::Option<FontSize>,
    pub font_weight: ::core::option::Option<FontWeight>,
    pub color: ::core::option::Option<::std::string::String>,
    pub system: ::core::option::Option<SystemRelation>,
    pub end_length: ::core::option::Option<::core::primitive::f64>,
    pub text_x: ::core::option::Option<::core::primitive::f64>,
    pub text_y: ::core::option::Option<::core::primitive::f64>,
    pub content: ::std::string::String,
}
pub type EndingNumber = ::std::string::String;
#[derive(Debug)]
pub struct Extend {
    pub type_: ::core::option::Option<StartStopContinue>,
    pub default_x: ::core::option::Option<::core::primitive::f64>,
    pub default_y: ::core::option::Option<::core::primitive::f64>,
    pub relative_x: ::core::option::Option<::core::primitive::f64>,
    pub relative_y: ::core::option::Option<::core::primitive::f64>,
    pub color: ::core::option::Option<::std::string::String>,
}
#[derive(Debug)]
pub enum Fan {
    Accel,
    Rit,
    None,
}
#[derive(Debug)]
pub struct Feature {
    pub type_: ::core::option::Option<::std::string::String>,
    pub content: ::std::string::String,
}
#[derive(Debug)]
pub struct Fermata {
    pub type_: ::core::option::Option<UprightInverted>,
    pub default_x: ::core::option::Option<::core::primitive::f64>,
    pub default_y: ::core::option::Option<::core::primitive::f64>,
    pub relative_x: ::core::option::Option<::core::primitive::f64>,
    pub relative_y: ::core::option::Option<::core::primitive::f64>,
    pub font_family: ::core::option::Option<::std::string::String>,
    pub font_style: ::core::option::Option<FontStyle>,
    pub font_size: ::core::option::Option<FontSize>,
    pub font_weight: ::core::option::Option<FontWeight>,
    pub color: ::core::option::Option<::std::string::String>,
    pub id: ::core::option::Option<::std::string::String>,
    pub content: FermataShape,
}
#[derive(Debug)]
pub enum FermataShape {
    Normal,
    Angled,
    Square,
    DoubleAngled,
    DoubleSquare,
    DoubleDot,
    HalfCurve,
    Curlew,
}
pub type Fifths = ::num::BigInt;
#[derive(Debug)]
pub struct Figure {
    pub content: FigureContent,
}
#[derive(Debug)]
pub struct FigureContent {
    pub prefix: ::core::option::Option<StyleText>,
    pub figure_number: ::core::option::Option<StyleText>,
    pub suffix: ::core::option::Option<StyleText>,
    pub extend: ::core::option::Option<Extend>,
    pub footnote: ::core::option::Option<FormattedText>,
    pub level: ::core::option::Option<Level>,
}
#[derive(Debug)]
pub struct FiguredBass {
    pub default_x: ::core::option::Option<::core::primitive::f64>,
    pub default_y: ::core::option::Option<::core::primitive::f64>,
    pub relative_x: ::core::option::Option<::core::primitive::f64>,
    pub relative_y: ::core::option::Option<::core::primitive::f64>,
    pub font_family: ::core::option::Option<::std::string::String>,
    pub font_style: ::core::option::Option<FontStyle>,
    pub font_size: ::core::option::Option<FontSize>,
    pub font_weight: ::core::option::Option<FontWeight>,
    pub color: ::core::option::Option<::std::string::String>,
    pub halign: ::core::option::Option<LeftCenterRight>,
    pub valign: ::core::option::Option<Valign>,
    pub placement: ::core::option::Option<AboveBelow>,
    pub print_object: ::core::option::Option<YesNo>,
    pub print_dot: ::core::option::Option<YesNo>,
    pub print_spacing: ::core::option::Option<YesNo>,
    pub print_lyric: ::core::option::Option<YesNo>,
    pub parentheses: ::core::option::Option<YesNo>,
    pub id: ::core::option::Option<::std::string::String>,
    pub content: FiguredBassContent,
}
#[derive(Debug)]
pub struct FiguredBassContent {
    pub figure: ::std::vec::Vec<Figure>,
    pub duration: ::core::option::Option<::core::primitive::f64>,
    pub footnote: ::core::option::Option<FormattedText>,
    pub level: ::core::option::Option<Level>,
}
#[derive(Debug)]
pub struct Fingering {
    pub substitution: ::core::option::Option<YesNo>,
    pub alternate: ::core::option::Option<YesNo>,
    pub default_x: ::core::option::Option<::core::primitive::f64>,
    pub default_y: ::core::option::Option<::core::primitive::f64>,
    pub relative_x: ::core::option::Option<::core::primitive::f64>,
    pub relative_y: ::core::option::Option<::core::primitive::f64>,
    pub font_family: ::core::option::Option<::std::string::String>,
    pub font_style: ::core::option::Option<FontStyle>,
    pub font_size: ::core::option::Option<FontSize>,
    pub font_weight: ::core::option::Option<FontWeight>,
    pub color: ::core::option::Option<::std::string::String>,
    pub placement: ::core::option::Option<AboveBelow>,
    pub content: ::std::string::String,
}
#[derive(Debug)]
pub struct FirstFret {
    pub text: ::core::option::Option<::std::string::String>,
    pub location: ::core::option::Option<LeftRight>,
    pub content: ::num::BigUint,
}
pub type FontFamily = ::std::string::String;
#[derive(Debug)]
pub enum FontSize {
    F64(::core::primitive::f64),
    XxSmall,
    XSmall,
    Small,
    Medium,
    Large,
    XLarge,
    XxLarge,
}
#[derive(Debug)]
pub enum FontStyle {
    Normal,
    Italic,
}
#[derive(Debug)]
pub enum FontWeight {
    Normal,
    Bold,
}
#[derive(Debug)]
pub struct ForPart {
    pub number: ::core::option::Option<::num::BigUint>,
    pub id: ::core::option::Option<::std::string::String>,
    pub content: ForPartContent,
}
#[derive(Debug)]
pub struct ForPartContent {
    pub part_clef: ::core::option::Option<PartClef>,
    pub part_transpose: PartTranspose,
}
#[derive(Debug)]
pub struct FormattedSymbol {
    pub justify: ::core::option::Option<LeftCenterRight>,
    pub default_x: ::core::option::Option<::core::primitive::f64>,
    pub default_y: ::core::option::Option<::core::primitive::f64>,
    pub relative_x: ::core::option::Option<::core::primitive::f64>,
    pub relative_y: ::core::option::Option<::core::primitive::f64>,
    pub font_family: ::core::option::Option<::std::string::String>,
    pub font_style: ::core::option::Option<FontStyle>,
    pub font_size: ::core::option::Option<FontSize>,
    pub font_weight: ::core::option::Option<FontWeight>,
    pub color: ::core::option::Option<::std::string::String>,
    pub halign: ::core::option::Option<LeftCenterRight>,
    pub valign: ::core::option::Option<Valign>,
    pub underline: ::core::option::Option<::num::BigUint>,
    pub overline: ::core::option::Option<::num::BigUint>,
    pub line_through: ::core::option::Option<::num::BigUint>,
    pub rotation: ::core::option::Option<::core::primitive::f64>,
    pub letter_spacing: ::core::option::Option<NumberOrNormal>,
    pub line_height: ::core::option::Option<NumberOrNormal>,
    pub dir: ::core::option::Option<TextDirection>,
    pub enclosure: ::core::option::Option<EnclosureShape>,
    pub content: ::std::string::String,
}
#[derive(Debug)]
pub struct FormattedSymbolId {
    pub justify: ::core::option::Option<LeftCenterRight>,
    pub default_x: ::core::option::Option<::core::primitive::f64>,
    pub default_y: ::core::option::Option<::core::primitive::f64>,
    pub relative_x: ::core::option::Option<::core::primitive::f64>,
    pub relative_y: ::core::option::Option<::core::primitive::f64>,
    pub font_family: ::core::option::Option<::std::string::String>,
    pub font_style: ::core::option::Option<FontStyle>,
    pub font_size: ::core::option::Option<FontSize>,
    pub font_weight: ::core::option::Option<FontWeight>,
    pub color: ::core::option::Option<::std::string::String>,
    pub halign: ::core::option::Option<LeftCenterRight>,
    pub valign: ::core::option::Option<Valign>,
    pub underline: ::core::option::Option<::num::BigUint>,
    pub overline: ::core::option::Option<::num::BigUint>,
    pub line_through: ::core::option::Option<::num::BigUint>,
    pub rotation: ::core::option::Option<::core::primitive::f64>,
    pub letter_spacing: ::core::option::Option<NumberOrNormal>,
    pub line_height: ::core::option::Option<NumberOrNormal>,
    pub dir: ::core::option::Option<TextDirection>,
    pub enclosure: ::core::option::Option<EnclosureShape>,
    pub id: ::core::option::Option<::std::string::String>,
    pub content: ::std::string::String,
}
#[derive(Debug)]
pub struct FormattedText {
    pub justify: ::core::option::Option<LeftCenterRight>,
    pub default_x: ::core::option::Option<::core::primitive::f64>,
    pub default_y: ::core::option::Option<::core::primitive::f64>,
    pub relative_x: ::core::option::Option<::core::primitive::f64>,
    pub relative_y: ::core::option::Option<::core::primitive::f64>,
    pub font_family: ::core::option::Option<::std::string::String>,
    pub font_style: ::core::option::Option<FontStyle>,
    pub font_size: ::core::option::Option<FontSize>,
    pub font_weight: ::core::option::Option<FontWeight>,
    pub color: ::core::option::Option<::std::string::String>,
    pub halign: ::core::option::Option<LeftCenterRight>,
    pub valign: ::core::option::Option<Valign>,
    pub underline: ::core::option::Option<::num::BigUint>,
    pub overline: ::core::option::Option<::num::BigUint>,
    pub line_through: ::core::option::Option<::num::BigUint>,
    pub rotation: ::core::option::Option<::core::primitive::f64>,
    pub letter_spacing: ::core::option::Option<NumberOrNormal>,
    pub line_height: ::core::option::Option<NumberOrNormal>,
    pub lang: ::core::option::Option<::std::string::String>,
    pub space: ::core::option::Option<xml::Space>,
    pub dir: ::core::option::Option<TextDirection>,
    pub enclosure: ::core::option::Option<EnclosureShape>,
    pub content: ::std::string::String,
}
#[derive(Debug)]
pub struct FormattedTextId {
    pub justify: ::core::option::Option<LeftCenterRight>,
    pub default_x: ::core::option::Option<::core::primitive::f64>,
    pub default_y: ::core::option::Option<::core::primitive::f64>,
    pub relative_x: ::core::option::Option<::core::primitive::f64>,
    pub relative_y: ::core::option::Option<::core::primitive::f64>,
    pub font_family: ::core::option::Option<::std::string::String>,
    pub font_style: ::core::option::Option<FontStyle>,
    pub font_size: ::core::option::Option<FontSize>,
    pub font_weight: ::core::option::Option<FontWeight>,
    pub color: ::core::option::Option<::std::string::String>,
    pub halign: ::core::option::Option<LeftCenterRight>,
    pub valign: ::core::option::Option<Valign>,
    pub underline: ::core::option::Option<::num::BigUint>,
    pub overline: ::core::option::Option<::num::BigUint>,
    pub line_through: ::core::option::Option<::num::BigUint>,
    pub rotation: ::core::option::Option<::core::primitive::f64>,
    pub letter_spacing: ::core::option::Option<NumberOrNormal>,
    pub line_height: ::core::option::Option<NumberOrNormal>,
    pub lang: ::core::option::Option<::std::string::String>,
    pub space: ::core::option::Option<xml::Space>,
    pub dir: ::core::option::Option<TextDirection>,
    pub enclosure: ::core::option::Option<EnclosureShape>,
    pub id: ::core::option::Option<::std::string::String>,
    pub content: ::std::string::String,
}
#[derive(Debug)]
pub struct Forward {
    pub content: ForwardContent,
}
#[derive(Debug)]
pub struct ForwardContent {
    pub duration: ::core::primitive::f64,
    pub footnote: ::core::option::Option<FormattedText>,
    pub level: ::core::option::Option<Level>,
    pub voice: ::core::option::Option<::std::string::String>,
    pub staff: ::core::option::Option<::num::BigUint>,
}
#[derive(Debug)]
pub struct Frame {
    pub default_x: ::core::option::Option<::core::primitive::f64>,
    pub default_y: ::core::option::Option<::core::primitive::f64>,
    pub relative_x: ::core::option::Option<::core::primitive::f64>,
    pub relative_y: ::core::option::Option<::core::primitive::f64>,
    pub color: ::core::option::Option<::std::string::String>,
    pub halign: ::core::option::Option<LeftCenterRight>,
    pub valign: ::core::option::Option<ValignImage>,
    pub height: ::core::option::Option<::core::primitive::f64>,
    pub width: ::core::option::Option<::core::primitive::f64>,
    pub unplayed: ::core::option::Option<::std::string::String>,
    pub id: ::core::option::Option<::std::string::String>,
    pub content: FrameContent,
}
#[derive(Debug)]
pub struct FrameContent {
    pub frame_strings: ::num::BigUint,
    pub frame_frets: ::num::BigUint,
    pub first_fret: ::core::option::Option<FirstFret>,
    pub frame_note: ::std::vec::Vec<FrameNote>,
}
#[derive(Debug)]
pub struct FrameNote {
    pub content: FrameNoteContent,
}
#[derive(Debug)]
pub struct FrameNoteContent {
    pub string: String,
    pub fret: Fret,
    pub fingering: ::core::option::Option<Fingering>,
    pub barre: ::core::option::Option<Barre>,
}
#[derive(Debug)]
pub struct Fret {
    pub font_family: ::core::option::Option<::std::string::String>,
    pub font_style: ::core::option::Option<FontStyle>,
    pub font_size: ::core::option::Option<FontSize>,
    pub font_weight: ::core::option::Option<FontWeight>,
    pub color: ::core::option::Option<::std::string::String>,
    pub content: ::num::BigUint,
}
#[derive(Debug)]
pub struct Glass {
    pub smufl: ::core::option::Option<::std::string::String>,
    pub content: GlassValue,
}
#[derive(Debug)]
pub enum GlassValue {
    GlassHarmonica,
    GlassHarp,
    WindChimes,
}
#[derive(Debug)]
pub struct Glissando {
    pub type_: StartStop,
    pub number: ::num::BigUint,
    pub line_type: ::core::option::Option<LineType>,
    pub dash_length: ::core::option::Option<::core::primitive::f64>,
    pub space_length: ::core::option::Option<::core::primitive::f64>,
    pub default_x: ::core::option::Option<::core::primitive::f64>,
    pub default_y: ::core::option::Option<::core::primitive::f64>,
    pub relative_x: ::core::option::Option<::core::primitive::f64>,
    pub relative_y: ::core::option::Option<::core::primitive::f64>,
    pub font_family: ::core::option::Option<::std::string::String>,
    pub font_style: ::core::option::Option<FontStyle>,
    pub font_size: ::core::option::Option<FontSize>,
    pub font_weight: ::core::option::Option<FontWeight>,
    pub color: ::core::option::Option<::std::string::String>,
    pub id: ::core::option::Option<::std::string::String>,
    pub content: ::std::string::String,
}
#[derive(Debug)]
pub struct Glyph {
    pub type_: ::std::string::String,
    pub content: ::std::string::String,
}
pub type GlyphType = ::std::string::String;
#[derive(Debug)]
pub struct Grace {
    pub steal_time_previous: ::core::option::Option<::core::primitive::f64>,
    pub steal_time_following: ::core::option::Option<::core::primitive::f64>,
    pub make_time: ::core::option::Option<::core::primitive::f64>,
    pub slash: ::core::option::Option<YesNo>,
}
#[derive(Debug)]
pub struct GroupBarline {
    pub color: ::core::option::Option<::std::string::String>,
    pub content: GroupBarlineValue,
}
#[derive(Debug)]
pub enum GroupBarlineValue {
    Yes,
    No,
    Mensurstrich,
}
#[derive(Debug)]
pub struct GroupName {
    pub default_x: ::core::option::Option<::core::primitive::f64>,
    pub default_y: ::core::option::Option<::core::primitive::f64>,
    pub relative_x: ::core::option::Option<::core::primitive::f64>,
    pub relative_y: ::core::option::Option<::core::primitive::f64>,
    pub font_family: ::core::option::Option<::std::string::String>,
    pub font_style: ::core::option::Option<FontStyle>,
    pub font_size: ::core::option::Option<FontSize>,
    pub font_weight: ::core::option::Option<FontWeight>,
    pub color: ::core::option::Option<::std::string::String>,
    pub justify: ::core::option::Option<LeftCenterRight>,
    pub content: ::std::string::String,
}
#[derive(Debug)]
pub struct GroupSymbol {
    pub default_x: ::core::option::Option<::core::primitive::f64>,
    pub default_y: ::core::option::Option<::core::primitive::f64>,
    pub relative_x: ::core::option::Option<::core::primitive::f64>,
    pub relative_y: ::core::option::Option<::core::primitive::f64>,
    pub color: ::core::option::Option<::std::string::String>,
    pub content: GroupSymbolValue,
}
#[derive(Debug)]
pub enum GroupSymbolValue {
    None,
    Brace,
    Line,
    Bracket,
    Square,
}
#[derive(Debug)]
pub struct Grouping {
    pub type_: StartStopSingle,
    pub number: ::std::string::String,
    pub member_of: ::core::option::Option<::std::string::String>,
    pub id: ::core::option::Option<::std::string::String>,
    pub content: GroupingContent,
}
#[derive(Debug)]
pub struct GroupingContent {
    pub feature: ::std::vec::Vec<Feature>,
}
#[derive(Debug)]
pub struct HammerOnPullOff {
    pub type_: StartStop,
    pub number: ::num::BigUint,
    pub default_x: ::core::option::Option<::core::primitive::f64>,
    pub default_y: ::core::option::Option<::core::primitive::f64>,
    pub relative_x: ::core::option::Option<::core::primitive::f64>,
    pub relative_y: ::core::option::Option<::core::primitive::f64>,
    pub font_family: ::core::option::Option<::std::string::String>,
    pub font_style: ::core::option::Option<FontStyle>,
    pub font_size: ::core::option::Option<FontSize>,
    pub font_weight: ::core::option::Option<FontWeight>,
    pub color: ::core::option::Option<::std::string::String>,
    pub placement: ::core::option::Option<AboveBelow>,
    pub content: ::std::string::String,
}
#[derive(Debug)]
pub struct Handbell {
    pub default_x: ::core::option::Option<::core::primitive::f64>,
    pub default_y: ::core::option::Option<::core::primitive::f64>,
    pub relative_x: ::core::option::Option<::core::primitive::f64>,
    pub relative_y: ::core::option::Option<::core::primitive::f64>,
    pub font_family: ::core::option::Option<::std::string::String>,
    pub font_style: ::core::option::Option<FontStyle>,
    pub font_size: ::core::option::Option<FontSize>,
    pub font_weight: ::core::option::Option<FontWeight>,
    pub color: ::core::option::Option<::std::string::String>,
    pub placement: ::core::option::Option<AboveBelow>,
    pub content: HandbellValue,
}
#[derive(Debug)]
pub enum HandbellValue {
    Belltree,
    Damp,
    Echo,
    Gyro,
    HandMartellato,
    MalletLift,
    MalletTable,
    Martellato,
    MartellatoLift,
    MutedMartellato,
    PluckLift,
    Swing,
}
#[derive(Debug)]
pub struct HarmonClosed {
    pub location: ::core::option::Option<HarmonClosedLocation>,
    pub content: HarmonClosedValue,
}
#[derive(Debug)]
pub enum HarmonClosedLocation {
    Right,
    Bottom,
    Left,
    Top,
}
#[derive(Debug)]
pub enum HarmonClosedValue {
    Yes,
    No,
    Half,
}
#[derive(Debug)]
pub struct HarmonMute {
    pub default_x: ::core::option::Option<::core::primitive::f64>,
    pub default_y: ::core::option::Option<::core::primitive::f64>,
    pub relative_x: ::core::option::Option<::core::primitive::f64>,
    pub relative_y: ::core::option::Option<::core::primitive::f64>,
    pub font_family: ::core::option::Option<::std::string::String>,
    pub font_style: ::core::option::Option<FontStyle>,
    pub font_size: ::core::option::Option<FontSize>,
    pub font_weight: ::core::option::Option<FontWeight>,
    pub color: ::core::option::Option<::std::string::String>,
    pub placement: ::core::option::Option<AboveBelow>,
    pub content: HarmonMuteContent,
}
#[derive(Debug)]
pub struct HarmonMuteContent {
    pub harmon_closed: HarmonClosed,
}
#[derive(Debug)]
pub struct Harmonic {
    pub print_object: ::core::option::Option<YesNo>,
    pub default_x: ::core::option::Option<::core::primitive::f64>,
    pub default_y: ::core::option::Option<::core::primitive::f64>,
    pub relative_x: ::core::option::Option<::core::primitive::f64>,
    pub relative_y: ::core::option::Option<::core::primitive::f64>,
    pub font_family: ::core::option::Option<::std::string::String>,
    pub font_style: ::core::option::Option<FontStyle>,
    pub font_size: ::core::option::Option<FontSize>,
    pub font_weight: ::core::option::Option<FontWeight>,
    pub color: ::core::option::Option<::std::string::String>,
    pub placement: ::core::option::Option<AboveBelow>,
    pub content: ::std::vec::Vec<HarmonicContent>,
}
#[derive(Debug)]
pub enum HarmonicContent {
    Natural(Empty),
    Artificial(Empty),
    BasePitch(Empty),
    TouchingPitch(Empty),
    SoundingPitch(Empty),
}
#[derive(Debug)]
pub struct Harmony {
    pub type_: ::core::option::Option<HarmonyType>,
    pub print_object: ::core::option::Option<YesNo>,
    pub print_frame: ::core::option::Option<YesNo>,
    pub arrangement: ::core::option::Option<HarmonyArrangement>,
    pub default_x: ::core::option::Option<::core::primitive::f64>,
    pub default_y: ::core::option::Option<::core::primitive::f64>,
    pub relative_x: ::core::option::Option<::core::primitive::f64>,
    pub relative_y: ::core::option::Option<::core::primitive::f64>,
    pub font_family: ::core::option::Option<::std::string::String>,
    pub font_style: ::core::option::Option<FontStyle>,
    pub font_size: ::core::option::Option<FontSize>,
    pub font_weight: ::core::option::Option<FontWeight>,
    pub color: ::core::option::Option<::std::string::String>,
    pub placement: ::core::option::Option<AboveBelow>,
    pub system: ::core::option::Option<SystemRelation>,
    pub id: ::core::option::Option<::std::string::String>,
    pub content: ::std::vec::Vec<HarmonyContent>,
}
#[derive(Debug)]
pub enum HarmonyContent {
    Root(Root),
    Numeral(Numeral),
    Function(StyleText),
    Kind(Kind),
    Inversion(Inversion),
    Bass(Bass),
    Degree(Degree),
    Frame(Frame),
    Offset(Offset),
    Footnote(FormattedText),
    Level(Level),
    Staff(::num::BigUint),
}
#[derive(Debug)]
pub struct HarmonyAlter {
    pub print_object: ::core::option::Option<YesNo>,
    pub default_x: ::core::option::Option<::core::primitive::f64>,
    pub default_y: ::core::option::Option<::core::primitive::f64>,
    pub relative_x: ::core::option::Option<::core::primitive::f64>,
    pub relative_y: ::core::option::Option<::core::primitive::f64>,
    pub font_family: ::core::option::Option<::std::string::String>,
    pub font_style: ::core::option::Option<FontStyle>,
    pub font_size: ::core::option::Option<FontSize>,
    pub font_weight: ::core::option::Option<FontWeight>,
    pub color: ::core::option::Option<::std::string::String>,
    pub location: ::core::option::Option<LeftRight>,
    pub content: ::core::primitive::f64,
}
#[derive(Debug)]
pub enum HarmonyArrangement {
    Vertical,
    Horizontal,
    Diagonal,
}
#[derive(Debug)]
pub enum HarmonyType {
    Explicit,
    Implied,
    Alternate,
}
#[derive(Debug)]
pub struct HarpPedals {
    pub default_x: ::core::option::Option<::core::primitive::f64>,
    pub default_y: ::core::option::Option<::core::primitive::f64>,
    pub relative_x: ::core::option::Option<::core::primitive::f64>,
    pub relative_y: ::core::option::Option<::core::primitive::f64>,
    pub font_family: ::core::option::Option<::std::string::String>,
    pub font_style: ::core::option::Option<FontStyle>,
    pub font_size: ::core::option::Option<FontSize>,
    pub font_weight: ::core::option::Option<FontWeight>,
    pub color: ::core::option::Option<::std::string::String>,
    pub halign: ::core::option::Option<LeftCenterRight>,
    pub valign: ::core::option::Option<Valign>,
    pub id: ::core::option::Option<::std::string::String>,
    pub content: HarpPedalsContent,
}
#[derive(Debug)]
pub struct HarpPedalsContent {
    pub pedal_tuning: ::std::vec::Vec<PedalTuning>,
}
#[derive(Debug)]
pub struct HeelToe {
    pub default_x: ::core::option::Option<::core::primitive::f64>,
    pub default_y: ::core::option::Option<::core::primitive::f64>,
    pub relative_x: ::core::option::Option<::core::primitive::f64>,
    pub relative_y: ::core::option::Option<::core::primitive::f64>,
    pub font_family: ::core::option::Option<::std::string::String>,
    pub font_style: ::core::option::Option<FontStyle>,
    pub font_size: ::core::option::Option<FontSize>,
    pub font_weight: ::core::option::Option<FontWeight>,
    pub color: ::core::option::Option<::std::string::String>,
    pub placement: ::core::option::Option<AboveBelow>,
    pub substitution: ::core::option::Option<YesNo>,
}
#[derive(Debug)]
pub struct Hole {
    pub default_x: ::core::option::Option<::core::primitive::f64>,
    pub default_y: ::core::option::Option<::core::primitive::f64>,
    pub relative_x: ::core::option::Option<::core::primitive::f64>,
    pub relative_y: ::core::option::Option<::core::primitive::f64>,
    pub font_family: ::core::option::Option<::std::string::String>,
    pub font_style: ::core::option::Option<FontStyle>,
    pub font_size: ::core::option::Option<FontSize>,
    pub font_weight: ::core::option::Option<FontWeight>,
    pub color: ::core::option::Option<::std::string::String>,
    pub placement: ::core::option::Option<AboveBelow>,
    pub content: HoleContent,
}
#[derive(Debug)]
pub struct HoleContent {
    pub hole_type: ::core::option::Option<::std::string::String>,
    pub hole_closed: HoleClosed,
    pub hole_shape: ::core::option::Option<::std::string::String>,
}
#[derive(Debug)]
pub struct HoleClosed {
    pub location: ::core::option::Option<HoleClosedLocation>,
    pub content: HoleClosedValue,
}
#[derive(Debug)]
pub enum HoleClosedLocation {
    Right,
    Bottom,
    Left,
    Top,
}
#[derive(Debug)]
pub enum HoleClosedValue {
    Yes,
    No,
    Half,
}
#[derive(Debug)]
pub struct HorizontalTurn {
    pub default_x: ::core::option::Option<::core::primitive::f64>,
    pub default_y: ::core::option::Option<::core::primitive::f64>,
    pub relative_x: ::core::option::Option<::core::primitive::f64>,
    pub relative_y: ::core::option::Option<::core::primitive::f64>,
    pub font_family: ::core::option::Option<::std::string::String>,
    pub font_style: ::core::option::Option<FontStyle>,
    pub font_size: ::core::option::Option<FontSize>,
    pub font_weight: ::core::option::Option<FontWeight>,
    pub color: ::core::option::Option<::std::string::String>,
    pub placement: ::core::option::Option<AboveBelow>,
    pub start_note: ::core::option::Option<StartNote>,
    pub trill_step: ::core::option::Option<TrillStep>,
    pub two_note_turn: ::core::option::Option<TwoNoteTurn>,
    pub accelerate: ::core::option::Option<YesNo>,
    pub beats: ::core::option::Option<::core::primitive::f64>,
    pub second_beat: ::core::option::Option<::core::primitive::f64>,
    pub last_beat: ::core::option::Option<::core::primitive::f64>,
    pub slash: ::core::option::Option<YesNo>,
}
#[derive(Debug)]
pub struct Identification {
    pub content: IdentificationContent,
}
#[derive(Debug)]
pub struct IdentificationContent {
    pub creator: ::std::vec::Vec<TypedText>,
    pub rights: ::std::vec::Vec<TypedText>,
    pub encoding: ::core::option::Option<Encoding>,
    pub source: ::core::option::Option<::std::string::String>,
    pub relation: ::std::vec::Vec<TypedText>,
    pub miscellaneous: ::core::option::Option<Miscellaneous>,
}
#[derive(Debug)]
pub struct Image {
    pub source: ::std::string::String,
    pub type_: ::std::string::String,
    pub height: ::core::option::Option<::core::primitive::f64>,
    pub width: ::core::option::Option<::core::primitive::f64>,
    pub default_x: ::core::option::Option<::core::primitive::f64>,
    pub default_y: ::core::option::Option<::core::primitive::f64>,
    pub relative_x: ::core::option::Option<::core::primitive::f64>,
    pub relative_y: ::core::option::Option<::core::primitive::f64>,
    pub halign: ::core::option::Option<LeftCenterRight>,
    pub valign: ::core::option::Option<ValignImage>,
    pub id: ::core::option::Option<::std::string::String>,
}
#[derive(Debug)]
pub struct Instrument {
    pub id: ::std::string::String,
}
#[derive(Debug)]
pub struct InstrumentChange {
    pub id: ::std::string::String,
    pub content: ::std::vec::Vec<InstrumentChangeContent>,
}
#[derive(Debug)]
pub enum InstrumentChangeContent {
    InstrumentSound(::std::string::String),
    Solo(Empty),
    Ensemble(PositiveIntegerOrEmpty),
    VirtualInstrument(VirtualInstrument),
}
#[derive(Debug)]
pub struct InstrumentLink {
    pub id: ::std::string::String,
}
#[derive(Debug)]
pub struct Interchangeable {
    pub symbol: ::core::option::Option<TimeSymbol>,
    pub separator: ::core::option::Option<TimeSeparator>,
    pub content: ::std::vec::Vec<InterchangeableContent>,
}
#[derive(Debug)]
pub enum InterchangeableContent {
    TimeRelation(TimeRelation),
    Beats(::std::string::String),
    BeatType(::std::string::String),
}
#[derive(Debug)]
pub struct Inversion {
    pub text: ::core::option::Option<::std::string::String>,
    pub default_x: ::core::option::Option<::core::primitive::f64>,
    pub default_y: ::core::option::Option<::core::primitive::f64>,
    pub relative_x: ::core::option::Option<::core::primitive::f64>,
    pub relative_y: ::core::option::Option<::core::primitive::f64>,
    pub font_family: ::core::option::Option<::std::string::String>,
    pub font_style: ::core::option::Option<FontStyle>,
    pub font_size: ::core::option::Option<FontSize>,
    pub font_weight: ::core::option::Option<FontWeight>,
    pub color: ::core::option::Option<::std::string::String>,
    pub content: ::num::BigUint,
}
#[derive(Debug)]
pub struct Key {
    pub number: ::core::option::Option<::num::BigUint>,
    pub default_x: ::core::option::Option<::core::primitive::f64>,
    pub default_y: ::core::option::Option<::core::primitive::f64>,
    pub relative_x: ::core::option::Option<::core::primitive::f64>,
    pub relative_y: ::core::option::Option<::core::primitive::f64>,
    pub font_family: ::core::option::Option<::std::string::String>,
    pub font_style: ::core::option::Option<FontStyle>,
    pub font_size: ::core::option::Option<FontSize>,
    pub font_weight: ::core::option::Option<FontWeight>,
    pub color: ::core::option::Option<::std::string::String>,
    pub print_object: ::core::option::Option<YesNo>,
    pub id: ::core::option::Option<::std::string::String>,
    pub content: ::std::vec::Vec<KeyContent>,
}
#[derive(Debug)]
pub enum KeyContent {
    Cancel(Cancel),
    Fifths(::num::BigInt),
    Mode(::std::string::String),
    KeyStep(Step),
    KeyAlter(::core::primitive::f64),
    KeyAccidental(KeyAccidental),
    KeyOctave(KeyOctave),
}
#[derive(Debug)]
pub struct KeyAccidental {
    pub smufl: ::core::option::Option<::std::string::String>,
    pub content: AccidentalValue,
}
#[derive(Debug)]
pub struct KeyOctave {
    pub number: ::num::BigUint,
    pub cancel: ::core::option::Option<YesNo>,
    pub content: ::num::BigInt,
}
#[derive(Debug)]
pub struct Kind {
    pub use_symbols: ::core::option::Option<YesNo>,
    pub text: ::core::option::Option<::std::string::String>,
    pub stack_degrees: ::core::option::Option<YesNo>,
    pub parentheses_degrees: ::core::option::Option<YesNo>,
    pub bracket_degrees: ::core::option::Option<YesNo>,
    pub default_x: ::core::option::Option<::core::primitive::f64>,
    pub default_y: ::core::option::Option<::core::primitive::f64>,
    pub relative_x: ::core::option::Option<::core::primitive::f64>,
    pub relative_y: ::core::option::Option<::core::primitive::f64>,
    pub font_family: ::core::option::Option<::std::string::String>,
    pub font_style: ::core::option::Option<FontStyle>,
    pub font_size: ::core::option::Option<FontSize>,
    pub font_weight: ::core::option::Option<FontWeight>,
    pub color: ::core::option::Option<::std::string::String>,
    pub halign: ::core::option::Option<LeftCenterRight>,
    pub valign: ::core::option::Option<Valign>,
    pub content: KindValue,
}
#[derive(Debug)]
pub enum KindValue {
    Major,
    Minor,
    Augmented,
    Diminished,
    Dominant,
    MajorSeventh,
    MinorSeventh,
    DiminishedSeventh,
    AugmentedSeventh,
    HalfDiminished,
    MajorMinor,
    MajorSixth,
    MinorSixth,
    DominantNinth,
    MajorNinth,
    MinorNinth,
    Dominant11Th,
    Major11Th,
    Minor11Th,
    Dominant13Th,
    Major13Th,
    Minor13Th,
    SuspendedSecond,
    SuspendedFourth,
    Neapolitan,
    Italian,
    French,
    German,
    Pedal,
    Power,
    Tristan,
    Other,
    None,
}
#[derive(Debug)]
pub enum LeftCenterRight {
    Left,
    Center,
    Right,
}
#[derive(Debug)]
pub enum LeftRight {
    Left,
    Right,
}
#[derive(Debug)]
pub struct Level {
    pub reference: ::core::option::Option<YesNo>,
    pub type_: ::core::option::Option<StartStopSingle>,
    pub parentheses: ::core::option::Option<YesNo>,
    pub bracket: ::core::option::Option<YesNo>,
    pub size: ::core::option::Option<SymbolSize>,
    pub content: ::std::string::String,
}
#[derive(Debug)]
pub struct LineDetail {
    pub line: ::num::BigUint,
    pub width: ::core::option::Option<::core::primitive::f64>,
    pub color: ::core::option::Option<::std::string::String>,
    pub line_type: ::core::option::Option<LineType>,
    pub print_object: ::core::option::Option<YesNo>,
}
#[derive(Debug)]
pub enum LineEnd {
    Up,
    Down,
    Both,
    Arrow,
    None,
}
#[derive(Debug)]
pub enum LineLength {
    Short,
    Medium,
    Long,
}
#[derive(Debug)]
pub enum LineShape {
    Straight,
    Curved,
}
#[derive(Debug)]
pub enum LineType {
    Solid,
    Dashed,
    Dotted,
    Wavy,
}
#[derive(Debug)]
pub struct LineWidth {
    pub type_: ::std::string::String,
    pub content: ::core::primitive::f64,
}
pub type LineWidthType = ::std::string::String;
#[derive(Debug)]
pub struct Link {
    pub href: ::std::string::String,
    pub type_: ::core::option::Option<xlink::Type>,
    pub role: ::core::option::Option<::std::string::String>,
    pub title: ::core::option::Option<::std::string::String>,
    pub show: xlink::Show,
    pub actuate: xlink::Actuate,
    pub name: ::core::option::Option<::std::string::String>,
    pub element: ::core::option::Option<::std::string::String>,
    pub position: ::core::option::Option<::num::BigUint>,
    pub default_x: ::core::option::Option<::core::primitive::f64>,
    pub default_y: ::core::option::Option<::core::primitive::f64>,
    pub relative_x: ::core::option::Option<::core::primitive::f64>,
    pub relative_y: ::core::option::Option<::core::primitive::f64>,
}
#[derive(Debug)]
pub struct Listen {
    pub content: ::std::vec::Vec<ListenContent>,
}
#[derive(Debug)]
pub enum ListenContent {
    Assess(Assess),
    Wait(Wait),
    OtherListen(OtherListening),
}
#[derive(Debug)]
pub struct Listening {
    pub content: ::std::vec::Vec<ListeningContent>,
}
#[derive(Debug)]
pub enum ListeningContent {
    Sync(Sync),
    OtherListening(OtherListening),
    Offset(Offset),
}
#[derive(Debug)]
pub struct Lyric {
    pub number: ::core::option::Option<::std::string::String>,
    pub name: ::core::option::Option<::std::string::String>,
    pub justify: ::core::option::Option<LeftCenterRight>,
    pub default_x: ::core::option::Option<::core::primitive::f64>,
    pub default_y: ::core::option::Option<::core::primitive::f64>,
    pub relative_x: ::core::option::Option<::core::primitive::f64>,
    pub relative_y: ::core::option::Option<::core::primitive::f64>,
    pub placement: ::core::option::Option<AboveBelow>,
    pub color: ::core::option::Option<::std::string::String>,
    pub print_object: ::core::option::Option<YesNo>,
    pub time_only: ::core::option::Option<::std::string::String>,
    pub id: ::core::option::Option<::std::string::String>,
    pub content: ::std::vec::Vec<LyricContent>,
}
#[derive(Debug)]
pub enum LyricContent {
    Syllabic(Syllabic),
    Text(TextElementData),
    Elision(Elision),
    Extend(Extend),
    Laughing(Empty),
    Humming(Empty),
    EndLine(Empty),
    EndParagraph(Empty),
    Footnote(FormattedText),
    Level(Level),
}
#[derive(Debug)]
pub struct LyricFont {
    pub number: ::core::option::Option<::std::string::String>,
    pub name: ::core::option::Option<::std::string::String>,
    pub font_family: ::core::option::Option<::std::string::String>,
    pub font_style: ::core::option::Option<FontStyle>,
    pub font_size: ::core::option::Option<FontSize>,
    pub font_weight: ::core::option::Option<FontWeight>,
}
#[derive(Debug)]
pub struct LyricLanguage {
    pub number: ::core::option::Option<::std::string::String>,
    pub name: ::core::option::Option<::std::string::String>,
    pub lang: ::std::string::String,
}
#[derive(Debug)]
pub enum MarginType {
    Odd,
    Even,
    Both,
}
#[derive(Debug)]
pub struct MeasureLayout {
    pub content: MeasureLayoutContent,
}
#[derive(Debug)]
pub struct MeasureLayoutContent {
    pub measure_distance: ::core::option::Option<::core::primitive::f64>,
}
#[derive(Debug)]
pub struct MeasureNumbering {
    pub system: ::core::option::Option<SystemRelationNumber>,
    pub staff: ::core::option::Option<::num::BigUint>,
    pub multiple_rest_always: ::core::option::Option<YesNo>,
    pub multiple_rest_range: ::core::option::Option<YesNo>,
    pub default_x: ::core::option::Option<::core::primitive::f64>,
    pub default_y: ::core::option::Option<::core::primitive::f64>,
    pub relative_x: ::core::option::Option<::core::primitive::f64>,
    pub relative_y: ::core::option::Option<::core::primitive::f64>,
    pub font_family: ::core::option::Option<::std::string::String>,
    pub font_style: ::core::option::Option<FontStyle>,
    pub font_size: ::core::option::Option<FontSize>,
    pub font_weight: ::core::option::Option<FontWeight>,
    pub color: ::core::option::Option<::std::string::String>,
    pub halign: ::core::option::Option<LeftCenterRight>,
    pub valign: ::core::option::Option<Valign>,
    pub content: MeasureNumberingValue,
}
#[derive(Debug)]
pub enum MeasureNumberingValue {
    None,
    Measure,
    System,
}
#[derive(Debug)]
pub struct MeasureRepeat {
    pub type_: StartStop,
    pub slashes: ::core::option::Option<::num::BigUint>,
    pub content: PositiveIntegerOrEmpty,
}
#[derive(Debug)]
pub struct MeasureStyle {
    pub number: ::core::option::Option<::num::BigUint>,
    pub font_family: ::core::option::Option<::std::string::String>,
    pub font_style: ::core::option::Option<FontStyle>,
    pub font_size: ::core::option::Option<FontSize>,
    pub font_weight: ::core::option::Option<FontWeight>,
    pub color: ::core::option::Option<::std::string::String>,
    pub id: ::core::option::Option<::std::string::String>,
    pub content: MeasureStyleContent,
}
#[derive(Debug)]
pub enum MeasureStyleContent {
    MultipleRest(MultipleRest),
    MeasureRepeat(MeasureRepeat),
    BeatRepeat(BeatRepeat),
    Slash(Slash),
}
pub type MeasureText = ::std::string::String;
#[derive(Debug)]
pub struct Membrane {
    pub smufl: ::core::option::Option<::std::string::String>,
    pub content: MembraneValue,
}
#[derive(Debug)]
pub enum MembraneValue {
    BassDrum,
    BassDrumOnSide,
    Bongos,
    ChineseTomtom,
    CongaDrum,
    Cuica,
    GobletDrum,
    IndoAmericanTomtom,
    JapaneseTomtom,
    MilitaryDrum,
    SnareDrum,
    SnareDrumSnaresOff,
    Tabla,
    Tambourine,
    TenorDrum,
    Timbales,
    Tomtom,
}
#[derive(Debug)]
pub struct Metal {
    pub smufl: ::core::option::Option<::std::string::String>,
    pub content: MetalValue,
}
#[derive(Debug)]
pub enum MetalValue {
    Agogo,
    Almglocken,
    Bell,
    BellPlate,
    BellTree,
    BrakeDrum,
    Cencerro,
    ChainRattle,
    ChineseCymbal,
    Cowbell,
    CrashCymbals,
    Crotale,
    CymbalTongs,
    DomedGong,
    FingerCymbals,
    Flexatone,
    Gong,
    HiHat,
    HighHatCymbals,
    Handbell,
    JawHarp,
    JingleBells,
    MusicalSaw,
    ShellBells,
    Sistrum,
    SizzleCymbal,
    SleighBells,
    SuspendedCymbal,
    TamTam,
    TamTamWithBeater,
    Triangle,
    VietnameseHat,
}
#[derive(Debug)]
pub struct Metronome {
    pub default_x: ::core::option::Option<::core::primitive::f64>,
    pub default_y: ::core::option::Option<::core::primitive::f64>,
    pub relative_x: ::core::option::Option<::core::primitive::f64>,
    pub relative_y: ::core::option::Option<::core::primitive::f64>,
    pub font_family: ::core::option::Option<::std::string::String>,
    pub font_style: ::core::option::Option<FontStyle>,
    pub font_size: ::core::option::Option<FontSize>,
    pub font_weight: ::core::option::Option<FontWeight>,
    pub color: ::core::option::Option<::std::string::String>,
    pub halign: ::core::option::Option<LeftCenterRight>,
    pub valign: ::core::option::Option<Valign>,
    pub print_object: ::core::option::Option<YesNo>,
    pub justify: ::core::option::Option<LeftCenterRight>,
    pub parentheses: ::core::option::Option<YesNo>,
    pub id: ::core::option::Option<::std::string::String>,
    pub content: ::std::vec::Vec<MetronomeContent>,
}
#[derive(Debug)]
pub enum MetronomeContent {
    BeatUnit(NoteTypeValue),
    BeatUnitDot(Empty),
    BeatUnitTied(BeatUnitTied),
    PerMinute(PerMinute),
    MetronomeArrows(Empty),
    MetronomeNote(MetronomeNote),
    MetronomeRelation(::std::string::String),
}
#[derive(Debug)]
pub struct MetronomeBeam {
    pub number: ::num::BigUint,
    pub content: BeamValue,
}
#[derive(Debug)]
pub struct MetronomeNote {
    pub content: MetronomeNoteContent,
}
#[derive(Debug)]
pub struct MetronomeNoteContent {
    pub metronome_type: NoteTypeValue,
    pub metronome_dot: ::std::vec::Vec<Empty>,
    pub metronome_beam: ::std::vec::Vec<MetronomeBeam>,
    pub metronome_tied: ::core::option::Option<MetronomeTied>,
    pub metronome_tuplet: ::core::option::Option<MetronomeTuplet>,
}
#[derive(Debug)]
pub struct MetronomeTied {
    pub type_: StartStop,
}
#[derive(Debug)]
pub struct MetronomeTuplet {
    pub type_: StartStop,
    pub bracket: ::core::option::Option<YesNo>,
    pub show_number: ::core::option::Option<ShowTuplet>,
    pub content: MetronomeTupletContent,
}
#[derive(Debug)]
pub struct MetronomeTupletContent {
    pub actual_notes: ::num::BigUint,
    pub normal_notes: ::num::BigUint,
    pub normal_type: ::core::option::Option<NoteTypeValue>,
    pub normal_dot: ::std::vec::Vec<Empty>,
}
pub type Midi128 = ::num::BigUint;
pub type Midi16 = ::num::BigUint;
pub type Midi16384 = ::num::BigUint;
#[derive(Debug)]
pub struct MidiDevice {
    pub port: ::core::option::Option<::num::BigUint>,
    pub id: ::core::option::Option<::std::string::String>,
    pub content: ::std::string::String,
}
#[derive(Debug)]
pub struct MidiInstrument {
    pub id: ::std::string::String,
    pub content: MidiInstrumentContent,
}
#[derive(Debug)]
pub struct MidiInstrumentContent {
    pub midi_channel: ::core::option::Option<::num::BigUint>,
    pub midi_name: ::core::option::Option<::std::string::String>,
    pub midi_bank: ::core::option::Option<::num::BigUint>,
    pub midi_program: ::core::option::Option<::num::BigUint>,
    pub midi_unpitched: ::core::option::Option<::num::BigUint>,
    pub volume: ::core::option::Option<::core::primitive::f64>,
    pub pan: ::core::option::Option<::core::primitive::f64>,
    pub elevation: ::core::option::Option<::core::primitive::f64>,
}
pub type Millimeters = ::core::primitive::f64;
pub type Milliseconds = ::num::BigUint;
#[derive(Debug)]
pub struct Miscellaneous {
    pub content: MiscellaneousContent,
}
#[derive(Debug)]
pub struct MiscellaneousContent {
    pub miscellaneous_field: ::std::vec::Vec<MiscellaneousField>,
}
#[derive(Debug)]
pub struct MiscellaneousField {
    pub name: ::std::string::String,
    pub content: ::std::string::String,
}
pub type Mode = ::std::string::String;
#[derive(Debug)]
pub struct Mordent {
    pub default_x: ::core::option::Option<::core::primitive::f64>,
    pub default_y: ::core::option::Option<::core::primitive::f64>,
    pub relative_x: ::core::option::Option<::core::primitive::f64>,
    pub relative_y: ::core::option::Option<::core::primitive::f64>,
    pub font_family: ::core::option::Option<::std::string::String>,
    pub font_style: ::core::option::Option<FontStyle>,
    pub font_size: ::core::option::Option<FontSize>,
    pub font_weight: ::core::option::Option<FontWeight>,
    pub color: ::core::option::Option<::std::string::String>,
    pub placement: ::core::option::Option<AboveBelow>,
    pub start_note: ::core::option::Option<StartNote>,
    pub trill_step: ::core::option::Option<TrillStep>,
    pub two_note_turn: ::core::option::Option<TwoNoteTurn>,
    pub accelerate: ::core::option::Option<YesNo>,
    pub beats: ::core::option::Option<::core::primitive::f64>,
    pub second_beat: ::core::option::Option<::core::primitive::f64>,
    pub last_beat: ::core::option::Option<::core::primitive::f64>,
    pub long: ::core::option::Option<YesNo>,
    pub approach: ::core::option::Option<AboveBelow>,
    pub departure: ::core::option::Option<AboveBelow>,
}
#[derive(Debug)]
pub struct MultipleRest {
    pub use_symbols: ::core::option::Option<YesNo>,
    pub content: ::num::BigUint,
}
#[derive(Debug)]
pub enum Mute {
    On,
    Off,
    Straight,
    Cup,
    HarmonNoStem,
    HarmonStem,
    Bucket,
    Plunger,
    Hat,
    Solotone,
    Practice,
    StopMute,
    StopHand,
    Echo,
    Palm,
}
#[derive(Debug)]
pub struct NameDisplay {
    pub print_object: ::core::option::Option<YesNo>,
    pub content: ::std::vec::Vec<NameDisplayContent>,
}
#[derive(Debug)]
pub enum NameDisplayContent {
    DisplayText(FormattedText),
    AccidentalText(AccidentalText),
}
#[derive(Debug)]
pub struct NonArpeggiate {
    pub type_: TopBottom,
    pub number: ::core::option::Option<::num::BigUint>,
    pub default_x: ::core::option::Option<::core::primitive::f64>,
    pub default_y: ::core::option::Option<::core::primitive::f64>,
    pub relative_x: ::core::option::Option<::core::primitive::f64>,
    pub relative_y: ::core::option::Option<::core::primitive::f64>,
    pub placement: ::core::option::Option<AboveBelow>,
    pub color: ::core::option::Option<::std::string::String>,
    pub id: ::core::option::Option<::std::string::String>,
}
pub type NonNegativeDecimal = ::core::primitive::f64;
#[derive(Debug)]
pub struct Notations {
    pub print_object: ::core::option::Option<YesNo>,
    pub id: ::core::option::Option<::std::string::String>,
    pub content: ::std::vec::Vec<NotationsContent>,
}
#[derive(Debug)]
pub enum NotationsContent {
    Footnote(FormattedText),
    Level(Level),
    Tied(Tied),
    Slur(Slur),
    Tuplet(Tuplet),
    Glissando(Glissando),
    Slide(Slide),
    Ornaments(Ornaments),
    Technical(Technical),
    Articulations(Articulations),
    Dynamics(Dynamics),
    Fermata(Fermata),
    Arpeggiate(Arpeggiate),
    NonArpeggiate(NonArpeggiate),
    AccidentalMark(AccidentalMark),
    OtherNotation(OtherNotation),
}
#[derive(Debug)]
pub struct Note {
    pub default_x: ::core::option::Option<::core::primitive::f64>,
    pub default_y: ::core::option::Option<::core::primitive::f64>,
    pub relative_x: ::core::option::Option<::core::primitive::f64>,
    pub relative_y: ::core::option::Option<::core::primitive::f64>,
    pub font_family: ::core::option::Option<::std::string::String>,
    pub font_style: ::core::option::Option<FontStyle>,
    pub font_size: ::core::option::Option<FontSize>,
    pub font_weight: ::core::option::Option<FontWeight>,
    pub color: ::core::option::Option<::std::string::String>,
    pub print_object: ::core::option::Option<YesNo>,
    pub print_dot: ::core::option::Option<YesNo>,
    pub print_spacing: ::core::option::Option<YesNo>,
    pub print_lyric: ::core::option::Option<YesNo>,
    pub print_leger: ::core::option::Option<YesNo>,
    pub dynamics: ::core::option::Option<::core::primitive::f64>,
    pub end_dynamics: ::core::option::Option<::core::primitive::f64>,
    pub attack: ::core::option::Option<::core::primitive::f64>,
    pub release: ::core::option::Option<::core::primitive::f64>,
    pub time_only: ::core::option::Option<::std::string::String>,
    pub pizzicato: ::core::option::Option<YesNo>,
    pub id: ::core::option::Option<::std::string::String>,
    pub content: ::std::vec::Vec<NoteContent>,
}
#[derive(Debug)]
pub enum NoteContent {
    Grace(Grace),
    Chord(Empty),
    Pitch(Pitch),
    Unpitched(Unpitched),
    Rest(Rest),
    Tie(Tie),
    Cue(Empty),
    Duration(::core::primitive::f64),
    Instrument(Instrument),
    Footnote(FormattedText),
    Level(Level),
    Voice(::std::string::String),
    Type(NoteType),
    Dot(EmptyPlacement),
    Accidental(Accidental),
    TimeModification(TimeModification),
    Stem(Stem),
    Notehead(Notehead),
    NoteheadText(NoteheadText),
    Staff(::num::BigUint),
    Beam(Beam),
    Notations(Notations),
    Lyric(Lyric),
    Play(Play),
    Listen(Listen),
}
#[derive(Debug)]
pub struct NoteSize {
    pub type_: NoteSizeType,
    pub content: ::core::primitive::f64,
}
#[derive(Debug)]
pub enum NoteSizeType {
    Cue,
    Grace,
    GraceCue,
    Large,
}
#[derive(Debug)]
pub struct NoteType {
    pub size: ::core::option::Option<SymbolSize>,
    pub content: NoteTypeValue,
}
#[derive(Debug)]
pub enum NoteTypeValue {
    _1024Th,
    _512Th,
    _256Th,
    _128Th,
    _64Th,
    _32Nd,
    _16Th,
    Eighth,
    Quarter,
    Half,
    Whole,
    Breve,
    Long,
    Maxima,
}
#[derive(Debug)]
pub struct Notehead {
    pub filled: ::core::option::Option<YesNo>,
    pub parentheses: ::core::option::Option<YesNo>,
    pub font_family: ::core::option::Option<::std::string::String>,
    pub font_style: ::core::option::Option<FontStyle>,
    pub font_size: ::core::option::Option<FontSize>,
    pub font_weight: ::core::option::Option<FontWeight>,
    pub color: ::core::option::Option<::std::string::String>,
    pub smufl: ::core::option::Option<::std::string::String>,
    pub content: NoteheadValue,
}
#[derive(Debug)]
pub struct NoteheadText {
    pub content: ::std::vec::Vec<NoteheadTextContent>,
}
#[derive(Debug)]
pub enum NoteheadTextContent {
    DisplayText(FormattedText),
    AccidentalText(AccidentalText),
}
#[derive(Debug)]
pub enum NoteheadValue {
    Slash,
    Triangle,
    Diamond,
    Square,
    Cross,
    X,
    CircleX,
    InvertedTriangle,
    ArrowDown,
    ArrowUp,
    Circled,
    Slashed,
    BackSlashed,
    Normal,
    Cluster,
    CircleDot,
    LeftTriangle,
    Rectangle,
    None,
    Do,
    Re,
    Mi,
    Fa,
    FaUp,
    So,
    La,
    Ti,
    Other,
}
pub type NumberLevel = ::num::BigUint;
pub type NumberOfLines = ::num::BigUint;
#[derive(Debug)]
pub enum NumberOrNormal {
    F64(::core::primitive::f64),
    Normal,
}
#[derive(Debug)]
pub struct Numeral {
    pub content: NumeralContent,
}
#[derive(Debug)]
pub struct NumeralContent {
    pub numeral_root: NumeralRoot,
    pub numeral_alter: ::core::option::Option<HarmonyAlter>,
    pub numeral_key: ::core::option::Option<NumeralKey>,
}
#[derive(Debug)]
pub struct NumeralKey {
    pub print_object: ::core::option::Option<YesNo>,
    pub content: NumeralKeyContent,
}
#[derive(Debug)]
pub struct NumeralKeyContent {
    pub numeral_fifths: ::num::BigInt,
    pub numeral_mode: NumeralMode,
}
#[derive(Debug)]
pub enum NumeralMode {
    Major,
    Minor,
    NaturalMinor,
    MelodicMinor,
    HarmonicMinor,
}
#[derive(Debug)]
pub struct NumeralRoot {
    pub text: ::core::option::Option<::std::string::String>,
    pub default_x: ::core::option::Option<::core::primitive::f64>,
    pub default_y: ::core::option::Option<::core::primitive::f64>,
    pub relative_x: ::core::option::Option<::core::primitive::f64>,
    pub relative_y: ::core::option::Option<::core::primitive::f64>,
    pub font_family: ::core::option::Option<::std::string::String>,
    pub font_style: ::core::option::Option<FontStyle>,
    pub font_size: ::core::option::Option<FontSize>,
    pub font_weight: ::core::option::Option<FontWeight>,
    pub color: ::core::option::Option<::std::string::String>,
    pub content: ::num::BigUint,
}
pub type NumeralValue = ::num::BigUint;
pub type Octave = ::num::BigInt;
#[derive(Debug)]
pub struct OctaveShift {
    pub type_: UpDownStopContinue,
    pub number: ::core::option::Option<::num::BigUint>,
    pub size: ::num::BigUint,
    pub dash_length: ::core::option::Option<::core::primitive::f64>,
    pub space_length: ::core::option::Option<::core::primitive::f64>,
    pub default_x: ::core::option::Option<::core::primitive::f64>,
    pub default_y: ::core::option::Option<::core::primitive::f64>,
    pub relative_x: ::core::option::Option<::core::primitive::f64>,
    pub relative_y: ::core::option::Option<::core::primitive::f64>,
    pub font_family: ::core::option::Option<::std::string::String>,
    pub font_style: ::core::option::Option<FontStyle>,
    pub font_size: ::core::option::Option<FontSize>,
    pub font_weight: ::core::option::Option<FontWeight>,
    pub color: ::core::option::Option<::std::string::String>,
    pub id: ::core::option::Option<::std::string::String>,
}
#[derive(Debug)]
pub struct Offset {
    pub sound: ::core::option::Option<YesNo>,
    pub content: ::core::primitive::f64,
}
#[derive(Debug)]
pub enum OnOff {
    On,
    Off,
}
#[derive(Debug)]
pub struct Opus {
    pub href: ::std::string::String,
    pub type_: ::core::option::Option<xlink::Type>,
    pub role: ::core::option::Option<::std::string::String>,
    pub title: ::core::option::Option<::std::string::String>,
    pub show: xlink::Show,
    pub actuate: xlink::Actuate,
}
#[derive(Debug)]
pub struct Ornaments {
    pub id: ::core::option::Option<::std::string::String>,
    pub content: ::std::vec::Vec<OrnamentsContent>,
}
#[derive(Debug)]
pub enum OrnamentsContent {
    TrillMark(EmptyTrillSound),
    Turn(HorizontalTurn),
    DelayedTurn(HorizontalTurn),
    InvertedTurn(HorizontalTurn),
    DelayedInvertedTurn(HorizontalTurn),
    VerticalTurn(EmptyTrillSound),
    InvertedVerticalTurn(EmptyTrillSound),
    Shake(EmptyTrillSound),
    WavyLine(WavyLine),
    Mordent(Mordent),
    InvertedMordent(Mordent),
    Schleifer(EmptyPlacement),
    Tremolo(Tremolo),
    Haydn(EmptyTrillSound),
    OtherOrnament(OtherPlacementText),
    AccidentalMark(AccidentalMark),
}
#[derive(Debug)]
pub struct OtherAppearance {
    pub type_: ::std::string::String,
    pub content: ::std::string::String,
}
#[derive(Debug)]
pub struct OtherDirection {
    pub print_object: ::core::option::Option<YesNo>,
    pub default_x: ::core::option::Option<::core::primitive::f64>,
    pub default_y: ::core::option::Option<::core::primitive::f64>,
    pub relative_x: ::core::option::Option<::core::primitive::f64>,
    pub relative_y: ::core::option::Option<::core::primitive::f64>,
    pub font_family: ::core::option::Option<::std::string::String>,
    pub font_style: ::core::option::Option<FontStyle>,
    pub font_size: ::core::option::Option<FontSize>,
    pub font_weight: ::core::option::Option<FontWeight>,
    pub color: ::core::option::Option<::std::string::String>,
    pub halign: ::core::option::Option<LeftCenterRight>,
    pub valign: ::core::option::Option<Valign>,
    pub smufl: ::core::option::Option<::std::string::String>,
    pub id: ::core::option::Option<::std::string::String>,
    pub content: ::std::string::String,
}
#[derive(Debug)]
pub struct OtherListening {
    pub type_: ::std::string::String,
    pub player: ::core::option::Option<::std::string::String>,
    pub time_only: ::core::option::Option<::std::string::String>,
    pub content: ::std::string::String,
}
#[derive(Debug)]
pub struct OtherNotation {
    pub type_: StartStopSingle,
    pub number: ::num::BigUint,
    pub print_object: ::core::option::Option<YesNo>,
    pub default_x: ::core::option::Option<::core::primitive::f64>,
    pub default_y: ::core::option::Option<::core::primitive::f64>,
    pub relative_x: ::core::option::Option<::core::primitive::f64>,
    pub relative_y: ::core::option::Option<::core::primitive::f64>,
    pub font_family: ::core::option::Option<::std::string::String>,
    pub font_style: ::core::option::Option<FontStyle>,
    pub font_size: ::core::option::Option<FontSize>,
    pub font_weight: ::core::option::Option<FontWeight>,
    pub color: ::core::option::Option<::std::string::String>,
    pub placement: ::core::option::Option<AboveBelow>,
    pub smufl: ::core::option::Option<::std::string::String>,
    pub id: ::core::option::Option<::std::string::String>,
    pub content: ::std::string::String,
}
#[derive(Debug)]
pub struct OtherPlacementText {
    pub default_x: ::core::option::Option<::core::primitive::f64>,
    pub default_y: ::core::option::Option<::core::primitive::f64>,
    pub relative_x: ::core::option::Option<::core::primitive::f64>,
    pub relative_y: ::core::option::Option<::core::primitive::f64>,
    pub font_family: ::core::option::Option<::std::string::String>,
    pub font_style: ::core::option::Option<FontStyle>,
    pub font_size: ::core::option::Option<FontSize>,
    pub font_weight: ::core::option::Option<FontWeight>,
    pub color: ::core::option::Option<::std::string::String>,
    pub placement: ::core::option::Option<AboveBelow>,
    pub smufl: ::core::option::Option<::std::string::String>,
    pub content: ::std::string::String,
}
#[derive(Debug)]
pub struct OtherPlay {
    pub type_: ::std::string::String,
    pub content: ::std::string::String,
}
#[derive(Debug)]
pub struct OtherText {
    pub smufl: ::core::option::Option<::std::string::String>,
    pub content: ::std::string::String,
}
#[derive(Debug)]
pub enum OverUnder {
    Over,
    Under,
}
#[derive(Debug)]
pub struct PageLayout {
    pub content: PageLayoutContent,
}
#[derive(Debug)]
pub struct PageLayoutContent {
    pub page_height: ::core::option::Option<::core::primitive::f64>,
    pub page_width: ::core::option::Option<::core::primitive::f64>,
    pub page_margins: ::std::vec::Vec<PageMargins>,
}
#[derive(Debug)]
pub struct PageMargins {
    pub type_: ::core::option::Option<MarginType>,
    pub content: PageMarginsContent,
}
#[derive(Debug)]
pub struct PageMarginsContent {
    pub left_margin: ::core::primitive::f64,
    pub right_margin: ::core::primitive::f64,
    pub top_margin: ::core::primitive::f64,
    pub bottom_margin: ::core::primitive::f64,
}
#[derive(Debug)]
pub struct PartClef {
    pub content: PartClefContent,
}
#[derive(Debug)]
pub struct PartClefContent {
    pub sign: ClefSign,
    pub line: ::core::option::Option<::num::BigInt>,
    pub clef_octave_change: ::core::option::Option<::num::BigInt>,
}
#[derive(Debug)]
pub struct PartGroup {
    pub type_: StartStop,
    pub number: ::std::string::String,
    pub content: PartGroupContent,
}
#[derive(Debug)]
pub struct PartGroupContent {
    pub group_name: ::core::option::Option<GroupName>,
    pub group_name_display: ::core::option::Option<NameDisplay>,
    pub group_abbreviation: ::core::option::Option<GroupName>,
    pub group_abbreviation_display: ::core::option::Option<NameDisplay>,
    pub group_symbol: ::core::option::Option<GroupSymbol>,
    pub group_barline: ::core::option::Option<GroupBarline>,
    pub group_time: ::core::option::Option<Empty>,
    pub footnote: ::core::option::Option<FormattedText>,
    pub level: ::core::option::Option<Level>,
}
#[derive(Debug)]
pub struct PartLink {
    pub href: ::std::string::String,
    pub type_: ::core::option::Option<xlink::Type>,
    pub role: ::core::option::Option<::std::string::String>,
    pub title: ::core::option::Option<::std::string::String>,
    pub show: xlink::Show,
    pub actuate: xlink::Actuate,
    pub content: PartLinkContent,
}
#[derive(Debug)]
pub struct PartLinkContent {
    pub instrument_link: ::std::vec::Vec<InstrumentLink>,
    pub group_link: ::std::vec::Vec<::std::string::String>,
}
#[derive(Debug)]
pub struct PartList {
    pub content: ::std::vec::Vec<PartListContent>,
}
#[derive(Debug)]
pub enum PartListContent {
    PartGroup(PartGroup),
    ScorePart(ScorePart),
}
#[derive(Debug)]
pub struct PartName {
    pub default_x: ::core::option::Option<::core::primitive::f64>,
    pub default_y: ::core::option::Option<::core::primitive::f64>,
    pub relative_x: ::core::option::Option<::core::primitive::f64>,
    pub relative_y: ::core::option::Option<::core::primitive::f64>,
    pub font_family: ::core::option::Option<::std::string::String>,
    pub font_style: ::core::option::Option<FontStyle>,
    pub font_size: ::core::option::Option<FontSize>,
    pub font_weight: ::core::option::Option<FontWeight>,
    pub color: ::core::option::Option<::std::string::String>,
    pub print_object: ::core::option::Option<YesNo>,
    pub justify: ::core::option::Option<LeftCenterRight>,
    pub content: ::std::string::String,
}
#[derive(Debug)]
pub struct PartSymbol {
    pub top_staff: ::core::option::Option<::num::BigUint>,
    pub bottom_staff: ::core::option::Option<::num::BigUint>,
    pub default_x: ::core::option::Option<::core::primitive::f64>,
    pub default_y: ::core::option::Option<::core::primitive::f64>,
    pub relative_x: ::core::option::Option<::core::primitive::f64>,
    pub relative_y: ::core::option::Option<::core::primitive::f64>,
    pub color: ::core::option::Option<::std::string::String>,
    pub content: GroupSymbolValue,
}
#[derive(Debug)]
pub struct PartTranspose {
    pub content: PartTransposeContent,
}
#[derive(Debug)]
pub struct PartTransposeContent {
    pub diatonic: ::core::option::Option<::num::BigInt>,
    pub chromatic: ::core::primitive::f64,
    pub octave_change: ::core::option::Option<::num::BigInt>,
    pub double: ::core::option::Option<Double>,
}
#[derive(Debug)]
pub struct Pedal {
    pub type_: PedalType,
    pub number: ::core::option::Option<::num::BigUint>,
    pub line: ::core::option::Option<YesNo>,
    pub sign: ::core::option::Option<YesNo>,
    pub abbreviated: ::core::option::Option<YesNo>,
    pub default_x: ::core::option::Option<::core::primitive::f64>,
    pub default_y: ::core::option::Option<::core::primitive::f64>,
    pub relative_x: ::core::option::Option<::core::primitive::f64>,
    pub relative_y: ::core::option::Option<::core::primitive::f64>,
    pub font_family: ::core::option::Option<::std::string::String>,
    pub font_style: ::core::option::Option<FontStyle>,
    pub font_size: ::core::option::Option<FontSize>,
    pub font_weight: ::core::option::Option<FontWeight>,
    pub color: ::core::option::Option<::std::string::String>,
    pub halign: ::core::option::Option<LeftCenterRight>,
    pub valign: ::core::option::Option<Valign>,
    pub id: ::core::option::Option<::std::string::String>,
}
#[derive(Debug)]
pub struct PedalTuning {
    pub content: PedalTuningContent,
}
#[derive(Debug)]
pub struct PedalTuningContent {
    pub pedal_step: Step,
    pub pedal_alter: ::core::primitive::f64,
}
#[derive(Debug)]
pub enum PedalType {
    Start,
    Stop,
    Sostenuto,
    Change,
    Continue,
    Discontinue,
    Resume,
}
#[derive(Debug)]
pub struct PerMinute {
    pub font_family: ::core::option::Option<::std::string::String>,
    pub font_style: ::core::option::Option<FontStyle>,
    pub font_size: ::core::option::Option<FontSize>,
    pub font_weight: ::core::option::Option<FontWeight>,
    pub content: ::std::string::String,
}
pub type Percent = ::core::primitive::f64;
#[derive(Debug)]
pub struct Percussion {
    pub default_x: ::core::option::Option<::core::primitive::f64>,
    pub default_y: ::core::option::Option<::core::primitive::f64>,
    pub relative_x: ::core::option::Option<::core::primitive::f64>,
    pub relative_y: ::core::option::Option<::core::primitive::f64>,
    pub font_family: ::core::option::Option<::std::string::String>,
    pub font_style: ::core::option::Option<FontStyle>,
    pub font_size: ::core::option::Option<FontSize>,
    pub font_weight: ::core::option::Option<FontWeight>,
    pub color: ::core::option::Option<::std::string::String>,
    pub halign: ::core::option::Option<LeftCenterRight>,
    pub valign: ::core::option::Option<Valign>,
    pub enclosure: ::core::option::Option<EnclosureShape>,
    pub id: ::core::option::Option<::std::string::String>,
    pub content: PercussionContent,
}
#[derive(Debug)]
pub enum PercussionContent {
    Glass(Glass),
    Metal(Metal),
    Wood(Wood),
    Pitched(Pitched),
    Membrane(Membrane),
    Effect(Effect),
    Timpani(Timpani),
    Beater(Beater),
    Stick(Stick),
    StickLocation(StickLocation),
    OtherPercussion(OtherText),
}
#[derive(Debug)]
pub struct Pitch {
    pub content: PitchContent,
}
#[derive(Debug)]
pub struct PitchContent {
    pub step: Step,
    pub alter: ::core::option::Option<::core::primitive::f64>,
    pub octave: ::num::BigInt,
}
#[derive(Debug)]
pub struct Pitched {
    pub smufl: ::core::option::Option<::std::string::String>,
    pub content: PitchedValue,
}
#[derive(Debug)]
pub enum PitchedValue {
    Celesta,
    Chimes,
    Glockenspiel,
    Lithophone,
    Mallet,
    Marimba,
    SteelDrums,
    Tubaphone,
    TubularChimes,
    Vibraphone,
    Xylophone,
}
#[derive(Debug)]
pub struct PlacementText {
    pub default_x: ::core::option::Option<::core::primitive::f64>,
    pub default_y: ::core::option::Option<::core::primitive::f64>,
    pub relative_x: ::core::option::Option<::core::primitive::f64>,
    pub relative_y: ::core::option::Option<::core::primitive::f64>,
    pub font_family: ::core::option::Option<::std::string::String>,
    pub font_style: ::core::option::Option<FontStyle>,
    pub font_size: ::core::option::Option<FontSize>,
    pub font_weight: ::core::option::Option<FontWeight>,
    pub color: ::core::option::Option<::std::string::String>,
    pub placement: ::core::option::Option<AboveBelow>,
    pub content: ::std::string::String,
}
#[derive(Debug)]
pub struct Play {
    pub id: ::core::option::Option<::std::string::String>,
    pub content: ::std::vec::Vec<PlayContent>,
}
#[derive(Debug)]
pub enum PlayContent {
    Ipa(::std::string::String),
    Mute(Mute),
    SemiPitched(SemiPitched),
    OtherPlay(OtherPlay),
}
#[derive(Debug)]
pub struct Player {
    pub id: ::std::string::String,
    pub content: PlayerContent,
}
#[derive(Debug)]
pub struct PlayerContent {
    pub player_name: ::std::string::String,
}
pub type PositiveDecimal = ::core::primitive::f64;
pub type PositiveDivisions = ::core::primitive::f64;
#[derive(Debug)]
pub enum PositiveIntegerOrEmpty {
    BigUint(::num::BigUint),
    String(::std::string::String),
}
#[derive(Debug)]
pub struct PrincipalVoice {
    pub type_: StartStop,
    pub symbol: PrincipalVoiceSymbol,
    pub default_x: ::core::option::Option<::core::primitive::f64>,
    pub default_y: ::core::option::Option<::core::primitive::f64>,
    pub relative_x: ::core::option::Option<::core::primitive::f64>,
    pub relative_y: ::core::option::Option<::core::primitive::f64>,
    pub font_family: ::core::option::Option<::std::string::String>,
    pub font_style: ::core::option::Option<FontStyle>,
    pub font_size: ::core::option::Option<FontSize>,
    pub font_weight: ::core::option::Option<FontWeight>,
    pub color: ::core::option::Option<::std::string::String>,
    pub halign: ::core::option::Option<LeftCenterRight>,
    pub valign: ::core::option::Option<Valign>,
    pub id: ::core::option::Option<::std::string::String>,
    pub content: ::std::string::String,
}
#[derive(Debug)]
pub enum PrincipalVoiceSymbol {
    Hauptstimme,
    Nebenstimme,
    Plain,
    None,
}
#[derive(Debug)]
pub struct Print {
    pub staff_spacing: ::core::option::Option<::core::primitive::f64>,
    pub new_system: ::core::option::Option<YesNo>,
    pub new_page: ::core::option::Option<YesNo>,
    pub blank_page: ::core::option::Option<::num::BigUint>,
    pub page_number: ::core::option::Option<::std::string::String>,
    pub id: ::core::option::Option<::std::string::String>,
    pub content: PrintContent,
}
#[derive(Debug)]
pub struct PrintContent {
    pub page_layout: ::core::option::Option<PageLayout>,
    pub system_layout: ::core::option::Option<SystemLayout>,
    pub staff_layout: ::std::vec::Vec<StaffLayout>,
    pub measure_layout: ::core::option::Option<MeasureLayout>,
    pub measure_numbering: ::core::option::Option<MeasureNumbering>,
    pub part_name_display: ::core::option::Option<NameDisplay>,
    pub part_abbreviation_display: ::core::option::Option<NameDisplay>,
}
#[derive(Debug)]
pub struct Release {
    pub offset: ::core::option::Option<::core::primitive::f64>,
}
#[derive(Debug)]
pub struct Repeat {
    pub direction: BackwardForward,
    pub times: ::core::option::Option<::num::BigUint>,
    pub after_jump: ::core::option::Option<YesNo>,
    pub winged: ::core::option::Option<Winged>,
}
#[derive(Debug)]
pub struct Rest {
    pub measure: ::core::option::Option<YesNo>,
    pub content: RestContent,
}
#[derive(Debug)]
pub struct RestContent {
    pub display_step: ::core::option::Option<Step>,
    pub display_octave: ::core::option::Option<::num::BigInt>,
}
#[derive(Debug)]
pub enum RightLeftMiddle {
    Right,
    Left,
    Middle,
}
#[derive(Debug)]
pub struct Root {
    pub content: RootContent,
}
#[derive(Debug)]
pub struct RootContent {
    pub root_step: RootStep,
    pub root_alter: ::core::option::Option<HarmonyAlter>,
}
#[derive(Debug)]
pub struct RootStep {
    pub text: ::core::option::Option<::std::string::String>,
    pub default_x: ::core::option::Option<::core::primitive::f64>,
    pub default_y: ::core::option::Option<::core::primitive::f64>,
    pub relative_x: ::core::option::Option<::core::primitive::f64>,
    pub relative_y: ::core::option::Option<::core::primitive::f64>,
    pub font_family: ::core::option::Option<::std::string::String>,
    pub font_style: ::core::option::Option<FontStyle>,
    pub font_size: ::core::option::Option<FontSize>,
    pub font_weight: ::core::option::Option<FontWeight>,
    pub color: ::core::option::Option<::std::string::String>,
    pub content: Step,
}
pub type RotationDegrees = ::core::primitive::f64;
#[derive(Debug)]
pub struct Scaling {
    pub content: ScalingContent,
}
#[derive(Debug)]
pub struct ScalingContent {
    pub millimeters: ::core::primitive::f64,
    pub tenths: ::core::primitive::f64,
}
#[derive(Debug)]
pub struct Scordatura {
    pub id: ::core::option::Option<::std::string::String>,
    pub content: ScordaturaContent,
}
#[derive(Debug)]
pub struct ScordaturaContent {
    pub accord: ::std::vec::Vec<Accord>,
}
#[derive(Debug)]
pub struct ScoreInstrument {
    pub id: ::std::string::String,
    pub content: ::std::vec::Vec<ScoreInstrumentContent>,
}
#[derive(Debug)]
pub enum ScoreInstrumentContent {
    InstrumentName(::std::string::String),
    InstrumentAbbreviation(::std::string::String),
    InstrumentSound(::std::string::String),
    Solo(Empty),
    Ensemble(PositiveIntegerOrEmpty),
    VirtualInstrument(VirtualInstrument),
}
#[derive(Debug)]
pub struct ScorePart {
    pub id: ::std::string::String,
    pub content: ::std::vec::Vec<ScorePartContent>,
}
#[derive(Debug)]
pub enum ScorePartContent {
    Identification(Identification),
    PartLink(PartLink),
    PartName(PartName),
    PartNameDisplay(NameDisplay),
    PartAbbreviation(PartName),
    PartAbbreviationDisplay(NameDisplay),
    Group(::std::string::String),
    ScoreInstrument(ScoreInstrument),
    Player(Player),
    MidiDevice(MidiDevice),
    MidiInstrument(MidiInstrument),
}
pub type ScorePartwiseElement = ScorePartwiseElementType;
#[derive(Debug)]
pub struct ScorePartwiseElementType {
    pub version: ::std::string::String,
    pub content: ScorePartwiseElementTypeContent,
}
#[derive(Debug)]
pub struct ScorePartwiseElementTypeContent {
    pub work: ::core::option::Option<Work>,
    pub movement_number: ::core::option::Option<::std::string::String>,
    pub movement_title: ::core::option::Option<::std::string::String>,
    pub identification: ::core::option::Option<Identification>,
    pub defaults: ::core::option::Option<Defaults>,
    pub credit: ::std::vec::Vec<Credit>,
    pub part_list: PartList,
    pub part: ::std::vec::Vec<ScorePartwisePartElementType>,
}
pub type ScoreTimewiseElement = ScoreTimewiseElementType;
#[derive(Debug)]
pub struct ScoreTimewiseElementType {
    pub version: ::std::string::String,
    pub content: ScoreTimewiseElementTypeContent,
}
#[derive(Debug)]
pub struct ScoreTimewiseElementTypeContent {
    pub work: ::core::option::Option<Work>,
    pub movement_number: ::core::option::Option<::std::string::String>,
    pub movement_title: ::core::option::Option<::std::string::String>,
    pub identification: ::core::option::Option<Identification>,
    pub defaults: ::core::option::Option<Defaults>,
    pub credit: ::std::vec::Vec<Credit>,
    pub part_list: PartList,
    pub measure: ::std::vec::Vec<ScoreTimewiseMeasureElementType>,
}
#[derive(Debug)]
pub struct Segno {
    pub default_x: ::core::option::Option<::core::primitive::f64>,
    pub default_y: ::core::option::Option<::core::primitive::f64>,
    pub relative_x: ::core::option::Option<::core::primitive::f64>,
    pub relative_y: ::core::option::Option<::core::primitive::f64>,
    pub font_family: ::core::option::Option<::std::string::String>,
    pub font_style: ::core::option::Option<FontStyle>,
    pub font_size: ::core::option::Option<FontSize>,
    pub font_weight: ::core::option::Option<FontWeight>,
    pub color: ::core::option::Option<::std::string::String>,
    pub halign: ::core::option::Option<LeftCenterRight>,
    pub valign: ::core::option::Option<Valign>,
    pub id: ::core::option::Option<::std::string::String>,
    pub smufl: ::core::option::Option<::std::string::String>,
}
#[derive(Debug)]
pub enum SemiPitched {
    High,
    MediumHigh,
    Medium,
    MediumLow,
    Low,
    VeryLow,
}
pub type Semitones = ::core::primitive::f64;
#[derive(Debug)]
pub enum ShowFrets {
    Numbers,
    Letters,
}
#[derive(Debug)]
pub enum ShowTuplet {
    Actual,
    Both,
    None,
}
#[derive(Debug)]
pub struct Slash {
    pub type_: StartStop,
    pub use_dots: ::core::option::Option<YesNo>,
    pub use_stems: ::core::option::Option<YesNo>,
    pub content: ::core::option::Option<SlashContent>,
}
#[derive(Debug)]
pub struct SlashContent {
    pub slash_type: ::core::option::Option<NoteTypeValue>,
    pub slash_dot: ::std::vec::Vec<Empty>,
    pub except_voice: ::std::vec::Vec<::std::string::String>,
}
#[derive(Debug)]
pub struct Slide {
    pub type_: StartStop,
    pub number: ::num::BigUint,
    pub line_type: ::core::option::Option<LineType>,
    pub dash_length: ::core::option::Option<::core::primitive::f64>,
    pub space_length: ::core::option::Option<::core::primitive::f64>,
    pub default_x: ::core::option::Option<::core::primitive::f64>,
    pub default_y: ::core::option::Option<::core::primitive::f64>,
    pub relative_x: ::core::option::Option<::core::primitive::f64>,
    pub relative_y: ::core::option::Option<::core::primitive::f64>,
    pub font_family: ::core::option::Option<::std::string::String>,
    pub font_style: ::core::option::Option<FontStyle>,
    pub font_size: ::core::option::Option<FontSize>,
    pub font_weight: ::core::option::Option<FontWeight>,
    pub color: ::core::option::Option<::std::string::String>,
    pub accelerate: ::core::option::Option<YesNo>,
    pub beats: ::core::option::Option<::core::primitive::f64>,
    pub first_beat: ::core::option::Option<::core::primitive::f64>,
    pub last_beat: ::core::option::Option<::core::primitive::f64>,
    pub id: ::core::option::Option<::std::string::String>,
    pub content: ::std::string::String,
}
#[derive(Debug)]
pub struct Slur {
    pub type_: StartStopContinue,
    pub number: ::num::BigUint,
    pub line_type: ::core::option::Option<LineType>,
    pub dash_length: ::core::option::Option<::core::primitive::f64>,
    pub space_length: ::core::option::Option<::core::primitive::f64>,
    pub default_x: ::core::option::Option<::core::primitive::f64>,
    pub default_y: ::core::option::Option<::core::primitive::f64>,
    pub relative_x: ::core::option::Option<::core::primitive::f64>,
    pub relative_y: ::core::option::Option<::core::primitive::f64>,
    pub placement: ::core::option::Option<AboveBelow>,
    pub orientation: ::core::option::Option<OverUnder>,
    pub bezier_x: ::core::option::Option<::core::primitive::f64>,
    pub bezier_y: ::core::option::Option<::core::primitive::f64>,
    pub bezier_x2: ::core::option::Option<::core::primitive::f64>,
    pub bezier_y2: ::core::option::Option<::core::primitive::f64>,
    pub bezier_offset: ::core::option::Option<::core::primitive::f64>,
    pub bezier_offset_2: ::core::option::Option<::core::primitive::f64>,
    pub color: ::core::option::Option<::std::string::String>,
    pub id: ::core::option::Option<::std::string::String>,
}
pub type SmuflAccidentalGlyphName = ::std::string::String;
pub type SmuflCodaGlyphName = ::std::string::String;
pub type SmuflGlyphName = ::std::string::String;
pub type SmuflLyricsGlyphName = ::std::string::String;
pub type SmuflPictogramGlyphName = ::std::string::String;
pub type SmuflSegnoGlyphName = ::std::string::String;
pub type SmuflWavyLineGlyphName = ::std::string::String;
#[derive(Debug)]
pub struct Sound {
    pub tempo: ::core::option::Option<::core::primitive::f64>,
    pub dynamics: ::core::option::Option<::core::primitive::f64>,
    pub dacapo: ::core::option::Option<YesNo>,
    pub segno: ::core::option::Option<::std::string::String>,
    pub dalsegno: ::core::option::Option<::std::string::String>,
    pub coda: ::core::option::Option<::std::string::String>,
    pub tocoda: ::core::option::Option<::std::string::String>,
    pub divisions: ::core::option::Option<::core::primitive::f64>,
    pub forward_repeat: ::core::option::Option<YesNo>,
    pub fine: ::core::option::Option<::std::string::String>,
    pub time_only: ::core::option::Option<::std::string::String>,
    pub pizzicato: ::core::option::Option<YesNo>,
    pub pan: ::core::option::Option<::core::primitive::f64>,
    pub elevation: ::core::option::Option<::core::primitive::f64>,
    pub damper_pedal: ::core::option::Option<YesNoNumber>,
    pub soft_pedal: ::core::option::Option<YesNoNumber>,
    pub sostenuto_pedal: ::core::option::Option<YesNoNumber>,
    pub id: ::core::option::Option<::std::string::String>,
    pub content: ::std::vec::Vec<SoundContent>,
}
#[derive(Debug)]
pub enum SoundContent {
    InstrumentChange(InstrumentChange),
    MidiDevice(MidiDevice),
    MidiInstrument(MidiInstrument),
    Play(Play),
    Swing(Swing),
    Offset(Offset),
}
#[derive(Debug)]
pub struct StaffDetails {
    pub number: ::core::option::Option<::num::BigUint>,
    pub show_frets: ::core::option::Option<ShowFrets>,
    pub print_object: ::core::option::Option<YesNo>,
    pub print_spacing: ::core::option::Option<YesNo>,
    pub content: StaffDetailsContent,
}
#[derive(Debug)]
pub struct StaffDetailsContent {
    pub staff_type: ::core::option::Option<StaffType>,
    pub staff_lines: ::core::option::Option<::num::BigUint>,
    pub line_detail: ::std::vec::Vec<LineDetail>,
    pub staff_tuning: ::std::vec::Vec<StaffTuning>,
    pub capo: ::core::option::Option<::num::BigUint>,
    pub staff_size: ::core::option::Option<StaffSize>,
}
#[derive(Debug)]
pub struct StaffDivide {
    pub type_: StaffDivideSymbol,
    pub default_x: ::core::option::Option<::core::primitive::f64>,
    pub default_y: ::core::option::Option<::core::primitive::f64>,
    pub relative_x: ::core::option::Option<::core::primitive::f64>,
    pub relative_y: ::core::option::Option<::core::primitive::f64>,
    pub font_family: ::core::option::Option<::std::string::String>,
    pub font_style: ::core::option::Option<FontStyle>,
    pub font_size: ::core::option::Option<FontSize>,
    pub font_weight: ::core::option::Option<FontWeight>,
    pub color: ::core::option::Option<::std::string::String>,
    pub halign: ::core::option::Option<LeftCenterRight>,
    pub valign: ::core::option::Option<Valign>,
    pub id: ::core::option::Option<::std::string::String>,
}
#[derive(Debug)]
pub enum StaffDivideSymbol {
    Down,
    Up,
    UpDown,
}
#[derive(Debug)]
pub struct StaffLayout {
    pub number: ::core::option::Option<::num::BigUint>,
    pub content: StaffLayoutContent,
}
#[derive(Debug)]
pub struct StaffLayoutContent {
    pub staff_distance: ::core::option::Option<::core::primitive::f64>,
}
pub type StaffLine = ::num::BigUint;
pub type StaffLinePosition = ::num::BigInt;
pub type StaffNumber = ::num::BigUint;
#[derive(Debug)]
pub struct StaffSize {
    pub scaling: ::core::option::Option<::core::primitive::f64>,
    pub content: ::core::primitive::f64,
}
#[derive(Debug)]
pub struct StaffTuning {
    pub line: ::num::BigUint,
    pub content: StaffTuningContent,
}
#[derive(Debug)]
pub struct StaffTuningContent {
    pub tuning_step: Step,
    pub tuning_alter: ::core::option::Option<::core::primitive::f64>,
    pub tuning_octave: ::num::BigInt,
}
#[derive(Debug)]
pub enum StaffType {
    Ossia,
    Editorial,
    Cue,
    Alternate,
    Regular,
}
#[derive(Debug)]
pub enum StartNote {
    Upper,
    Main,
    Below,
}
#[derive(Debug)]
pub enum StartStop {
    Start,
    Stop,
}
#[derive(Debug)]
pub enum StartStopChangeContinue {
    Start,
    Stop,
    Change,
    Continue,
}
#[derive(Debug)]
pub enum StartStopContinue {
    Start,
    Stop,
    Continue,
}
#[derive(Debug)]
pub enum StartStopDiscontinue {
    Start,
    Stop,
    Discontinue,
}
#[derive(Debug)]
pub enum StartStopSingle {
    Start,
    Stop,
    Single,
}
#[derive(Debug)]
pub struct Stem {
    pub default_x: ::core::option::Option<::core::primitive::f64>,
    pub default_y: ::core::option::Option<::core::primitive::f64>,
    pub relative_x: ::core::option::Option<::core::primitive::f64>,
    pub relative_y: ::core::option::Option<::core::primitive::f64>,
    pub color: ::core::option::Option<::std::string::String>,
    pub content: StemValue,
}
#[derive(Debug)]
pub enum StemValue {
    Down,
    Up,
    Double,
    None,
}
#[derive(Debug)]
pub enum Step {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}
#[derive(Debug)]
pub struct Stick {
    pub tip: ::core::option::Option<TipDirection>,
    pub parentheses: ::core::option::Option<YesNo>,
    pub dashed_circle: ::core::option::Option<YesNo>,
    pub content: StickContent,
}
#[derive(Debug)]
pub struct StickContent {
    pub stick_type: StickType,
    pub stick_material: StickMaterial,
}
#[derive(Debug)]
pub enum StickLocation {
    Center,
    Rim,
    CymbalBell,
    CymbalEdge,
}
#[derive(Debug)]
pub enum StickMaterial {
    Soft,
    Medium,
    Hard,
    Shaded,
    X,
}
#[derive(Debug)]
pub enum StickType {
    BassDrum,
    DoubleBassDrum,
    Glockenspiel,
    Gum,
    Hammer,
    Superball,
    Timpani,
    Wound,
    Xylophone,
    Yarn,
}
#[derive(Debug)]
pub struct String {
    pub default_x: ::core::option::Option<::core::primitive::f64>,
    pub default_y: ::core::option::Option<::core::primitive::f64>,
    pub relative_x: ::core::option::Option<::core::primitive::f64>,
    pub relative_y: ::core::option::Option<::core::primitive::f64>,
    pub font_family: ::core::option::Option<::std::string::String>,
    pub font_style: ::core::option::Option<FontStyle>,
    pub font_size: ::core::option::Option<FontSize>,
    pub font_weight: ::core::option::Option<FontWeight>,
    pub color: ::core::option::Option<::std::string::String>,
    pub placement: ::core::option::Option<AboveBelow>,
    pub content: ::num::BigUint,
}
#[derive(Debug)]
pub struct StringMute {
    pub type_: OnOff,
    pub default_x: ::core::option::Option<::core::primitive::f64>,
    pub default_y: ::core::option::Option<::core::primitive::f64>,
    pub relative_x: ::core::option::Option<::core::primitive::f64>,
    pub relative_y: ::core::option::Option<::core::primitive::f64>,
    pub font_family: ::core::option::Option<::std::string::String>,
    pub font_style: ::core::option::Option<FontStyle>,
    pub font_size: ::core::option::Option<FontSize>,
    pub font_weight: ::core::option::Option<FontWeight>,
    pub color: ::core::option::Option<::std::string::String>,
    pub halign: ::core::option::Option<LeftCenterRight>,
    pub valign: ::core::option::Option<Valign>,
    pub id: ::core::option::Option<::std::string::String>,
}
pub type StringNumber = ::num::BigUint;
#[derive(Debug)]
pub struct StrongAccent {
    pub default_x: ::core::option::Option<::core::primitive::f64>,
    pub default_y: ::core::option::Option<::core::primitive::f64>,
    pub relative_x: ::core::option::Option<::core::primitive::f64>,
    pub relative_y: ::core::option::Option<::core::primitive::f64>,
    pub font_family: ::core::option::Option<::std::string::String>,
    pub font_style: ::core::option::Option<FontStyle>,
    pub font_size: ::core::option::Option<FontSize>,
    pub font_weight: ::core::option::Option<FontWeight>,
    pub color: ::core::option::Option<::std::string::String>,
    pub placement: ::core::option::Option<AboveBelow>,
    pub type_: UpDown,
}
#[derive(Debug)]
pub struct StyleText {
    pub default_x: ::core::option::Option<::core::primitive::f64>,
    pub default_y: ::core::option::Option<::core::primitive::f64>,
    pub relative_x: ::core::option::Option<::core::primitive::f64>,
    pub relative_y: ::core::option::Option<::core::primitive::f64>,
    pub font_family: ::core::option::Option<::std::string::String>,
    pub font_style: ::core::option::Option<FontStyle>,
    pub font_size: ::core::option::Option<FontSize>,
    pub font_weight: ::core::option::Option<FontWeight>,
    pub color: ::core::option::Option<::std::string::String>,
    pub content: ::std::string::String,
}
#[derive(Debug)]
pub struct Supports {
    pub type_: YesNo,
    pub element: ::std::string::String,
    pub attribute: ::core::option::Option<::std::string::String>,
    pub value: ::core::option::Option<::std::string::String>,
}
#[derive(Debug)]
pub struct Swing {
    pub content: ::std::vec::Vec<SwingContent>,
}
#[derive(Debug)]
pub enum SwingContent {
    Straight(Empty),
    First(::num::BigUint),
    Second(::num::BigUint),
    SwingType(SwingTypeValue),
    SwingStyle(::std::string::String),
}
#[derive(Debug)]
pub enum SwingTypeValue {
    _16Th,
    Eighth,
}
#[derive(Debug)]
pub enum Syllabic {
    Single,
    Begin,
    End,
    Middle,
}
#[derive(Debug)]
pub enum SymbolSize {
    Full,
    Cue,
    GraceCue,
    Large,
}
#[derive(Debug)]
pub struct Sync {
    pub type_: SyncType,
    pub latency: ::core::option::Option<::num::BigUint>,
    pub player: ::core::option::Option<::std::string::String>,
    pub time_only: ::core::option::Option<::std::string::String>,
}
#[derive(Debug)]
pub enum SyncType {
    None,
    Tempo,
    MostlyTempo,
    MostlyEvent,
    Event,
    AlwaysEvent,
}
#[derive(Debug)]
pub struct SystemDividers {
    pub content: SystemDividersContent,
}
#[derive(Debug)]
pub struct SystemDividersContent {
    pub left_divider: EmptyPrintObjectStyleAlign,
    pub right_divider: EmptyPrintObjectStyleAlign,
}
#[derive(Debug)]
pub struct SystemLayout {
    pub content: SystemLayoutContent,
}
#[derive(Debug)]
pub struct SystemLayoutContent {
    pub system_margins: ::core::option::Option<SystemMargins>,
    pub system_distance: ::core::option::Option<::core::primitive::f64>,
    pub top_system_distance: ::core::option::Option<::core::primitive::f64>,
    pub system_dividers: ::core::option::Option<SystemDividers>,
}
#[derive(Debug)]
pub struct SystemMargins {
    pub content: SystemMarginsContent,
}
#[derive(Debug)]
pub struct SystemMarginsContent {
    pub left_margin: ::core::primitive::f64,
    pub right_margin: ::core::primitive::f64,
}
#[derive(Debug)]
pub enum SystemRelation {
    OnlyTop,
    AlsoTop,
    None,
}
#[derive(Debug)]
pub enum SystemRelationNumber {
    OnlyTop,
    OnlyBottom,
    AlsoTop,
    AlsoBottom,
    None,
}
#[derive(Debug)]
pub struct Tap {
    pub hand: ::core::option::Option<TapHand>,
    pub default_x: ::core::option::Option<::core::primitive::f64>,
    pub default_y: ::core::option::Option<::core::primitive::f64>,
    pub relative_x: ::core::option::Option<::core::primitive::f64>,
    pub relative_y: ::core::option::Option<::core::primitive::f64>,
    pub font_family: ::core::option::Option<::std::string::String>,
    pub font_style: ::core::option::Option<FontStyle>,
    pub font_size: ::core::option::Option<FontSize>,
    pub font_weight: ::core::option::Option<FontWeight>,
    pub color: ::core::option::Option<::std::string::String>,
    pub placement: ::core::option::Option<AboveBelow>,
    pub content: ::std::string::String,
}
#[derive(Debug)]
pub enum TapHand {
    Left,
    Right,
}
#[derive(Debug)]
pub struct Technical {
    pub id: ::core::option::Option<::std::string::String>,
    pub content: ::std::vec::Vec<TechnicalContent>,
}
#[derive(Debug)]
pub enum TechnicalContent {
    UpBow(EmptyPlacement),
    DownBow(EmptyPlacement),
    Harmonic(Harmonic),
    OpenString(EmptyPlacement),
    ThumbPosition(EmptyPlacement),
    Fingering(Fingering),
    Pluck(PlacementText),
    DoubleTongue(EmptyPlacement),
    TripleTongue(EmptyPlacement),
    Stopped(EmptyPlacementSmufl),
    SnapPizzicato(EmptyPlacement),
    Fret(Fret),
    String(String),
    HammerOn(HammerOnPullOff),
    PullOff(HammerOnPullOff),
    Bend(Bend),
    Tap(Tap),
    Heel(HeelToe),
    Toe(HeelToe),
    Fingernails(EmptyPlacement),
    Hole(Hole),
    Arrow(Arrow),
    Handbell(Handbell),
    BrassBend(EmptyPlacement),
    Flip(EmptyPlacement),
    Smear(EmptyPlacement),
    Open(EmptyPlacementSmufl),
    HalfMuted(EmptyPlacementSmufl),
    HarmonMute(HarmonMute),
    Golpe(EmptyPlacement),
    OtherTechnical(OtherPlacementText),
}
pub type Tenths = ::core::primitive::f64;
#[derive(Debug)]
pub enum TextDirection {
    Ltr,
    Rtl,
    Lro,
    Rlo,
}
#[derive(Debug)]
pub struct TextElementData {
    pub font_family: ::core::option::Option<::std::string::String>,
    pub font_style: ::core::option::Option<FontStyle>,
    pub font_size: ::core::option::Option<FontSize>,
    pub font_weight: ::core::option::Option<FontWeight>,
    pub color: ::core::option::Option<::std::string::String>,
    pub underline: ::core::option::Option<::num::BigUint>,
    pub overline: ::core::option::Option<::num::BigUint>,
    pub line_through: ::core::option::Option<::num::BigUint>,
    pub rotation: ::core::option::Option<::core::primitive::f64>,
    pub letter_spacing: ::core::option::Option<NumberOrNormal>,
    pub lang: ::core::option::Option<::std::string::String>,
    pub dir: ::core::option::Option<TextDirection>,
    pub content: ::std::string::String,
}
#[derive(Debug)]
pub struct Tie {
    pub type_: StartStop,
    pub time_only: ::core::option::Option<::std::string::String>,
}
#[derive(Debug)]
pub struct Tied {
    pub type_: TiedType,
    pub number: ::core::option::Option<::num::BigUint>,
    pub line_type: ::core::option::Option<LineType>,
    pub dash_length: ::core::option::Option<::core::primitive::f64>,
    pub space_length: ::core::option::Option<::core::primitive::f64>,
    pub default_x: ::core::option::Option<::core::primitive::f64>,
    pub default_y: ::core::option::Option<::core::primitive::f64>,
    pub relative_x: ::core::option::Option<::core::primitive::f64>,
    pub relative_y: ::core::option::Option<::core::primitive::f64>,
    pub placement: ::core::option::Option<AboveBelow>,
    pub orientation: ::core::option::Option<OverUnder>,
    pub bezier_x: ::core::option::Option<::core::primitive::f64>,
    pub bezier_y: ::core::option::Option<::core::primitive::f64>,
    pub bezier_x2: ::core::option::Option<::core::primitive::f64>,
    pub bezier_y2: ::core::option::Option<::core::primitive::f64>,
    pub bezier_offset: ::core::option::Option<::core::primitive::f64>,
    pub bezier_offset_2: ::core::option::Option<::core::primitive::f64>,
    pub color: ::core::option::Option<::std::string::String>,
    pub id: ::core::option::Option<::std::string::String>,
}
#[derive(Debug)]
pub enum TiedType {
    Start,
    Stop,
    Continue,
    LetRing,
}
#[derive(Debug)]
pub struct Time {
    pub number: ::core::option::Option<::num::BigUint>,
    pub symbol: ::core::option::Option<TimeSymbol>,
    pub separator: ::core::option::Option<TimeSeparator>,
    pub default_x: ::core::option::Option<::core::primitive::f64>,
    pub default_y: ::core::option::Option<::core::primitive::f64>,
    pub relative_x: ::core::option::Option<::core::primitive::f64>,
    pub relative_y: ::core::option::Option<::core::primitive::f64>,
    pub font_family: ::core::option::Option<::std::string::String>,
    pub font_style: ::core::option::Option<FontStyle>,
    pub font_size: ::core::option::Option<FontSize>,
    pub font_weight: ::core::option::Option<FontWeight>,
    pub color: ::core::option::Option<::std::string::String>,
    pub halign: ::core::option::Option<LeftCenterRight>,
    pub valign: ::core::option::Option<Valign>,
    pub print_object: ::core::option::Option<YesNo>,
    pub id: ::core::option::Option<::std::string::String>,
    pub content: ::std::vec::Vec<TimeContent>,
}
#[derive(Debug)]
pub enum TimeContent {
    Beats(::std::string::String),
    BeatType(::std::string::String),
    Interchangeable(Interchangeable),
    SenzaMisura(::std::string::String),
}
#[derive(Debug)]
pub struct TimeModification {
    pub content: TimeModificationContent,
}
#[derive(Debug)]
pub struct TimeModificationContent {
    pub actual_notes: ::num::BigUint,
    pub normal_notes: ::num::BigUint,
    pub normal_type: ::core::option::Option<NoteTypeValue>,
    pub normal_dot: ::std::vec::Vec<Empty>,
}
pub type TimeOnly = ::std::string::String;
#[derive(Debug)]
pub enum TimeRelation {
    Parentheses,
    Bracket,
    Equals,
    Slash,
    Space,
    Hyphen,
}
#[derive(Debug)]
pub enum TimeSeparator {
    None,
    Horizontal,
    Diagonal,
    Vertical,
    Adjacent,
}
#[derive(Debug)]
pub enum TimeSymbol {
    Common,
    Cut,
    SingleNumber,
    Note,
    DottedNote,
    Normal,
}
#[derive(Debug)]
pub struct Timpani {
    pub smufl: ::core::option::Option<::std::string::String>,
}
#[derive(Debug)]
pub enum TipDirection {
    Up,
    Down,
    Left,
    Right,
    Northwest,
    Northeast,
    Southeast,
    Southwest,
}
#[derive(Debug)]
pub enum TopBottom {
    Top,
    Bottom,
}
#[derive(Debug)]
pub struct Transpose {
    pub number: ::core::option::Option<::num::BigUint>,
    pub id: ::core::option::Option<::std::string::String>,
    pub content: TransposeContent,
}
#[derive(Debug)]
pub struct TransposeContent {
    pub diatonic: ::core::option::Option<::num::BigInt>,
    pub chromatic: ::core::primitive::f64,
    pub octave_change: ::core::option::Option<::num::BigInt>,
    pub double: ::core::option::Option<Double>,
}
#[derive(Debug)]
pub struct Tremolo {
    pub type_: TremoloType,
    pub default_x: ::core::option::Option<::core::primitive::f64>,
    pub default_y: ::core::option::Option<::core::primitive::f64>,
    pub relative_x: ::core::option::Option<::core::primitive::f64>,
    pub relative_y: ::core::option::Option<::core::primitive::f64>,
    pub font_family: ::core::option::Option<::std::string::String>,
    pub font_style: ::core::option::Option<FontStyle>,
    pub font_size: ::core::option::Option<FontSize>,
    pub font_weight: ::core::option::Option<FontWeight>,
    pub color: ::core::option::Option<::std::string::String>,
    pub placement: ::core::option::Option<AboveBelow>,
    pub smufl: ::core::option::Option<::std::string::String>,
    pub content: ::num::BigInt,
}
pub type TremoloMarks = ::num::BigInt;
#[derive(Debug)]
pub enum TremoloType {
    Start,
    Stop,
    Single,
    Unmeasured,
}
pub type TrillBeats = ::core::primitive::f64;
#[derive(Debug)]
pub enum TrillStep {
    Whole,
    Half,
    Unison,
}
#[derive(Debug)]
pub struct Tuplet {
    pub type_: StartStop,
    pub number: ::core::option::Option<::num::BigUint>,
    pub bracket: ::core::option::Option<YesNo>,
    pub show_number: ::core::option::Option<ShowTuplet>,
    pub show_type: ::core::option::Option<ShowTuplet>,
    pub line_shape: ::core::option::Option<LineShape>,
    pub default_x: ::core::option::Option<::core::primitive::f64>,
    pub default_y: ::core::option::Option<::core::primitive::f64>,
    pub relative_x: ::core::option::Option<::core::primitive::f64>,
    pub relative_y: ::core::option::Option<::core::primitive::f64>,
    pub placement: ::core::option::Option<AboveBelow>,
    pub id: ::core::option::Option<::std::string::String>,
    pub content: TupletContent,
}
#[derive(Debug)]
pub struct TupletContent {
    pub tuplet_actual: ::core::option::Option<TupletPortion>,
    pub tuplet_normal: ::core::option::Option<TupletPortion>,
}
#[derive(Debug)]
pub struct TupletDot {
    pub font_family: ::core::option::Option<::std::string::String>,
    pub font_style: ::core::option::Option<FontStyle>,
    pub font_size: ::core::option::Option<FontSize>,
    pub font_weight: ::core::option::Option<FontWeight>,
    pub color: ::core::option::Option<::std::string::String>,
}
#[derive(Debug)]
pub struct TupletNumber {
    pub font_family: ::core::option::Option<::std::string::String>,
    pub font_style: ::core::option::Option<FontStyle>,
    pub font_size: ::core::option::Option<FontSize>,
    pub font_weight: ::core::option::Option<FontWeight>,
    pub color: ::core::option::Option<::std::string::String>,
    pub content: ::num::BigUint,
}
#[derive(Debug)]
pub struct TupletPortion {
    pub content: TupletPortionContent,
}
#[derive(Debug)]
pub struct TupletPortionContent {
    pub tuplet_number: ::core::option::Option<TupletNumber>,
    pub tuplet_type: ::core::option::Option<TupletType>,
    pub tuplet_dot: ::std::vec::Vec<TupletDot>,
}
#[derive(Debug)]
pub struct TupletType {
    pub font_family: ::core::option::Option<::std::string::String>,
    pub font_style: ::core::option::Option<FontStyle>,
    pub font_size: ::core::option::Option<FontSize>,
    pub font_weight: ::core::option::Option<FontWeight>,
    pub color: ::core::option::Option<::std::string::String>,
    pub content: NoteTypeValue,
}
#[derive(Debug)]
pub enum TwoNoteTurn {
    Whole,
    Half,
    None,
}
#[derive(Debug)]
pub struct TypedText {
    pub type_: ::core::option::Option<::std::string::String>,
    pub content: ::std::string::String,
}
#[derive(Debug)]
pub struct Unpitched {
    pub content: UnpitchedContent,
}
#[derive(Debug)]
pub struct UnpitchedContent {
    pub display_step: ::core::option::Option<Step>,
    pub display_octave: ::core::option::Option<::num::BigInt>,
}
#[derive(Debug)]
pub enum UpDown {
    Up,
    Down,
}
#[derive(Debug)]
pub enum UpDownStopContinue {
    Up,
    Down,
    Stop,
    Continue,
}
#[derive(Debug)]
pub enum UprightInverted {
    Upright,
    Inverted,
}
#[derive(Debug)]
pub enum Valign {
    Top,
    Middle,
    Bottom,
    Baseline,
}
#[derive(Debug)]
pub enum ValignImage {
    Top,
    Middle,
    Bottom,
}
#[derive(Debug)]
pub struct VirtualInstrument {
    pub content: VirtualInstrumentContent,
}
#[derive(Debug)]
pub struct VirtualInstrumentContent {
    pub virtual_library: ::core::option::Option<::std::string::String>,
    pub virtual_name: ::core::option::Option<::std::string::String>,
}
#[derive(Debug)]
pub struct Wait {
    pub player: ::core::option::Option<::std::string::String>,
    pub time_only: ::core::option::Option<::std::string::String>,
}
#[derive(Debug)]
pub struct WavyLine {
    pub type_: StartStopContinue,
    pub number: ::core::option::Option<::num::BigUint>,
    pub smufl: ::core::option::Option<::std::string::String>,
    pub default_x: ::core::option::Option<::core::primitive::f64>,
    pub default_y: ::core::option::Option<::core::primitive::f64>,
    pub relative_x: ::core::option::Option<::core::primitive::f64>,
    pub relative_y: ::core::option::Option<::core::primitive::f64>,
    pub placement: ::core::option::Option<AboveBelow>,
    pub color: ::core::option::Option<::std::string::String>,
    pub start_note: ::core::option::Option<StartNote>,
    pub trill_step: ::core::option::Option<TrillStep>,
    pub two_note_turn: ::core::option::Option<TwoNoteTurn>,
    pub accelerate: ::core::option::Option<YesNo>,
    pub beats: ::core::option::Option<::core::primitive::f64>,
    pub second_beat: ::core::option::Option<::core::primitive::f64>,
    pub last_beat: ::core::option::Option<::core::primitive::f64>,
}
#[derive(Debug)]
pub struct Wedge {
    pub type_: WedgeType,
    pub number: ::core::option::Option<::num::BigUint>,
    pub spread: ::core::option::Option<::core::primitive::f64>,
    pub niente: ::core::option::Option<YesNo>,
    pub line_type: ::core::option::Option<LineType>,
    pub dash_length: ::core::option::Option<::core::primitive::f64>,
    pub space_length: ::core::option::Option<::core::primitive::f64>,
    pub default_x: ::core::option::Option<::core::primitive::f64>,
    pub default_y: ::core::option::Option<::core::primitive::f64>,
    pub relative_x: ::core::option::Option<::core::primitive::f64>,
    pub relative_y: ::core::option::Option<::core::primitive::f64>,
    pub color: ::core::option::Option<::std::string::String>,
    pub id: ::core::option::Option<::std::string::String>,
}
#[derive(Debug)]
pub enum WedgeType {
    Crescendo,
    Diminuendo,
    Stop,
    Continue,
}
#[derive(Debug)]
pub enum Winged {
    None,
    Straight,
    Curved,
    DoubleStraight,
    DoubleCurved,
}
#[derive(Debug)]
pub struct Wood {
    pub smufl: ::core::option::Option<::std::string::String>,
    pub content: WoodValue,
}
#[derive(Debug)]
pub enum WoodValue {
    BambooScraper,
    BoardClapper,
    Cabasa,
    Castanets,
    CastanetsWithHandle,
    Claves,
    FootballRattle,
    Guiro,
    LogDrum,
    Maraca,
    Maracas,
    Quijada,
    Rainstick,
    Ratchet,
    RecoReco,
    SandpaperBlocks,
    SlitDrum,
    TempleBlock,
    Vibraslap,
    Whip,
    WoodBlock,
}
#[derive(Debug)]
pub struct Work {
    pub content: WorkContent,
}
#[derive(Debug)]
pub struct WorkContent {
    pub work_number: ::core::option::Option<::std::string::String>,
    pub work_title: ::core::option::Option<::std::string::String>,
    pub opus: ::core::option::Option<Opus>,
}
#[derive(Debug)]
pub enum YesNo {
    Yes,
    No,
}
#[derive(Debug)]
pub enum YesNoNumber {
    Yes,
    No,
    F64(::core::primitive::f64),
}
pub type YyyyMmDd = ::std::string::String;
#[derive(Debug)]
pub struct AttributesDirectiveElementType {
    pub default_x: ::core::option::Option<::core::primitive::f64>,
    pub default_y: ::core::option::Option<::core::primitive::f64>,
    pub relative_x: ::core::option::Option<::core::primitive::f64>,
    pub relative_y: ::core::option::Option<::core::primitive::f64>,
    pub font_family: ::core::option::Option<::std::string::String>,
    pub font_style: ::core::option::Option<FontStyle>,
    pub font_size: ::core::option::Option<FontSize>,
    pub font_weight: ::core::option::Option<FontWeight>,
    pub color: ::core::option::Option<::std::string::String>,
    pub lang: ::core::option::Option<::std::string::String>,
    pub content: ::std::string::String,
}
#[derive(Debug)]
pub struct ScorePartwisePartElementType {
    pub id: ::std::string::String,
    pub content: ScorePartwisePartElementTypeContent,
}
#[derive(Debug)]
pub struct ScorePartwisePartElementTypeContent {
    pub measure: ::std::vec::Vec<ScorePartwisePartMeasureElementType>,
}
#[derive(Debug)]
pub struct ScoreTimewiseMeasureElementType {
    pub number: ::std::string::String,
    pub text: ::core::option::Option<::std::string::String>,
    pub implicit: ::core::option::Option<YesNo>,
    pub non_controlling: ::core::option::Option<YesNo>,
    pub width: ::core::option::Option<::core::primitive::f64>,
    pub id: ::core::option::Option<::std::string::String>,
    pub content: ScoreTimewiseMeasureElementTypeContent,
}
#[derive(Debug)]
pub struct ScoreTimewiseMeasureElementTypeContent {
    pub part: ::std::vec::Vec<ScoreTimewiseMeasurePartElementType>,
}
#[derive(Debug)]
pub struct ScorePartwisePartMeasureElementType {
    pub number: ::std::string::String,
    pub text: ::core::option::Option<::std::string::String>,
    pub implicit: ::core::option::Option<YesNo>,
    pub non_controlling: ::core::option::Option<YesNo>,
    pub width: ::core::option::Option<::core::primitive::f64>,
    pub id: ::core::option::Option<::std::string::String>,
    pub content: ::std::vec::Vec<ScorePartwisePartMeasureElementTypeContent>,
}
#[derive(Debug)]
pub enum ScorePartwisePartMeasureElementTypeContent {
    Note(Note),
    Backup(Backup),
    Forward(Forward),
    Direction(Direction),
    Attributes(Attributes),
    Harmony(Harmony),
    FiguredBass(FiguredBass),
    Print(Print),
    Sound(Sound),
    Listening(Listening),
    Barline(Barline),
    Grouping(Grouping),
    Link(Link),
    Bookmark(Bookmark),
}
#[derive(Debug)]
pub struct ScoreTimewiseMeasurePartElementType {
    pub id: ::std::string::String,
    pub content: ::std::vec::Vec<ScoreTimewiseMeasurePartElementTypeContent>,
}
#[derive(Debug)]
pub enum ScoreTimewiseMeasurePartElementTypeContent {
    Note(Note),
    Backup(Backup),
    Forward(Forward),
    Direction(Direction),
    Attributes(Attributes),
    Harmony(Harmony),
    FiguredBass(FiguredBass),
    Print(Print),
    Sound(Sound),
    Listening(Listening),
    Barline(Barline),
    Grouping(Grouping),
    Link(Link),
    Bookmark(Bookmark),
}
pub mod xlink {
    #[derive(Debug)]
    pub enum Actuate {
        OnRequest,
        OnLoad,
        Other,
        None,
    }
    pub type Href = ::std::string::String;
    pub type Role = ::std::string::String;
    #[derive(Debug)]
    pub enum Show {
        New,
        Replace,
        Embed,
        Other,
        None,
    }
    pub type Title = ::std::string::String;
    #[derive(Debug)]
    pub enum Type {
        Simple,
    }
}
pub mod xml {
    pub type Base = ::std::string::String;
    pub type Id = ::std::string::String;
    pub type Lang = ::std::string::String;
    #[derive(Debug)]
    pub enum Space {
        Default,
        Preserve,
    }
}
pub mod xs {
    #[derive(Debug, Default)]
    pub struct Entities(pub ::std::vec::Vec<::std::string::String>);
    #[derive(Debug, Default)]
    pub struct Entity(pub ::std::vec::Vec<::std::string::String>);
    pub type Id = ::std::string::String;
    pub type Idref = ::std::string::String;
    #[derive(Debug, Default)]
    pub struct Idrefs(pub ::std::vec::Vec<::std::string::String>);
    pub type NcName = ::std::string::String;
    pub type Nmtoken = ::std::string::String;
    #[derive(Debug, Default)]
    pub struct Nmtokens(pub ::std::vec::Vec<::std::string::String>);
    pub type Notation = ::std::string::String;
    pub type Name = ::std::string::String;
    pub type QName = ::std::string::String;
    pub type AnySimpleType = ::std::string::String;
    pub type AnyUri = ::std::string::String;
    pub type Base64Binary = ::std::string::String;
    pub type Boolean = ::core::primitive::bool;
    pub type Byte = ::core::primitive::i8;
    pub type Date = ::std::string::String;
    pub type DateTime = ::std::string::String;
    pub type Decimal = ::core::primitive::f64;
    pub type Double = ::core::primitive::f64;
    pub type Duration = ::std::string::String;
    pub type Float = ::core::primitive::f32;
    pub type GDay = ::std::string::String;
    pub type GMonth = ::std::string::String;
    pub type GMonthDay = ::std::string::String;
    pub type GYear = ::std::string::String;
    pub type GYearMonth = ::std::string::String;
    pub type HexBinary = ::std::string::String;
    pub type Int = ::core::primitive::i32;
    pub type Integer = ::num::BigInt;
    pub type Language = ::std::string::String;
    pub type Long = ::core::primitive::i64;
    pub type NegativeInteger = ::num::BigInt;
    pub type NonNegativeInteger = ::num::BigUint;
    pub type NonPositiveInteger = ::num::BigInt;
    pub type NormalizedString = ::std::string::String;
    pub type PositiveInteger = ::num::BigUint;
    pub type Short = ::core::primitive::i16;
    pub type String = ::std::string::String;
    pub type Time = ::std::string::String;
    pub type Token = ::std::string::String;
    pub type UnsignedByte = ::core::primitive::u8;
    pub type UnsignedInt = ::core::primitive::u32;
    pub type UnsignedLong = ::core::primitive::u64;
    pub type UnsignedShort = ::core::primitive::u16;
}
