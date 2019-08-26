use crate::media::*;
use crate::schema::tv_show;
use diesel::prelude::*;

#[derive(Identifiable, Associations, Queryable, Serialize, Deserialize, PartialEq, Debug)]
#[belongs_to(Media, foreign_key = "id")]
#[table_name = "tv_show"]
pub struct TVShow {
    pub id: i32,
}

#[derive(Insertable, Serialize, Deserialize, PartialEq, Debug)]
#[table_name = "tv_show"]
pub struct InsertableTVShow {
    pub id: i32,
}

impl TVShow {
    pub fn get(conn: &diesel::PgConnection, req_id: i32) -> Result<Media, diesel::result::Error> {
        use crate::schema::media::dsl::*;
        let result = media
            .select(MEDIA_ALL_COLUMNS)
            .filter(id.eq(req_id))
            .first(conn)?;
        Ok(result)
    }

    pub fn get_all(conn: &diesel::PgConnection) -> Result<Vec<Media>, diesel::result::Error> {
        use crate::schema::media;
        let result = media::dsl::media
            .inner_join(tv_show::dsl::tv_show)
            .select(MEDIA_ALL_COLUMNS)
            .load(conn)?;
        Ok(result)
    }
}

impl InsertableTVShow {
    pub fn insert(&self, conn: &diesel::PgConnection) -> Result<usize, diesel::result::Error> {
        let count = diesel::insert_into(tv_show::table)
            .values(self)
            .execute(conn)?;
        Ok(count)
    }
}
