const { model, Schema } = require("mongoose");

const adminUserSchema = new Schema({
  username: {
    require: true,
    type: String,
    unique: true,
  },
  password: {
    require: true,
    type: String,
  },
  admin: {
    require: true,
    type: Boolean,
    default: false,
  },
});

module.exports = model("AdminUser", adminUserSchema);
