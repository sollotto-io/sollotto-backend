const { ApolloServer } = require("apollo-server");
const express = require("express");
const mongoose = require("mongoose");
const typeDefs = require("./typedefs/index");
const resolvers = require("./resolvers/index");
const { MONGODB } = require("./config");
const cron = require("node-cron");
const cors = require("cors");
const path = require("path");
const { changeDraw } = require("./utils/changeDraw");

const app = express();
app.use("/static", express.static(path.join(__dirname, "public")));
app.all("/", function (req, res, next) {
	res.header("Access-Control-Allow-Origin", "*");
	res.header("Access-Control-Allow-Headers", "X-Requested-With");
	next();
});

app.use(cors()); // not having cors enabled will cause an access control error
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
      // .then(() => {
      //   console.log("inside cron then");
      //   cron.schedule("02 3 * * wed,sat", () => {changeDraw()},
      //   {
      //     scheduled: true,
      //     timezone: "Atlantic/Azores"
      //   }
      //   );
      // })
      // .then(() => {
      //   console.log("inside cron then");
      //   cron.schedule("*/1 * * * *", () => {changeDraw()},
      //   {
      //     scheduled: true,
      //     timezone: "Atlantic/Azores"
      //   }
      //   );
      // })
      .catch((err) => {
        console.log(err);
      });
  })
  .catch((err) => {
    console.log(err);
  });

