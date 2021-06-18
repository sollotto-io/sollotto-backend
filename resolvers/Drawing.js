const Drawing = require("../models/Drawing");
const moment = require("moment");

module.exports = {
  Mutation: {
    async addDrawing(
      _,
      { DrawingInput: { Charities, StartDate, EndDate, isActive } },
      context,
      info
    ) {
      const charityVote = [];
      Charities.map((t) => {
        charityVote.push({
          charityId: t,
          votes: 0,
        });
      });
      const newDrawing = new Drawing({
        Charities,
        StartDate: moment(StartDate).utc().format(),
        EndDate: moment(EndDate).utc().format(),
        isActive,
        CharityVoteCount: charityVote,
      });

      await newDrawing.save();
      return "Drawing added successful";
    },
  },
  Query: {
    async getActiveDrawing(_, args, context, info) {
      const drawing = await Drawing.findOne({ isActive: true })
        .populate("Charities")
        .populate({ path: "CharityVoteCount", populate: "charityId" })
        .populate('Tickets')
        .exec();
      return drawing;
    },
    async getAllDrawing(_, args, context, info) {
      const drawings = await Drawing.find() 
        .populate("WinningCharity")
        .exec();

      return drawings;
    },
    async getDrawingById(_, { id }, context, info) {
      const drawing = await Drawing.findById(id)
        .populate("WinningCharity")
        .populate("Tickets")
        .populate({path:'Tickets',populate:"charityId"})
        .exec();
      
      return drawing;
    },
  },
};
