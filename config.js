const dotenv = require('dotenv');
dotenv.config();
module.exports = {
    MONGODB:
      `mongodb://RushiC:rushi.c004@cluster0-shard-00-00.1fjri.mongodb.net:27017,cluster0-shard-00-01.1fjri.mongodb.net:27017,cluster0-shard-00-02.1fjri.mongodb.net:27017/sollotto-offchain?ssl=true&replicaSet=atlas-12x6gn-shard-0&authSource=admin&retryWrites=true&w=majority`,
    CHARITY_STATUS :{
          IN_QUE: "IN QUE",
          VOTE_NOW:"VOTE NOW",
          NOT_ELIGIBLE:"NOT ELIGIBLE"
    }
    }

