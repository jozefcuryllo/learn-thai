use gtk::{prelude::*, Label};
use gtk::{Application, ApplicationWindow, Button};
use pango::glib::random_int_range;
use rodio::{Decoder, OutputStream, Sink};
use std::fmt;
use std::fs::File;
use std::io::BufReader;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;

struct Letter {
    letter: String,
    english_letter: String,
    pronunciation: String,
    example: Option<String>,
    example_meaning: Option<String>,
    consonant: bool,
}

#[derive(Clone)]
struct Context {
    curr_index: usize,
}

impl fmt::Display for Letter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut res = format!("Letter: {}\n", self.letter).to_owned();

        match &self.example {
            Some(example) => res = res + format!("Example: {}\n", example).as_str(),
            None => (),
        };

        res = res + format!("Pronunciation: {}\n", self.pronunciation).as_str();

        match &self.example_meaning {
            Some(example_meaning) => {
                res = res + format!("Example meaning: {}\n", example_meaning).as_str()
            }
            None => (),
        };

        res = res + format!("English letter: {}\n", self.english_letter).as_str();

        write!(f, "{}", res)
    }
}

fn play(letters_rc: Arc<Mutex<Vec<Letter>>>, current_index: usize) {
    thread::spawn(move || {
        let binding = letters_rc.lock().unwrap();
        let l = &binding.get(current_index).unwrap();
        l.play_letter();
    });
}

fn compose_view(
    l: &&Letter,
    label_1: &Label,
    label_3: &Label,
    label_4: &Label,
) -> () {
    let label_markup_1 = format!(
        "<span font_desc='{} Normal 40'>{}</span>  <span font_desc='{} Normal 30'>{}</span>",
        "Noto Looped Thai UI", l.letter, "Arial", l.letter
    );
    label_1.set_markup_with_mnemonic(&label_markup_1);
    label_4.set_markup(&format!("English letter: {}", &l.english_letter));
    label_4.hide();

    let txt = match &l.example {
        Some(example) => format!(
            "<span font_desc='Noto Looped Thai UI Normal'>Example: {}, {}, {}</span>",
            example.to_string(),
            l.pronunciation,
            l.example_meaning.as_ref().unwrap().to_string()
        ),
        None => format!("{}", ""),
    };

    label_3.set_markup(&txt);
    label_3.hide();
}

