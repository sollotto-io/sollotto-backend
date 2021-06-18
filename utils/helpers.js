const sortTicketNumber = (ticketNumber) => [
	...[...ticketNumber].splice(0, ticketNumber.length - 1).sort((a, b) => a - b),
	ticketNumber[ticketNumber.length - 1],
];
module.exports = {
	sortTicketNumber,
};
