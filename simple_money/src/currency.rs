use Lotus::LotusBuilder;

struct CurrencyData {
    exponent: u32,
    locale: Locale,
    minor_units: u64,
    name: &'static str,
    symbol: &'static str,
    symbol_first: bool,
}

enum Locale {
    USA,
    EU,
    India,
    Sweden,
}

const aed_currency_data: CurrencyData =  CurrencyData {
    exponent: 2,
    locale: Locale::USA,
    minor_units: 25,
    name: "United Arab Emirates Dirham",
    symbol: "د.إ",
    symbol_first: false,
};

const afn_currency_data: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::USA,
    minor_units: 100,
    name: "Afghan Afghani",
    symbol: "؋",
    symbol_first: false,
};

const all_currency_data: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::EU,
    minor_units: 1,
    name: "Albanian Lek",
    symbol: "L",
    symbol_first: false,
};

const amd_currency_data: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::USA,
    minor_units: 10,
    name: "Armenian Dram",
    symbol: "դր.",
    symbol_first: false,
};

const ang_currency_data: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::USA,
    minor_units: 10,
    name: "Armenian Dram",
    symbol: "դր.",
    symbol_first: false,
};

const aoa_currency_data: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::USA,
    minor_units: 10,
    name: "Angolan Kwanza",
    symbol: "Kz",
    symbol_first: false,
};

const ars_currency_data: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::EU,
    minor_units: 1,
    name: "Argentine Peso",
    symbol: "$",
    symbol_first: true,
};

const aud_currency_data: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::USA,
    minor_units: 5,
    name: "Australian Dollar",
    symbol: "$",
    symbol_first: true,
};

const awg_currency_data: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::USA,
    minor_units: 5,
    name: "Aruban Florin",
    symbol: "ƒ",
    symbol_first: false,
};

const azn_currency_data: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::USA,
    minor_units: 5,
    name: "Bosnia and Herzegovina Convertible Mark",
    symbol: "KM",
    symbol_first: true,
};

const bam_currency_data: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::USA,
    minor_units: 5,
    name: "Bosnia and Herzegovina Convertible Mark",
    symbol: "KM",
    symbol_first: true,
};

const bbd_currency_data: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::USA,
    minor_units: 1,
    name: "Barbadian Dollar",
    symbol: "$",
    symbol_first: true,
};

const bdt_currency_data: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::India,
    minor_units: 1,
    name: "Bangladeshi Taka",
    symbol: "৳",
    symbol_first: true,
};

const bgn_currency_data: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::India,
    minor_units: 1,
    name: "Bulgarian Lev",
    symbol: "лв.",
    symbol_first: false,
};

const bhd_currency_data: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::India,
    minor_units: 1,
    name: "Bulgarian Lev",
    symbol: "лв.",
    symbol_first: false,
};

const bif_currency_data: CurrencyData = CurrencyData {
    exponent: 0,
    locale: Locale::USA,
    minor_units: 100,
    name: "Burundia Franc",
    symbol: "Fr",
    symbol_first: false,
};

const bmd_currency_data: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::USA,
    minor_units: 1,
    name: "Bermudian Dollar",
    symbol: "$",
    symbol_first: true,
};

const bnd_currency_data: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::USA,
    minor_units: 1,
    name: "Brunei Dollar",
    symbol: "$",
    symbol_first: true,
};

const bob_currency_data: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::USA,
    minor_units: 10,
    name: "Bolivian Boliviano",
    symbol: "Bs.",
    symbol_first: true,
};

const brl_currency_data: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::USA,
    minor_units: 5,
    name: "Brazilian real",
    symbol: "R$",
    symbol_first: true,
};

const bsd_currency_data: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::USA,
    minor_units: 1,
    name: "Bahamian Dollar",
    symbol: "$",
    symbol_first: true,
};

const btn_currency_data: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::USA,
    minor_units: 5,
    name: "Bhutanese Ngultrum",
    symbol: "Nu.",
    symbol_first: false,
};

const bwp_currency_data: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 5,
    name: "Botswana Pula",
    symbol: "P",
    symbol_first: true,
};

const byn_currency_data: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::Sweden,
    minor_units: 1,
    name: "Belarusian Ruble",
    symbol: "Br",
    symbol_first: false,
};

const byr_currency_data: CurrencyData = CurrencyData{
    exponent: 0,
    locale: Locale::Sweden,
    minor_units: 100,
    name: "Belarusian Ruble",
    symbol: "Br",
    symbol_first: false,
};

const bzd_currency_data: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 1,
    name: "Belize Dollar",
    symbol: "$",
    symbol_first: true,
};

const cad_currency_data: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 5,
    name: "Canadian Dollar",
    symbol: "$",
    symbol_first: true,
};

const cdf_currency_data: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 1,
    name: "Congolese Franc",
    symbol: "Fr",
    symbol_first: false,
};

const chf_currency_data: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 5,
    name: "Swiss Franc",
    symbol: "Fr",
    symbol_first: true,
};

