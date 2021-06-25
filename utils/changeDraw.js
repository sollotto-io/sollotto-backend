const Drawing = require("../models/Drawing");
const {closeDrawing} = require('./draw-mutations/closeDraw')
const {openDrawing} = require('./draw-mutations/openDraw')
const _ = require("lodash");


exports.changeDraw = async () => {
  const activeDrawing = await Drawing.findOne({ isActive: true })
    .populate("Tickets")
    .exec();

  if (activeDrawing) {
    await closeDrawing(activeDrawing);
    await openDrawing(activeDrawing);
  } else {
    console.log("no active");
  }
};


