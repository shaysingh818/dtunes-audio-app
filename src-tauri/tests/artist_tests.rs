use dtunes_audio_app_lib::dtunes_api; 

#[cfg(test)]
mod artist_instance {

    use crate::dtunes_api::artist::Artist;
    use crate::dtunes_api::audio_file::AudioFile;
    use rusqlite::{Connection, Result};

    /* test database instance */
    static DB_PATH: &str = "../database/tests/dtunes-test-artists.sqlite3";

    #[test]
    fn test_create_and_view_artists() -> Result<()> {
        let conn = Connection::open(DB_PATH)?;

        for i in 0..5 {
            let artist_value = format!("artist_{}", i);
            let mut my_artist: Artist = Artist::new(&artist_value, "storage_path");
            my_artist.insert(&conn)?;
        }

        let file_names = vec!["artist_0", "artist_1", "artist_2", "artist_3", "artist_4"];

        let artists: Vec<Artist> = Artist::retrieve(&conn)?;
        assert_eq!(artists.len(), 5);

        let mut counter = 0;
        for file in artists {
            assert_eq!(&file.artist_name, &file_names[counter]);
            counter += 1;
        }

        conn.execute("DELETE FROM ARTIST", [])?;
        let _ = conn.close();
        Ok(())
    }

    #[test]
    fn test_update_artist() -> Result<()> {
        /* Create connection and insert playlist into db  */
        let conn = Connection::open(DB_PATH)?;
        let mut my_artist: Artist = Artist::new("artist_name", "thumbnail");
        let modified_time = my_artist.last_modified.clone();
        my_artist.insert(&conn)?;

        /* update playlist */
        my_artist.artist_name = String::from("artist_update_1");
        my_artist.update(&conn, "1")?;

        /* ensure that modified times are different */
        let recent_modified_time = my_artist.last_modified;
        assert_ne!(recent_modified_time, modified_time);
        conn.execute("DELETE FROM ARTIST", [])?;
        Ok(())
    }

    #[test]
    fn test_view_artist_by_id() -> Result<()> {
        /* Create connection and insert playlist into db  */
        let conn = Connection::open(DB_PATH)?;
        let mut my_artist: Artist = Artist::new("artist_name", "thumbnail");
        my_artist.insert(&conn)?;

        let my_retrieved_artist = Artist::view(&conn, "1")?;
        assert_eq!(my_retrieved_artist.artist_id, 1);
        assert_eq!(my_retrieved_artist.artist_name, "artist_name");
        conn.execute("DELETE FROM ARTIST", [])?;
        Ok(())
    }

    #[test]
    fn test_delete_artist_by_id() -> Result<()> {
        let conn = Connection::open(DB_PATH)?;
        let mut my_artist: Artist = Artist::new("artist_name", "thumbnail");
        my_artist.insert(&conn)?;
        let artist = Artist::view(&conn, "1")?;

        assert_eq!(artist.artist_id, 1);
        assert_eq!(artist.artist_name, "artist_name");
        assert_eq!(artist.artist_thumbnail, "thumbnail");

        Artist::delete(&conn, "1")?;
        let result = conn.execute("SELECT * FROM ARTIST where ARTIST_ID =?", [&"1"])?;
        assert_eq!(result, 1);
        Ok(())
    }

    #[test]
    fn test_add_artist_audio_file() -> Result<()> {
        let conn = Connection::open(DB_PATH)?;
        let mut my_artist: Artist = Artist::new("artist_name", "thumbnail");
        my_artist.insert(&conn)?;
        let artist = Artist::view(&conn, "1")?;

        for i in 0..5 {
            let file_value = format!("audio_file_{}", i);
            let mut my_file: AudioFile =
                AudioFile::new(&file_value, "storage_path", "thumbnail", 0, "2");
            my_file.insert(&conn)?;

            let audio_file_id = format!("{}", i + 1);
            let audio_file = AudioFile::view(&conn, &audio_file_id)?;
            my_artist.add_audio_file(&conn, audio_file.audio_file_id)?;
        }

        let file_names = vec![
            "audio_file_0",
            "audio_file_1",
            "audio_file_2",
            "audio_file_3",
            "audio_file_4",
        ];

        let mut counter = 0;
        let audio_files = Artist::retrieve_audio_files(&conn, "1")?;
        for item in audio_files {
            assert_eq!(&item.file_name, file_names[counter]);
            assert_eq!(item.audio_file_id, counter + 1);
            counter += 1;
        }

        conn.execute("DELETE FROM ARTIST_AUDIO_FILE", [])?;
        conn.execute("DELETE FROM AUDIO_FILE", [])?;
        conn.execute("DELETE FROM ARTIST", [])?;
        let _ = conn.close();
        Ok(())
    }

    #[test]
    fn test_remove_artist_audio_file() -> Result<()> {
        let conn = Connection::open(DB_PATH)?;
        let mut my_artist: Artist = Artist::new("artist_name", "thumbnail");
        my_artist.insert(&conn)?;

        for i in 0..5 {
            let file_value = format!("audio_file_{}", i);
            let mut my_file: AudioFile =
                AudioFile::new(&file_value, "storage_path", "thumbnail", 0, "2");
            my_file.insert(&conn)?;

            let audio_file_id = format!("{}", i + 1);
            let audio_file = AudioFile::view(&conn, &audio_file_id)?;
            my_artist.add_audio_file(&conn, audio_file.audio_file_id)?;
        }

        my_artist.remove_audio_file(&conn, 3);
        my_artist.remove_audio_file(&conn, 4);
        my_artist.remove_audio_file(&conn, 5);

        let mut counter = 0;
        let audio_files: Vec<AudioFile> = Artist::retrieve_audio_files(&conn, "1")?;
        for item in audio_files {
            assert_eq!(item.audio_file_id, counter + 1);
            counter += 1;
        }

        conn.execute("DELETE FROM ARTIST_AUDIO_FILE", [])?;
        conn.execute("DELETE FROM AUDIO_FILE", [])?;
        conn.execute("DELETE FROM ARTIST", [])?;
        let _ = conn.close();
        Ok(())
    }
}