fn build_ui(app: &gtk::Application, shared_state: Arc<Mutex<Context>>) -> ApplicationWindow {
    let letters = get_letters();

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Learn Thai")
        .default_width(600)
        .default_height(400)
        .build();

    let button_next = Button::with_label("Next");
    let button_prev = Button::with_label("Previous");
    let button_random = Button::with_label("Random");
    let button_show = Button::with_label("Show");

    let v_box = gtk::Box::new(gtk::Orientation::Vertical, 10);
    let h_box_letters = gtk::Box::new(gtk::Orientation::Horizontal, 10);
    let h_box_show_hide = gtk::Box::new(gtk::Orientation::Horizontal, 10);
    let h_box_buttons = gtk::Box::new(gtk::Orientation::Horizontal, 10);

    let label_1 = Label::new(Some(""));
    let label_3 = Label::new(Some(""));
    label_3.hide();
    let label_4 = Label::new(Some(""));
    label_4.hide();

    h_box_letters.pack_start(&label_1, true, true, 0);
    h_box_letters.set_hexpand(true);

    h_box_show_hide.pack_start(&button_show, true, false, 0);
    h_box_show_hide.set_hexpand(true);

    h_box_buttons.pack_start(&button_prev, true, false, 0);
    h_box_buttons.pack_start(&button_next, true, false, 0);
    h_box_buttons.pack_start(&button_random, true, false, 0);
    h_box_buttons.set_hexpand(true);

    v_box.pack_start(&h_box_letters, false, false, 0);
    v_box.pack_start(&label_4, false, false, 0);
    v_box.pack_start(&label_3, false, false, 0);
    v_box.pack_start(&h_box_show_hide, false, false, 0);
    v_box.pack_start(&h_box_buttons, false, false, 0);

    let label_1_rc = Rc::new(label_1);
    let label_3_rc = Rc::new(label_3);
    let label_4_rc = Rc::new(label_4);
    let letters_rc = Arc::new(Mutex::new(letters));

    let letters_rc_1 = letters_rc.clone();
    let shared_state_clone_1 = Arc::clone(&shared_state);
    let label_1_rc_1 = label_1_rc.clone();
    let label_3_rc_1 = label_3_rc.clone();
    let label_4_rc_1 = label_4_rc.clone();
    button_next.connect_clicked(move |_| {
        let mut current_index = shared_state_clone_1.lock().unwrap().curr_index;

        if current_index < letters_rc_1.lock().unwrap().len() - 1 {
            shared_state_clone_1.lock().unwrap().curr_index += 1;
            current_index += 1;
        }
        let binding = letters_rc_1.lock().unwrap();
        compose_view(
            &binding.get(current_index).unwrap(),
            &label_1_rc_1,
            &label_3_rc_1,
            &label_4_rc_1,
        );
    });

    let letters_rc_2 = letters_rc.clone();
    let shared_state_clone_2 = Arc::clone(&shared_state);
    let label_1_rc_2 = label_1_rc.clone();
    let label_3_rc_2 = label_3_rc.clone();
    let label_4_rc_2 = label_4_rc.clone();
    button_prev.connect_clicked(move |_| {
        let mut current_index = shared_state_clone_2.lock().unwrap().curr_index;

        if current_index > 0 {
            shared_state_clone_2.lock().unwrap().curr_index -= 1;
            current_index -= 1;
        }

        let binding = letters_rc_2.lock().unwrap();
        compose_view(
            &binding.get(current_index).unwrap(),
            &label_1_rc_2,
            &label_3_rc_2,
            &label_4_rc_2,
        );
    });

    let letters_rc_3 = letters_rc.clone();
    let shared_state_clone_3 = Arc::clone(&shared_state);
    let label_1_rc_3 = label_1_rc.clone();
    let label_3_rc_3 = label_3_rc.clone();
    let label_4_rc_3 = label_4_rc.clone();
    button_random.connect_clicked(move |_| {
        let r = random_int_range(0, letters_rc_3.lock().unwrap().len() as i32);
        shared_state_clone_3.lock().unwrap().curr_index = r as usize;

        let binding = letters_rc_3.lock().unwrap();
        compose_view(
            &binding.get(r as usize).unwrap(),
            &label_1_rc_3,
            &label_3_rc_3,
            &label_4_rc_3,
        );
    });

    let letters_rc_4 = letters_rc.clone();
    let shared_state_clone_4 = Arc::clone(&shared_state);
    let label_3_rc_4 = label_3_rc.clone();
    let label_4_rc_4 = label_4_rc.clone();
    button_show.connect_clicked(move |_| {
        label_4_rc_4.show();
        label_3_rc_4.show();

        let letters_rc_2_clone = Arc::clone(&letters_rc_4);
        play(
            letters_rc_2_clone,
            shared_state_clone_4.lock().unwrap().curr_index,
        );
    });

    let current_index = 0;
    let letters_rc_0 = letters_rc.clone();
    let binding = letters_rc_0.lock().unwrap();
    let label_1_rc_0 = label_1_rc.clone();
    let label_3_rc_0 = label_3_rc.clone();
    let label_4_rc_0 = label_4_rc.clone();
    compose_view(
        &binding.get(current_index).unwrap(),
        &label_1_rc_0,
        &label_3_rc_0,
        &label_4_rc_0,
    );

    window.add(&v_box);

    window.show_all();
    label_3_rc_0.hide();
    label_4_rc_0.hide();

    return window;
}

fn main() {
    let application = Application::builder()
        .application_id("com.example.learn-thai")
        .build();

    let shared_state = Arc::new(Mutex::new(Context { curr_index: 0 }));

    application.connect_activate(move |app| {
        build_ui(app, shared_state.clone());
    });

    application.run();
}

impl Letter {
    fn play_letter(&self) {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let path = format!(
            "audio/{}.mp3",
            if self.consonant {
                self.example.clone().unwrap() // consonants group always have an example
            } else {
                self.letter.clone()
            }
        );

        match File::open(path) {
            Ok(f) => {
                let file = BufReader::new(f);
                let source = Decoder::new(file).unwrap();
                let sink = Sink::try_new(&stream_handle).unwrap();

                sink.append(source);
                sink.sleep_until_end();
            }
            Err(e) => {
                println!("Can't open audio file: {}", e)
            }
        };
    }
}

