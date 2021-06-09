const gql = require("graphql-tag");

module.exports = gql`
  type Ticket {
    id: ID!
    walletID: [Int]
    ticketArray: [Int]
    DataWallet: [Int]
    charityId: Int!
    LotteryId:Int!
  }

  extend type Query{
    getUserTickets( walletID: [Int!], LotteryId:Int! ) : [Ticket]!
  }

  extend type Mutation {
    addTicket(
      walletID: [Int]
      ticketArray: [Int]
      DataWallet: [Int]
      charityId: Int  
    LotteryId:Int
    ): String!
  }
`;
