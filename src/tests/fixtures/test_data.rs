use crate::models::library::Library;
use crate::tests::fixtures::raw_library::get_empty_library;
pub async fn get_test_library() -> Library {
    let lib = get_empty_library().await;

    sqlx::query(DB102_TEST_DATA)
        .execute(&mut *lib.db.get().await.unwrap())
        .await
        .unwrap();

    lib
}

pub const DB102_TEST_DATA: &str = r#"
PRAGMA foreign_keys=OFF;
BEGIN TRANSACTION;
INSERT INTO folders VALUES(0,'/tmp/','uuid');

INSERT INTO entries VALUES(0,0,'maxwell.png','maxwell.png','.png',NULL,NULL,NULL);
INSERT INTO entries VALUES(1,0,'doge.png','doge.png','.png',NULL,NULL,NULL);
INSERT INTO entries VALUES(2,0,'doge_and_maxwell.png','doge_and_maxwell.png','.jpeg',NULL,NULL,NULL);
INSERT INTO entries VALUES(3,0,'OIIA.png','OIIA.png','.png',NULL,NULL,NULL);
INSERT INTO entries VALUES(4,0,'somwhere/far/away.png','away.png','.png',NULL,NULL,NULL);

INSERT INTO tags VALUES(1000,'Cat',NULL,NULL,NULL,0,NULL,NULL);
INSERT INTO tags VALUES(1001,'Maxwell',NULL,NULL,NULL,0,NULL,NULL);
INSERT INTO tags VALUES(1002,'Meme',NULL,NULL,NULL,0,NULL,NULL);
INSERT INTO tags VALUES(1003,'Doge',NULL,NULL,NULL,'',NULL,NULL);
INSERT INTO tags VALUES(1004,'Dog',NULL,NULL,NULL,'',NULL,NULL);
INSERT INTO tags VALUES(1005,'OIIA',NULL,NULL,NULL,'',NULL,NULL);

INSERT INTO tag_parents VALUES(1000, 1001);
INSERT INTO tag_parents VALUES(1002, 1001);
INSERT INTO tag_parents VALUES(1002, 1003);
INSERT INTO tag_parents VALUES(1004, 1003);
INSERT INTO tag_parents VALUES(1002, 1005);
INSERT INTO tag_parents VALUES(1000, 1005);

INSERT INTO tag_entries VALUES(1001,0);
INSERT INTO tag_entries VALUES(1003,1);
INSERT INTO tag_entries VALUES(1003,2);
INSERT INTO tag_entries VALUES(1001,2);
INSERT INTO tag_entries VALUES(1005,3);

INSERT INTO text_fields VALUES('A very dingus cat', 1, 'DESCRIPTION', 0, 0);
COMMIT;
"#;
