const moment = require("moment");
const User = require("../models/User");

module.exports = {
  Mutations: {
     
  },
  Query: {
    async addUser(_,{Userpk}, context, args){
      const user =await User.findOne({UserPK:Userpk})

      if(user){
        return user.TokenValue;
      }
      const newUser = new User({
        UserPK:Userpk,
        TokenValue:10
      })
      await newUser.save();
      return newUser.TokenValue;
    },
  }
};
