use Lotus::LotusBuilder;

struct CurrencyData {
    exponent: u8,
    locale: Locale,
    minor_units: u64,
    name: &'static str,
    symbol: &'static str,
    symbol_first: bool,
}

enum Locale {
    USA,
    EU,
    // TODO: Separate digits for India based on Indian numbering system
    India,
    Poland,
}

const AED_CURRENCY_DATA: CurrencyData =  CurrencyData {
    exponent: 2,
    locale: Locale::USA,
    minor_units: 25,
    name: "United Arab Emirates Dirham",
    symbol: "د.إ",
    symbol_first: false,
};

const AFN_CURRENCY_DATA: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::USA,
    minor_units: 100,
    name: "Afghan Afghani",
    symbol: "؋",
    symbol_first: false,
};

const ALL_CURRENCY_DATA: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::EU,
    minor_units: 1,
    name: "Albanian Lek",
    symbol: "L",
    symbol_first: false,
};

const AMD_CURRENCY_DATA: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::USA,
    minor_units: 10,
    name: "Armenian Dram",
    symbol: "դր.",
    symbol_first: false,
};

const ANG_CURRENCY_DATA: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::USA,
    minor_units: 10,
    name: "Armenian Dram",
    symbol: "դր.",
    symbol_first: false,
};

const AOA_CURRENCY_DATA: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::USA,
    minor_units: 10,
    name: "Angolan Kwanza",
    symbol: "Kz",
    symbol_first: false,
};

const ARS_CURRENCY_DATA: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::EU,
    minor_units: 1,
    name: "Argentine Peso",
    symbol: "$",
    symbol_first: true,
};

const AUD_CURRENCY_DATA: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::USA,
    minor_units: 5,
    name: "Australian Dollar",
    symbol: "$",
    symbol_first: true,
};

const AWG_CURRENCY_DATA: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::USA,
    minor_units: 5,
    name: "Aruban Florin",
    symbol: "ƒ",
    symbol_first: false,
};

const AZN_CURRENCY_DATA: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::USA,
    minor_units: 5,
    name: "Bosnia and Herzegovina Convertible Mark",
    symbol: "KM",
    symbol_first: true,
};

const BAM_CURRENCY_DATA: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::USA,
    minor_units: 5,
    name: "Bosnia and Herzegovina Convertible Mark",
    symbol: "KM",
    symbol_first: true,
};

const BBD_CURRENCY_DATA: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::USA,
    minor_units: 1,
    name: "Barbadian Dollar",
    symbol: "$",
    symbol_first: true,
};

const BDT_CURRENCY_DATA: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::India,
    minor_units: 1,
    name: "Bangladeshi Taka",
    symbol: "৳",
    symbol_first: true,
};

const BGN_CURRENCY_DATA: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::India,
    minor_units: 1,
    name: "Bulgarian Lev",
    symbol: "лв.",
    symbol_first: false,
};

const BHD_CURRENCY_DATA: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::India,
    minor_units: 1,
    name: "Bulgarian Lev",
    symbol: "лв.",
    symbol_first: false,
};

const BIF_CURRENCY_DATA: CurrencyData = CurrencyData {
    exponent: 0,
    locale: Locale::USA,
    minor_units: 100,
    name: "Burundia Franc",
    symbol: "Fr",
    symbol_first: false,
};

const BMD_CURRENCY_DATA: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::USA,
    minor_units: 1,
    name: "Bermudian Dollar",
    symbol: "$",
    symbol_first: true,
};

const BND_CURRENCY_DATA: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::USA,
    minor_units: 1,
    name: "Brunei Dollar",
    symbol: "$",
    symbol_first: true,
};

const BOB_CURRENCY_DATA: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::USA,
    minor_units: 10,
    name: "Bolivian Boliviano",
    symbol: "Bs.",
    symbol_first: true,
};

const BRL_CURRENCY_DATA: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::USA,
    minor_units: 5,
    name: "Brazilian real",
    symbol: "R$",
    symbol_first: true,
};

const BSD_CURRENCY_DATA: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::USA,
    minor_units: 1,
    name: "Bahamian Dollar",
    symbol: "$",
    symbol_first: true,
};

const BTN_CURRENCY_DATA: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::USA,
    minor_units: 5,
    name: "Bhutanese Ngultrum",
    symbol: "Nu.",
    symbol_first: false,
};

const BWP_CURRENCY_DATA: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 5,
    name: "Botswana Pula",
    symbol: "P",
    symbol_first: true,
};

const BYN_CURRENCY_DATA: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::Poland,
    minor_units: 1,
    name: "Belarusian Ruble",
    symbol: "Br",
    symbol_first: false,
};

const BYR_CURRENCY_DATA: CurrencyData = CurrencyData{
    exponent: 0,
    locale: Locale::Poland,
    minor_units: 100,
    name: "Belarusian Ruble",
    symbol: "Br",
    symbol_first: false,
};

const BZD_CURRENCY_DATA: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 1,
    name: "Belize Dollar",
    symbol: "$",
    symbol_first: true,
};

const CAD_CURRENCY_DATA: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 5,
    name: "Canadian Dollar",
    symbol: "$",
    symbol_first: true,
};

const CDF_CURRENCY_DATA: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 1,
    name: "Congolese Franc",
    symbol: "Fr",
    symbol_first: false,
};

const CHF_CURRENCY_DATA: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 5,
    name: "Swiss Franc",
    symbol: "Fr",
    symbol_first: true,
};

