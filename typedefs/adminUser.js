const { gql } = require("apollo-server-express");

module.exports = gql`
  type user {
    username: String!
    password: String!
  }

  type AuthPayLoad {
    id: ID!
    token: String!
    username: String!
    admin: Boolean
  }
  type UserPayload {
    username: String!
    admin: Boolean!
    id: ID!
  }

  input userInput {
    username: String!
    password: String!
    admin: Boolean
  }
  input changePasswordInput {
    id: ID!
    password: String!
  }
  input userEditInput {
    id: ID!
    username: String!
    admin: Boolean
  }

  extend type Query {
    getAllUsers: [UserPayload]!
  }
  extend type Mutation {
    signupUser(userInput: userInput!): AuthPayLoad!
    loginUser(userInput: userInput!): AuthPayLoad!
    updateUser(userEditInput: userEditInput): UserPayload!
    updateUserRole(userId: ID!, admin: Boolean!): UserPayload!
    changePassword(changePasswordInput: changePasswordInput): String!
    deleteUser(userId: ID!): String!
  }
`;
