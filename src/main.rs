use gtk::prelude::*;
use gtk::{
	Application, ApplicationWindow, Box as GtkBox, Button, Label, Orientation, PolicyType,
	ScrolledWindow, Stack, StackTransitionType, StackSwitcher, ComboBoxText,
};
use gtk::glib;
use std::{collections::HashMap, cell::RefCell, rc::Rc};
use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;

const APP_ID: &str = "org.corvid.Corvid";

#[derive(Debug, Clone, PartialEq, Eq)]
struct Meaning {
	translations: HashMap<String, String>,
}

impl Meaning {
	fn new() -> Self {
		Self { translations: HashMap::new() }
	}

	fn add_translation(&mut self, code: &str, text: &str) {
		self.translations.insert(code.to_string(), text.to_string());
	}

	fn get_translation(&self, code: &str) -> Option<String> {
		self.translations.get(code).cloned()
	}
}

#[derive(Debug, Clone)]
struct Question {
	presented_word: String,
	correct: Meaning,
	choices: Vec<Meaning>,
	language_code: String,
}

impl Question {
	fn new(presented: String, correct: Meaning, mut choices: Vec<Meaning>, lang: &str) -> Self {
		choices.shuffle(&mut thread_rng());
		Self {
			presented_word: presented,
			correct,
			choices,
			language_code: lang.to_string(),
		}
	}
}

struct Game {
	vocab: Vec<Meaning>,
	source_lang: String,
	target_lang: String,
	current: Option<Question>,
	score_correct: u32,
	score_wrong: u32,
	num_choices: u32,
}

impl Game {
	fn new(vocab: Vec<Meaning>, source_lang: &str, target_lang: &str, num_choices: u32) -> Self {
		Self {
			vocab,
			source_lang: source_lang.to_string(),
			target_lang: target_lang.to_string(),
			current: None,
			score_correct: 0,
			score_wrong: 0,
			num_choices,
		}
	}

	fn next_question(&mut self) {
		let mut rng = thread_rng();
		let idx = rng.gen_range(0..self.vocab.len());
		let correct = self.vocab[idx].clone();

		let num_wrong = (self.num_choices - 1) as usize;
		let mut others: Vec<Meaning> = self.vocab
			.iter()
			.filter(|m| *m != &correct)
			.cloned()
			.collect();
		others.shuffle(&mut rng);
		others.truncate(num_wrong);

		let mut choices = vec![correct.clone()];
		choices.extend(others);
		choices.shuffle(&mut rng);

		let presented = correct
			.get_translation(&self.target_lang)
			.unwrap_or_else(|| "???".to_string());

		self.current = Some(Question::new(presented, correct, choices, &self.target_lang));
	}


	fn check_answer(&mut self, choice_index: usize) -> bool {
		if let Some(q) = &self.current {
			let chosen = &q.choices[choice_index];
			let correct = &q.correct;
			let result = chosen == correct;
			if result {
				self.score_correct += 1;
			} else {
				self.score_wrong += 1;
			}
			result
		} else {
			false
		}
	}
}