const CLF_CURRENCY_DATA: CurrencyData = CurrencyData{
    exponent: 4,
    locale: Locale::EU,
    minor_units: 5,
    name: "Unidad de Fomento",
    symbol: "UF",
    symbol_first: true,
};

const CLP_CURRENCY_DATA: CurrencyData = CurrencyData{
    exponent: 0,
    locale: Locale::EU,
    minor_units: 1,
    name: "Chilean Peso",
    symbol: "$",
    symbol_first: true,
};

const CNY_CURRENCY_DATA: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 1,
    name: "Chinese Renminbi Yuan",
    symbol: "¥",
    symbol_first: true,
};

const COP_CURRENCY_DATA: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::EU,
    minor_units: 20,
    name: "Colombian Peso",
    symbol: "$",
    symbol_first: true,
};

const CRC_CURRENCY_DATA: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::EU,
    minor_units: 100,
    name: "Costa Rican Colon",
    symbol: "₡",
    symbol_first: true,
};

const CUC_CURRENCY_DATA: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::USA,
    minor_units: 1,
    name: "Cuban Convertible Peso",
    symbol: "$",
    symbol_first: false,
};

const CUP_CURRENCY_DATA: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::USA,
    minor_units: 1,
    name: "Cuban Peso",
    symbol: "$",
    symbol_first: true,
};

const CVE_CURRENCY_DATA: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::USA,
    minor_units: 100,
    name: "Cape Verdean Escudo",
    symbol: "$",
    symbol_first: false,
};

const CZK_CURRENCY_DATA: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::Poland,
    minor_units: 100,
    name: "Czech Koruna",
    symbol: "Kč",
    symbol_first: false,
};

const DJF_CURRENCY_DATA: CurrencyData = CurrencyData {
    exponent: 0,
    locale: Locale::USA,
    minor_units: 100,
    name: "Djiboutian Franc",
    symbol: "Fdj",
    symbol_first: false,
};

const DKK_CURRENCY_DATA: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::EU,
    minor_units: 50,
    name: "Danish Krone",
    symbol: "kr.",
    symbol_first: false,
};

const DOP_CURRENCY_DATA: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::USA,
    minor_units: 100,
    name: "Dominican Peso",
    symbol: "$",
    symbol_first: true,
};

const DZD_CURRENCY_DATA: CurrencyData = CurrencyData{
    exponent: 2,
    locale:  Locale::USA,
    minor_units: 100,
    name: "Algerian Dinar",
    symbol: "د.ج",
    symbol_first: false,
};

const EGP_CURRENCY_DATA: CurrencyData= CurrencyData {
    exponent: 2,
    locale: Locale::USA,
    minor_units: 25,
    name: "Egyptian Pound",
    symbol: "ج.م",
    symbol_first: true,
};

const ERN_CURRENCY_DATA: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 1,
    name: "Eritrean Nakfa",
    symbol: "Nfk",
    symbol_first: false,
};

const ETB_CURRENCY_DATA: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::USA,
    minor_units: 1,
    name: "Ethiopian Birr",
    symbol: "Br",
    symbol_first: false,
};

const EUR_CURRENCY_DATA: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::EU,
    minor_units: 1,
    name: "Euro",
    symbol: "€",
    symbol_first: true,
};

const FJD_CURRENCY_DATA: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::EU,
    minor_units: 5,
    name: "Fijian Dollar",
    symbol: "$",
    symbol_first: false,
};

const FKP_CURRENCY_DATA: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::EU,
    minor_units: 1,
    name: "Falkland Pound",
    symbol: "£",
    symbol_first: false,
};

const GBP_CURRENCY_DATA: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::USA,
    minor_units: 1,
    name: "British Pound",
    symbol: "£",
    symbol_first: true,
};

const GEL_CURRENCY_DATA: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 1,
    name: "Georgian Lari",
    symbol: "ლ",
    symbol_first: false,
};

const GHS_CURRENCY_DATA: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::USA,
    minor_units: 1,
    name: "Ghanaian Cedi",
    symbol: "₵",
    symbol_first: true,
};

const GIP_CURRENCY_DATA: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::USA,
    minor_units: 1,
    name: "Gibraltar Pound",
    symbol: "£",
    symbol_first: true,
};

const GNF_CURRENCY_DATA: CurrencyData = CurrencyData {
    exponent: 0,
    locale: Locale::USA,
    minor_units: 100,
    name: "Guinean Franc",
    symbol: "Fr",
    symbol_first: false,
};

const GTQ_CURRENCY_DATA: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::USA,
    minor_units: 1,
    name: "Guatemalan Quetzal",
    symbol: "Q",
    symbol_first: true,
};

const GYD_CURRENCY_DATA: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::USA,
    minor_units: 100,
    name: "Guyanese Dollar",
    symbol: "$",
    symbol_first: false,
};

const HKD_CURRENCY_DATA: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 10,
    name: "Hong Kong Dollar",
    symbol: "$",
    symbol_first: true,
};

const HNL_CURRENCY_DATA: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 5,
    name: "Honduran Lempira",
    symbol: "L",
    symbol_first: true,
};

const HRK_CURRENCY_DATA: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::EU,
    minor_units: 1,
    name: "Croatian Kuna",
    symbol: "kn",
    symbol_first: false,
};

const HTG_CURRENCY_DATA: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 5,
    name: "Haitian Gourde",
    symbol: "G",
    symbol_first: false,
};

