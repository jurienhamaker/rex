use graphql_client::{GraphQLQuery, Response};
use std::error::Error;
use reqwest;
use tokio;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/gql/schema.graphql",
    query_path = "src/gql/num_query.graphql",
    response_derives = "Debug"
)]
pub struct NumQuery;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "src/gql/schema.graphql",
    query_path = "src/gql/name_query.graphql",
    response_derives = "Debug"
)]
pub struct NameQuery;

#[tokio::main]
pub async fn fetch_dex_num(num: num_query::Variables) -> Result<graphql_client::Response<num_query::ResponseData>, Box<dyn Error>> {
    let request_body = NumQuery::build_query(num);

    let client = reqwest::Client::new();
    let res = client.post("https://graphqlpokemon.favware.tech/").json(&request_body).send().await?;
    let response_body: Response<num_query::ResponseData> = res.json().await?;
    //println!("{:#?}", response_body);
    Ok(response_body)
}

#[tokio::main]
async fn _fetch_dex_name(name: name_query::Variables) -> Result<(), Box<dyn Error>> {
    let request_body = NameQuery::build_query(name);

    let client = reqwest::Client::new();
    let res = client.post("https://graphqlpokemon.favware.tech/").json(&request_body).send().await?;
    let response_body: Response<name_query::ResponseData> = res.json().await?;
    println!("{:#?}", response_body);
    Ok(())
}

pub fn run() -> Response<num_query::ResponseData> {
        let dexnum = num_query::Variables{
            num: 658
        };
        fetch_dex_num(dexnum).expect("Query unsuccessful!")

        /* let dexname = name_query::Variables{
            pokemon: String::from(matches.value_of("name").unwrap())
        };
        fetch_dex_name(dexname).expect("Query unsuccessful!"); */
}
