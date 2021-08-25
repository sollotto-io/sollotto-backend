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
});

module.exports = model("AdminUser", adminUserSchema);
