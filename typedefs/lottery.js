const {gql} = require("apollo-server-express");


module.exports = gql`
  type Lottery {
    id: ID!
    LotteryId: Int
    TicketPrice: Float
    TotalPoolValue: Float
    LotteryDataAccount: String
  }



  extend type Query {
    getLotteryInfo : Lottery!
    }
`; 