const HUF_CURRENCY_DATA: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::Poland,
    minor_units: 5,
    name: "Hungarian Forint",
    symbol: "Ft",
    symbol_first: false,
};

const IDR_CURRENCY_DATA: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::USA,
    minor_units: 5000,
    name: "Indonesian Rupiah",
    symbol: "Rp",
    symbol_first: true,
};

const ILS_CURRENCY_DATA: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::USA,
    minor_units: 10,
    name: "Israeli New Sheqel",
    symbol: "₪",
    symbol_first: true,
};

const INR_CURRENCY_DATA: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::India,
    minor_units: 50,
    name: "Indian Rupee",
    symbol: "₹",
    symbol_first: true,
};

const IQD_CURRENCY_DATA: CurrencyData  = CurrencyData {
    exponent: 3,
    locale: Locale::USA,
    minor_units: 50000,
    name: "Iraqi Dinar",
    symbol: "ع.د",
    symbol_first: false,
};

const IRR_CURRENCY_DATA: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::USA,
    minor_units: 5000,
    name: "Iranian Rial",
    symbol: "﷼",
    symbol_first: true,
};

const ISK_CURRENCY_DATA: CurrencyData = CurrencyData{
    exponent: 0,
    locale: Locale::EU,
    minor_units: 1,
    name: "Icelandic Krona",
    symbol: "kr.",
    symbol_first: true,
};

const JMD_CURRENCY_DATA: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::USA,
    minor_units: 1,
    name: "Jamaican Dollar",
    symbol: "$",
    symbol_first: true,
};

const JOD_CURRENCY_DATA: CurrencyData = CurrencyData {
    exponent: 3,
    locale: Locale::USA,
    minor_units: 5,
    name: "Jordanian Dinar",
    symbol: "د.ا",
    symbol_first: true,
};

const JPY_CURRENCY_DATA: CurrencyData = CurrencyData{
    exponent: 0,
    locale: Locale::USA,
    minor_units: 1,
    name: "Japanese Yen",
    symbol: "¥",
    symbol_first: true,
};

const KES_CURRENCY_DATA: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::USA,
    minor_units: 50,
    name: "Kenyan Shilling",
    symbol: "KSh",
    symbol_first: true,
};

const KGS_CURRENCY_DATA: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::USA,
    minor_units: 1,
    name: "Kyrgzstani Som",
    symbol: "som",
    symbol_first: false,
};

const KHR_CURRENCY_DATA: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::USA,
    minor_units: 5000,
    name: "Cambodian Riel",
    symbol: "៛",
    symbol_first: false,
};

const KMF_CURRENCY_DATA: CurrencyData = CurrencyData{
    exponent: 0,
    locale: Locale::USA,
    minor_units: 100,
    name: "Comorian Franc",
    symbol: "Fr",
    symbol_first: false,
};

const KPW_CURRENCY_DATA: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::USA,
    minor_units: 1,
    name: "North Korean Won",
    symbol: "W",
    symbol_first: false,
};

const KRW_CURRENCY_DATA: CurrencyData = CurrencyData {
    exponent: 0,
    locale: Locale::USA,
    minor_units: 1,
    name: "South Korean Won",
    symbol: "W",
    symbol_first: true,
};

const KWD_CURRENCY_DATA: CurrencyData = CurrencyData {
    exponent: 3,
    locale: Locale::USA,
    minor_units: 5,
    name: "Kuwaiti Dinar",
    symbol: "د.ك",
    symbol_first: true,
};

const KYD_CURRENCY_DATA: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::USA,
    minor_units: 1,
    name: "Cayman Islands Dollar",
    symbol: "$",
    symbol_first: true,
};

const KZT_CURRENCY_DATA: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::USA,
    minor_units: 100,
    name: "Kazakhstani Tenge",
    symbol: "₸",
    symbol_first: false,
};

const LAK_CURRENCY_DATA: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::USA,
    minor_units: 10,
    name: "Lao Kip",
    symbol: "K",
    symbol_first: false,
};

const LBP_CURRENCY_DATA: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::USA,
    minor_units: 25000,
    name: "Lebanese Pound",
    symbol: "ل.ل",
    symbol_first: true,
};

const LKR_CURRENCY_DATA: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::USA,
    minor_units: 100,
    name: "Sri Lankan Rupee",
    symbol: "₨",
    symbol_first: false,
};

const LRD_CURRENCY_DATA: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::USA,
    minor_units: 5,
    name: "Liberian Dollar",
    symbol: "$",
    symbol_first: false,
};

const LSL_CURRENCY_DATA: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::USA,
    minor_units: 1,
    name: "Lesotho Loti",
    symbol: "L",
    symbol_first: false,
};

const LYD_CURRENCY_DATA: CurrencyData = CurrencyData {
    exponent: 3,
    locale: Locale::USA,
    minor_units: 50,
    name: "Libyan Dinar",
    symbol: "ل.د",
    symbol_first: false,              
};

const MAD_CURRENCY_DATA: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::USA,
    minor_units: 1,
    name: "Morocco Dirham",
    symbol: "د.م.",
    symbol_first: false,
};


const MDL_CURRENCY_DATA: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::USA,
    minor_units: 1,
    name: "Moldovan Leu",
    symbol: "L",
    symbol_first: false,
};

const MKD_CURRENCY_DATA: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::USA,
    minor_units: 100,
    name: "Macedonian Denar",
    symbol: "ден",
    symbol_first: false,
};

