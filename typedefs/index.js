const root = require("./root");
const charity = require("./charity");
const drawing = require("./drawing");
const lottery = require("./lottery");
const ticket = require("./ticket");
const user = require("./user");
const raffle = require("./raffle");
const launch = require("./launchpad");
const pool = require("./pool");
const adminUser = require("./adminUser");
const model4 = require("./model4");
const typedefs = [
  root,
  charity,
  ticket,
  drawing,
  lottery,
  user,
  raffle,
  launch,
  pool,
  adminUser,
  model4,
];

module.exports = typedefs;
