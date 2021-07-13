const { Connection } = require("@solana/web3.js");
const dotenv = require("dotenv");
dotenv.config();
module.exports = {
	MONGODB: process.env.MONGO_DB,
	CHARITY_STATUS: {
		IN_QUE: "IN QUE",
		VOTE_NOW: "VOTE NOW",
		NOT_ELIGIBLE: "NOT ELIGIBLE",
	},
	 connection : new Connection('https://api.devnet.solana.com'),
	 
};