const MMK_CURRENCY_DATA: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 50,
    name: "Myanmar Kyat",
    symbol: "K",
    symbol_first: false,
};

const MNT_CURRENCY_DATA: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::USA,
    minor_units: 2000,
    name: "Mongolian Tögrög",
    symbol: "₮",
    symbol_first: false,
};

const MOP_CURRENCY_DATA: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::USA,
    minor_units: 10,
    name: "Macanese Pataca",
    symbol: "P",
    symbol_first: false,
};

const MUR_CURRENCY_DATA: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 100,
    name: "Mauritian Rupee",
    symbol: "₨",
    symbol_first: true,
};

const MVR_CURRENCY_DATA: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 100,
    name: "Maldivian Rufiyaa",
    symbol: "MVR",
    symbol_first: false,
};

const MWK_CURRENCY_DATA: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 1,
    name: "Malawian Kwacha",
    symbol: "MK",
    symbol_first: false,
};

const MXN_CURRENCY_DATA: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 5,
    name: "Mexican Peso",
    symbol: "$",
    symbol_first: true,
};

const MYR_CURRENCY_DATA: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 5,
    name: "Malaysian Ringgit",
    symbol: "RM",
    symbol_first: true,
};

const MZN_CURRENCY_DATA: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 1,
    name: "Mozambican Metical",
    symbol: "MTn",
    symbol_first: true,
};

const NAD_CURRENCY_DATA: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 5,
    name: "Namibian Dollar",
    symbol: "$",
    symbol_first: false,
};

const NGN_CURRENCY_DATA: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 50,
    name: "Nigerian Naira",
    symbol: "₦",
    symbol_first: true,
};

const NIO_CURRENCY_DATA: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 5,
    name: "Nicaraguan Córdoba",
    symbol: "C$",
    symbol_first: true,
};

const NOK_CURRENCY_DATA: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 100,
    name: "Norwegian Krone",
    symbol: "kr",
    symbol_first: false,
};

const NPR_CURRENCY_DATA: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 1,
    name: "Nepalese Rupee",
    symbol: "रु",
    symbol_first: true,
};

const NZD_CURRENCY_DATA: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 10,
    name: "New Zealand Dollar",
    symbol: "$",
    symbol_first: true,
};

const OMR_CURRENCY_DATA: CurrencyData = CurrencyData{
    exponent: 3,
    locale: Locale::USA,
    minor_units: 5,
    name: "Omani Rial",
    symbol: "ر.ع.",
    symbol_first: true,
};

const PAB_CURRENCY_DATA: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 1,
    name: "Panamanian Balboa",
    symbol: "B/.",
    symbol_first: true,
};

const PEN_CURRENCY_DATA: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 1,
    name: "Peruvian Sol",
    symbol: "S/",
    symbol_first: true,
};

const PGK_CURRENCY_DATA: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 5,
    name: "Papua New Guinean Kina",
    symbol: "K",
    symbol_first: false,
};

const PHP_CURRENCY_DATA: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 1,
    name: "Philippine Peso",
    symbol: "₱",
    symbol_first: true,
};

const PKR_CURRENCY_DATA: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 100,
    name: "Pakistani Rupee",
    symbol: "₨",
    symbol_first: true,
};

const PLN_CURRENCY_DATA: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::Poland,
    minor_units: 1,
    name: "Polish Złoty",
    symbol: "zł",
    symbol_first: false,
};

const PYG_CURRENCY_DATA: CurrencyData = CurrencyData {
    exponent: 0,
    locale: Locale::Poland,
    minor_units: 5000,
    name: "Paraguayan Guaraní",
    symbol: "₲",
    symbol_first: true,
};

const QAR_CURRENCY_DATA: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::Poland,
    minor_units: 1,
    name: "Qatari Riyal",
    symbol: "ر.ق",
    symbol_first: false,
};

const RON_CURRENCY_DATA: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::EU,
    minor_units: 1,
    name: "Romanian Leu",
    symbol: "ر.ق",
    symbol_first: false,
};

const RSD_CURRENCY_DATA: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 100,
    name: "Serbian Dinar",
    symbol: "РСД",
    symbol_first: true,
};

const RUB_CURRENCY_DATA: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::EU,
    minor_units: 1,
    name: "Russian Ruble",
    symbol: "₽",
    symbol_first: false,
};

const RWF_CURRENCY_DATA: CurrencyData = CurrencyData{
    exponent: 0,
    locale: Locale::USA,
    minor_units: 100,
    name: "Rwandan Franc",
    symbol: "FRw",
    symbol_first: false,
};

const SAR_CURRENCY_DATA: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 5,
    name: "Saudi Riyal",
    symbol: "ر.س",
    symbol_first: true,
};

const SBD_CURRENCY_DATA: CurrencyData = CurrencyData{ 
    exponent: 2,
    locale: Locale::USA,
    minor_units: 10,
    name: "Solomon Islands Dollar",
    symbol: "$",
    symbol_first: false,
};

const SCR_CURRENCY_DATA: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 1,
    name: "Seychellois Rupee",
    symbol: "₨",
    symbol_first: false,
};

const SDG_CURRENCY_DATA: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 1,
    name: "Sudanese Pound",
    symbol: "£",
    symbol_first: true,
};

const SEK_CURRENCY_DATA: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::EU,
    minor_units: 100,
    name: "Swedish Krona",
    symbol: "kr",
    symbol_first: false,
};

