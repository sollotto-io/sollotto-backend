const AdminUser = require("../models/AdminUser");

const { UserInputError } = require("apollo-server-express");
const bcrypt = require("bcrypt");
const jwt = require("jsonwebtoken");
module.exports = {
  Mutations: {
    async signupUser(_, { userInput: { username, password } }, context, info) {
      if (!username || !password) {
        throw new UserInputError("Please fill all the  fields");
      }
      const alreadyExist = await AdminUser.findOne({ username: username });
      if (alreadyExist) {
        throw new UserInputError("That username already exist");
      }

      const user = new AdminUser({
        username,
        password: bcrypt.hashSync(password, 10),
      });

      const newUser = await user.save();

      if (newUser) {
        return {
          token: jwt.sign(username, process.env.JWT_SECRET),
          username: username,
        };
      } else {
        throw new Error("unable to login");
      }
    },
    async loginUser(_, { userInput: { username, password } }, context, info) {
      const Exist = await AdminUser.findOne({ username: username });
      if (!Exist) {
        throw new UserInputError("That username does Not exist");
      }

      const passwordMatch = bcrypt.compareSync(password, Exist.password);
      if (!passwordMatch) throw new Error("Incorrect credentials");
      return {
        token: jwt.sign(username, process.env.JWT_SECRET),
        username: username,
      };
    },
  },
};
