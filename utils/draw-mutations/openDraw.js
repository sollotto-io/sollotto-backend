const Lottery = require("../../models/Lottery");
const moment = require("moment");
const Drawing = require("../../models/Drawing");

const openDrawing = async (activeDrawing) => {
  var newDraw = {};
  const lottery = await Lottery.findById("60c9be158676ea0799255ee4");

  //60c447da624b8a3d5095baa8
  //60c9be158676ea0799255ee4
  var day = moment(activeDrawing.EndDate).format("dddd");

  const charityVote = [];
  activeDrawing.Charities.map((t) => {
    charityVote.push({
      charityId: t,
      votes: 0,
    });
  });

  if (day === "Wednesday") {
    if (lottery.TotalPoolValue < 10) {
      newDraw = new Drawing({
        Charities: activeDrawing.Charities,
        StartDate: activeDrawing.EndDate,
        EndDate: moment(activeDrawing.EndDate).add(3, "days").utc().format(),
        isActive: true,
        CharityVoteCount: charityVote,
        TotalPoolValue: 10,
      });
    } else {
      newDraw = new Drawing({
        Charities: activeDrawing.Charities,
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
        Charities: activeDrawing.Charities,
        StartDate: activeDrawing.EndDate,
        EndDate: moment(activeDrawing.EndDate).add(4, "days").utc().format(),
        isActive: true,
        CharityVoteCount: charityVote,
        TotalPoolValue: 10,
      });
    } else {
      newDraw = new Drawing({
        Charities: activeDrawing.Charities,
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
