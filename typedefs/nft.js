const { gql } = require("apollo-server-express");

module.exports = gql`
  type nftTicket {
    walletId: String!
    dataAccountId: String!
    transactionId: String!
  }
  type prize {
    image: String!
    address: String!
    name: String!
    collectionName: String!
  }

  input prizeInput {
    image: String!
    address: String!
    name: String!
    collectionName: String!
  }
  enum Status {
    draft
    live
    completed
  }

  type nft {
    id: ID!
    prizes: [prize]!
    endDate: String!
    ticketPrice: Float!
    status: Status!
    tickets: [nftTicket]
  }

  input nftInput {
    prizes: [prizeInput]!
    endDate: String!
    ticketPrice: Float!
    status: Status!
  }
  extend type Query {
    getAllNfts: [nft]
    getActiveNft: nft
  }
  extend type Mutation {
    addNFT(nftInput: nftInput): nft!
    updateNFt(nftId: ID!, nftInput: nftInput): nft
    addNftTicket(
      walletId: String!
      dataAccountId: String!
      transactionId: String!
    ): nftTicket
  }
`;