fn create_animal_vocab() -> Vec<Meaning> {
	let mut vocab = Vec::new();

	let mut add = |es_full: &str, en_full: &str, fr_full: &str, de_full: &str| {
		let mut m = Meaning::new();
		m.add_translation("es", es_full);
		m.add_translation("en", en_full);
		m.add_translation("fr", fr_full);
		m.add_translation("de", de_full);
		vocab.push(m);
	};

	add("El perro",         "The dog",           "Le chien",        "Der Hund");
	add("El gato",          "The cat",           "Le chat",         "Die Katze");
	add("El cerdo",         "The pig",           "Le cochon",       "Das Schwein");
	add("El caballo",       "The horse",         "Le cheval",       "Das Pferd");
	add("El pájaro",        "The bird",          "L’oiseau",        "Der Vogel");
	add("La vaca",          "The cow",           "La vache",        "Die Kuh");
	add("La oveja",         "The sheep",         "Le mouton",       "Das Schaf");
	add("El ratón",         "The mouse",         "La souris",       "Die Maus");
	add("El zorro",         "The fox",           "Le renard",       "Der Fuchs");
	add("El conejo",        "The rabbit",        "Le lapin",        "Der Hase");
	add("El pato",          "The duck",          "Le canard",       "Die Ente");
	add("La tortuga",       "The turtle",        "La tortue",       "Die Schildkröte");
	add("La serpiente",     "The snake",         "Le serpent",      "Die Schlange");
	add("El león",          "The lion",          "Le lion",         "Der Löwe");
	add("El tigre",         "The tiger",         "Le tigre",        "Der Tiger");
	add("El elefante",      "The elephant",      "L’éléphant",      "Der Elefant");
	add("El mono",          "The monkey",        "Le singe",        "Der Affe");
	add("El oso",           "The bear",          "L’ours",          "Der Bär");
	add("El camello",       "The camel",         "Le chameau",      "Das Kamel");
	add("El rinoceronte",   "The rhinoceros",    "Le rhinocéros",   "Das Nashorn");
	add("El ciervo",        "The deer",          "Le cerf",         "Der Hirsch");
	add("La rana",          "The frog",          "La grenouille",   "Der Frosch");
	add("El lobo",          "The wolf",          "Le loup",         "Der Wolf");
	add("La cebra",         "The zebra",         "La zèbre",        "Das Zebra");
	add("La jirafa",        "The giraffe",       "La girafe",       "Die Giraffe");
	add("El hipopótamo",    "The hippopotamus",  "L’hippopotame",   "Das Nilpferd");
	add("El canguro",       "The kangaroo",      "Le kangourou",    "Das Känguru");
	add("El koala",         "The koala",         "Le koala",        "Das Koala");
	add("El pingüino",      "The penguin",       "Le manchot",      "Der Pinguin");
	add("La ballena",       "The whale",         "La baleine",      "Der Wal");
	add("El delfín",        "The dolphin",       "Le dauphin",      "Der Delfin");
	add("El tiburón",       "The shark",         "Le requin",       "Der Hai");
	add("El cocodrilo",     "The crocodile",     "Le crocodile",    "Das Krokodil");
	add("El búho",          "The owl",           "Le hibou",        "Die Eule");
	add("El águila",        "The eagle",         "L’aigle",         "Der Adler");
	add("El pavo real",     "The peacock",       "Le paon",         "Der Pfau");
	add("La mariposa",      "The butterfly",     "Le papillon",     "Der Schmetterling");
	add("La hormiga",       "The ant",           "La fourmi",       "Die Ameise");
	add("La araña",         "The spider",        "L’araignée",      "Die Spinne");
	add("La abeja",         "The bee",           "L’abeille",       "Die Biene");
	add("El alce",          "The moose",         "L’élan",          "Der Elch");
	add("El jaguar",        "The jaguar",        "Le jaguar",       "Der Jaguar");
	add("El búfalo",        "The buffalo",       "Le buffle",       "Der Büffel");
	add("El puma",          "The puma",          "Le puma",         "Der Puma");
	add("La liebre",        "The hare",          "Le lièvre",       "Der Hase");
	add("El corzo",         "The roe deer",      "Le chevreuil",    "Der Reh");
	add("El flamenco",      "The flamingo",      "Le flamant rose", "Der Flamingo");
	add("El zorrillo",      "The skunk",         "Le mouffette",    "Der Stinktier");
	add("El tejón",         "The badger",        "Le blaireau",     "Der Dachs");
	add("El castor",        "The beaver",        "Le castor",       "Der Biber");
	add("El lince",         "The lynx",          "Le lynx",         "Der Luchs");
	add("La comadreja",     "The weasel",        "La belette",      "Das Wiesel");
	add("La mofeta",        "The skunk",         "La moufette",     "Der Stinktier");
	add("El pavo",          "The turkey",        "Le dindon",       "Der Truthahn");
	add("La gacela",        "The gazelle",       "La gazelle",      "Die Gazelle");
	add("El antílope",      "The antelope",      "L’antilope",      "Die Antilope");
	add("El caracol",       "The snail",         "L’escargot",      "Die Schnecke");
	add("La luciérnaga",    "The firefly",       "La luciole",      "Das Glühwürmchen");
	add("El quetzal",       "The quetzal",       "Le quetzal",      "Der Quetzal");
	add("La avestruz",      "The ostrich",       "L’autruche",      "Der Strauß");

	vocab
}