const clf_currency_data: CurrencyData = CurrencyData{
    exponent: 4,
    locale: Locale::EU,
    minor_units: 5,
    name: "Unidad de Fomento",
    symbol: "UF",
    symbol_first: true,
};

const clp_currency_data: CurrencyData = CurrencyData{
    exponent: 0,
    locale: Locale::EU,
    minor_units: 1,
    name: "Chilean Peso",
    symbol: "$",
    symbol_first: true,
};

const cny_currency_data: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 1,
    name: "Chinese Renminbi Yuan",
    symbol: "¥",
    symbol_first: true,
};

const cop_currency_data: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::EU,
    minor_units: 20,
    name: "Colombian Peso",
    symbol: "$",
    symbol_first: true,
};

const crc_currency_data: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::EU,
    minor_units: 100,
    name: "Costa Rican Colon",
    symbol: "₡",
    symbol_first: true,
};

const cuc_currency_data: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::USA,
    minor_units: 1,
    name: "Cuban Convertible Peso",
    symbol: "$",
    symbol_first: false,
};

const cup_currency_data: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::USA,
    minor_units: 1,
    name: "Cuban Peso",
    symbol: "$",
    symbol_first: true,
};

const cve_currency_data: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::USA,
    minor_units: 100,
    name: "Cape Verdean Escudo",
    symbol: "$",
    symbol_first: false,
};

const czk_currency_data: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::Sweden,
    minor_units: 100,
    name: "Czech Koruna",
    symbol: "Kč",
    symbol_first: false,
};

const djf_currency_data: CurrencyData = CurrencyData {
    exponent: 0,
    locale: Locale::USA,
    minor_units: 100,
    name: "Djiboutian Franc",
    symbol: "Fdj",
    symbol_first: false,
};

const dkk_currency_data: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::EU,
    minor_units: 50,
    name: "Danish Krone",
    symbol: "kr.",
    symbol_first: false,
};

const dop_currency_data: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::USA,
    minor_units: 100,
    name: "Dominican Peso",
    symbol: "$",
    symbol_first: true,
};

const dzd_currency_data: CurrencyData = CurrencyData{
    exponent: 2,
    locale:  Locale::USA,
    minor_units: 100,
    name: "Algerian Dinar",
    symbol: "د.ج",
    symbol_first: false,
};

const egp_currency_data: CurrencyData= CurrencyData {
    exponent: 2,
    locale: Locale::USA,
    minor_units: 25,
    name: "Egyptian Pound",
    symbol: "ج.م",
    symbol_first: true,
};

const ern_currency_data: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 1,
    name: "Eritrean Nakfa",
    symbol: "Nfk",
    symbol_first: false,
};

const etb_currency_data: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::USA,
    minor_units: 1,
    name: "Ethiopian Birr",
    symbol: "Br",
    symbol_first: false,
};

const eur_currency_data: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::EU,
    minor_units: 1,
    name: "Euro",
    symbol: "€",
    symbol_first: true,
};

const fjd_currency_data: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::EU,
    minor_units: 5,
    name: "Fijian Dollar",
    symbol: "$",
    symbol_first: false,
};

const fkp_currency_data: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::EU,
    minor_units: 1,
    name: "Falkland Pound",
    symbol: "£",
    symbol_first: false,
};

const gbp_currency_data: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::USA,
    minor_units: 1,
    name: "British Pound",
    symbol: "£",
    symbol_first: true,
};

const gel_currency_data: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 1,
    name: "Georgian Lari",
    symbol: "ლ",
    symbol_first: false,
};

const ghs_currency_data: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::USA,
    minor_units: 1,
    name: "Ghanaian Cedi",
    symbol: "₵",
    symbol_first: true,
};

const gip_currency_data: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::USA,
    minor_units: 1,
    name: "Gibraltar Pound",
    symbol: "£",
    symbol_first: true,
};

const gnf_currency_data: CurrencyData = CurrencyData {
    exponent: 0,
    locale: Locale::USA,
    minor_units: 100,
    name: "Guinean Franc",
    symbol: "Fr",
    symbol_first: false,
};

const gtq_currency_data: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::USA,
    minor_units: 1,
    name: "Guatemalan Quetzal",
    symbol: "Q",
    symbol_first: true,
};

const gyd_currency_data: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::USA,
    minor_units: 100,
    name: "Guyanese Dollar",
    symbol: "$",
    symbol_first: false,
};

const hkd_currency_data: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 10,
    name: "Hong Kong Dollar",
    symbol: "$",
    symbol_first: true,
};

const hnl_currency_data: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 5,
    name: "Honduran Lempira",
    symbol: "L",
    symbol_first: true,
};

const hrk_currency_data: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::EU,
    minor_units: 1,
    name: "Croatian Kuna",
    symbol: "kn",
    symbol_first: false,
};

const htg_currency_data: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 5,
    name: "Haitian Gourde",
    symbol: "G",
    symbol_first: false,
};

const huf_currency_data: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::Sweden,
    minor_units: 5,
    name: "Hungarian Forint",
    symbol: "Ft",
    symbol_first: false,
};

