const { gql } = require("apollo-server-express");

module.exports = gql`
  type Pool {
    id: ID!
    tokenName: String!
    tokenLogo: String!
    dueDate: String!
    tokenAddress: String!
    status: Boolean
  }
  input poolInput {
    tokenName: String!
    tokenLogo: String!
    dueDate: String!
    tokenAddress: String!
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
