const gql = require("graphql-tag");

module.exports = gql`
  type Charity {
    id: ID!
    catagoryName: String!
    projectDetails: String!
    currentVotes: Int
    addedBy: String!
    lifeTimeVotes: Int
    Status:String!
  }
  input charityInput {
    catagoryName: String!
    projectDetails: String!
    addedBy: String!
    Status:String!
  }

  extend type Mutation {
      addCatagory(charityInput:charityInput): Charity!
  }

`