const idr_currency_data: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::USA,
    minor_units: 5000,
    name: "Indonesian Rupiah",
    symbol: "Rp",
    symbol_first: true,
};

const ils_currency_data: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::USA,
    minor_units: 10,
    name: "Israeli New Sheqel",
    symbol: "₪",
    symbol_first: true,
};

const inr_currency_data: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::India,
    minor_units: 50,
    name: "Indian Rupee",
    symbol: "₹",
    symbol_first: true,
};

const iqd_currency_data: CurrencyData  = CurrencyData {
    exponent: 3,
    locale: Locale::USA,
    minor_units: 50000,
    name: "Iraqi Dinar",
    symbol: "ع.د",
    symbol_first: false,
};

const irr_currency_data: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::USA,
    minor_units: 5000,
    name: "Iranian Rial",
    symbol: "﷼",
    symbol_first: true,
};

const isk_currency_data: CurrencyData = CurrencyData{
    exponent: 0,
    locale: Locale::EU,
    minor_units: 1,
    name: "Icelandic Krona",
    symbol: "kr.",
    symbol_first: true,
};

const jmd_currency_data: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::USA,
    minor_units: 1,
    name: "Jamaican Dollar",
    symbol: "$",
    symbol_first: true,
};

const jod_currency_data: CurrencyData = CurrencyData {
    exponent: 3,
    locale: Locale::USA,
    minor_units: 5,
    name: "Jordanian Dinar",
    symbol: "د.ا",
    symbol_first: true,
};

const jpy_currency_data: CurrencyData = CurrencyData{
    exponent: 0,
    locale: Locale::USA,
    minor_units: 1,
    name: "Japanese Yen",
    symbol: "¥",
    symbol_first: true,
};

const kes_currency_data: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::USA,
    minor_units: 50,
    name: "Kenyan Shilling",
    symbol: "KSh",
    symbol_first: true,
};

const kgs_currency_data: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::USA,
    minor_units: 1,
    name: "Kyrgzstani Som",
    symbol: "som",
    symbol_first: false,
};

const khr_currency_data: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::USA,
    minor_units: 5000,
    name: "Cambodian Riel",
    symbol: "៛",
    symbol_first: false,
};

const kmf_currency_data: CurrencyData = CurrencyData{
    exponent: 0,
    locale: Locale::USA,
    minor_units: 100,
    name: "Comorian Franc",
    symbol: "Fr",
    symbol_first: false,
};

const kpw_currency_data: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::USA,
    minor_units: 1,
    name: "North Korean Won",
    symbol: "W",
    symbol_first: false,
};

const krw_currency_data: CurrencyData = CurrencyData {
    exponent: 0,
    locale: Locale::USA,
    minor_units: 1,
    name: "South Korean Won",
    symbol: "W",
    symbol_first: true,
};

const kwd_currency_data: CurrencyData = CurrencyData {
    exponent: 3,
    locale: Locale::USA,
    minor_units: 5,
    name: "Kuwaiti Dinar",
    symbol: "د.ك",
    symbol_first: true,
};

const kyd_currency_data: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::USA,
    minor_units: 1,
    name: "Cayman Islands Dollar",
    symbol: "$",
    symbol_first: true,
};

const kzt_currency_data: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::USA,
    minor_units: 100,
    name: "Kazakhstani Tenge",
    symbol: "₸",
    symbol_first: false,
};

const lak_currency_data: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::USA,
    minor_units: 10,
    name: "Lao Kip",
    symbol: "K",
    symbol_first: false,
};

const lbp_currency_data: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::USA,
    minor_units: 25000,
    name: "Lebanese Pound",
    symbol: "ل.ل",
    symbol_first: true,
};

const lkr_currency_data: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::USA,
    minor_units: 100,
    name: "Sri Lankan Rupee",
    symbol: "₨",
    symbol_first: false,
};

const lrd_currency_data: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::USA,
    minor_units: 5,
    name: "Liberian Dollar",
    symbol: "$",
    symbol_first: false,
};

const lsl_currency_data: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::USA,
    minor_units: 1,
    name: "Lesotho Loti",
    symbol: "L",
    symbol_first: false,
};

const lyd_currency_data: CurrencyData = CurrencyData {
    exponent: 3,
    locale: Locale::USA,
    minor_units: 50,
    name: "Libyan Dinar",
    symbol: "ل.د",
    symbol_first: false,              
};

const mad_currency_data: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::USA,
    minor_units: 1,
    name: "Morocco Dirham",
    symbol: "د.م.",
    symbol_first: false,
};


const mdl_currency_data: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::USA,
    minor_units: 1,
    name: "Moldovan Leu",
    symbol: "L",
    symbol_first: false,
};

const mkd_currency_data: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::USA,
    minor_units: 100,
    name: "Macedonian Denar",
    symbol: "ден",
    symbol_first: false,
};

const mmk_currency_data: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 50,
    name: "Myanmar Kyat",
    symbol: "K",
    symbol_first: false,
};

