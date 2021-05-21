const {gql} = require("apollo-server");

module.exports = gql`
  type Pool {
    id: ID!
    PoolName: String,
    Pool: String,
    PrizePool: Int,
    TimeRemaining: String,
    PoolARP: String,
    TotalDeposit: Int,
    TotalLiquidity: Int,
    Odds: String,
  }
  input poolInput {
    PoolName: String!,
    Pool: String!,
    PrizePool: Int!,
    TimeRemaining: String!,
    PoolARP: String!,
    TotalDeposit: Int!,
    TotalLiquidity: Int!,
    Odds: String!,
  }

  extend type Query{
    getAllPools: [Pool]!
  }
  extend type Mutation {
      addPool(poolInput:poolInput): Pool!
  }

`