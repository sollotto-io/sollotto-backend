const gql = require("graphql-tag");

module.exports = gql`
  type Lottery {
    id: ID!

    TicketPrice: Float
    TotalPoolValue: Float
    LotteryDataAccount: [Int]
  }

  input LotteryInput {
    LotteryDataAccount: [Int]
   }

  # extend type Query {
  #   }
  extend type Mutation {
    addLottery(LotteryInput:LotteryInput ): String
    }
`;
