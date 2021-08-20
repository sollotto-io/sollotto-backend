const { gql } = require("apollo-server-express");

module.exports = gql`
  type Pool {
    id: ID!
    tokenName: String!
    tokenLogo: String!
    prizePool: Float!
    DueDate: String!
    tokenAddress: String!
    depositLimit: Int!
    numberOfWinners: Int!
    status: Boolean
  }
  input poolInput {
    tokenName: String!
    tokenLogo: String!
    prizePool: Float!
    DueDate: String!
    tokenAddress: String!
    depositLimit: Int!
    numberOfWinners: Int!
  }

  extend type Query {
    getAllPools: [Pool]
    getSinglePool(poolId: ID!): Pool
  }
  extend type Mutation {
    addPool(poolInput: poolInput): Pool!
    updatePool(poolId: ID!, poolInput: poolInput): Pool!
    changePoolStatus(poolId: ID!, status: Boolean): Pool!
  }
`;