const mnt_currency_data: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::USA,
    minor_units: 2000,
    name: "Mongolian Tögrög",
    symbol: "₮",
    symbol_first: false,
};

const mop_currency_data: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::USA,
    minor_units: 10,
    name: "Macanese Pataca",
    symbol: "P",
    symbol_first: false,
};

const mur_currency_data: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 100,
    name: "Mauritian Rupee",
    symbol: "₨",
    symbol_first: true,
};

const mvr_currency_data: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 100,
    name: "Maldivian Rufiyaa",
    symbol: "MVR",
    symbol_first: false,
};

const mwk_currency_data: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 1,
    name: "Malawian Kwacha",
    symbol: "MK",
    symbol_first: false,
};

const mxn_currency_data: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 5,
    name: "Mexican Peso",
    symbol: "$",
    symbol_first: true,
};

const myr_currency_data: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 5,
    name: "Malaysian Ringgit",
    symbol: "RM",
    symbol_first: true,
};

const mzn_currency_data: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 1,
    name: "Mozambican Metical",
    symbol: "MTn",
    symbol_first: true,
};

const nad_currency_data: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 5,
    name: "Namibian Dollar",
    symbol: "$",
    symbol_first: false,
};

const ngn_currency_data: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 50,
    name: "Nigerian Naira",
    symbol: "₦",
    symbol_first: true,
};

const nio_currency_data: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 5,
    name: "Nicaraguan Córdoba",
    symbol: "C$",
    symbol_first: true,
};

const nok_currency_data: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 100,
    name: "Norwegian Krone",
    symbol: "kr",
    symbol_first: false,
};

const npr_currency_data: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 1,
    name: "Nepalese Rupee",
    symbol: "रु",
    symbol_first: true,
};

const nzd_currency_data: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 10,
    name: "New Zealand Dollar",
    symbol: "$",
    symbol_first: true,
};

const omr_currency_data: CurrencyData = CurrencyData{
    exponent: 3,
    locale: Locale::USA,
    minor_units: 5,
    name: "Omani Rial",
    symbol: "ر.ع.",
    symbol_first: true,
};

const pab_currency_data: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 1,
    name: "Panamanian Balboa",
    symbol: "B/.",
    symbol_first: true,
};

const pen_currency_data: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 1,
    name: "Peruvian Sol",
    symbol: "S/",
    symbol_first: true,
};

const pgk_currency_data: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 5,
    name: "Papua New Guinean Kina",
    symbol: "K",
    symbol_first: false,
};

const php_currency_data: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 1,
    name: "Philippine Peso",
    symbol: "₱",
    symbol_first: true,
};

const pkr_currency_data: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 100,
    name: "Pakistani Rupee",
    symbol: "₨",
    symbol_first: true,
};

const pln_currency_data: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::Sweden,
    minor_units: 1,
    name: "Polish Złoty",
    symbol: "zł",
    symbol_first: false,
};

const pyg_currency_data: CurrencyData = CurrencyData {
    exponent: 0,
    locale: Locale::Sweden,
    minor_units: 5000,
    name: "Paraguayan Guaraní",
    symbol: "₲",
    symbol_first: true,
};

const qar_currency_data: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::Sweden,
    minor_units: 1,
    name: "Qatari Riyal",
    symbol: "ر.ق",
    symbol_first: false,
};

const ron_currency_data: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::EU,
    minor_units: 1,
    name: "Romanian Leu",
    symbol: "ر.ق",
    symbol_first: false,
};

const rsd_currency_data: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 100,
    name: "Serbian Dinar",
    symbol: "РСД",
    symbol_first: true,
};

const rub_currency_data: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::EU,
    minor_units: 1,
    name: "Russian Ruble",
    symbol: "₽",
    symbol_first: false,
};

const rwf_currency_data: CurrencyData = CurrencyData{
    exponent: 0,
    locale: Locale::USA,
    minor_units: 100,
    name: "Rwandan Franc",
    symbol: "FRw",
    symbol_first: false,
};

const sar_currency_data: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 5,
    name: "Saudi Riyal",
    symbol: "ر.س",
    symbol_first: true,
};

const sbd_currency_data: CurrencyData = CurrencyData{ 
    exponent: 2,
    locale: Locale::USA,
    minor_units: 10,
    name: "Solomon Islands Dollar",
    symbol: "$",
    symbol_first: false,
};

const scr_currency_data: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 1,
    name: "Seychellois Rupee",
    symbol: "₨",
    symbol_first: false,
};

const sdg_currency_data: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 1,
    name: "Sudanese Pound",
    symbol: "£",
    symbol_first: true,
};

const sek_currency_data: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::Sweden,
    minor_units: 100,
    name: "Swedish Krona",
    symbol: "kr",
    symbol_first: false,
};

const sgd_currency_data: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 1,
    name: "Singapore Dollar",
    symbol: "$",
    symbol_first: true,
};

const shp_currency_data: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 1,
    name: "Saint Helenian Pound",
    symbol: "£",
    symbol_first: false,
};

