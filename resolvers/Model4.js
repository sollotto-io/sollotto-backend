const Model4 = require("../models/Model4");

module.exports = {
  Query: {
    async getModel4() {
      const model4 = await Model4.find();
      return model4[0];
    },
  },
};
