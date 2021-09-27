const { gql } = require("apollo-server-express");

module.exports = gql`
  type PassLaunches {
    id: ID
    winnersWalletsId: [String]
    finishDate: String
  }
  type LaunchPad {
    id: ID!
    tokenName: String!
    tokenLogo: String!
    status: Boolean!
    totalWinners: Int!
    passLaunches: [PassLaunches]
    dueDate: String!
    endDate: String!
    maxDeposit: Int!
    tokenAddress: String!
    frequency: Int!
  }
  input LaunchPadInput {
    tokenName: String!
    tokenLogo: String!
    totalWinners: Int!
    dueDate: String!
    maxDeposit: Int!
    tokenAddress: String!
    frequency: Int!
  }
  extend type Query {
    getAllLaunched: [LaunchPad]
    getLaunchPadById(Id: ID!): LaunchPad
  }
  extend type Mutation {
    AddLaunchPad(LaunchPadInput: LaunchPadInput): LaunchPad
    changeLaunchState(Id: ID!, status: Boolean): LaunchPad
    EditLaunchPad(Id: ID!, LaunchPadInput: LaunchPadInput): LaunchPad
  }
`;