const skk_currency_data: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 50,
    name: "Slovak Koruna",
    symbol: "Sk",
    symbol_first: true,
};

const sll_currency_data: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 1000,
    name: "Sierra Leonean Leone",
    symbol: "Le",
    symbol_first: false,
};

const sos_currency_data: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 1,
    name: "Somali Shilling",
    symbol: "Sh",
    symbol_first: false,
};

const srd_currency_data: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 1,
    name: "Surinamese Dollar",
    symbol: "$",
    symbol_first: false,
};

const ssp_currency_data: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 5,
    name: "South Sudanese Pound",
    symbol: "£",
    symbol_first: false,
};

const std_currency_data: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 10000,
    name: "São Tomé and Príncipe Dobra",
    symbol: "Db",
    symbol_first: false,
};

const stn_currency_data: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 10,
    name: "São Tomé and Príncipe Dobra",
    symbol: "Db",
    symbol_first: false,
};

const svc_currency_data: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 1,
    name: "Salvadoran Colón",
    symbol: "₡",
    symbol_first: true,
};

const syp_currency_data: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 100,
    name: "Syrian Pound",
    symbol: "£S",
    symbol_first: false,
};

const szl_currency_data: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 1,
    name: "Swazi Lilangeni",
    symbol: "E",
    symbol_first: true,
};

const thb_currency_data: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 1,
    name: "Thai Baht",
    symbol: "฿",
    symbol_first: true,
};

const tjs_currency_data: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::USA,
    minor_units: 1,
    name: "Tajikistani Somoni",
    symbol: "ЅМ",
    symbol_first: false,
};

const tmt_currency_data: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::USA,
    minor_units: 1,
    name: "Turkmenistani Manat",
    symbol: "T",
    symbol_first: false,
};

const tnd_currency_data: CurrencyData = CurrencyData{
    exponent: 3,
    locale: Locale::USA,
    minor_units: 10,
    name: "Tunisian Dinar",
    symbol: "د.ت",
    symbol_first: false,
};

const top_currency_data: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 1,
    name: "Tongan Paʻanga",
    symbol: "T$",
    symbol_first: true,
};

const try_currency_data: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::EU,
    minor_units: 1,
    name: "Turkish Lira",
    symbol: "₺",
    symbol_first: true,
};

const ttd_currency_data: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 1,
    name: "Trinidad and Tobago Dollar",
    symbol: "$",
    symbol_first: false,
};

const twd_currency_data: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 50,
    name: "New Taiwan Dollar",
    symbol: "$",
    symbol_first: true,
};

const tzs_currency_data: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 5000,
    name: "Tanzanian Shilling",
    symbol: "Sh",
    symbol_first: true,
};

const uah_currency_data: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 1,
    name: "Ukrainian Hryvnia",
    symbol: "₴",
    symbol_first: false,
};

const ugx_currency_data: CurrencyData = CurrencyData{
    exponent: 0,
    locale: Locale::USA,
    minor_units: 1000,
    name: "Ugandan Shilling",
    symbol: "USh",
    symbol_first: false,
};

const usd_currency_data: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 1,
    name: "United States Dollar",
    symbol: "$",
    symbol_first: true,
};

const uyu_currency_data: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::EU,
    minor_units: 100,
    name: "Uruguayan Peso",
    symbol: "$U",
    symbol_first: true,
};

const uyw_currency_data: CurrencyData = CurrencyData{
    exponent: 4,
    locale: Locale::EU,
    minor_units: 1000,
    name: "Unidad Previsional",
    symbol: "UP",
    symbol_first: true,
};

const uzs_currency_data: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 100,
    name: "Uzbekistan Som",
    symbol: "so'm",
    symbol_first: false,
};

const ves_currency_data: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::EU,
    minor_units: 1,
    name: "Venezuelan Bolívar Soberano",
    symbol: "Bs",
    symbol_first: true,
};

const vnd_currency_data: CurrencyData = CurrencyData{
    exponent: 0,
    locale: Locale::EU,
    minor_units: 100,
    name: "Vietnamese Đồng",
    symbol: "₫",
    symbol_first: false,
};

const vuv_currency_data: CurrencyData = CurrencyData{
    exponent: 0,
    locale: Locale::USA,
    minor_units: 1,
    name: "Vanuatu Vatu",
    symbol: "Vt",
    symbol_first: true,
};

const wst_currency_data: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 10,
    name: "Samoan Tala",
    symbol: "T",
    symbol_first: false,
};

const xaf_currency_data: CurrencyData = CurrencyData{
    exponent: 0,
    locale: Locale::USA,
    minor_units: 100,
    name: "Central African Cfa Franc",
    symbol: "CFA",
    symbol_first: false,
};

const xag_currency_data: CurrencyData = CurrencyData{
    exponent: 0,
    locale: Locale::USA,
    minor_units: 100,
    name: "Silver (Troy Ounce)",
    symbol: "oz t",
    symbol_first: false,
};

