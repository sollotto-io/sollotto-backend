const { ApolloServer, gql } = require("apollo-server-express");
const express = require("express");
const mongoose = require("mongoose");
const typeDefs = require("./typedefs/index");
const resolvers = require("./resolvers/index");
const { MONGODB } = require("./config");
const cron = require("node-cron");
const cors = require("cors");
const path = require("path");
const { changeDraw } = require("./utils/changeDraw");
const { resetDb } = require("./utils/resetDB");
// var CryptoJS = require("crypto-js");
// const { Account } = require("@solana/web3.js");
const multer = require('multer')

async function startServer() {
  const app = express();
  const server = new ApolloServer({
    typeDefs,
    resolvers,
  });
  await server.start();
  app.use(cors()); 

  app.use("/static", express.static(path.join(__dirname, "public")));
  app.all("/", function (req, res, next) {
    res.header("Access-Control-Allow-Origin", "*");
    res.header("Access-Control-Allow-Headers", "X-Requested-With");
    next();
  });
  server.applyMiddleware({ app: app });
 
  const storage = multer.diskStorage({
    destination: function (req, file, cb) {
      cb(null, 'public')
    },
    filename: (req, file, cb) => {
      cb(null,file.originalname)
    }
  })
  
  const upload = multer({ storage: storage }).single('file')
  mongoose.set("useFindAndModify", false);
mongoose
  .connect(MONGODB, { useNewUrlParser: true, useUnifiedTopology: true })
  .then(() => {
    console.log(`MongoDb Connected`);
    return console.log("hello");
    // .then(() => {
    //   console.log("inside cron then");
    //   cron.schedule("0 0 * * wed,sat", () => {changeDraw()},
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
  })
  .catch((err) => {
    console.log(err);
  });


  app.listen({ port: process.env.PORT || 5000 }, ()=>{console.log("server running at port 5000")})
  app.post('/upload', (req, res) => {
  upload(req, res, (err) => {
    if (err) {
      res.sendStatus(500);
    }
    res.send(req.file);
  });
});
}
startServer();


