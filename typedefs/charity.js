const gql = require("graphql-tag");

module.exports = gql`
  type Charity {
    id: ID!
    ID:Int!,
    charityName: String!
    projectDetails: String!
    currentVotes: Int
    addedBy: String!
    lifeTimeVotes: Int
    lifeTimeWins:Int
    Status:String!
    Years: String!
    watchURL:String!
    watchGrade: String!
    Impact:String!
    webURL: String!
  }
  input charityInput {
    ID:Int!
    charityName: String!
    projectDetails: String!
    addedBy: String!
    Status:String!
    Years: String!
    watchURL:String!
    watchGrade: String!
    Impact:String!
    webURL: String
  }

  extend type Query{
    getAllCharities: [Charity]!
    getActiveCharities:[Charity!]!
  }
  extend type Mutation {
      addCharity(charityInput:charityInput): Charity!
  }

`