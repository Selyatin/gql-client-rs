mod structs;

use crate::structs::{inputs::SinglePostVariables, SinglePost};
use gql_client::Client;

// Initialize endpoint
const ENDPOINT: &'static str = "https://graphqlzero.almansi.me/api";

#[tokio::test]
pub async fn properly_parses_json_errors() {
  let client = Client::new(ENDPOINT);

  // Send incorrect query
  let query = r#"
    query SinglePostQuery($id: ID!) {
      post(id: $id) {
        id1
      }
    }
  "#;

  let variables = SinglePostVariables { id: 2 };
  let errors = client
    .query_with_vars_unwrap::<SinglePost, SinglePostVariables>(query, variables)
    .await
    .err();

  assert_eq!(errors.is_some(), true);
  assert!(errors.unwrap().json().as_ref().unwrap().len() > 0 as usize);
}
