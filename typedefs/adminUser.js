const { gql } = require("apollo-server-express");

module.exports = gql`
  type user {
    username: String!
    password: String!
  }

  type AuthPayLoad {
    token: String!
    username: String!
  }

  input userInput {
    username: String!
    password: String!
  }

  extend type Mutation {
    signupUser(userInput: userInput!): AuthPayLoad!
    loginUser(userInput: userInput!): AuthPayLoad!
  }
`;
