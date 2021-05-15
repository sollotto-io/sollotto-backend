const User  = require('../models/user')

module.exports = {
Mutation :{
     async addUser(_,{walletId},context,info){

      const newUser = new User({
        walletId
      })
     const res =  await newUser.save();
     return {
      ...res._doc,
      id: res._id,
     }

    }
}

}