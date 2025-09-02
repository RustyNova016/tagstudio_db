use crate::models::library::Library;

pub async fn get_empty_library() -> Library {
    let lib = Library::in_memory().unwrap();

    sqlx::query(DB101_SCHEMA)
        .execute(&mut *lib.db.get().await.unwrap())
        .await
        .unwrap();

    lib
}

pub const DB101_SCHEMA: &str = r#"
PRAGMA foreign_keys=OFF;
BEGIN TRANSACTION;
CREATE TABLE namespaces (
	namespace VARCHAR NOT NULL, 
	name VARCHAR NOT NULL, 
	PRIMARY KEY (namespace)
);
INSERT INTO namespaces VALUES('tagstudio-standard','TagStudio Standard');
INSERT INTO namespaces VALUES('tagstudio-pastels','TagStudio Pastels');
INSERT INTO namespaces VALUES('tagstudio-shades','TagStudio Shades');
INSERT INTO namespaces VALUES('tagstudio-earth-tones','TagStudio Earth Tones');
INSERT INTO namespaces VALUES('tagstudio-grayscale','TagStudio Grayscale');
INSERT INTO namespaces VALUES('tagstudio-neon','TagStudio Neon');
CREATE TABLE folders (
	id INTEGER NOT NULL, 
	path VARCHAR NOT NULL, 
	uuid VARCHAR NOT NULL, 
	PRIMARY KEY (id), 
	UNIQUE (path), 
	UNIQUE (uuid)
);
CREATE TABLE value_type (
	"key" VARCHAR NOT NULL, 
	name VARCHAR NOT NULL, 
	type VARCHAR(9) NOT NULL, 
	is_default BOOLEAN NOT NULL, 
	position INTEGER NOT NULL, 
	PRIMARY KEY ("key")
);
INSERT INTO value_type VALUES('TITLE','Title','TEXT_LINE',1,0);
INSERT INTO value_type VALUES('AUTHOR','Author','TEXT_LINE',0,1);
INSERT INTO value_type VALUES('ARTIST','Artist','TEXT_LINE',0,2);
INSERT INTO value_type VALUES('URL','URL','TEXT_LINE',0,3);
INSERT INTO value_type VALUES('DESCRIPTION','Description','TEXT_BOX',0,4);
INSERT INTO value_type VALUES('NOTES','Notes','TEXT_BOX',0,5);
INSERT INTO value_type VALUES('COLLATION','Collation','TEXT_LINE',0,9);
INSERT INTO value_type VALUES('DATE','Date','DATETIME',0,10);
INSERT INTO value_type VALUES('DATE_CREATED','Date Created','DATETIME',0,11);
INSERT INTO value_type VALUES('DATE_MODIFIED','Date Modified','DATETIME',0,12);
INSERT INTO value_type VALUES('DATE_TAKEN','Date Taken','DATETIME',0,13);
INSERT INTO value_type VALUES('DATE_PUBLISHED','Date Published','DATETIME',0,14);
INSERT INTO value_type VALUES('BOOK','Book','TEXT_LINE',0,17);
INSERT INTO value_type VALUES('COMIC','Comic','TEXT_LINE',0,18);
INSERT INTO value_type VALUES('SERIES','Series','TEXT_LINE',0,19);
INSERT INTO value_type VALUES('MANGA','Manga','TEXT_LINE',0,20);
INSERT INTO value_type VALUES('SOURCE','Source','TEXT_LINE',0,21);
INSERT INTO value_type VALUES('DATE_UPLOADED','Date Uploaded','DATETIME',0,22);
INSERT INTO value_type VALUES('DATE_RELEASED','Date Released','DATETIME',0,23);
INSERT INTO value_type VALUES('VOLUME','Volume','TEXT_LINE',0,24);
INSERT INTO value_type VALUES('ANTHOLOGY','Anthology','TEXT_LINE',0,25);
INSERT INTO value_type VALUES('MAGAZINE','Magazine','TEXT_LINE',0,26);
INSERT INTO value_type VALUES('PUBLISHER','Publisher','TEXT_LINE',0,27);
INSERT INTO value_type VALUES('GUEST_ARTIST','Guest Artist','TEXT_LINE',0,28);
INSERT INTO value_type VALUES('COMPOSER','Composer','TEXT_LINE',0,29);
INSERT INTO value_type VALUES('COMMENTS','Comments','TEXT_LINE',0,30);
CREATE TABLE preferences (
	"key" VARCHAR NOT NULL, 
	value JSON NOT NULL, 
	PRIMARY KEY ("key")
);
CREATE TABLE tag_colors (
	slug VARCHAR NOT NULL, 
	namespace VARCHAR NOT NULL, 
	name VARCHAR NOT NULL, 
	"primary" VARCHAR NOT NULL, 
	secondary VARCHAR, 
	color_border BOOLEAN NOT NULL, 
	PRIMARY KEY (slug, namespace), 
	FOREIGN KEY(namespace) REFERENCES namespaces (namespace)
);
INSERT INTO tag_colors VALUES('red','tagstudio-standard','Red','#E22C3C',NULL,0);
INSERT INTO tag_colors VALUES('red-orange','tagstudio-standard','Red Orange','#E83726',NULL,0);
INSERT INTO tag_colors VALUES('orange','tagstudio-standard','Orange','#ED6022',NULL,0);
INSERT INTO tag_colors VALUES('amber','tagstudio-standard','Amber','#FA9A2C',NULL,0);
INSERT INTO tag_colors VALUES('yellow','tagstudio-standard','Yellow','#FFD63D',NULL,0);
INSERT INTO tag_colors VALUES('lime','tagstudio-standard','Lime','#92E649',NULL,0);
INSERT INTO tag_colors VALUES('green','tagstudio-standard','Green','#45D649',NULL,0);
INSERT INTO tag_colors VALUES('teal','tagstudio-standard','Teal','#22D589',NULL,0);
INSERT INTO tag_colors VALUES('cyan','tagstudio-standard','Cyan','#3DDBDB',NULL,0);
INSERT INTO tag_colors VALUES('blue','tagstudio-standard','Blue','#3B87F0',NULL,0);
INSERT INTO tag_colors VALUES('indigo','tagstudio-standard','Indigo','#874FF5',NULL,0);
INSERT INTO tag_colors VALUES('purple','tagstudio-standard','Purple','#BB4FF0',NULL,0);
INSERT INTO tag_colors VALUES('pink','tagstudio-standard','Pink','#FF62AF',NULL,0);
INSERT INTO tag_colors VALUES('magenta','tagstudio-standard','Magenta','#F64680',NULL,0);
INSERT INTO tag_colors VALUES('coral','tagstudio-pastels','Coral','#F2525F',NULL,0);
INSERT INTO tag_colors VALUES('salmon','tagstudio-pastels','Salmon','#F66348',NULL,0);
INSERT INTO tag_colors VALUES('light-orange','tagstudio-pastels','Light Orange','#FF9450',NULL,0);
INSERT INTO tag_colors VALUES('light-amber','tagstudio-pastels','Light Amber','#FFBA57',NULL,0);
INSERT INTO tag_colors VALUES('light-yellow','tagstudio-pastels','Light Yellow','#FFE173',NULL,0);
INSERT INTO tag_colors VALUES('light-lime','tagstudio-pastels','Light Lime','#C9FF7A',NULL,0);
INSERT INTO tag_colors VALUES('light-green','tagstudio-pastels','Light Green','#81FF76',NULL,0);
INSERT INTO tag_colors VALUES('mint','tagstudio-pastels','Mint','#68FFB4',NULL,0);
INSERT INTO tag_colors VALUES('sky-blue','tagstudio-pastels','Sky Blue','#8EFFF4',NULL,0);
INSERT INTO tag_colors VALUES('light-blue','tagstudio-pastels','Light Blue','#64C6FF',NULL,0);
INSERT INTO tag_colors VALUES('lavender','tagstudio-pastels','Lavender','#908AF6',NULL,0);
INSERT INTO tag_colors VALUES('lilac','tagstudio-pastels','Lilac','#DF95FF',NULL,0);
INSERT INTO tag_colors VALUES('light-pink','tagstudio-pastels','Light Pink','#FF87BA',NULL,0);
INSERT INTO tag_colors VALUES('burgundy','tagstudio-shades','Burgundy','#6E1C24',NULL,0);
INSERT INTO tag_colors VALUES('auburn','tagstudio-shades','Auburn','#A13220',NULL,0);
INSERT INTO tag_colors VALUES('olive','tagstudio-shades','Olive','#4C652E',NULL,0);
INSERT INTO tag_colors VALUES('dark-teal','tagstudio-shades','Dark Teal','#1F5E47',NULL,0);
INSERT INTO tag_colors VALUES('navy','tagstudio-shades','Navy','#104B98',NULL,0);
INSERT INTO tag_colors VALUES('dark_lavender','tagstudio-shades','Dark Lavender','#3D3B6C',NULL,0);
INSERT INTO tag_colors VALUES('berry','tagstudio-shades','Berry','#9F2AA7',NULL,0);
INSERT INTO tag_colors VALUES('black','tagstudio-grayscale','Black','#111018',NULL,0);
INSERT INTO tag_colors VALUES('dark-gray','tagstudio-grayscale','Dark Gray','#242424',NULL,0);
INSERT INTO tag_colors VALUES('gray','tagstudio-grayscale','Gray','#53525A',NULL,0);
INSERT INTO tag_colors VALUES('light-gray','tagstudio-grayscale','Light Gray','#AAAAAA',NULL,0);
INSERT INTO tag_colors VALUES('white','tagstudio-grayscale','White','#F2F1F8',NULL,0);
INSERT INTO tag_colors VALUES('dark-brown','tagstudio-earth-tones','Dark Brown','#4C2315',NULL,0);
INSERT INTO tag_colors VALUES('brown','tagstudio-earth-tones','Brown','#823216',NULL,0);
INSERT INTO tag_colors VALUES('light-brown','tagstudio-earth-tones','Light Brown','#BE5B2D',NULL,0);
INSERT INTO tag_colors VALUES('blonde','tagstudio-earth-tones','Blonde','#EFC664',NULL,0);
INSERT INTO tag_colors VALUES('peach','tagstudio-earth-tones','Peach','#F1C69C',NULL,0);
INSERT INTO tag_colors VALUES('warm-gray','tagstudio-earth-tones','Warm Gray','#625550',NULL,0);
INSERT INTO tag_colors VALUES('cool-gray','tagstudio-earth-tones','Cool Gray','#515768',NULL,0);
INSERT INTO tag_colors VALUES('neon-red','tagstudio-neon','Neon Red','#180607','#E22C3C',1);
INSERT INTO tag_colors VALUES('neon-red-orange','tagstudio-neon','Neon Red Orange','#220905','#E83726',1);
INSERT INTO tag_colors VALUES('neon-orange','tagstudio-neon','Neon Orange','#1F0D05','#ED6022',1);
INSERT INTO tag_colors VALUES('neon-amber','tagstudio-neon','Neon Amber','#251507','#FA9A2C',1);
INSERT INTO tag_colors VALUES('neon-yellow','tagstudio-neon','Neon Yellow','#2B1C0B','#FFD63D',1);
INSERT INTO tag_colors VALUES('neon-lime','tagstudio-neon','Neon Lime','#1B220C','#92E649',1);
INSERT INTO tag_colors VALUES('neon-green','tagstudio-neon','Neon Green','#091610','#45D649',1);
INSERT INTO tag_colors VALUES('neon-teal','tagstudio-neon','Neon Teal','#09191D','#22D589',1);
INSERT INTO tag_colors VALUES('neon-cyan','tagstudio-neon','Neon Cyan','#0B191C','#3DDBDB',1);
INSERT INTO tag_colors VALUES('neon-blue','tagstudio-neon','Neon Blue','#09101C','#3B87F0',1);
INSERT INTO tag_colors VALUES('neon-indigo','tagstudio-neon','Neon Indigo','#150B24','#874FF5',1);
INSERT INTO tag_colors VALUES('neon-purple','tagstudio-neon','Neon Purple','#1E0B26','#BB4FF0',1);
INSERT INTO tag_colors VALUES('neon-pink','tagstudio-neon','Neon Pink','#210E15','#FF62AF',1);
INSERT INTO tag_colors VALUES('neon-magenta','tagstudio-neon','Neon Magenta','#220A13','#F64680',1);
INSERT INTO tag_colors VALUES('neon-white','tagstudio-neon','Neon White','#131315','#F2F1F8',1);
CREATE TABLE entries (
	id INTEGER NOT NULL, 
	folder_id INTEGER NOT NULL, 
	path VARCHAR NOT NULL, 
	filename VARCHAR NOT NULL, 
	suffix VARCHAR NOT NULL, 
	date_created DATETIME, 
	date_modified DATETIME, 
	date_added DATETIME, 
	PRIMARY KEY (id), 
	FOREIGN KEY(folder_id) REFERENCES folders (id), 
	UNIQUE (path)
);
CREATE TABLE boolean_fields (
	value BOOLEAN NOT NULL, 
	id INTEGER NOT NULL, 
	type_key VARCHAR NOT NULL, 
	entry_id INTEGER NOT NULL, 
	position INTEGER NOT NULL, 
	PRIMARY KEY (id), 
	FOREIGN KEY(type_key) REFERENCES value_type ("key"), 
	FOREIGN KEY(entry_id) REFERENCES entries (id)
);
CREATE TABLE text_fields (
	value VARCHAR, 
	id INTEGER NOT NULL, 
	type_key VARCHAR NOT NULL, 
	entry_id INTEGER NOT NULL, 
	position INTEGER NOT NULL, 
	PRIMARY KEY (id), 
	FOREIGN KEY(type_key) REFERENCES value_type ("key"), 
	FOREIGN KEY(entry_id) REFERENCES entries (id)
);
CREATE TABLE datetime_fields (
	value VARCHAR, 
	id INTEGER NOT NULL, 
	type_key VARCHAR NOT NULL, 
	entry_id INTEGER NOT NULL, 
	position INTEGER NOT NULL, 
	PRIMARY KEY (id), 
	FOREIGN KEY(type_key) REFERENCES value_type ("key"), 
	FOREIGN KEY(entry_id) REFERENCES entries (id)
);
CREATE TABLE tags (
	id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT, 
	name VARCHAR NOT NULL, 
	shorthand VARCHAR, 
	color_namespace VARCHAR, 
	color_slug VARCHAR, 
	is_category BOOLEAN NOT NULL, 
	icon VARCHAR, 
	disambiguation_id INTEGER, 
	FOREIGN KEY(color_namespace, color_slug) REFERENCES tag_colors (namespace, slug)
);
CREATE TABLE tag_parents (
	parent_id INTEGER NOT NULL, 
	child_id INTEGER NOT NULL, 
	PRIMARY KEY (parent_id, child_id), 
	FOREIGN KEY(parent_id) REFERENCES tags (id), 
	FOREIGN KEY(child_id) REFERENCES tags (id)
);
CREATE TABLE tag_entries (
	tag_id INTEGER NOT NULL, 
	entry_id INTEGER NOT NULL, 
	PRIMARY KEY (tag_id, entry_id), 
	FOREIGN KEY(tag_id) REFERENCES tags (id), 
	FOREIGN KEY(entry_id) REFERENCES entries (id)
);
CREATE TABLE tag_aliases (
	id INTEGER NOT NULL, 
	name VARCHAR NOT NULL, 
	tag_id INTEGER NOT NULL, 
	PRIMARY KEY (id), 
	FOREIGN KEY(tag_id) REFERENCES tags (id)
);
CREATE TABLE versions (
	"key" VARCHAR NOT NULL, 
	value INTEGER NOT NULL, 
	PRIMARY KEY ("key")
);

COMMIT;
"#;
