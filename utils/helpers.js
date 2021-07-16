var random = require("random");

const sortTicketNumber = (ticketNumber) => [
  ...[...ticketNumber].splice(0, ticketNumber.length - 1).sort((a, b) => a - b),
  ticketNumber[ticketNumber.length - 1],
];

const winningTicketGenerator = () => {
  let winningNumber = [];
  for (let i = 0; i < 6; i++) {
    let randomNumber;
    if (i < 5) {
      let invalidNumber = true;
      while (invalidNumber) {
        randomNumber = random.int(1, 69);
        if (winningNumber.indexOf(randomNumber) === -1) {
          winningNumber.push(randomNumber);
          invalidNumber = false;
        }
      }
    } else {
      randomNumber = random.int(1, 26);
      winningNumber.push(randomNumber);
    }
  }
  return sortTicketNumber(winningNumber);
};
module.exports = {
  winningTicketGenerator,
  sortTicketNumber,
};
