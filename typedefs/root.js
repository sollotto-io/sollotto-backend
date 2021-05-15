const gql = require("graphql-tag");

const root = gql`
type Query{
    root:String
}
type Mutation{
    root:String
}

`
module.exports = root;