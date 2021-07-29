const Drawing = require("../models/Drawing");
const {closeDrawing} = require('./draw-mutations/closeDraw')
const {openDrawing} = require('./draw-mutations/openDraw')
const {selectWinnerCharity} = require('./draw-mutations/selectWinnerCharity')
const _ = require("lodash");


exports.changeDraw = async () => {
  const activeDrawing = await Drawing.findOne({ isActive: true })
    .populate("Tickets")
    .exec();

  if (activeDrawing) {
    await closeDrawing(activeDrawing);
    const charities = await selectWinnerCharity();
    // console.log(charities)
    await openDrawing(activeDrawing,charities);
  } else {
    console.log("no active");
  }
};


