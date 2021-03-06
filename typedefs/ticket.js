const { gql } = require("apollo-server-express");

module.exports = gql`
  type Ticket {
    id: ID!
    walletID: [Int]
    ticketArray: [Int]
    DataWallet: [Int]
    charityId: Charity!
    TransactionId: String!
  }

  # extend type Query{
  #
  # }

  extend type Query {
    getTicketsByUserCount(walletId: [Int]!): Int!
    getAllTickets: [Ticket]!
    getTicketsCount: Int!
  }

  extend type Mutation {
    addTicket(
      walletID: [Int]
      ticketArray: [Int]
      DataWallet: [Int]
      charityId: String!
      drawingId: String!
      TransactionId: String!
      UserPK: String!
    ): String
  }
`;
