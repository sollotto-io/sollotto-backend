const { gql } = require("apollo-server-express");

module.exports = gql`
  type passModel4 {
    id: ID
    winningWalletId: String
    finishDate: String
  }
  type Model4 {
    id: ID!
    endDate: String!
    passModel4: [passModel4]
  }
  extend type Query {
    getModel4: Model4!
  }
`;