fn get_letters() -> Vec<Letter> {
    return vec![
        Letter {
            letter: String::from("ก"),
            english_letter: String::from("g"),
            pronunciation: String::from("gɔɔ-gài"),
            example: Some(String::from("ก ไก่")),
            example_meaning: Some(String::from("chicken")),
            consonant: true,
        },
        Letter {
            letter: String::from("ข"),
            english_letter: String::from("k"),
            pronunciation: String::from("kɔ̌ɔ-kài"),
            example: Some(String::from("ข ไข่")),
            example_meaning: Some(String::from("egg")),
            consonant: true,
        },
        Letter {
            letter: String::from("ฃ"),
            english_letter: String::from("k"),
            pronunciation: String::from("kɔ̌ɔ-kùuat"),
            example: Some(String::from("ฃ ขวด")),
            example_meaning: Some(String::from("bottle (no longer in use)")),
            consonant: true,
        },
        Letter {
            letter: String::from("ค"),
            english_letter: String::from("k"),
            pronunciation: String::from("kɔɔ-kwaai"),
            example: Some(String::from("ค ควาย")),
            example_meaning: Some(String::from("buffalo")),
            consonant: true,
        },
        Letter {
            letter: String::from("ฅ"),
            english_letter: String::from("k"),
            pronunciation: String::from("kɔɔ-kon"),
            example: Some(String::from("ฅ คน")),
            example_meaning: Some(String::from("person (no longer a direct object)")),
            consonant: true,
        },
        Letter {
            letter: String::from("ฆ"),
            english_letter: String::from("k"),
            pronunciation: String::from("kɔɔ-rá-kang"),
            example: Some(String::from("ฆ ระฆัง")),
            example_meaning: Some(String::from("bell")),
            consonant: true,
        },
        Letter {
            letter: String::from("ง"),
            english_letter: String::from("ng"),
            pronunciation: String::from("ngɔɔ-nguu"),
            example: Some(String::from("ง งู")),
            example_meaning: Some(String::from("snake")),
            consonant: true,
        },
        Letter {
            letter: String::from("จ"),
            english_letter: String::from("j"),
            pronunciation: String::from("jɔɔ-jaan"),
            example: Some(String::from("จ จาน")),
            example_meaning: Some(String::from("plate")),
            consonant: true,
        },
        Letter {
            letter: String::from("ฉ"),
            english_letter: String::from("ch"),
            pronunciation: String::from("chɔ̌ɔ-chìng"),
            example: Some(String::from("ฉ ฉิ่ง")),
            example_meaning: Some(String::from("cymbals")),
            consonant: true,
        },
        Letter {
            letter: String::from("ช"),
            english_letter: String::from("ch"),
            pronunciation: String::from("chɔɔ-cháang"),
            example: Some(String::from("ช ช้าง")),
            example_meaning: Some(String::from("elephant")),
            consonant: true,
        },
        Letter {
            letter: String::from("ซ"),
            english_letter: String::from("s"),
            pronunciation: String::from("sɔɔ-sôo"),
            example: Some(String::from("ซ โซ่")),
            example_meaning: Some(String::from("chain")),
            consonant: true,
        },
        Letter {
            letter: String::from("ฌ"),
            english_letter: String::from("ch"),
            pronunciation: String::from("chɔɔ-chəə"),
            example: Some(String::from("ฌ เฌอ")),
            example_meaning: Some(String::from("tree")),
            consonant: true,
        },
        Letter {
            letter: String::from("ญ"),
            english_letter: String::from("y"),
            pronunciation: String::from("yɔɔ-yǐng"),
            example: Some(String::from("ญ หญิง")),
            example_meaning: Some(String::from("woman")),
            consonant: true,
        },
        Letter {
            letter: String::from("ฎ"),
            english_letter: String::from("d"),
            pronunciation: String::from("dɔɔ-chá-daa"),
            example: Some(String::from("ฎ ชฎา")),
            example_meaning: Some(String::from("headdress")),
            consonant: true,
        },
        Letter {
            letter: String::from("ฏ"),
            english_letter: String::from("dt"),
            pronunciation: String::from("dtɔɔ-bpà-dtàk"),
            example: Some(String::from("ฏ ปฏัก")),
            example_meaning: Some(String::from("goad")),
            consonant: true,
        },
        Letter {
            letter: String::from("ฐ"),
            english_letter: String::from("t"),
            pronunciation: String::from("tɔ̌ɔ-tǎan"),
            example: Some(String::from("ฐ ฐาน")),
            example_meaning: Some(String::from("pedestal")),
            consonant: true,
        },
        Letter {
            letter: String::from("ฑ"),
            english_letter: String::from("t"),
            pronunciation: String::from("tɔɔ-mon-too"),
            example: Some(String::from("ฑ มณโฑ")),
            example_meaning: Some(String::from("Montho")),
            consonant: true,
        },
        Letter {
            letter: String::from("ฒ"),
            english_letter: String::from("t"),
            pronunciation: String::from("tɔɔ-pûu-tâo"),
            example: Some(String::from("ฒ ผู้เฒ่า")),
            example_meaning: Some(String::from("elder")),
            consonant: true,
        },
        Letter {
            letter: String::from("ณ"),
            english_letter: String::from("n"),
            pronunciation: String::from("nɔɔ-neen"),
            example: Some(String::from("ณ เณร")),
            example_meaning: Some(String::from("novice monk")),
            consonant: true,
        },
        Letter {
            letter: String::from("ด"),
            english_letter: String::from("d"),
            pronunciation: String::from("dɔɔ-dèk"),
            example: Some(String::from("ด เด็ก")),
            example_meaning: Some(String::from("child")),
            consonant: true,
        },
        Letter {
            letter: String::from("ต"),
            english_letter: String::from("dt"),
            pronunciation: String::from("dtɔɔ-dtào"),
            example: Some(String::from("ต เต่า")),
            example_meaning: Some(String::from("turtle")),
            consonant: true,
        },
        Letter {
            letter: String::from("ถ"),
            english_letter: String::from("t"),
            pronunciation: String::from("tɔ̌ɔ-tǔng"),
            example: Some(String::from("ถ ถุง")),
            example_meaning: Some(String::from("sack")),
            consonant: true,
        },
        Letter {
            letter: String::from("ท"),
            english_letter: String::from("t"),
            pronunciation: String::from("tɔɔ-tá-hǎan"),
            example: Some(String::from("ท ทหาร")),
            example_meaning: Some(String::from("soldier")),
            consonant: true,
        },
        Letter {
            letter: String::from("ธ"),
            english_letter: String::from("t"),
            pronunciation: String::from("tɔɔ-tong"),
            example: Some(String::from("ธ ธง")),
            example_meaning: Some(String::from("flag")),
            consonant: true,
        },
        Letter {
            letter: String::from("น"),
            english_letter: String::from("n"),
            pronunciation: String::from("nɔɔ-nǔu"),
            example: Some(String::from("น หนู")),
            example_meaning: Some(String::from("mouse")),
            consonant: true,
        },
        Letter {
            letter: String::from("บ"),
            english_letter: String::from("b"),
            pronunciation: String::from("bɔɔ-bai-mái"),
            example: Some(String::from("บ ใบไม้")),
            example_meaning: Some(String::from("leaf")),
            consonant: true,
        },
        Letter {
            letter: String::from("ป"),
            english_letter: String::from("bp"),
            pronunciation: String::from("bpɔɔ-bplaa"),
            example: Some(String::from("ป ปลา")),
            example_meaning: Some(String::from("fish")),
            consonant: true,
        },
        Letter {
            letter: String::from("ผ"),
            english_letter: String::from("p"),
            pronunciation: String::from("pɔ̌ɔ-pʉ̂ng"),
            example: Some(String::from("ผ ผึ้ง")),
            example_meaning: Some(String::from("bee")),
            consonant: true,
        },
        Letter {
            letter: String::from("ฝ"),
            english_letter: String::from("f"),
            pronunciation: String::from("fɔ̌ɔ-fǎa"),
            example: Some(String::from("ฝ ฝา")),
            example_meaning: Some(String::from("lid")),
            consonant: true,
        },
        Letter {
            letter: String::from("พ"),
            english_letter: String::from("p"),
            pronunciation: String::from("pɔɔ-paan"),
            example: Some(String::from("พ พาน")),
            example_meaning: Some(String::from("tray")),
            consonant: true,
        },
        Letter {
            letter: String::from("ฟ"),
            english_letter: String::from("f"),
            pronunciation: String::from("fɔɔ-fan"),
            example: Some(String::from("ฟ ฟัน")),
            example_meaning: Some(String::from("teeth")),
            consonant: true,
        },
        Letter {
            letter: String::from("ภ"),
            english_letter: String::from("p"),
            pronunciation: String::from("pɔɔ-sǎm-pao"),
            example: Some(String::from("ภ สำเภา")),
            example_meaning: Some(String::from("junk boat")),
            consonant: true,
        },
        Letter {
            letter: String::from("ม"),
            english_letter: String::from("m"),
            pronunciation: String::from("mɔɔ-máa"),
            example: Some(String::from("ม ม้า")),
            example_meaning: Some(String::from("horse")),
            consonant: true,
        },
        Letter {
            letter: String::from("ย"),
            english_letter: String::from("y"),
            pronunciation: String::from("yɔɔ-yák"),
            example: Some(String::from("ย ยักษ์")),
            example_meaning: Some(String::from("giant")),
            consonant: true,
        },
        Letter {
            letter: String::from("ร"),
            english_letter: String::from("r"),
            pronunciation: String::from("rɔɔ-rʉʉa"),
            example: Some(String::from("ร เรือ")),
            example_meaning: Some(String::from("boat")),
            consonant: true,
        },
        Letter {
            letter: String::from("ล"),
            english_letter: String::from("l"),
            pronunciation: String::from("lɔɔ-ling"),
            example: Some(String::from("ล ลิง")),
            example_meaning: Some(String::from("monkey")),
            consonant: true,
        },
        Letter {
            letter: String::from("ว"),
            english_letter: String::from("w"),
            pronunciation: String::from("wɔɔ-wɛ̌ɛn"),
            example: Some(String::from("ว แหวน")),
            example_meaning: Some(String::from("ring")),
            consonant: true,
        },
        Letter {
            letter: String::from("ศ"),
            english_letter: String::from("s"),
            pronunciation: String::from("sɔ̌ɔ-sǎa-laa"),
            example: Some(String::from("ศ ศาลา")),
            example_meaning: Some(String::from("pavilion")),
            consonant: true,
        },
        Letter {
            letter: String::from("ษ"),
            english_letter: String::from("s"),
            pronunciation: String::from("sɔ̌ɔ-rʉʉ-sǐi"),
            example: Some(String::from("ษ ฤๅษี")),
            example_meaning: Some(String::from("hermit")),
            consonant: true,
        },
        Letter {
            letter: String::from("ส"),
            english_letter: String::from("s"),
            pronunciation: String::from("sɔ̌ɔ-sʉ̌ʉa"),
            example: Some(String::from("ส เสือ")),
            example_meaning: Some(String::from("tiger")),
            consonant: true,
        },
        Letter {
            letter: String::from("ห"),
            english_letter: String::from("h"),
            pronunciation: String::from("hɔ̌ɔ-hìip"),
            example: Some(String::from("ห หีบ")),
            example_meaning: Some(String::from("chest")),
            consonant: true,
        },
        Letter {
            letter: String::from("ฬ"),
            english_letter: String::from("l"),
            pronunciation: String::from("lɔɔ-jù-laa"),
            example: Some(String::from("ฬ จุฬา")),
            example_meaning: Some(String::from("kite")),
            consonant: true,
        },
        Letter {
            letter: String::from("อ"),
            english_letter: String::from("o"),
            pronunciation: String::from("ɔɔ-àang"),
            example: Some(String::from("อ อ่าง")),
            example_meaning: Some(String::from("basin")),
            consonant: true,
        },
        Letter {
            letter: String::from("ฮ"),
            english_letter: String::from("h"),
            pronunciation: String::from("hɔɔ-nók-hûuk"),
            example: Some(String::from("ฮ นกฮูก")),
            example_meaning: Some(String::from("owl")),
            consonant: true,
        },
        Letter {
            letter: String::from("อะ"),
            english_letter: String::from("a"),
            pronunciation: String::from("sara a"),
            example: None,
            example_meaning: None,
            consonant: false,
        },
        Letter {
            letter: String::from("อิ"),
            english_letter: String::from("i"),
            pronunciation: String::from("sara i"),
            example: None,
            example_meaning: None,
            consonant: false,
        },
        Letter {
            letter: String::from("อึ"),
            english_letter: String::from("ʉ"),
            pronunciation: String::from("sara ue"),
            example: None,
            example_meaning: None,
            consonant: false,
        },
        Letter {
            letter: String::from("อุ"),
            english_letter: String::from("u"),
            pronunciation: String::from("sara u"),
            example: None,
            example_meaning: None,
            consonant: false,
        },
        Letter {
            letter: String::from("เอะ"),
            english_letter: String::from("e"),
            pronunciation: String::from("sara e"),
            example: None,
            example_meaning: None,
            consonant: false,
        },
        Letter {
            letter: String::from("แอะ"),
            english_letter: String::from("ɛ"),
            pronunciation: String::from("sara ae"),
            example: None,
            example_meaning: None,
            consonant: false,
        },
        Letter {
            letter: String::from("โอะ"),
            english_letter: String::from("o"),
            pronunciation: String::from("sara o"),
            example: None,
            example_meaning: None,
            consonant: false,
        },
        Letter {
            letter: String::from("เอาะ"),
            english_letter: String::from("ɔ"),
            pronunciation: String::from("sara o"),
            example: None,
            example_meaning: None,
            consonant: false,
        },
        Letter {
            letter: String::from("เออะ"),
            english_letter: String::from("ə"),
            pronunciation: String::from("sara oe"),
            example: None,
            example_meaning: None,
            consonant: false,
        },
        Letter {
            letter: String::from("เอียะ"),
            english_letter: String::from("ia"),
            pronunciation: String::from("sara ia"),
            example: None,
            example_meaning: None,
            consonant: false,
        },
        Letter {
            letter: String::from("เอือะ"),
            english_letter: String::from("uea"),
            pronunciation: String::from("sara uea"),
            example: None,
            example_meaning: None,
            consonant: false,
        },
        Letter {
            letter: String::from("อัวะ"),
            english_letter: String::from("ua"),
            pronunciation: String::from("sara ua"),
            example: None,
            example_meaning: None,
            consonant: false,
        },
        Letter {
            letter: String::from("อำ"),
            english_letter: String::from("am"),
            pronunciation: String::from("sara am"),
            example: None,
            example_meaning: None,
            consonant: false,
        },
        Letter {
            letter: String::from("ไอ"),
            english_letter: String::from("ai"),
            pronunciation: String::from("sara ai"),
            example: None,
            example_meaning: None,
            consonant: false,
        },
        Letter {
            letter: String::from("ใอ"),
            english_letter: String::from("ai"),
            pronunciation: String::from("sara ai"),
            example: None,
            example_meaning: None,
            consonant: false,
        },
        Letter {
            letter: String::from("เอา"),
            english_letter: String::from("ao"),
            pronunciation: String::from("sara ao"),
            example: None,
            example_meaning: None,
            consonant: false,
        },
        Letter {
            letter: String::from("อา"),
            english_letter: String::from("aa"),
            pronunciation: String::from("sara a"),
            example: None,
            example_meaning: None,
            consonant: false,
        },
        Letter {
            letter: String::from("อี"),
            english_letter: String::from("ii"),
            pronunciation: String::from("sara i"),
            example: None,
            example_meaning: None,
            consonant: false,
        },
        Letter {
            letter: String::from("อือ"),
            english_letter: String::from("ʉʉ"),
            pronunciation: String::from("sara ue"),
            example: None,
            example_meaning: None,
            consonant: false,
        },
        Letter {
            letter: String::from("อู"),
            english_letter: String::from("uu"),
            pronunciation: String::from("sara u"),
            example: None,
            example_meaning: None,
            consonant: false,
        },
        Letter {
            letter: String::from("เอ"),
            english_letter: String::from("ee"),
            pronunciation: String::from("sara e"),
            example: None,
            example_meaning: None,
            consonant: false,
        },
        Letter {
            letter: String::from("แอ"),
            english_letter: String::from("ɛɛ"),
            pronunciation: String::from("sara ae"),
            example: None,
            example_meaning: None,
            consonant: false,
        },
        Letter {
            letter: String::from("โอ"),
            english_letter: String::from("oo"),
            pronunciation: String::from("sara o"),
            example: None,
            example_meaning: None,
            consonant: false,
        },
        Letter {
            letter: String::from("ออ"),
            english_letter: String::from("ɔ"),
            pronunciation: String::from("sara o"),
            example: None,
            example_meaning: None,
            consonant: false,
        },
        Letter {
            letter: String::from("เออ"),
            english_letter: String::from("əə"),
            pronunciation: String::from("sara oe"),
            example: None,
            example_meaning: None,
            consonant: false,
        },
        Letter {
            letter: String::from("เอีย"),
            english_letter: String::from("iaa"),
            pronunciation: String::from("sara ia"),
            example: None,
            example_meaning: None,
            consonant: false,
        },
        Letter {
            letter: String::from("เอือ"),
            english_letter: String::from("uea"),
            pronunciation: String::from("sara uea"),
            example: None,
            example_meaning: None,
            consonant: false,
        },
        Letter {
            letter: String::from("อัว"),
            english_letter: String::from("uaa"),
            pronunciation: String::from("sara ua"),
            example: None,
            example_meaning: None,
            consonant: false,
        },
    ];
}
