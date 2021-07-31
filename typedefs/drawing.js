const {gql} = require("apollo-server-express");


module.exports = gql`
  type CharityVoteCount {
    charityId: Charity
    votes: Int
  }
  type Drawing {
    id: ID!
    Charities: [Charity]
    CharityVoteCount: [CharityVoteCount]
    StartDate: String
    EndDate: String
    WinnerWallet: [[Int]]
    TotalRegistrations: Int
    isActive: Boolean
    WinningCharity: [Charity]
     WinningNumbers: [Int]
    Tickets: [Ticket]
    TotalPoolValue:Float
  }
input DrawingInput {
  Charities: [String]
  StartDate: String
  EndDate: String
  isActive:Boolean
}  
  extend type Query {
    getActiveDrawing : Drawing
    getAllDrawing: [Drawing]
    getDrawingById(id:ID):Drawing
  }
  extend type Mutation {
    addDrawing(DrawingInput:DrawingInput) : String
  }
`;