fn create_food_vocab() -> Vec<Meaning> {
	let mut vocab = Vec::new();

	let mut add = |es_full: &str, en_full: &str, fr_full: &str, de_full: &str| {
		let mut m = Meaning::new();
		m.add_translation("es", es_full);
		m.add_translation("en", en_full);
		m.add_translation("fr", fr_full);
		m.add_translation("de", de_full);
		vocab.push(m);
	};

	add("La manzana",       "The apple",        "La pomme",        "Der Apfel");
	add("El plátano",       "The banana",       "La banane",       "Die Banane");
	add("El pan",           "The bread",        "Le pain",         "Das Brot");
	add("El queso",         "The cheese",       "Le fromage",      "Der Käse");
	add("El pollo",         "The chicken",      "Le poulet",       "Das Hähnchen");
	add("El huevo",         "The egg",          "L’œuf",           "Das Ei");
	add("El pescado",       "The fish",         "Le poisson",      "Der Fisch");
	add("La carne",         "The meat",         "La viande",       "Das Fleisch");
	add("La leche",         "The milk",         "Le lait",         "Die Milch");
	add("La naranja",       "The orange",       "L’orange",        "Die Orange");
	add("La pasta",         "The pasta",        "Les pâtes",       "Die Pasta");
	add("El arroz",         "The rice",         "Le riz",          "Der Reis");
	add("La sal",           "The salt",         "Le sel",          "Das Salz");
	add("El sándwich",      "The sandwich",     "Le sandwich",     "Das Sandwich");
	add("La sopa",          "The soup",         "La soupe",        "Die Suppe");
	add("El azúcar",        "The sugar",        "Le sucre",        "Der Zucker");
	add("El té",            "The tea",          "Le thé",          "Der Tee");
	add("El tomate",        "The tomato",       "La tomate",       "Die Tomate");
	add("La verdura",       "The vegetable",    "Le légume",       "Das Gemüse");
	add("El agua",          "The water",        "L’eau",           "Das Wasser");
	add("La uva",           "The grape",        "Le raisin",       "Die Traube");
	add("La fresa",         "The strawberry",   "La fraise",       "Die Erdbeere");
	add("La sandía",        "The watermelon",   "La pastèque",     "Die Wassermelone");
	add("El mango",         "The mango",        "La mangue",       "Die Mango");
	add("El melocotón",     "The peach",        "La pêche",        "Der Pfirsich");
 	add("La pera",          "The pear",         "La poire",        "Die Birne");
	add("La cereza",        "The cherry",       "La cerise",       "Die Kirsche");
	add("El limón",         "The lemon",        "Le citron",       "Die Zitrone");
	add("La lima",          "The lime",         "Le citron vert",  "Die Limette");
	add("La cebolla",       "The onion",        "L’oignon",        "Die Zwiebel");
	add("El ajo",           "The garlic",       "L’ail",           "Der Knoblauch");
	add("La patata",        "The potato",       "La pomme de terre", "Die Kartoffel");
	add("La zanahoria",     "The carrot",       "La carotte",      "Die Karotte");
	add("El brócoli",       "The broccoli",     "Le brocoli",      "Der Brokkoli");
	add("La lechuga",       "The lettuce",      "La laitue",       "Der Kopfsalat");
	add("El pepino",        "The cucumber",     "Le concombre",    "Die Gurke");
	add("El champiñón",     "The mushroom",     "Le champignon",   "Der Pilz");
	add("El pimiento",      "The pepper",       "Le poivron",      "Die Paprika");
	add("El maíz",          "The corn",         "Le maïs",         "Der Mais");
	add("La espinaca",      "The spinach",      "L’épinard",       "Der Spinat");
	add("El yogur",         "The yogurt",       "Le yaourt",       "Der Joghurt");
	add("La piña",          "The pineapple",    "L’ananas",        "Die Ananas");
	add("El aguacate",      "The avocado",      "L’avocat",        "Die Avocado");
	add("El pepino",        "The cucumber",     "Le concombre",    "Die Gurke");
	add("El tomate cherry", "The cherry tomato", "La tomate cerise","Die Cherrytomate");
	add("El calabacín",     "The zucchini",     "La courgette",    "Die Zucchini");
	add("La berenjena",     "The eggplant",     "L’aubergine",     "Die Aubergine");
	add("El pavo",          "The turkey",       "La dinde",        "Der Truthahn");
	add("La langosta",      "The lobster",      "Le homard",       "Der Hummer");
	add("El camarón",       "The shrimp",       "La crevette",     "Die Garnele");
	add("El cangrejo",      "The crab",         "Le crabe",        "Die Krabbe");
	add("La trucha",        "The trout",        "La truite",       "Die Forelle");
	add("El salmón",        "The salmon",       "Le saumon",       "Der Lachs");
	add("La almeja",        "The clam",         "La palourde",     "Die Muschel");
	add("La ostra",         "The oyster",       "L’huitre",        "Die Auster");
	add("La morcilla",      "The black pudding","Le boudin",       "Der Blutwurst");
	add("El queso fresco",  "The fresh cheese", "Le fromage frais","Der Frischkäse");
	add("La crema",         "The cream",        "La crème",        "Die Sahne");
	add("El vinagre",       "The vinegar",      "Le vinaigre",     "Der Essig");
	add("El aceite",        "The oil",          "L’huile",         "Das Öl");
	add("La miel",          "The honey",        "Le miel",         "Der Honig");

	vocab
}

