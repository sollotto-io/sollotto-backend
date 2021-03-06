const { gql } = require("apollo-server-express");

module.exports = gql`
  type Charity {
    id: ID!
    charityName: String!
    projectDetails: String!
    ImageURL: String
    fundUse: String!
    currentVotes: Int
    addedBy: String!
    lifeTimeVotes: Int
    lifeTimeWins: Int
    Status: Boolean!
    Years: String!
    URL: String!
    isWatch: Boolean
    Grade: String!
    Impact: String!
    webURL: String!
    socialMedia: String
    nominationVotes: Int
    lifeTimeNominationVotes: Int
    LastNominationVote: String
    publicKey: String!
  }
  input charityInput {
    charityName: String
    projectDetails: String
    ImageURL: String
    fundUse: String
    addedBy: String
    Status: Boolean
    Years: String
    isWatch: Boolean
    URL: String
    Grade: String
    Impact: String
    webURL: String
    socialMedia: String
    publicKey: String
  }
  scalar Upload
  extend type Query {
    getAllCharities: [Charity]!
  }
  extend type Mutation {
    addCharity(charityInput: charityInput): String!
    addNominationVotes(charityId: ID!, UserPk: String!, Votes: Int!): String!
    deleteCharity(charityId: ID!, Status: Boolean!): String!
    updateCharity(charityId: ID!, charityInput: charityInput): String!
  }
`;
