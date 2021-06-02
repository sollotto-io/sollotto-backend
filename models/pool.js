const { model, Schema } = require("mongoose");

const poolSchema = new Schema({
    PoolName: String,
    Pool: String,
    PrizePool: {type:Number, required: false},
    TimeRemaining: String,
    PoolARP:  {type:Number},
    TotalDeposit: {type:Number, required: false},
    TotalLiquidity: {type:Number, required: false},
    Odds: String,
    currentTicketPrice:{type: Number }
});

module.exports = model("Pool", poolSchema);
