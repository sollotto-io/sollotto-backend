const { model, Schema } = require("mongoose");

const launchSchema = new Schema({
  PoolName: { type: String, required: true },
  PoolImage: { type: String, required: true },
  Status:{type:Boolean, required:true},
  TotalWinners:{type:Number},
  TimeRemaining:{type:String},
  MaxDeposit:{type:Number},
 
});

module.exports = model("Launch", launchSchema);