const SGD_CURRENCY_DATA: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 1,
    name: "Singapore Dollar",
    symbol: "$",
    symbol_first: true,
};

const SHP_CURRENCY_DATA: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 1,
    name: "Saint Helenian Pound",
    symbol: "£",
    symbol_first: false,
};

const SKK_CURRENCY_DATA: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 50,
    name: "Slovak Koruna",
    symbol: "Sk",
    symbol_first: true,
};

const SLL_CURRENCY_DATA: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 1000,
    name: "Sierra Leonean Leone",
    symbol: "Le",
    symbol_first: false,
};

const SOS_CURRENCY_DATA: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 1,
    name: "Somali Shilling",
    symbol: "Sh",
    symbol_first: false,
};

const SRD_CURRENCY_DATA: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 1,
    name: "Surinamese Dollar",
    symbol: "$",
    symbol_first: false,
};

const SSP_CURRENCY_DATA: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 5,
    name: "South Sudanese Pound",
    symbol: "£",
    symbol_first: false,
};

const STD_CURRENCY_DATA: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 10000,
    name: "São Tomé and Príncipe Dobra",
    symbol: "Db",
    symbol_first: false,
};

const STN_CURRENCY_DATA: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 10,
    name: "São Tomé and Príncipe Dobra",
    symbol: "Db",
    symbol_first: false,
};

const SVC_CURRENCY_DATA: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 1,
    name: "Salvadoran Colón",
    symbol: "₡",
    symbol_first: true,
};

const SYP_CURRENCY_DATA: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 100,
    name: "Syrian Pound",
    symbol: "£S",
    symbol_first: false,
};

const SZL_CURRENCY_DATA: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 1,
    name: "Swazi Lilangeni",
    symbol: "E",
    symbol_first: true,
};

const THB_CURRENCY_DATA: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 1,
    name: "Thai Baht",
    symbol: "฿",
    symbol_first: true,
};

const TJS_CURRENCY_DATA: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::USA,
    minor_units: 1,
    name: "Tajikistani Somoni",
    symbol: "ЅМ",
    symbol_first: false,
};

const TMT_CURRENCY_DATA: CurrencyData = CurrencyData {
    exponent: 2,
    locale: Locale::USA,
    minor_units: 1,
    name: "Turkmenistani Manat",
    symbol: "T",
    symbol_first: false,
};

const TND_CURRENCY_DATA: CurrencyData = CurrencyData{
    exponent: 3,
    locale: Locale::USA,
    minor_units: 10,
    name: "Tunisian Dinar",
    symbol: "د.ت",
    symbol_first: false,
};

const TOP_CURRENCY_DATA: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 1,
    name: "Tongan Paʻanga",
    symbol: "T$",
    symbol_first: true,
};

const TRY_CURRENCY_DATA: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::EU,
    minor_units: 1,
    name: "Turkish Lira",
    symbol: "₺",
    symbol_first: true,
};

const TTD_CURRENCY_DATA: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 1,
    name: "Trinidad and Tobago Dollar",
    symbol: "$",
    symbol_first: false,
};

const TWD_CURRENCY_DATA: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 50,
    name: "New Taiwan Dollar",
    symbol: "$",
    symbol_first: true,
};

const TZS_CURRENCY_DATA: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 5000,
    name: "Tanzanian Shilling",
    symbol: "Sh",
    symbol_first: true,
};

const UAH_CURRENCY_DATA: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 1,
    name: "Ukrainian Hryvnia",
    symbol: "₴",
    symbol_first: false,
};

const UGX_CURRENCY_DATA: CurrencyData = CurrencyData{
    exponent: 0,
    locale: Locale::USA,
    minor_units: 1000,
    name: "Ugandan Shilling",
    symbol: "USh",
    symbol_first: false,
};

const USD_CURRENCY_DATA: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 1,
    name: "United States Dollar",
    symbol: "$",
    symbol_first: true,
};

const UYU_CURRENCY_DATA: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::EU,
    minor_units: 100,
    name: "Uruguayan Peso",
    symbol: "$U",
    symbol_first: true,
};

const UYW_CURRENCY_DATA: CurrencyData = CurrencyData{
    exponent: 4,
    locale: Locale::EU,
    minor_units: 1000,
    name: "Unidad Previsional",
    symbol: "UP",
    symbol_first: true,
};

const UZS_CURRENCY_DATA: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 100,
    name: "Uzbekistan Som",
    symbol: "so'm",
    symbol_first: false,
};

const VES_CURRENCY_DATA: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::EU,
    minor_units: 1,
    name: "Venezuelan Bolívar Soberano",
    symbol: "Bs",
    symbol_first: true,
};

const VND_CURRENCY_DATA: CurrencyData = CurrencyData{
    exponent: 0,
    locale: Locale::EU,
    minor_units: 100,
    name: "Vietnamese Đồng",
    symbol: "₫",
    symbol_first: false,
};

const VUV_CURRENCY_DATA: CurrencyData = CurrencyData{
    exponent: 0,
    locale: Locale::USA,
    minor_units: 1,
    name: "Vanuatu Vatu",
    symbol: "Vt",
    symbol_first: true,
};

const WST_CURRENCY_DATA: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 10,
    name: "Samoan Tala",
    symbol: "T",
    symbol_first: false,
};

