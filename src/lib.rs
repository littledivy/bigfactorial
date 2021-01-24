#[doc(hidden)]
#[macro_export]
macro_rules! to_bigint {
    ($e:expr) => {
        (BigInt::parse_bytes($e, 10))
    };
}

#[macro_export]
macro_rules! fact {
    (0) => (to_bigint!(b"1"));
    (1) => (to_bigint!(b"1"));
    (2) => (to_bigint!(b"2"));
    (3) => (to_bigint!(b"6"));
    (4) => (to_bigint!(b"24"));
    (5) => (to_bigint!(b"120"));
    (6) => (to_bigint!(b"720"));
    (7) => (to_bigint!(b"5040"));
    (8) => (to_bigint!(b"40320"));
    (9) => (to_bigint!(b"362880"));
    (10) => (to_bigint!(b"3628800"));
    (11) => (to_bigint!(b"39916800"));
    (12) => (to_bigint!(b"479001600"));
    (13) => (to_bigint!(b"6227020800"));
    (14) => (to_bigint!(b"87178291200"));
    (15) => (to_bigint!(b"1307674368000"));
    (16) => (to_bigint!(b"20922789888000"));
    (17) => (to_bigint!(b"355687428096000"));
    (18) => (to_bigint!(b"6402373705728000"));
    (19) => (to_bigint!(b"121645100408832000"));
    (20) => (to_bigint!(b"2432902008176640000"));
    (21) => (to_bigint!(b"51090942171709440000"));
    (22) => (to_bigint!(b"1124000727777607680000"));
    (23) => (to_bigint!(b"25852016738884976640000"));
    (24) => (to_bigint!(b"620448401733239439360000"));
    (25) => (to_bigint!(b"15511210043330985984000000"));
    (26) => (to_bigint!(b"403291461126605635584000000"));
    (27) => (to_bigint!(b"10888869450418352160768000000"));
    (28) => (to_bigint!(b"304888344611713860501504000000"));
    (29) => (to_bigint!(b"8841761993739701954543616000000"));
    (30) => (to_bigint!(b"265252859812191058636308480000000"));
    (31) => (to_bigint!(b"8222838654177922817725562880000000"));
    (32) => (to_bigint!(b"263130836933693530167218012160000000"));
    (33) => (to_bigint!(b"8683317618811886495518194401280000000"));
    (34) => (to_bigint!(b"295232799039604140847618609643520000000"));
    (35) => (to_bigint!(b"10333147966386144929666651337523200000000"));
    (36) => (to_bigint!(b"371993326789901217467999448150835200000000"));
    (37) => (to_bigint!(b"13763753091226345046315979581580902400000000"));
    (38) => (to_bigint!(b"523022617466601111760007224100074291200000000"));
    (39) => (to_bigint!(b"20397882081197443358640281739902897356800000000"));
    (40) => (to_bigint!(b"815915283247897734345611269596115894272000000000"));
    (41) => (to_bigint!(b"33452526613163807108170062053440751665152000000000"));
    (42) => (to_bigint!(b"1405006117752879898543142606244511569936384000000000"));
    (43) => (to_bigint!(b"60415263063373835637355132068513997507264512000000000"));
    (44) => (to_bigint!(b"2658271574788448768043625811014615890319638528000000000"));
    (45) => (to_bigint!(b"119622220865480194561963161495657715064383733760000000000"));
    (46) => (to_bigint!(b"5502622159812088949850305428800254892961651752960000000000"));
    (47) => (to_bigint!(b"258623241511168180642964355153611979969197632389120000000000"));
    (48) => (to_bigint!(b"12413915592536072670862289047373375038521486354677760000000000"));
    (49) => (to_bigint!(b"608281864034267560872252163321295376887552831379210240000000000"));
    (50) => (to_bigint!(b"30414093201713378043612608166064768844377641568960512000000000000"));
    (51) => (to_bigint!(b"1551118753287382280224243016469303211063259720016986112000000000000"));
    (52) => (to_bigint!(b"80658175170943878571660636856403766975289505440883277824000000000000"));
    (53) => (to_bigint!(b"4274883284060025564298013753389399649690343788366813724672000000000000"));
    (54) => (to_bigint!(b"230843697339241380472092742683027581083278564571807941132288000000000000"));
    (55) => (to_bigint!(b"12696403353658275925965100847566516959580321051449436762275840000000000000"));
    (56) => (to_bigint!(b"710998587804863451854045647463724949736497978881168458687447040000000000000"));
    (57) => (to_bigint!(b"40526919504877216755680601905432322134980384796226602145184481280000000000000"));
    (58) => (to_bigint!(b"2350561331282878571829474910515074683828862318181142924420699914240000000000000"));
    (59) => (to_bigint!(b"138683118545689835737939019720389406345902876772687432540821294940160000000000000"));
    (60) => (to_bigint!(b"8320987112741390144276341183223364380754172606361245952449277696409600000000000000"));
    (61) => (to_bigint!(b"507580213877224798800856812176625227226004528988036003099405939480985600000000000000"));
    (62) => (to_bigint!(b"31469973260387937525653122354950764088012280797258232192163168247821107200000000000000"));
    (63) => (to_bigint!(b"1982608315404440064116146708361898137544773690227268628106279599612729753600000000000000"));
    (64) => (to_bigint!(b"126886932185884164103433389335161480802865516174545192198801894375214704230400000000000000"));
    (65) => (to_bigint!(b"8247650592082470666723170306785496252186258551345437492922123134388955774976000000000000000"));
    (66) => (to_bigint!(b"544344939077443064003729240247842752644293064388798874532860126869671081148416000000000000000"));
    (67) => (to_bigint!(b"36471110918188685288249859096605464427167635314049524593701628500267962436943872000000000000000"));
    (68) => (to_bigint!(b"2480035542436830599600990418569171581047399201355367672371710738018221445712183296000000000000000"));
    (69) => (to_bigint!(b"171122452428141311372468338881272839092270544893520369393648040923257279754140647424000000000000000"));
    (70) => (to_bigint!(b"11978571669969891796072783721689098736458938142546425857555362864628009582789845319680000000000000000"));
    (71) => (to_bigint!(b"850478588567862317521167644239926010288584608120796235886430763388588680378079017697280000000000000000"));
    (72) => (to_bigint!(b"61234458376886086861524070385274672740778091784697328983823014963978384987221689274204160000000000000000"));
    (73) => (to_bigint!(b"4470115461512684340891257138125051110076800700282905015819080092370422104067183317016903680000000000000000"));
    (74) => (to_bigint!(b"330788544151938641225953028221253782145683251820934971170611926835411235700971565459250872320000000000000000"));
    (75) => (to_bigint!(b"24809140811395398091946477116594033660926243886570122837795894512655842677572867409443815424000000000000000000"));
    (76) => (to_bigint!(b"1885494701666050254987932260861146558230394535379329335672487982961844043495537923117729972224000000000000000000"));
    (77) => (to_bigint!(b"145183092028285869634070784086308284983740379224208358846781574688061991349156420080065207861248000000000000000000"));
    (78) => (to_bigint!(b"11324281178206297831457521158732046228731749579488251990048962825668835325234200766245086213177344000000000000000000"));
    (79) => (to_bigint!(b"894618213078297528685144171539831652069808216779571907213868063227837990693501860533361810841010176000000000000000000"));
    (80) => (to_bigint!(b"71569457046263802294811533723186532165584657342365752577109445058227039255480148842668944867280814080000000000000000000"));
    (81) => (to_bigint!(b"5797126020747367985879734231578109105412357244731625958745865049716390179693892056256184534249745940480000000000000000000"));
    (82) => (to_bigint!(b"475364333701284174842138206989404946643813294067993328617160934076743994734899148613007131808479167119360000000000000000000"));
    (83) => (to_bigint!(b"39455239697206586511897471180120610571436503407643446275224357528369751562996629334879591940103770870906880000000000000000000"));
    (84) => (to_bigint!(b"3314240134565353266999387579130131288000666286242049487118846032383059131291716864129885722968716753156177920000000000000000000"));
    (85) => (to_bigint!(b"281710411438055027694947944226061159480056634330574206405101912752560026159795933451040286452340924018275123200000000000000000000"));
    (86) => (to_bigint!(b"24227095383672732381765523203441259715284870552429381750838764496720162249742450276789464634901319465571660595200000000000000000000"));
    (87) => (to_bigint!(b"2107757298379527717213600518699389595229783738061356212322972511214654115727593174080683423236414793504734471782400000000000000000000"));
    (88) => (to_bigint!(b"185482642257398439114796845645546284380220968949399346684421580986889562184028199319100141244804501828416633516851200000000000000000000"));
    (89) => (to_bigint!(b"16507955160908461081216919262453619309839666236496541854913520707833171034378509739399912570787600662729080382999756800000000000000000000"));
    (90) => (to_bigint!(b"1485715964481761497309522733620825737885569961284688766942216863704985393094065876545992131370884059645617234469978112000000000000000000000"));
    (91) => (to_bigint!(b"135200152767840296255166568759495142147586866476906677791741734597153670771559994765685283954750449427751168336768008192000000000000000000000"));
    (92) => (to_bigint!(b"12438414054641307255475324325873553077577991715875414356840239582938137710983519518443046123837041347353107486982656753664000000000000000000000"));
    (93) => (to_bigint!(b"1156772507081641574759205162306240436214753229576413535186142281213246807121467315215203289516844845303838996289387078090752000000000000000000000"));
    (94) => (to_bigint!(b"108736615665674308027365285256786601004186803580182872307497374434045199869417927630229109214583415458560865651202385340530688000000000000000000000"));
    (95) => (to_bigint!(b"10329978488239059262599702099394727095397746340117372869212250571234293987594703124871765375385424468563282236864226607350415360000000000000000000000"));
    (96) => (to_bigint!(b"991677934870949689209571401541893801158183648651267795444376054838492222809091499987689476037000748982075094738965754305639874560000000000000000000000"));
    (97) => (to_bigint!(b"96192759682482119853328425949563698712343813919172976158104477319333745612481875498805879175589072651261284189679678167647067832320000000000000000000000"));
    (98) => (to_bigint!(b"9426890448883247745626185743057242473809693764078951663494238777294707070023223798882976159207729119823605850588608460429412647567360000000000000000000000"));
    (99) => (to_bigint!(b"933262154439441526816992388562667004907159682643816214685929638952175999932299156089414639761565182862536979208272237582511852109168640000000000000000000000"));
}

#[cfg(test)]
mod test {
    use crate::fact;
    use num_bigint::{BigInt, ToBigInt};
    #[test]
    fn test_fact() {
        assert_eq!(
            fact!(20).unwrap(),
            2432902008176640000_i64.to_bigint().unwrap()
        );
    }
}