const xau_currency_data: CurrencyData = CurrencyData{
    exponent: 0,
    locale: Locale::USA,
    minor_units: 100,
    name: "Gold (Troy Ounce)",
    symbol: "oz t",
    symbol_first: false,
};

const xba_currency_data: CurrencyData = CurrencyData{
    exponent: 0,
    locale: Locale::USA,
    minor_units: 100,
    name: "European Composite Unit",
    symbol: "",
    symbol_first: false,
};

const xbb_currency_data: CurrencyData = CurrencyData{
    exponent: 0,
    locale: Locale::USA,
    minor_units: 100,
    name: "European Monetary Unit",
    symbol: "",
    symbol_first: false,
};

const xbc_currency_data: CurrencyData = CurrencyData{
    exponent: 0,
    locale: Locale::USA,
    minor_units: 100,
    name: "European Unit of Account 9",
    symbol: "",
    symbol_first: false,
};

const xbd_currency_data: CurrencyData = CurrencyData{
    exponent: 0,
    locale: Locale::USA,
    minor_units: 100,
    name: "European Unit of Account 17",
    symbol: "",
    symbol_first: false,
};

const xcd_currency_data: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 1,
    name: "East Caribbean Dollar",
    symbol: "$",
    symbol_first: true,
};

const xdr_currency_data: CurrencyData = CurrencyData{
    exponent: 0,
    locale: Locale::USA,
    minor_units: 100,
    name: "Special Drawing Rights",
    symbol: "SDR",
    symbol_first: false,
};

const xof_currency_data: CurrencyData = CurrencyData{
    exponent: 0,
    locale: Locale::USA,
    minor_units: 100,
    name: "West African Cfa Franc",
    symbol: "Fr",
    symbol_first: false,
};

const xpd_currency_data: CurrencyData = CurrencyData{
    exponent: 0,
    locale: Locale::USA,
    minor_units: 100,
    name: "Palladium",
    symbol: "oz t",
    symbol_first: false,
};

const xpf_currency_data: CurrencyData = CurrencyData{ 
    exponent: 0,
    locale: Locale::USA,
    minor_units: 100,
    name: "Cfp Franc",
    symbol: "Fr",
    symbol_first: false,
};

const xpt_currency_data: CurrencyData = CurrencyData{
    exponent: 0,
    locale: Locale::USA,
    minor_units: 100,
    name: "Platinum",
    symbol: "oz t",
    symbol_first: false,
};

const xts_currency_data: CurrencyData = CurrencyData{
    exponent: 0,
    locale: Locale::USA,
    minor_units: 100,
    name: "Codes specifically reserved for testing purposes",
    symbol: "oz t",
    symbol_first: false,
};

const yer_currency_data: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 100,
    name: "Yemeni Rial",
    symbol: "﷼",
    symbol_first: false,
};

const zar_currency_data: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 10,
    name: "South African Rand",
    symbol: "R",
    symbol_first: true,
};

const zmk_currency_data: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 5,
    name: "Zambian Kwacha",
    symbol: "ZK",
    symbol_first: false,
};

const zmw_currency_data: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 5,
    name: "Zambian Kwacha",
    symbol: "K",
    symbol_first: true,
};

const zwl_currency_data: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 1,
    name: "Zimbabwe Dollar",
    symbol: "Z$",
    symbol_first: true,
};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Currency {
    INR,
    TWD,
    BYN,
    DOP,
    PAB,
    GTQ,
    LKR,
    HUF,
    XAF,
    CVE,
    GNF,
    USD,
    XDR,
    LAK,
    BBD,
    KWD,
    SHP,
    EGP,
    GYD,
    SGD,
    KMF,
    VND,
    BWP,
    PEN,
    XAG,
    HTG,
    MXN,
    RUB,
    ZMW,
    XBD,
    XCD,
    ETB,
    MZN,
    BND,
    OMR,
    TOP,
    KPW,
    SZL,
    MKD,
    GEL,
    VES,
    KES,
    QAR,
    KGS,
    DZD,
    DJF,
    XPF,
    IQD,
    CLF,
    AZN,
    BGN,
    PKR,
    UZS,
    LRD,
    CHF,
    BAM,
    GIP,
    BIF,
    SDG,
    AUD,
    NGN,
    MYR,
    AOA,
    BZD,
    XPD,
    MVR,
    AWG,
    ALL,
    XBB,
    XBA,
    JPY,
    LSL,
    SCR,
    XAU,
    RWF,
    BMD,
    TRY,
    LYD,
    NOK,
    SLL,
    MWK,
    NZD,
    CRC,
    UGX,
    CUC,
    TZS,
    STD,
    KZT,
    PGK,
    MMK,
    ZAR,
    SYP,
    ERN,
    SRD,
    TTD,
    UYU,
    SOS,
    BSD,
    TND,
    NAD,
    CNY,
    MOP,
    RON,
    KRW,
    BYR,
    TMT,
    CLP,
    BDT,
    HKD,
    STN,
    AFN,
    GHS,
    COP,
    LBP,
    MNT,
    UYW,
    ILS,
    ZWL,
    NPR,
    XPT,
    ISK,
    FJD,
    KHR,
    IRR,
    RSD,
    BOB,
    ZMK,
    CDF,
    ANG,
    DKK,
    XBC,
    GBP,
    HNL,
    AED,
    KYD,
    BTN,
    HRK,
    PYG,
    XTS,
    UAH,
    SAR,
    IDR,
    XOF,
    SVC,
    TJS,
    MDL,
    BRL,
    JOD,
    WST,
    SSP,
    JMD,
    EUR,
    THB,
    PHP,
    BHD,
    CZK,
    FKP,
    SEK,
    MAD,
    PLN,
    AMD,
    CUP,
    VUV,
    NIO,
    ARS,
    SKK,
    SBD,
    CAD,
    MUR,
}

