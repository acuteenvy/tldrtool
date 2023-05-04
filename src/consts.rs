use std::collections::HashMap;

use once_cell::sync::Lazy;

pub static MORE_INFORMATION: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert("en", "More information");
    m.insert("ar", "لمزيد من التفاصيل");
    m.insert("bn", "আরও তথ্য পাবেন");
    m.insert("bs", "Više informacija");
    m.insert("ca", "Més informació");
    m.insert("da", "Mere information");
    m.insert("de", "Weitere Informationen");
    m.insert("es", "Más información");
    m.insert("fa", "اطلاعات بیشتر");
    m.insert("fr", "Plus d'informations");
    m.insert("hi", "अधिक जानकारी");
    m.insert("id", "Informasi lebih lanjut");
    m.insert("it", "Maggiori informazioni");
    m.insert("ja", "詳しくはこちら");
    m.insert("ko", "더 많은 정보");
    m.insert("lo", "ຂໍ້ມູນເພີ່ມເຕີມ");
    m.insert("ml", "കൂടുതൽ വിവരങ്ങൾ");
    m.insert("ne", "थप जानकारी");
    m.insert("nl", "Meer informatie");
    m.insert("no", "Mer informasjon");
    m.insert("pl", "Więcej informacji");
    m.insert("pt_BR", "Mais informações");
    m.insert("pt_PT", "Mais informações");
    m.insert("ro", "Mai multe informații");
    m.insert("ru", "Больше информации");
    m.insert("sh", "Više informacija");
    m.insert("sr", "Više informacija na");
    m.insert("sv", "Mer information");
    m.insert("ta", "மேலும் விவரத்திற்கு");
    m.insert("th", "ข้อมูลเพิ่มเติม");
    m.insert("tr", "Daha fazla bilgi için");
    m.insert("uk", "Більше інформації");
    m.insert("uz", "Ko'proq malumot");
    m.insert("zh", "更多信息");
    m.insert("zh_TW", "更多資訊");

    m
});

