const gql = require("graphql-tag");

module.exports = gql`
  type Ticket {
    id: ID!
    walletID: [Int]
    ticketArray: [String]
    DataWallet: [Int]
    charityId: Int!
  }

  

  extend type Mutation {
    addTicket(
      walletID: [Int]
      ticketArray: [String]
      DataWallet: [Int]
      charityId: Int  
    ): String!
  }
`;
