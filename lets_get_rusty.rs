#[allow(dead_code)]
#[derive(Debug)]
struct Name {
    person_name: String, pronunciation1: String, pronunciation2: String, bengal: String, arabic: String, japanese: String, greek: String,
}
#[derive(Debug)]
struct Website {
    url: String,
}
#[derive(Debug)]
struct Person<'a> {
    my_name: Name, from: (&'a str, &'a str), email: String, website: Website, blog: Website, interested_in: (&'a str, &'a str), learning: String, collaborate: String, working: String, fev_lang: (&'a str, &'a str), quote: Website,
}
fn main() {
    let my_name: Name = Name {
        person_name: String::from("Kazi Rifat Morshed"),
        pronunciation1: String::from("/'kɑ:zi:  'ri:fæt  'mɔ:rʃɛd/"),
        pronunciation2: String::from("ka-zee  ree-fath  mor-shed"),
        bengal: String::from("কাজী রিফাত মোর্শেদ"),
        arabic: String::from("كازي رفعت مرشد"),
        japanese: String::from("カジ・リファット・モルシェド"),
        greek: String::from("Καζί Ριφάτ Μόρσεντ"),
    };

    let krm: Person = Person {
        my_name: my_name,
        from: ("Khulna", "Bangladesh🇧🇩"),
        email: String::from("kazirifat03.krm@gmail.com"),
        website: Website {
            url: String::from("https://kazirifatmorshed.github.io"),
        },
        blog: Website {
            url: String::from("https://blogofkazirifatjr.blogspot.com"),
        },
        interested_in: ("ArchLinux", "Jōgan"),
        learning: String::from("procrastinating"),
        collaborate: String::from("doing something useful (🤔)"),
        working: String::from("avoiding backlog(retake)😮‍💨"),
        fev_lang: ("java", "rust"),
        quote: Website {
            url: String::from("https://github.com/KaziRifatMorshed/quotes#kazirifatmorshed"),
        },
    };

    println!("👋 Hi, I`m Kazi Rifat Morshed 😄 \n{krm:#?}");
}