const XAF_CURRENCY_DATA: CurrencyData = CurrencyData{
    exponent: 0,
    locale: Locale::USA,
    minor_units: 100,
    name: "Central African Cfa Franc",
    symbol: "CFA",
    symbol_first: false,
};

const XAG_CURRENCY_DATA: CurrencyData = CurrencyData{
    exponent: 0,
    locale: Locale::USA,
    minor_units: 100,
    name: "Silver (Troy Ounce)",
    symbol: "oz t",
    symbol_first: false,
};

const XAU_CURRENCY_DATA: CurrencyData = CurrencyData{
    exponent: 0,
    locale: Locale::USA,
    minor_units: 100,
    name: "Gold (Troy Ounce)",
    symbol: "oz t",
    symbol_first: false,
};

const XBA_CURRENCY_DATA: CurrencyData = CurrencyData{
    exponent: 0,
    locale: Locale::USA,
    minor_units: 100,
    name: "European Composite Unit",
    symbol: "",
    symbol_first: false,
};

const XBB_CURRENCY_DATA: CurrencyData = CurrencyData{
    exponent: 0,
    locale: Locale::USA,
    minor_units: 100,
    name: "European Monetary Unit",
    symbol: "",
    symbol_first: false,
};

const XBC_CURRENCY_DATA: CurrencyData = CurrencyData{
    exponent: 0,
    locale: Locale::USA,
    minor_units: 100,
    name: "European Unit of Account 9",
    symbol: "",
    symbol_first: false,
};

const XBD_CURRENCY_DATA: CurrencyData = CurrencyData{
    exponent: 0,
    locale: Locale::USA,
    minor_units: 100,
    name: "European Unit of Account 17",
    symbol: "",
    symbol_first: false,
};

const XCD_CURRENCY_DATA: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 1,
    name: "East Caribbean Dollar",
    symbol: "$",
    symbol_first: true,
};

const XDR_CURRENCY_DATA: CurrencyData = CurrencyData{
    exponent: 0,
    locale: Locale::USA,
    minor_units: 100,
    name: "Special Drawing Rights",
    symbol: "SDR",
    symbol_first: false,
};

const XOF_CURRENCY_DATA: CurrencyData = CurrencyData{
    exponent: 0,
    locale: Locale::USA,
    minor_units: 100,
    name: "West African Cfa Franc",
    symbol: "Fr",
    symbol_first: false,
};

const XPD_CURRENCY_DATA: CurrencyData = CurrencyData{
    exponent: 0,
    locale: Locale::USA,
    minor_units: 100,
    name: "Palladium",
    symbol: "oz t",
    symbol_first: false,
};

const XPF_CURRENCY_DATA: CurrencyData = CurrencyData{ 
    exponent: 0,
    locale: Locale::USA,
    minor_units: 100,
    name: "Cfp Franc",
    symbol: "Fr",
    symbol_first: false,
};

const XPT_CURRENCY_DATA: CurrencyData = CurrencyData{
    exponent: 0,
    locale: Locale::USA,
    minor_units: 100,
    name: "Platinum",
    symbol: "oz t",
    symbol_first: false,
};

const XTS_CURRENCY_DATA: CurrencyData = CurrencyData{
    exponent: 0,
    locale: Locale::USA,
    minor_units: 100,
    name: "Codes specifically reserved for testing purposes",
    symbol: "oz t",
    symbol_first: false,
};

const YER_CURRENCY_DATA: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 100,
    name: "Yemeni Rial",
    symbol: "﷼",
    symbol_first: false,
};

const ZAR_CURRENCY_DATA: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 10,
    name: "South African Rand",
    symbol: "R",
    symbol_first: true,
};

const ZMK_CURRENCY_DATA: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 5,
    name: "Zambian Kwacha",
    symbol: "ZK",
    symbol_first: false,
};

const ZMW_CURRENCY_DATA: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 5,
    name: "Zambian Kwacha",
    symbol: "K",
    symbol_first: true,
};

const ZWL_CURRENCY_DATA: CurrencyData = CurrencyData{
    exponent: 2,
    locale: Locale::USA,
    minor_units: 1,
    name: "Zimbabwe Dollar",
    symbol: "Z$",
    symbol_first: true,
};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
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
    YER,
}

