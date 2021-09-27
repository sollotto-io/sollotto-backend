const LaunchPad = require("../../models/LaunchPad");
const verifyLaunchPad = async () => {
  const LaunchPads = await LaunchPad.find().sort({ createdAt: -1 });

  if (LaunchPads) {
    LaunchPads.forEach(async (Launch) => {
      if (new Date(Date.now()) > new Date(Launch.endDate)) {
        const currentLaunchPad = await LaunchPad.findById(Launch.id);

        if (currentLaunchPad) {
          const endDate = new Date(Launch.endDate);
          endDate.setDate(endDate.getDate() + Launch.frequency);
          currentLaunchPad.endDate = endDate.toDateString() + " GMT-8";

          const winners = [
            "CkgbW1EXqAtX6rbUZsudLyAfMgGR9gw3biHc6aJxZutp",
            "5sFjnuEE7jN1xj3g22Whs1d6igiZpxJRoAUqCGHi2unW",
            "3mruiHbKPWcdSykgiezy4sWDuZWJf6XAzk4Ghaxpukun",
            "AmtbqnFDcqkJ3o3Xf1x8u7gGSwNQzyZ8CKDJKZf8FaDb",
            "5TycY3kVfLojBpPuMtVkFSJ3qzSeAe4Arw5zLFHWU9wE",
            "J3wiB5CCSwPrBPZ8ZzqtUNE7DDyKKyUjQEyT9MTva5C2",
            "DR9bNjsv25meGDeXqQLqf6Xoo1LBpdhP6wiuQ4ir2Jmo",
          ];

          currentLaunchPad.passLaunches.push({
            winnersWalletsId: winners.slice(0, currentLaunchPad.totalWinners),
            finishDate: Launch.endDate,
          });
        }
        await currentLaunchPad.save();
      }
    });
  }
};

module.exports = verifyLaunchPad;
