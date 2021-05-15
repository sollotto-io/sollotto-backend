const { ApolloServer } = require("apollo-server");
const mongoose = require("mongoose");
const typeDefs = require('./typedefs/index');
const resolvers = require('./resolvers/index')
const { MONGODB } = require("./config");


const server = new ApolloServer({
    typeDefs,
    resolvers,
    // context: ({ req }) => ({ req }),
  });
  
  mongoose.set("useFindAndModify", false);
  mongoose
    .connect(MONGODB, { useNewUrlParser: true, useUnifiedTopology: true })
    .then(() => {
      console.log(`MongoDb Connected`);
      return server
        .listen({ port: 5000 })
        .then((res) => {
          console.log(`Server running on ${res.url}`);
        })
        .catch((err)=>{
            console.log(err)
        })
        
    })
    .catch((err)=>{
        console.log(err)
    });

// console.log("hello")