impl Currency {
    fn match_currency_to_data(&self) -> CurrencyData {
        match self {
            Currency::AED => AED_CURRENCY_DATA,
            Currency::AFN => AFN_CURRENCY_DATA,
            Currency::ALL => ALL_CURRENCY_DATA,
            Currency::AMD => AMD_CURRENCY_DATA,
            Currency::ANG => ANG_CURRENCY_DATA,
            Currency::AOA => AOA_CURRENCY_DATA,
            Currency::ARS => ARS_CURRENCY_DATA,
            Currency::AUD => AUD_CURRENCY_DATA,
            Currency::AWG => AWG_CURRENCY_DATA,
            Currency::AZN => AZN_CURRENCY_DATA,
            Currency::BAM => BAM_CURRENCY_DATA,
            Currency::BBD => BBD_CURRENCY_DATA,
            Currency::BDT => BDT_CURRENCY_DATA,
            Currency::BGN => BGN_CURRENCY_DATA,
            Currency::BHD => BHD_CURRENCY_DATA,
            Currency::BIF => BIF_CURRENCY_DATA,
            Currency::BMD => BMD_CURRENCY_DATA,
            Currency::BND => BND_CURRENCY_DATA,
            Currency::BOB => BOB_CURRENCY_DATA,
            Currency::BRL => BRL_CURRENCY_DATA,
            Currency::BSD => BSD_CURRENCY_DATA,
            Currency::BTN => BTN_CURRENCY_DATA,
            Currency::BWP => BWP_CURRENCY_DATA,
            Currency::BYN => BYN_CURRENCY_DATA,
            Currency::BYR => BYR_CURRENCY_DATA,
            Currency::BZD => BZD_CURRENCY_DATA,
            Currency::CAD => CAD_CURRENCY_DATA,
            Currency::CDF => CDF_CURRENCY_DATA,
            Currency::CHF => CHF_CURRENCY_DATA,
            Currency::CLF => CLF_CURRENCY_DATA,
            Currency::CLP => CLP_CURRENCY_DATA,
            Currency::CNY => CNY_CURRENCY_DATA,
            Currency::COP => COP_CURRENCY_DATA,
            Currency::CRC => CRC_CURRENCY_DATA,
            Currency::CUC => CUC_CURRENCY_DATA,
            Currency::CUP => CUP_CURRENCY_DATA,
            Currency::CVE => CVE_CURRENCY_DATA,
            Currency::CZK => CZK_CURRENCY_DATA,
            Currency::DJF => DJF_CURRENCY_DATA,
            Currency::DKK => DKK_CURRENCY_DATA,
            Currency::DOP => DOP_CURRENCY_DATA,
            Currency::DZD => DZD_CURRENCY_DATA,
            Currency::EGP => EGP_CURRENCY_DATA,
            Currency::ERN => ERN_CURRENCY_DATA,
            Currency::ETB => ETB_CURRENCY_DATA,
            Currency::EUR => EUR_CURRENCY_DATA,
            Currency::FJD => FJD_CURRENCY_DATA,
            Currency::FKP => FKP_CURRENCY_DATA,
            Currency::GBP => GBP_CURRENCY_DATA,
            Currency::GEL => GEL_CURRENCY_DATA,
            Currency::GHS => GHS_CURRENCY_DATA,
            Currency::GIP => GIP_CURRENCY_DATA,
            Currency::GNF => GNF_CURRENCY_DATA,
            Currency::GTQ => GTQ_CURRENCY_DATA,
            Currency::GYD => GYD_CURRENCY_DATA,
            Currency::HKD => HKD_CURRENCY_DATA,
            Currency::HNL => HNL_CURRENCY_DATA,
            Currency::HRK => HRK_CURRENCY_DATA,
            Currency::HTG => HTG_CURRENCY_DATA,
            Currency::HUF => HUF_CURRENCY_DATA,
            Currency::IDR => IDR_CURRENCY_DATA,
            Currency::ILS => ILS_CURRENCY_DATA,
            Currency::INR => INR_CURRENCY_DATA,
            Currency::IQD => IQD_CURRENCY_DATA,
            Currency::IRR => IRR_CURRENCY_DATA,
            Currency::ISK => ISK_CURRENCY_DATA,
            Currency::JMD => JMD_CURRENCY_DATA,
            Currency::JOD => JOD_CURRENCY_DATA,
            Currency::JPY => JPY_CURRENCY_DATA,
            Currency::KES => KES_CURRENCY_DATA,
            Currency::KGS => KGS_CURRENCY_DATA,
            Currency::KHR => KHR_CURRENCY_DATA,
            Currency::KMF => KMF_CURRENCY_DATA,
            Currency::KPW => KPW_CURRENCY_DATA,
            Currency::KRW => KRW_CURRENCY_DATA,
            Currency::KWD => KWD_CURRENCY_DATA,
            Currency::KYD => KYD_CURRENCY_DATA,
            Currency::KZT => KZT_CURRENCY_DATA,
            Currency::LAK => LAK_CURRENCY_DATA,
            Currency::LBP => LBP_CURRENCY_DATA,
            Currency::LKR => LKR_CURRENCY_DATA,
            Currency::LRD => LRD_CURRENCY_DATA,
            Currency::LSL => LSL_CURRENCY_DATA,
            Currency::LYD => LYD_CURRENCY_DATA,
            Currency::MAD => MAD_CURRENCY_DATA,
            Currency::MDL => MDL_CURRENCY_DATA,
            Currency::MKD => MKD_CURRENCY_DATA,
            Currency::MMK => MMK_CURRENCY_DATA,
            Currency::MNT => MNT_CURRENCY_DATA,
            Currency::MOP => MOP_CURRENCY_DATA,
            Currency::MUR => MUR_CURRENCY_DATA,
            Currency::MVR => MVR_CURRENCY_DATA,
            Currency::MWK => MWK_CURRENCY_DATA,
            Currency::MXN => MXN_CURRENCY_DATA,
            Currency::MYR => MYR_CURRENCY_DATA,
            Currency::MZN => MZN_CURRENCY_DATA,
            Currency::NAD => NAD_CURRENCY_DATA,
            Currency::NGN => NGN_CURRENCY_DATA,
            Currency::NIO => NIO_CURRENCY_DATA,
            Currency::NOK => NOK_CURRENCY_DATA,
            Currency::NPR => NPR_CURRENCY_DATA,
            Currency::NZD => NZD_CURRENCY_DATA,
            Currency::OMR => OMR_CURRENCY_DATA,
            Currency::PAB => PAB_CURRENCY_DATA,
            Currency::PEN => PEN_CURRENCY_DATA,
            Currency::PGK => PGK_CURRENCY_DATA,
            Currency::PHP => PHP_CURRENCY_DATA,
            Currency::PKR => PKR_CURRENCY_DATA,
            Currency::PLN => PLN_CURRENCY_DATA,
            Currency::PYG => PYG_CURRENCY_DATA,
            Currency::QAR => QAR_CURRENCY_DATA,
            Currency::RON => RON_CURRENCY_DATA,
            Currency::RSD => RSD_CURRENCY_DATA,
            Currency::RUB => RUB_CURRENCY_DATA,
            Currency::RWF => RWF_CURRENCY_DATA,
            Currency::SAR => SAR_CURRENCY_DATA,
            Currency::SBD => SBD_CURRENCY_DATA,
            Currency::SCR => SCR_CURRENCY_DATA,
            Currency::SDG => SDG_CURRENCY_DATA,
            Currency::SEK => SEK_CURRENCY_DATA,
            Currency::SGD => SGD_CURRENCY_DATA,
            Currency::SHP => SHP_CURRENCY_DATA,
            Currency::SKK => SKK_CURRENCY_DATA,
            Currency::SLL => SLL_CURRENCY_DATA,
            Currency::SOS => SOS_CURRENCY_DATA,
            Currency::SRD => SRD_CURRENCY_DATA,
            Currency::SSP => SSP_CURRENCY_DATA,
            Currency::STD => STD_CURRENCY_DATA,
            Currency::STN => STN_CURRENCY_DATA,
            Currency::SVC => SVC_CURRENCY_DATA,
            Currency::SYP => SYP_CURRENCY_DATA,
            Currency::SZL => SZL_CURRENCY_DATA,
            Currency::THB => THB_CURRENCY_DATA,
            Currency::TJS => TJS_CURRENCY_DATA,
            Currency::TMT => TMT_CURRENCY_DATA,
            Currency::TND => TND_CURRENCY_DATA,
            Currency::TOP => TOP_CURRENCY_DATA,
            Currency::TRY => TRY_CURRENCY_DATA,
            Currency::TTD => TTD_CURRENCY_DATA,
            Currency::TWD => TWD_CURRENCY_DATA,
            Currency::TZS => TZS_CURRENCY_DATA,
            Currency::UAH => UAH_CURRENCY_DATA,
            Currency::UGX => UGX_CURRENCY_DATA,
            Currency::USD => USD_CURRENCY_DATA,
            Currency::UYU => UYU_CURRENCY_DATA,
            Currency::UYW => UYW_CURRENCY_DATA,
            Currency::UZS => UZS_CURRENCY_DATA,
            Currency::VES => VES_CURRENCY_DATA,
            Currency::VND => VND_CURRENCY_DATA,
            Currency::VUV => VUV_CURRENCY_DATA,
            Currency::WST => WST_CURRENCY_DATA,
            Currency::XAF => XAF_CURRENCY_DATA,
            Currency::XAG => XAG_CURRENCY_DATA,
            Currency::XAU => XAU_CURRENCY_DATA,
            Currency::XBA => XBA_CURRENCY_DATA,
            Currency::XBB => XBB_CURRENCY_DATA,
            Currency::XBC => XBC_CURRENCY_DATA,
            Currency::XBD => XBD_CURRENCY_DATA,
            Currency::XCD => XCD_CURRENCY_DATA,
            Currency::XDR => XDR_CURRENCY_DATA,
            Currency::XOF => XOF_CURRENCY_DATA,
            Currency::XPD => XPD_CURRENCY_DATA,
            Currency::XPF => XPF_CURRENCY_DATA,
            Currency::XPT => XPT_CURRENCY_DATA,
            Currency::XTS => XTS_CURRENCY_DATA,
            Currency::ZAR => ZAR_CURRENCY_DATA,
            Currency::ZMK => ZMK_CURRENCY_DATA,
            Currency::ZMW => ZMW_CURRENCY_DATA,
            Currency::ZWL => ZWL_CURRENCY_DATA,
            Currency::YER => YER_CURRENCY_DATA,
        }
    }