impl Currency {
    fn match_currency_to_data(&self) -> CurrencyData {
        match self {
            Currency::AED => aed_currency_data,
            Currency::AFN => afn_currency_data,
            Currency::ALL => all_currency_data,
            Currency::AMD => amd_currency_data,
            Currency::ANG => ang_currency_data,
            Currency::AOA => aoa_currency_data,
            Currency::ARS => ars_currency_data,
            Currency::AUD => aud_currency_data,
            Currency::AWG => awg_currency_data,
            Currency::AZN => azn_currency_data,
            Currency::BAM => bam_currency_data,
            Currency::BBD => bbd_currency_data,
            Currency::BDT => bdt_currency_data,
            Currency::BGN => bgn_currency_data,
            Currency::BHD => bhd_currency_data,
            Currency::BIF => bif_currency_data,
            Currency::BMD => bmd_currency_data,
            Currency::BND => bnd_currency_data,
            Currency::BOB => bob_currency_data,
            Currency::BRL => brl_currency_data,
            Currency::BSD => bsd_currency_data,
            Currency::BTN => btn_currency_data,
            Currency::BWP => bwp_currency_data,
            Currency::BYN => byn_currency_data,
            Currency::BYR => byr_currency_data,
            Currency::BZD => bzd_currency_data,
            Currency::CAD => cad_currency_data,
            Currency::CDF => cdf_currency_data,
            Currency::CHF => chf_currency_data,
            Currency::CLF => clf_currency_data,
            Currency::CLP => clp_currency_data,
            Currency::CNY => cny_currency_data,
            Currency::COP => cop_currency_data,
            Currency::CRC => crc_currency_data,
            Currency::CUC => cuc_currency_data,
            Currency::CUP => cup_currency_data,
            Currency::CVE => cve_currency_data,
            Currency::CZK => czk_currency_data,
            Currency::DJF => djf_currency_data,
            Currency::DKK => dkk_currency_data,
            Currency::DOP => dop_currency_data,
            Currency::DZD => dzd_currency_data,
            Currency::EGP => egp_currency_data,
            Currency::ERN => ern_currency_data,
            Currency::ETB => etb_currency_data,
            Currency::EUR => eur_currency_data,
            Currency::FJD => fjd_currency_data,
            Currency::FKP => fkp_currency_data,
            Currency::GBP => gbp_currency_data,
            Currency::GEL => gel_currency_data,
            Currency::GHS => ghs_currency_data,
            Currency::GIP => gip_currency_data,
            Currency::GNF => gnf_currency_data,
            Currency::GTQ => gtq_currency_data,
            Currency::GYD => gyd_currency_data,
            Currency::HKD => hkd_currency_data,
            Currency::HNL => hnl_currency_data,
            Currency::HRK => hrk_currency_data,
            Currency::HTG => htg_currency_data,
            Currency::HUF => huf_currency_data,
            Currency::IDR => idr_currency_data,
            Currency::ILS => ils_currency_data,
            Currency::INR => inr_currency_data,
            Currency::IQD => iqd_currency_data,
            Currency::IRR => irr_currency_data,
            Currency::ISK => isk_currency_data,
            Currency::JMD => jmd_currency_data,
            Currency::JOD => jod_currency_data,
            Currency::JPY => jpy_currency_data,
            Currency::KES => kes_currency_data,
            Currency::KGS => kgs_currency_data,
            Currency::KHR => khr_currency_data,
            Currency::KMF => kmf_currency_data,
            Currency::KPW => kpw_currency_data,
            Currency::KRW => krw_currency_data,
            Currency::KWD => kwd_currency_data,
            Currency::KYD => kyd_currency_data,
            Currency::KZT => kzt_currency_data,
            Currency::LAK => lak_currency_data,
            Currency::LBP => lbp_currency_data,
            Currency::LKR => lkr_currency_data,
            Currency::LRD => lrd_currency_data,
            Currency::LSL => lsl_currency_data,
            Currency::LYD => lyd_currency_data,
            Currency::MAD => mad_currency_data,
            Currency::MDL => mdl_currency_data,
            Currency::MKD => mkd_currency_data,
            Currency::MMK => mmk_currency_data,
            Currency::MNT => mnt_currency_data,
            Currency::MOP => mop_currency_data,
            Currency::MUR => mur_currency_data,
            Currency::MVR => mvr_currency_data,
            Currency::MWK => mwk_currency_data,
            Currency::MXN => mxn_currency_data,
            Currency::MYR => myr_currency_data,
            Currency::MZN => mzn_currency_data,
            Currency::NAD => nad_currency_data,
            Currency::NGN => ngn_currency_data,
            Currency::NIO => nio_currency_data,
            Currency::NOK => nok_currency_data,
            Currency::NPR => npr_currency_data,
            Currency::NZD => nzd_currency_data,
            Currency::OMR => omr_currency_data,
            Currency::PAB => pab_currency_data,
            Currency::PEN => pen_currency_data,
            Currency::PGK => pgk_currency_data,
            Currency::PHP => php_currency_data,
            Currency::PKR => pkr_currency_data,
            Currency::PLN => pln_currency_data,
            Currency::PYG => pyg_currency_data,
            Currency::QAR => qar_currency_data,
            Currency::RON => ron_currency_data,
            Currency::RSD => rsd_currency_data,
            Currency::RUB => rub_currency_data,
            Currency::RWF => rwf_currency_data,
            Currency::SAR => sar_currency_data,
            Currency::SBD => sbd_currency_data,
            Currency::SCR => scr_currency_data,
            Currency::SDG => sdg_currency_data,
            Currency::SEK => sek_currency_data,
            Currency::SGD => sgd_currency_data,
            Currency::SHP => shp_currency_data,
            Currency::SKK => skk_currency_data,
            Currency::SLL => sll_currency_data,
            Currency::SOS => sos_currency_data,
            Currency::SRD => srd_currency_data,
            Currency::SSP => ssp_currency_data,
            Currency::STD => std_currency_data,
            Currency::STN => stn_currency_data,
            Currency::SVC => svc_currency_data,
            Currency::SYP => syp_currency_data,
            Currency::SZL => szl_currency_data,
            Currency::THB => thb_currency_data,
            Currency::TJS => tjs_currency_data,
            Currency::TMT => tmt_currency_data,
            Currency::TND => tnd_currency_data,
            Currency::TOP => top_currency_data,
            Currency::TRY => try_currency_data,
            Currency::TTD => ttd_currency_data,
            Currency::TWD => twd_currency_data,
            Currency::TZS => tzs_currency_data,
            Currency::UAH => uah_currency_data,
            Currency::UGX => ugx_currency_data,
            Currency::USD => usd_currency_data,
            Currency::UYU => uyu_currency_data,
            Currency::UYW => uyw_currency_data,
            Currency::UZS => uzs_currency_data,
            Currency::VES => ves_currency_data,
            Currency::VND => vnd_currency_data,
            Currency::VUV => vuv_currency_data,
            Currency::WST => wst_currency_data,
            Currency::XAF => xaf_currency_data,
            Currency::XAG => xag_currency_data,
            Currency::XAU => xau_currency_data,
            Currency::XBA => xba_currency_data,
            Currency::XBB => xbb_currency_data,
            Currency::XBC => xbc_currency_data,
            Currency::XBD => xbd_currency_data,
            Currency::XCD => xcd_currency_data,
            Currency::XDR => xdr_currency_data,
            Currency::XOF => xof_currency_data,
            Currency::XPD => xpd_currency_data,
            Currency::XPF => xpf_currency_data,
            Currency::XPT => xpt_currency_data,
            Currency::XTS => xts_currency_data,
            Currency::ZAR => zar_currency_data,
            Currency::ZMK => zmk_currency_data,
            Currency::ZMW => zmw_currency_data,
            Currency::ZWL => zwl_currency_data,
        }
    }

