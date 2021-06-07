const { ApolloServer } = require("apollo-server");
const express = require('express')
const mongoose = require("mongoose");
const typeDefs = require("./typedefs/index");
const resolvers = require("./resolvers/index");
const { MONGODB } = require("./config");
const { chooseLottery } = require("./utils/chooseLottery.js");
const cron = require("node-cron");
const cors = require('cors')


const app = express()
app.use(cors()) // not having cors enabled will cause an access control error
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
      .listen({ port: process.env.PORT || 5000 })
      .then((res) => {
        console.log(`Server running on ${res.url}`);
      })
        .then(() => {
          var an = 2;
          console.log("inside cron then");
          cron.schedule("0 0 * * WED,SAT", () => {
            const result = chooseLottery(an)
          
            result.then((a) => {
              an = a;
            });
          
          },{
            scheduled: true,
            timezone: "America/Danmarkshavn"
          });
        })
      .catch((err) => {
        console.log(err);
      });
  })
  .catch((err) => {
    console.log(err);
  });

				