    pub fn match_currency_to_lotus(&self) -> Result<crate::Lotus, String> {
       let currency_data = self.match_currency_to_data();

       let mut lotus_builder = LotusBuilder::default();
       lotus_builder.symbol(currency_data.symbol);
       lotus_builder.precision(currency_data.exponent);
       
       if currency_data.symbol_first {
           lotus_builder.format_positive("{symbol}{value}");
           lotus_builder.format_negative("{symbol}({value})");
           match currency_data.locale{
               Locale::USA | Locale::India | Locale::Poland => lotus_builder.format_zero("{symbol}0.00"),
               Locale::EU => lotus_builder.format_zero("{symbol}0,00"),
           };
       } else {
           lotus_builder.format_positive("{value}{symbol}");
           lotus_builder.format_negative("{value}{symbol}");
           match currency_data.locale{
               Locale::USA | Locale::India | Locale::Poland => lotus_builder.format_zero("0.00{symbol}"),
               Locale::EU => lotus_builder.format_zero("0,00{symbol}")
           };
       }

       match currency_data.locale{
           Locale::USA | Locale::India => {
               lotus_builder.thousand_str(",");
               lotus_builder.decimal_str(".");
           },
           Locale::Poland =>{
               lotus_builder.thousand_str(" ");
           }
           Locale::EU => {
               lotus_builder.thousand_str(".");
               lotus_builder.decimal_str(",");
           }
       }

       return lotus_builder.build();
    }
}