const { gql } = require("apollo-server-express");

module.exports = gql`
  type LaunchPad {
    id: ID!
    PoolName: String
    PoolImage: String
    Status: Boolean
    TotalWinners: Int
    TimeRemaining: String
    MaxDeposit: Int
  }
  input LaunchPadInput {
    PoolName: String
    PoolImage: String
    TotalWinners: Int
    TimeRemaining: String
    MaxDeposit: Int
  }
  extend type Query {
    getAllLaunched: [LaunchPad]
    getLaunchPadById(Id: ID!): LaunchPad
  }
  extend type Mutation {
    AddLaunchPad(LaunchPadInput: LaunchPadInput): String
    changeLaunchState(Id:ID! Status: Boolean): String
  }
`;
