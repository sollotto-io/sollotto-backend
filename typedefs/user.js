const gql = require("graphql-tag");

module.exports = gql`
  type User {
    id: ID!
    UserPK:String
    TokenValue:Int
  }
extend type Query {
    addUser(Userpk:String): Int!
}

`;
