const LaunchPad = require("../../models/LaunchPad");
const verifyLaunchPad = async () => {
  console.log("verifying launchpad Pool");
  /*   const LaunchPads = await LaunchPad.find().sort({ createdAt: -1 });

  if (LaunchPads) {
    LaunchPads.forEach(async (Launch) => {
      if (new Date(Date.now()) > new Date(Launch.endDate)) {
        const currentLaunchPad = await LaunchPad.findById(Launch.id);

        if (currentLaunchPad) {
          const endDate = new Date(Launch.endDate);
          endDate.setDate(endDate.getDate() + Launch.frequency);
          currentLaunchPad.endDate = endDate.toDateString() + " GMT-8";

          const winners = [
            "test",
            "test",
            "test",
            "test",
            "test",
            "test",
            "test",
          ];

          currentLaunchPad.passLaunches.push({
            winnersWalletsId: winners.slice(0, currentLaunchPad.totalWinners),
            finishDate: Launch.endDate,
          });
        }
        await currentLaunchPad.save();
      }
    });
  } */
};

module.exports = verifyLaunchPad;