pub static ALIAS_PAGES: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    m.insert(
        "en",
        "# example\n\n\
    > This command is an alias of `example`.\n\n\
    - View documentation for the original command:\n\n\
    `tldr example`",
    );

    m.insert(
        "ar",
        "# example\n\n\
    > هذا الأمر هو اسم مستعار لـ `example`.\n\n\
    - إعرض التوثيقات للأمر الأصلي:\n\n\
    `tldr example`",
    );

    m.insert(
        "bs",
        "# example\n\n\
    > Ova komanda je pseudonim za `example`.\n\n\
    - Pogledaj dokumentaciju za izvornu komandu:\n\n\
    `tldr example`",
    );

    m.insert(
        "ca",
        "# example\n\n\
    > Aquest comandament és un àlies de `example`.\n\n\
    - Veure documentació pel comandament original:\n\n\
    `tldr example`",
    );

    m.insert(
        "da",
        "# example\n\n\
    > Denne kommando er et alias af `example`.\n\n\
    - Se dokumentation for den oprindelige kommando:\n\n\
    `tldr example`",
    );

    m.insert(
        "de",
        "# example\n\n\
    > Dieser Befehl ist ein Alias von `example`.\n\n\
    - Zeige die Dokumentation für den originalen Befehl an:\n\n\
    `tldr example`",
    );

    m.insert(
        "es",
        "# example\n\n\
    > Este comando es un alias de `example`.\n\n\
    - Ver documentación para el comando original:\n\n\
    `tldr example`",
    );

    m.insert(
        "fr",
        "# example\n\n\
    > Cette commande est un alias de `example`.\n\n\
    - Voir la documentation de la commande originale :\n\n\
    `tldr example`",
    );

    m.insert(
        "hi",
        "# example\n\n\
    > यह आदेश `example` का उपनाम है।\n\n\
    - मूल आदेश के लिए दस्तावेज़ देखें:\n\n\
    `tldr example`",
    );

    m.insert(
        "id",
        "# example\n\n\
    > Perintah ini merupakan alias dari `example`.\n\n\
    - Menampilkan dokumentasi untuk perintah asli:\n\n\
    `tldr example`",
    );

    m.insert(
        "it",
        "# example\n\n\
    > Questo comando è un alias per `example`.\n\n\
    - Consulta la documentazione del comando originale:\n\n\
    `tldr example`",
    );

    m.insert(
        "ja",
        "# example\n\n\
    > このコマンドは `example` のエイリアスです。\n\n\
    - オリジナルのコマンドのドキュメントを表示する:\n\n\
    `tldr example`",
    );

    m.insert(
        "ko",
        "# example\n\n\
    > 이 명령은 `example` 의 에일리어스 (별칭) 입니다.\n\n\
    - 원본 명령의 도큐멘테이션 (설명서) 보기:\n\n\
    `tldr example`",
    );

    m.insert(
        "lo",
        "# example\n\n\
    > ຄຳສັ່ງນີ້ເປັນອີກຊື່ໜຶ່ງຂອງຄຳສັ່ງ `example`.\n\n\
    - ເປີດເບິ່ງລາຍລະອຽດຂອງຄຳສັ່ງແບບເຕັມ:\n\n\
    `tldr example`",
    );

    m.insert(
        "ml",
        "# example\n\n\
    > ഈ കമാൻഡ് `example` എന്നത്തിന്റെ അപരനാമമാണ്.\n\n\
    - യഥാർത്ഥ കമാൻഡിനായി ഡോക്യുമെന്റേഷൻ കാണുക:\n\n\
    `tldr example`",
    );

    m.insert(
        "ne",
        "# example\n\n\
    > यो आदेश `example` को उपनाम हो |\n\n\
    - मौलिक आदेशको लागि कागजात हेर्नुहोस्:\n\n\
    `tldr example`",
    );

    m.insert(
        "nl",
        "# example\n\n\
    > Dit commando is een alias van `example`.\n\n\
    - Bekijk de documentatie van het originele commando:\n\n\
    `tldr example`",
    );

    m.insert(
        "no",
        "# example\n\n\
    > Denne kommandoen er et alias for `example`.\n\n\
    - Vis dokumentasjonen for den opprinnelige kommandoen:\n\n\
    `tldr example`",
    );

    m.insert(
        "pl",
        "# example\n\n\
    > To polecenie jest aliasem `example`.\n\n\
    - Zobacz dokumentację oryginalnego polecenia:\n\n\
    `tldr example`",
    );

    m.insert(
        "pt_BR",
        "# example\n\n\
    > Este comando é um pseudônimo de `example`.\n\n\
    - Ver documentação sobre o comando original:\n\n\
    `tldr example`",
    );

    m.insert(
        "pt_PT",
        "# example\n\n\
    > Este comando é um alias de `example`.\n\n\
    - Ver documentação do comando original:\n\n\
    `tldr example`",
    );

    m.insert(
        "ru",
        "# example\n\n\
    > Эта команда — псевдоним для `example`.\n\n\
    - Смотри документацию для оригинальной команды:\n\n\
    `tldr example`",
    );

    m.insert(
        "sv",
        "# example\n\n\
    > Det här kommandot är ett alias för `example`.\n\n\
    - Se dokumentationen för orginalkommandot:\n\n\
    `tldr example`",
    );

    m.insert(
        "ta",
        "# example\n\n\
    > இக்கட்டளை `example` கட்டளையின் மற்றொருப் பெயர்.\n\n\
    - அக்கட்டளையின் விளக்கத்தைக் காண:\n\n\
    `tldr example`",
    );

    m.insert(
        "th",
        "# example\n\n\
    > คำสั่งนี้เป็นอีกชื่อหนึ่งของคำสั่ง `example`.\n\n\
    - เรียกดูรายละเอียดสำหรับคำสั่งตัวเต็ม:\n\n\
    `tldr example`",
    );

    m.insert(
        "tr",
        "# example\n\n\
    > Bu komut `example` için bir takma addır.\n\n\
    - Asıl komutun belgelerini görüntüleyin:\n\n\
    `tldr example`",
    );

    m.insert(
        "uk",
        "# example\n\n\
    > Ця команда є псевдонімом для `example`.\n\n\
    - Дивись документацію для оригінальної команди:\n\n\
    `tldr example`",
    );

    m.insert(
        "zh",
        "# example\n\n\
    > 这是 `example` 命令的一个别名。\n\n\
    - 原命令的文档在：\n\n\
    `tldr example`",
    );

    m.insert(
        "zh_TW",
        "# example\n\n\
    > 這是 `example` 命令的一個別名。\n\n\
    - 原命令的文件在：\n\n\
    `tldr example`",
    );

    m
});
