use bereal::{migrations, storage::establish_connection, util};
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    run().await
}

async fn run() {
    dotenv().ok();

    util::setup_tracing();
    tracing::info!("BeReal is starting");

    let connection = &mut establish_connection();
    migrations::run(connection).unwrap();

    // Storage is repository wrapping diesel connection
    // let storage = bereal::storage().await;

    let schema = bereal::bot::schema::root();
    let bot = bereal::bot::bot_from_env();

    bereal::dispatch(bot, schema).await;
}

// #[cfg(test)]
// mod tests {
//     use diesel::RunQueryDsl;

//     #[test]
//     fn test_typeorm() {
//         println!("hi");

//         diesel::insert_into(animals)
//             .values(&vec![])
//             .execute(&connection)?;
//     }
// }

// fn bogdan() {
//     user_to_warehouses.filter(user_id.eq("userId"))
//     // foo.filter(bar.or(baz))
//     //
//     //
//     foo.filter(id.eq(5).or(other.eq(6))).filter(foo.eq(7));
//     //
//     foo.filter(bar).or_filter(baz)
//     foo.filter(bar.or(baz))
//     //
//     // foo.filter(id.eq(5).or(other.eq(7)))
//     foo.filter(id.eq(7)).filter(id2.eq(7))
//     foo.filter(foo.eq)
//     //
//     //
//     // this.userToWarehouses
//     //     .createQueryBuilder("user_to_warehouses")
//     //     .delete()
//     //     .where("user_to_warehouses.user_id = :userId")
//     //     .andWhere()

//     // query: START TRANSACTION
//     // query: DELETE FROM "user_to_warehouses" WHERE user_to_warehouses.user_id = $1 AND user_to_warehouses.access_type = $2 OR (user_to_warehouses.id NOT IN ($3)
//     //           AND user_to_warehouses.access_type = $4) -- PARAMETERS: ["66a86d35-1795-4b13-b1f5-5ea12095f482","WRITE","7ca2182e-a6fd-40f2-b2c1-c3c777b7715e","WRITE"]
//     // query: COMMIT
// }

// fn diesel_complex_query() {
//     let versions = Version::belonging_to(krate)
//         .select(id)
//         .order(num.desc())
//         .limit(5);
//     let downloads = version_downloads
//         .filter(date.gt(now - 90.days()))
//         .filter(version_id.eq(any(versions)))
//         .order(date)
//         .load::<Download>(&mut conn)?;
// }

// SELECT version_downloads.*
//     WHERE data > (NOW() - '90 days')
//         AND version_id = ANY(
//         SELECT id FROM versions
//             WHERE crate_id = 1
//             ORDER BY num DESC
//             LIMIT t
//         )
//     ORDER BY date
