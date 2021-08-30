const { gql } = require("apollo-server-express");

module.exports = gql`
  type user {
    username: String!
    password: String!
  }

  type AuthPayLoad {
    token: String!
    username: String!
    admin: Boolean
  }
  type UserPayload {
    username: String!
    admin: Boolean!
  }

  input userInput {
    username: String!
    password: String!
    admin: Boolean
  }

  extend type Query {
    getAllUsers: [UserPayload]!
  }
  extend type Mutation {
    signupUser(userInput: userInput!): AuthPayLoad!
    loginUser(userInput: userInput!): AuthPayLoad!
  }
`;