fn create_verb_vocab() -> Vec<Meaning> {
	let mut vocab = Vec::new();

	let mut add = |es: &str, en: &str, fr: &str, de: &str| {
		let mut m = Meaning::new();
		m.add_translation("es", es);
		m.add_translation("en", en);
		m.add_translation("fr", fr);
		m.add_translation("de", de);
		vocab.push(m);
	};

	add("Ser",        "To be (essential/permanent)",     "Être",          "Sein");
	add("Estar",      "To be (state/location)",          "Être",          "Sein");
	add("Tener",      "To have",                         "Avoir",         "Haben");
	add("Hacer",      "To do / make",                    "Faire",         "Machen");
	add("Decir",      "To say (tell)",                   "Dire",          "Sagen");
	add("Ir",         "To go",                           "Aller",         "Gehen");
	add("Ver",        "To see",                          "Voir",          "Sehen");
	add("Dar",        "To give",                         "Donner",        "Geben");
	add("Saber",      "To know (facts)",                 "Savoir",        "Wissen");
	add("Querer",     "To want",                         "Vouloir",       "Wollen");
	add("Llegar",     "To arrive",                       "Arriver",       "Ankommen");
	add("Pasar",      "To pass (spend time)",            "Passer",        "Verbringen");
	add("Deber",      "To owe (should)",                 "Devoir",        "Sollen");
	add("Poner",      "To put",                          "Mettre",        "Stellen");
	add("Parecer",    "To seem",                         "Paraître",      "Scheinen");
	add("Quedar",     "To remain (stay)",                "Rester",        "Bleiben");
	add("Creer",      "To believe",                      "Croire",        "Glauben");
	add("Hablar",     "To speak",                        "Parler",        "Sprechen");
	add("Llevar",     "To carry (wear)",                 "Porter",        "Tragen");
	add("Dejar",      "To leave (behind)",               "Laisser",       "Lassen");
	add("Seguir",     "To follow",                       "Suivre",        "Folgen");
	add("Encontrar",  "To find",                         "Trouver",       "Finden");
	add("Llamar",     "To call",                         "Appeler",       "Rufen");
	add("Venir",      "To come",                         "Venir",         "Kommen");
	add("Pensar",     "To think",                        "Penser",        "Denken");
	add("Salir",      "To go out (leave)",               "Sortir",        "Ausgehen");
	add("Volver",     "To return (come back)",           "Revenir",       "Zurückkommen");
	add("Tomar",      "To take (drink)",                 "Prendre",       "Nehmen");
	add("Conocer",    "To know (people/places)",         "Connaître",     "Kennen");
	add("Vivir",      "To live",                         "Vivre",         "Leben");
	add("Sentir",     "To feel",                         "Ressentir",     "Fühlen");
	add("Mirar",      "To look at",                      "Regarder",      "Ansehen");
	add("Contar",     "To count (tell a story)",         "Raconter",      "Erzählen");
	add("Empezar",    "To begin",                        "Commencer",     "Beginnen");
	add("Esperar",    "To wait (hope)",                  "Attendre",      "Warten");
	add("Buscar",     "To search for",                   "Chercher",      "Suchen");
	add("Entrar",     "To enter",                        "Entrer",        "Eintreten");
	add("Trabajar",   "To work",                         "Travailler",    "Arbeiten");
	add("Escribir",   "To write",                        "Écrire",        "Schreiben");
	add("Perder",     "To lose",                         "Perdre",        "Verlieren");
	add("Producir",   "To produce",                      "Produire",      "Produzieren");
	add("Ocurrir",    "To happen",                       "Survenir",      "Geschehen");
	add("Entender",   "To understand",                   "Comprendre",    "Verstehen");
	add("Pedir",      "To request (ask for)",            "Demander",      "Bitten");
	add("Recibir",    "To receive",                      "Recevoir",      "Erhalten");
	add("Recordar",   "To remember",                     "Se souvenir",   "Erinnern");
	add("Terminar",   "To finish",                       "Terminer",      "Beenden");
	add("Permitir",   "To allow",                        "Permettre",     "Erlauben");
	add("Aparecer",   "To appear",                       "Apparaître",    "Erscheinen");

	vocab
}

