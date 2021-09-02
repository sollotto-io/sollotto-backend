const AdminUser = require("../models/AdminUser");

const { UserInputError } = require("apollo-server-express");
const bcrypt = require("bcrypt");
const jwt = require("jsonwebtoken");
module.exports = {
  Mutations: {
    async signupUser(
      _,
      { userInput: { username, password, admin } },
      context,
      info
    ) {
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
        admin: admin,
      });

      const newUser = await user.save();

      if (newUser) {
        return {
          token: jwt.sign({ username: username }, process.env.JWT_SECRET, {
            expiresIn: "2h",
          }),
          username: username,
          admin: admin,
          id: newUser._id,
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
        token: jwt.sign(
          { username: username, admin: Exist.admin },
          process.env.JWT_SECRET,
          {
            expiresIn: "2h",
          }
        ),
        username: username,
        admin: Exist.admin,
        id: Exist._id,
      };
    },
    async updateUser(
      _,
      { userEditInput: { username, admin, id } },
      context,
      info
    ) {
      const exist = await AdminUser.findOne({ username: username });
      if (exist) {
        throw new UserInputError(
          "That username you trying to update already exist"
        );
      }
      const user = await AdminUser.findById(id);
      if (!user) {
        throw new UserInputError("That username doesn't exist");
      }
      user.username = username;
      if (admin !== null) user.admin = admin;

      const updatedUser = await user.save();
      return {
        username: username,
        admin: updatedUser.admin,
        id: id,
      };
    },

    async updateUserRole(_, { userId, admin }, context, info) {
      const user = await AdminUser.findById(userId);
      if (!user) {
        throw new UserInputError("That user doesn't exist");
      }
      user.admin = admin;
      const updatedUser = await user.save();
      return {
        username: updatedUser.username,
        admin: updatedUser.admin,
        id: updatedUser._id,
      };
    },
    async changePassword(
      _,
      { changePasswordInput: { id, password } },
      context,
      info
    ) {
      const user = await AdminUser.findById(id);
      if (!user) {
        throw new UserInputError("That user doesn't exist");
      }

      user.password = bcrypt.hashSync(password, 10);

      try {
        await user.save();
        return "Password changed succesfully";
      } catch (e) {
        return "Password changed unsuccesfully";
      }
    },

    async deleteUser(_, { userId }, context, info) {
      const userDeleted = await AdminUser.findByIdAndDelete(userId)
        .then(() => true)
        .catch(() => false);
      if (userDeleted) return "user Succesfully deleted";
      return `could not delete user`;
    },
  },
  Query: {
    async getAllUsers(_, params, context, info) {
      const users = await AdminUser.find();

      return users;
    },
  },
};
