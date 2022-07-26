use aidoku::std::{defaults::defaults_get, String};

pub fn get_base_url() -> String {
	let code = get_lang_code();
	match code.as_str() {
		"tr" => String::from("https://asurascanstr.com"),
		_ => String::from("https://asurascans.com"),
	}
}

pub fn get_lang_code() -> String {
	let mut code = String::new();
	if let Ok(languages) = defaults_get("languages").as_array() {
		if let Ok(language) = languages.get(0).as_string() {
			code = language.read();
		}
	}
	code
}

pub fn get_tag_id(tag: String) -> String {
	let id = match tag.as_str() {
		"Action" => "aksiyon",
		"Adaptation" => "adaptasyon",
		"Adult" => "yetiskin",
		"Adventure" => "macera",
		"Another chance" => "another-chance",
		"apocalypse" => "apocalypse",
		"Comedy" => "komedi",
		"Coming Soon" => "coming-soon",
		"Cultivation" => "cultivation",
		"Demon" => "demon",
		"Discord" => "discord",
		"Drama" => "dram",
		"Dungeons" => "zindan",
		"Ecchi" => "ecchi",
		"Fantasy" => "fantezi",
		"Game" => "game",
		"Genius" => "genius",
		"Harem" => "harem",
		"Hero" => "hero",
		"Historical" => "historical",
		"Isekai" => "isekai",
		"Josei" => "josei",
		"Kool Kids" => "kool-kids",
		"Loli" => "loli",
		"Magic" => "buyu",
		"Martial Arts" => "dovus-sanatlari",
		"Mature" => "mature",
		"Mecha" => "mecha",
		"Modern Setting" => "modern-setting",
		"Monsters" => "canavar",
		"Murim" => "murim",
		"Mystery" => "gizem",
		"Necromancer" => "necromancer",
		"Noble" => "noble",
		"Overpowered" => "overpowered",
		"Pets" => "pets",
		"Post-Apocalyptic" => "kiyamet-sonrasi",
		"Psychological" => "psikoloji",
		"Rebirth" => "rebirth",
		"Reincarnation" => "reenkarnasyon",
		"Return" => "return",
		"Returned" => "geri-donen",
		"Returner" => "returner",
		"Revenge" => "intikam",
		"Romance" => "romantizm",
		"School Life" => "okul-hayati",
		"Sci-fi" => "bilim-kurgu",
		"Seinen" => "seinen",
		"Shoujo" => "shoujo",
		"Shounen" => "shounen",
		"Slice of Life" => "yasamdan-kesitler",
		"Super Hero" => "super-hero",
		"Superhero" => "superkahraman",
		"Supernatural" => "dogaustu",
		"Survival" => "hayatta-kalma",
		"System" => "sistem",
		"Time Travel" => "zamanda-yolculuk",
		"Time Travel (Future)" => "time-travel-future",
		"tower" => "kule",
		"Tragedy" => "trajedi",
		"Video Game" => "video-game",
		"Video Games" => "video-oyunlari",
		"Villain" => "villain",
		"Virtual Game" => "virtual-game",
		"Virtual Reality" => "sanal-gerceklik",
		"Virtual World" => "vr",
		"Webtoon" => "webtoon",
		"Wuxia" => "wuxia",
		_ => "",
	};
	String::from(id)
}
