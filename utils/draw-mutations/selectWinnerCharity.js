const Charity = require("../../models/Charity");
const User = require("../../models/User");
const _ = require('lodash')
const selectWinnerCharity = async () => {
  try {
    const charities = await Charity.find();
    const maxVoteCharities = await _.orderBy(
      charities,
      ["nominationVotes", "LastNominationVote"],
      ["desc", "desc"]
    );
    await Charity.updateMany(
      {},
      { nominationVotes: 0, LastNominationVote: null }
    );
    await User.updateMany({}, { TokenValue: 10 });

    return maxVoteCharities.slice(0, 4);
  } catch (err) {
    throw new Error(err);
  }
};

module.exports ={
    selectWinnerCharity
}