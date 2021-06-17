const dotenv = require('dotenv');
dotenv.config();
module.exports = {
    MONGODB:
      process.env.DEVELOPMENT,
    CHARITY_STATUS :{
          IN_QUE: "IN QUE",
          VOTE_NOW:"VOTE NOW",
          NOT_ELIGIBLE:"NOT ELIGIBLE"
    }
    }

