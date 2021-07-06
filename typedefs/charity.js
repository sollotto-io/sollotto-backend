const gql = require("graphql-tag");

module.exports = gql`
  type Charity {
    id: ID!
    charityName: String!
    projectDetails: String!
    fundUse:String!
    currentVotes: Int
    addedBy: String!
    lifeTimeVotes: Int
    lifeTimeWins:Int
    Status:String!
    Years: String!
    URL:String!
    isWatch:Boolean
    Grade: String!
    Impact:String!
    webURL: String!
    socialMedia:String
    nominationVotes:Int
  }
  input charityInput {
    charityName: String!
    projectDetails: String!
    fundUse:String!
    addedBy: String!
    Status:String!
    Years: String!
    isWatch:Boolean
    URL:String!
    Grade: String!
    Impact:String!
    webURL: String
    socialMedia:String
  }

  extend type Query{
    getAllCharities: [Charity]!
    getActiveCharities:[Charity!]!
  }
  extend type Mutation {
      addCharity(charityInput:charityInput): Charity!
      addNominationVotes(CharityId:ID!, UserPk:String!,Votes:Int!): String! 
  }

`