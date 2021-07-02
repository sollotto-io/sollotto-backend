const Lottery = require("../../models/Lottery");
const moment = require("moment");
const Drawing = require("../../models/Drawing");
const Charity = require('../../models/Charity')
const openDrawing = async (activeDrawing) => {
  var newDraw = {};
  const lottery = await Lottery.findById("60c447da624b8a3d5095baa8");

  //60c447da624b8a3d5095baa8
  //60c9be158676ea0799255ee4
  var day = moment(activeDrawing.EndDate).format("dddd");
  const charities = await Charity.find({Status:"Active"}).sort({ createdAt: -1 });
       
  var charityVote = [];
  var drawCharity = []
  charities.map((t) => {
    charityVote.push({
      charityId: t.id,
      votes: 0,
    });
    drawCharity.push(t.id)
  });

  console.log(drawCharity)

  if (day === "Wednesday") {
    if (lottery.TotalPoolValue < 10) {
      newDraw = new Drawing({
        Charities: drawCharity,
        StartDate: activeDrawing.EndDate,
        EndDate: moment(activeDrawing.EndDate).add(3, "days").utc().format(),
        isActive: true,
        CharityVoteCount: charityVote,
        TotalPoolValue: 10,
      });
    } else {
      newDraw = new Drawing({
        Charities: drawCharity,
        StartDate: activeDrawing.EndDate,
        EndDate: moment(activeDrawing.EndDate).add(3, "days").utc().format(),
        isActive: true,
        CharityVoteCount: charityVote,
        TotalPoolValue: lottery.TotalPoolValue,
      });
    }
    await newDraw.save();
    console.log("draw open");
  } else if (day === "Saturday") {
    if (lottery.TotalPoolValue < 10) {
      newDraw = new Drawing({
        Charities: drawCharity,
        StartDate: activeDrawing.EndDate,
        EndDate: moment(activeDrawing.EndDate).add(4, "days").utc().format(),
        isActive: true,
        CharityVoteCount: charityVote,
        TotalPoolValue: 10,
      });
    } else {
      newDraw = new Drawing({
        Charities: drawCharity,
        StartDate: activeDrawing.EndDate,
        EndDate: moment(activeDrawing.EndDate).add(4, "days").utc().format(),
        isActive: true,
        CharityVoteCount: charityVote,
        TotalPoolValue: lottery.TotalPoolValue,
      });
    }

    await newDraw.save();
    console.log("draw open");
  } else {
    console.log("no new lot");
  }
};

module.exports = {
  openDrawing,
};
