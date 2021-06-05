const gql = require("graphql-tag");

module.exports = gql`
  type CharityVoteCount {
    charityId: Int
    votes: Int
  }
  type Lottery {
    id: ID!
    Id: Int
    Charities: [Int]
    CharityVoteCount: [CharityVoteCount]
    TicketPrice: Float
    StartDate: String
    EndDate: String
    WinnerWallet: [Int]
    TotalPoolValue: Float
    TotalRegistrations: Int
    isActive:Boolean
    LotteryDataAccount: [Int]
  }

  input LotteryInput {
    Id: Int
    Charities: [Int]
    TicketPrice: Float
    StartDate:String
    EndDate:String

  }

  extend type Query{
    getupcomingLottery : Lottery
  }
  extend type Mutation {
    addLottery(LotteryInput: LotteryInput): String
  }
`;
