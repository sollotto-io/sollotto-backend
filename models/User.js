const { model, Schema, Types } = require("mongoose");

const UserSchema = new Schema({
  UserPK:String,
    TokenValue:{type:Number}
  
});

module.exports = model("User", UserSchema);