    pub fn match_currency_to_lotus(&self) -> Result<crate::Lotus, String> {
       let currency_data = self.match_currency_to_data();

       let lotus_builder = LotusBuilder::default();
       lotus_builder.symbol(currency_data.symbol);
       lotus_builder.precision(currency_data.exponent);
       
       if currency_data.symbol_first {
           lotus_builder.format_positive("{symbol}{value}");
           lotus_builder.format_negative("{symbol}({value})");
           match currency_data.locale{
               Locale::USA | Locale::India | Locale::Sweden => lotus_builder.format_zero("{symbol}0.00"),
               Locale::EU => lotus_builder.format_zero("{symbol}0,00"),
           }
       } else {
           lotus_builder.format_positive("{value}{symbol}");
           lotus_builder.format_negative("{value}{symbol}");
           match currency_data.locale{
               Locale::USA | Locale::India | Locale::Sweden => lotus_builder.format_zero("0.00{symbol}"),
               Locale::EU => lotus_builder.format_zero("0,00{symbol}")
           }
       }

       match currency_data.locale{
           Locale::USA | Locale::India | Locale::Sweden => {
               lotus_builder.thousand_str(",");
               lotus_builder.decimal_str(".");
           },
           Locale::EU => {
               lotus_builder.thousand_str(".");
               lotus_builder.decimal_str(",");
           }
       }

       return lotus_builder.build();
    }
}