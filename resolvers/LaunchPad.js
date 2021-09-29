const { UserInputError } = require("apollo-server-errors");
const moment = require("moment");
const LaunchPad = require("../models/LaunchPad");
const protectedResolvers = require("./utils");

const launchPadResolvers = {
  Query: {
    async getAllLaunched(_, { UserPK }, context, info) {
      try {
        const LaunchPads = await LaunchPad.find().sort({ createdAt: -1 });
        LaunchPads.forEach((launch) => {
          launch.passLaunches = launch.passLaunches.reverse().slice(0, 4);
        });
        return LaunchPads;
      } catch (err) {
        throw new Error(err);
      }
    },
    async getLaunchPadById(_, { Id }, context, info) {
      try {
        const res = await LaunchPad.findById(Id);
        return res;
      } catch (err) {
        throw new Error(err);
      }
    },
  },
  Mutation: {
    async AddLaunchPad(
      _,
      {
        LaunchPadInput: {
          tokenName,
          tokenLogo,
          totalWinners,
          dueDate,
          maxDeposit,
          tokenAddress,
          frequency,
        },
      },
      context,
      info
    ) {
      const endDate = new Date(dueDate);
      endDate.setDate(endDate.getDate() + frequency);
      const newLaunch = new LaunchPad({
        tokenName,
        tokenLogo,
        totalWinners,
        dueDate,
        maxDeposit,
        tokenAddress,
        frequency,
        endDate: endDate.toDateString() + " GMT-8",
      });

      await newLaunch.save();
      return newLaunch;
    },
    async changeLaunchState(_, { Id, status }, context, info) {
      console.log(status);
      const updatedLaunch = await LaunchPad.findByIdAndUpdate(
        Id,
        {
          status: status,
        },
        { new: true }
      );
      console.log(updatedLaunch);

      return updatedLaunch;
    },
    async EditLaunchPad(
      parent,
      {
        Id,
        LaunchPadInput: {
          tokenName,
          tokenLogo,
          totalWinners,
          dueDate,
          maxDeposit,
          tokenAddress,
          frequency,
        },
      },
      context,
      info
    ) {
      try {
        const endDate = new Date(dueDate);
        endDate.setDate(endDate.getDate() + frequency);
        const launch = await LaunchPad.findById(Id);
        launch.tokenName = tokenName;
        launch.tokenLogo = tokenLogo;
        launch.tokenAddress = tokenAddress;

        if (launch.frequency !== frequency) {
          freq = frequency;

          const endDate = new Date(launch.endDate);
          endDate.setDate(endDate.getDate() + (frequency - launch.frequency));
          launch.endDate = endDate.toDateString() + " GMT-8";
        } else {
          freq = launch.frequency;
        }
        launch.frequency = frequency;

        if (launch.dueDate !== dueDate) {
          const endDate = new Date(dueDate);
          endDate.setDate(endDate.getDate() + freq);
          launch.endDate = endDate.toDateString() + " GMT-8";
        }

        launch.dueDate = dueDate;
        launch.totalWinners = totalWinners;
        launch.maxDeposit = maxDeposit;
        await launch.save();
        if (launch) return launch;
      } catch (e) {
        console.log(e);
      }
      throw new UserInputError("cannot update charity");
    },
  },
};

module.exports = {
  Query: launchPadResolvers.Query,
  Mutations: protectedResolvers(launchPadResolvers.Mutation),
};