fn main() -> glib::ExitCode {
	let app = Application::builder().application_id(APP_ID).build();
	app.connect_activate(build_ui);
	app.run()
}

fn build_ui(app: &Application) {
	let window = ApplicationWindow::builder()
		.application(app)
		.title("Corvid")
		.default_width(400)
		.default_height(300)
 		.build();

	let stack = Stack::builder()
		.transition_type(StackTransitionType::SlideLeftRight)
		.vexpand(true)
		.hexpand(true)
		.build();
    
	let switcher = StackSwitcher::builder()
		.stack(&stack)
		.halign(gtk::Align::Center)
		.margin_bottom(8)
		.build();

	let game = Rc::new(RefCell::new(Game::new(create_animal_vocab(), "en", "es", 7)));
	let current_vocab = Rc::new(RefCell::new(create_animal_vocab as fn() -> Vec<Meaning>));
    
	let last_correct = Rc::new(RefCell::new(false));

	let open_vocab_btn = Button::with_label("Vocabulary");
	let prefs_btn = Button::with_label("Preferences");
	for btn in &[&open_vocab_btn, &prefs_btn] {
		btn.set_margin_top(12);
		btn.set_margin_bottom(12);
		btn.set_margin_start(12);
		btn.set_margin_end(12);
	}
	let main_menu = GtkBox::builder()
		.orientation(Orientation::Vertical)
		.spacing(8)
		.build();
	main_menu.append(&open_vocab_btn);
	main_menu.append(&prefs_btn);
	stack.add_named(&main_menu, Some("main_menu"));

	let back_btn_prefs = Button::with_label("Back");
	let source_lang_combo = ComboBoxText::new();
	let target_lang_combo = ComboBoxText::new();
    
	for (code, name) in &[
		("en", "English"),
		("es", "Spanish"),
		("fr", "French"),
		("de", "German"),
	] {
		source_lang_combo.append(Some(code), name);
		target_lang_combo.append(Some(code), name);
	}
	source_lang_combo.set_active_id(Some("en"));
	target_lang_combo.set_active_id(Some("es"));

	let prefs_box = GtkBox::builder()
		.orientation(Orientation::Vertical)
		.spacing(8)
		.margin_top(12)
		.margin_bottom(12)
		.margin_start(12)
		.margin_end(12)
		.build();
    
	prefs_box.append(&Label::new(Some("Known Language:")));
	prefs_box.append(&source_lang_combo);
	prefs_box.append(&Label::new(Some("Learning Language:")));
	prefs_box.append(&target_lang_combo);
	prefs_box.append(&back_btn_prefs);
	stack.add_named(&prefs_box, Some("preferences"));

	let topic_btn_animals = Button::with_label("Animals");
	let topic_btn_foods = Button::with_label("Foods");
	let topic_btn_basic_verbs = Button::with_label("Basic Verbs");
	let back_btn_topic = Button::with_label("Back");
	for btn in &[&topic_btn_animals, &topic_btn_foods, &topic_btn_basic_verbs, &back_btn_topic] {
		btn.set_margin_top(12);
		btn.set_margin_bottom(12);
		btn.set_margin_start(12);
		btn.set_margin_end(12);
	}
	let topic_box = GtkBox::builder()
		.orientation(Orientation::Vertical)
		.spacing(8)
		.vexpand(true)
		.build();
	topic_box.append(&topic_btn_animals);
	topic_box.append(&topic_btn_foods);
	topic_box.append(&topic_btn_basic_verbs);
	let topic_scroll = ScrolledWindow::builder()
		.hscrollbar_policy(PolicyType::Never)
		.min_content_width(360)
		.vexpand(true)
		.child(&topic_box)
		.build();
	let topic_container = GtkBox::builder()
		.orientation(Orientation::Vertical)
		.hexpand(true)
		.vexpand(true)
		.build();
	topic_container.append(&back_btn_topic);
	topic_container.append(&topic_scroll);
	stack.add_named(&topic_container, Some("topic_selection"));

	let easy_btn = Button::with_label("Easy (3 choices)");
	let medium_btn = Button::with_label("Medium (5 choices)");
	let hard_btn = Button::with_label("Hard (7 choices)");
	let back_btn_difficulty = Button::with_label("Back");
	let direction_label = Label::new(Some("Learning Direction:"));
	let direction_combo = ComboBoxText::new();
	direction_combo.append(Some("normal"), "Normal");
	direction_combo.append(Some("reverse"), "Reverse");
	direction_combo.set_active_id(Some("normal"));

	for btn in &[&easy_btn, &medium_btn, &hard_btn, &back_btn_difficulty] {
		btn.set_margin_top(12);
		btn.set_margin_bottom(12);
		btn.set_margin_start(12);
		btn.set_margin_end(12);
	}
	for label in &[&direction_label] {
		label.set_margin_top(12);
		label.set_margin_bottom(12);
		label.set_margin_start(12);
		label.set_margin_end(12);
	}
	for combo in &[&direction_combo] {
		combo.set_margin_top(12);
		combo.set_margin_bottom(12);
		combo.set_margin_start(12);
		combo.set_margin_end(12);
	}

	let difficulty_box = GtkBox::builder()
		.orientation(Orientation::Vertical)
		.spacing(8)
		.vexpand(true)
		.build();
    
	difficulty_box.append(&direction_label);
	difficulty_box.append(&direction_combo);
	difficulty_box.append(&easy_btn);
	difficulty_box.append(&medium_btn);
	difficulty_box.append(&hard_btn);

	let difficulty_scroll = ScrolledWindow::builder()
		.hscrollbar_policy(PolicyType::Never)
		.min_content_width(360)
		.vexpand(true)
		.child(&difficulty_box)
		.build();

	let difficulty_container = GtkBox::builder()
		.orientation(Orientation::Vertical)
		.hexpand(true)
		.vexpand(true)
		.build();
    
	difficulty_container.append(&back_btn_difficulty);
	difficulty_container.append(&difficulty_scroll);
	stack.add_named(&difficulty_container, Some("difficulty_selection"));

	let back_btn_quiz = Button::with_label("Back");
	let word_label = Label::new(None);
    
	for btn in &[&back_btn_quiz] {
		btn.set_margin_top(12);
		btn.set_margin_bottom(12);
		btn.set_margin_start(12);
		btn.set_margin_end(12);
	}
	for label in &[&word_label] {
		label.set_margin_top(12);
		label.set_margin_bottom(12);
		label.set_margin_start(12);
		label.set_margin_end(12);
	}

	let btns: Rc<RefCell<Vec<Button>>> = Rc::new(RefCell::new(
		(0..7).map(|i| {
			let b = Button::with_label(&format!("Choice {}", i + 1));
			b.set_margin_top(12);
			b.set_margin_bottom(12);
			b.set_margin_start(12);
			b.set_margin_end(12);
			b
		}).collect()
	));

	let quiz_box = GtkBox::builder()
		.orientation(Orientation::Vertical)
		.spacing(8)
		.vexpand(true)
		.build();
	quiz_box.append(&back_btn_quiz);
	quiz_box.append(&word_label);
	for b in btns.borrow().iter() {
		quiz_box.append(b);
	}

	let score_box = GtkBox::builder()
		.orientation(Orientation::Horizontal)
		.spacing(16)
		.margin_top(12)
		.margin_bottom(12)
		.halign(gtk::Align::Center)
		.hexpand(false)
		.build();
	let correct_label = Label::new(Some("Correct: 0"));
	let wrong_label = Label::new(Some("Wrong:   0"));
	score_box.append(&correct_label);
	score_box.append(&wrong_label);
	quiz_box.append(&score_box);
	stack.add_named(&quiz_box, Some("quiz_view"));

	let result_label = Label::new(None);
	let ok_btn = Button::with_label("OK");
    
	for label in &[&result_label] {
		label.set_margin_top(12);
 		label.set_margin_bottom(12);
		label.set_margin_start(12);
		label.set_margin_end(12);
	}
	for btn in &[&ok_btn] {
		btn.set_margin_top(12);
		btn.set_margin_bottom(12);
		btn.set_margin_start(12);
		btn.set_margin_end(12);
	}
    
	let result_box = GtkBox::builder()
		.orientation(Orientation::Vertical)
		.spacing(8)
		.build();
	result_box.append(&result_label);
	result_box.append(&ok_btn);
	stack.add_named(&result_box, Some("result_view"));

	{
		let s = stack.clone();
		open_vocab_btn.connect_clicked(move |_| {
			s.set_visible_child_name("topic_selection");
		});
	}

	{
		let s = stack.clone();
		prefs_btn.connect_clicked(move |_| {
			s.set_visible_child_name("preferences");
		});
	}

	{
		let s = stack.clone();
		back_btn_prefs.connect_clicked(move |_| {
			s.set_visible_child_name("main_menu");
		});
	}

	{
		let s = stack.clone();
		let current_vocab = current_vocab.clone();
		topic_btn_animals.connect_clicked(move |_| {
			current_vocab.replace(create_animal_vocab);
			s.set_visible_child_name("difficulty_selection");
		});
	}
    
	{
		let s = stack.clone();
		let current_vocab = current_vocab.clone();
		topic_btn_foods.connect_clicked(move |_| {
			current_vocab.replace(create_food_vocab);
			s.set_visible_child_name("difficulty_selection");
		});
	}
    
	{
		let s = stack.clone();
		let current_vocab = current_vocab.clone();
		topic_btn_basic_verbs.connect_clicked(move |_| {
			current_vocab.replace(create_verb_vocab);
			s.set_visible_child_name("difficulty_selection");
		});
	}

	let add_difficulty_handler = |difficulty: u32, btn: &Button| {
		let s = stack.clone();
		let game = game.clone();
		let word_label = word_label.clone();
		let btns = btns.clone();
		let correct_lbl = correct_label.clone();
		let wrong_lbl = wrong_label.clone();
		let source_combo = source_lang_combo.clone();
		let target_combo = target_lang_combo.clone();
		let direction_combo = direction_combo.clone();
		let current_vocab = current_vocab.clone();
        
		btn.connect_clicked(move |_| {
			let direction = direction_combo.active_id()
				.unwrap_or_else(|| glib::GString::from("normal"))
				.to_string();

			let original_source = source_combo.active_id()
				.unwrap_or_else(|| glib::GString::from("en"))
				.to_string();
			let original_target = target_combo.active_id()
				.unwrap_or_else(|| glib::GString::from("es"))
				.to_string();

			let (source, target) = if direction == "reverse" {
				(original_target.clone(), original_source.clone())
			} else {
				(original_source.clone(), original_target.clone())
			};

			game.replace(Game::new(
				current_vocab.borrow()(),
				&source,
				&target,
				difficulty
			));
            
			let mut g = game.borrow_mut();
			g.next_question();
			let q = g.current.as_ref().unwrap();
            
			word_label.set_text(&q.presented_word);
			correct_lbl.set_text(&format!("Correct: {}", g.score_correct));
			wrong_lbl.set_text(&format!("Wrong:   {}", g.score_wrong));
            
			for (i, button) in btns.borrow_mut().iter_mut().enumerate() {
				if (i as u32) < difficulty {
					let meaning = &q.choices[i];
					let txt = meaning
						.get_translation(&g.source_lang)
						.unwrap_or("???".to_string());
					button.set_label(&txt);
					button.show();
 				} else {
					button.hide();
				}
			}
			s.set_visible_child_name("quiz_view");
		});
	};

	add_difficulty_handler(3, &easy_btn);
	add_difficulty_handler(5, &medium_btn);
	add_difficulty_handler(7, &hard_btn);

	for (i, button) in btns.borrow().iter().cloned().enumerate() {
		let game = game.clone();
		let s = stack.clone();
		let result_lbl = result_label.clone();
		let last_correct = last_correct.clone();
		button.connect_clicked(move |_| {
			let mut g = game.borrow_mut();
			let correct = g.check_answer(i);
			*last_correct.borrow_mut() = correct;
			result_lbl.set_text(if correct { "Correct!" } else { "Wrong!" });
			s.set_visible_child_name("result_view");
		});
	}

	{
		let game = game.clone();
		let s = stack.clone();
		let word_label = word_label.clone();
		let btns = btns.clone();
		let correct_lbl = correct_label.clone();
		let wrong_lbl = wrong_label.clone();
		let last_correct = last_correct.clone();
		ok_btn.connect_clicked(move |_| {
			let mut g = game.borrow_mut();
			if *last_correct.borrow() {
				g.next_question();
			}
			let q = g.current.as_ref().unwrap();
			word_label.set_text(&q.presented_word);
			correct_lbl.set_text(&format!("Correct: {}", g.score_correct));
			wrong_lbl.set_text(&format!("Wrong:   {}", g.score_wrong));

			for (i, button) in btns.borrow_mut().iter_mut().enumerate() {
				if (i as u32) < g.num_choices {
					let meaning = &q.choices[i];
					let txt = meaning
						.get_translation(&g.source_lang)
						.unwrap_or("???".to_string());
					button.set_label(&txt);
					button.show();
				} else {
					button.hide();
				}
			}
			s.set_visible_child_name("quiz_view");
		});
	}

	{
		let s = stack.clone();
		back_btn_quiz.connect_clicked(move |_| {
		correct_label.set_text("Correct: 0");
		wrong_label.set_text("Wrong:   0");
		s.set_visible_child_name("topic_selection");
		});
	}
	{
		let s = stack.clone();
		back_btn_difficulty.connect_clicked(move |_| {
			s.set_visible_child_name("topic_selection");
		});
	}
	{
		let s = stack.clone();
		back_btn_topic.connect_clicked(move |_| {
			s.set_visible_child_name("main_menu");
		});
	}

	let root = GtkBox::builder()
		.orientation(Orientation::Vertical)
		.build();
	root.append(&switcher);
	root.append(&stack);
	window.set_child(Some(&root));
	window.show();
}
