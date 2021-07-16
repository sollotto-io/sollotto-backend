const gql = require("graphql-tag");

module.exports = gql`
  type Charity {
    id: ID!
    charityName: String!
    projectDetails: String!
    fundUse: String!
    currentVotes: Int
    addedBy: String!
    lifeTimeVotes: Int
    lifeTimeWins: Int
    Status: String!
    Years: String!
    URL: String!
    isWatch: Boolean
    Grade: String!
    Impact: String!
    webURL: String!
    socialMedia: String
    nominationVotes: Int
    lifeTimeNominationVotes:Int
    LastNominationVote: String
    publicKey: String!
  }
  input charityInput {
    charityName: String!
    projectDetails: String!
    fundUse: String!
    addedBy: String!
    Status: String!
    Years: String!
    isWatch: Boolean
    URL: String!
    Grade: String!
    Impact: String!
    webURL: String
    socialMedia: String
    publicKey: String!
  }

  extend type Query {
    getAllCharities: [Charity]!
  }
  extend type Mutation {
    addCharity(charityInput: charityInput): Charity!
    addNominationVotes(charityId: ID!, UserPk: String!, Votes: Int!): String!
  }
`;
