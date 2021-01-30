mod util;
mod dbs;
mod gql;

mod users;
mod articles;

use crate::util::constant::CFG;
use crate::gql::{build_schema, graphql, graphiql};

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    // tide logger
    tide::log::start();

    // Initialize the application with state.
    let schema = build_schema().await;
    let app_state = State { schema: schema };
    let mut app = tide::with_state(app_state);

    //environment variables defined in .env file
    let mut gql = app.at(CFG.get("GQL_URI").unwrap());
    gql.at(CFG.get("GQL_VER").unwrap()).post(graphql);
    gql.at(CFG.get("GIQL_VER").unwrap()).get(graphiql);

    app.listen(format!(
        "{}:{}",
        CFG.get("ADDR").unwrap(),
        CFG.get("PORT").unwrap()
    ))
    .await?;

    Ok(())
}

//  Tide application scope state.
#[derive(Clone)]
pub struct State {
    pub schema: async_graphql::Schema<
        gql::queries::QueryRoot,
        gql::mutations::MutationRoot,
        async_graphql::EmptySubscription,
    >,
}
