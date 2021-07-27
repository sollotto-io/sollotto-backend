const { gql } = require("apollo-server-express");

module.exports = gql`
  type Raffle {
    id: ID!
    raffleName: String
  publicKey: String
  Status: Boolean
  ImageURL:String
  }

  extend type Query{
    getAllRaffle : [Raffle]
    getActiveRaffle: Raffle
  }
  input raffleInput {
  raffleName: String
  publicKey : String
  ImageURL:String
  }

  extend type Mutation {
    addRaffle(raffleInput:raffleInput) : String
    changeRaffleStatus(raffleId: ID!, Status: Boolean): String
  }
`;
