const { gql } = require("apollo-server-express");

module.exports = gql`
  type Raffle {
    id: ID!
    raffleName: String!
    urlSlug: String!
    raffleImage: String!
    sollotoBranding: Boolean
    testingWA: String!
    liveWA: String!
    operatorWa: String!
    vanityUrl: String!
    raffleStatus: String!
    status: Boolean!
  }

  extend type Query {
    getAllRaffle: [Raffle]
    getActiveRaffle: Raffle
  }
  input raffleInput {
    raffleName: String!
    urlSlug: String!
    raffleImage: String!
    sollotoBranding: Boolean
    testingWA: String!
    liveWA: String!
    operatorWa: String!
    vanityUrl: String!
    raffleStatus: String!
  }

  extend type Mutation {
    addRaffle(raffleInput: raffleInput): String
    changeRaffleStatus(raffleId: ID!, status: Boolean): String
    editRaffle(raffleId: ID!, raffleInput: raffleInput): String
  }
`;
