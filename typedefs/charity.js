const gql = require("graphql-tag");

module.exports = gql`
  type Charity {
    id: ID!
    charityName: String!
    projectDetails: String!
    # currentVotes: Number
    addedBy: String!
    # lifeTimeVotes: Number
    Status:String!
  }
  input charityInput {
    charityName: String!
    projectDetails: String!
    addedBy: String!
    Status:String!
  }

  extend type Query{
    getAllCharities: [Charity]!
  }
  extend type Mutation {
      addCharity(charityInput:charityInput): Charity!
  }

`