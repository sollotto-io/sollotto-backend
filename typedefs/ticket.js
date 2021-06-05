const gql = require("graphql-tag");

module.exports = gql`
  type Ticket {
    id: ID!
    walletID: [Int]
    ticketArray: [String]
    DataWallet: [Int]
    charityId: Int!
    LotteryId:Int!
  }

  extend type Query{
    getDataWallets: [Ticket]! 
  }

  extend type Mutation {
    addTicket(
      walletID: [Int]
      ticketArray: [String]
      DataWallet: [Int]
      charityId: Int  
    LotteryId:Int
    ): String!
  }
`;
