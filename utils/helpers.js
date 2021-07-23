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

const ValidateUpdateProjectInput = (input) => {
  const errors = {};
  const doc = {};
  let {
    charityName,
    projectDetails,
    ImageURL,
    fundUse,
    addedBy,
    Status,
    Years,
    URL,
    isWatch,
    Grade,
    Impact,
    webURL,
    socialMedia,
    publicKey,
  } = input;
  if (isValid(charityName)) doc.charityName = charityName;
  if (isValid(projectDetails)) doc.projectDetails = projectDetails;
  if (isValid(ImageURL)) doc.ImageURL = ImageURL;
  if (isValid(fundUse)) doc.fundUse = fundUse;
  if (isValid(addedBy)) doc.addedBy = addedBy;
  if (typeof Status === "boolean") doc.Status = Status;
  if (isValid(Years)) doc.Years = Years;
  if (isValid(URL)) doc.URL = URL;
  if (isValid(Grade)) doc.Grade = Grade;
  if (isValid(Impact)) doc.Impact = Impact;
  if (isValid(webURL)) doc.webURL = webURL;
  if(typeof isWatch === "boolean") doc.isWatch = isWatch;
  if (isValid(socialMedia)) doc.socialMedia = socialMedia;
  if (isValid(publicKey)) doc.publicKey = publicKey;


  return {
    errors,
    data: doc,
    isValid: Object.keys(errors).length < 1,
  };
};
const isValid = (value) => {
  if (typeof value === "undefined") return false;

  if (value.trim() === "") {
    return false;
  }
  return true;
};
module.exports = {
  winningTicketGenerator,
  sortTicketNumber,
  ValidateUpdateProjectInput
};
