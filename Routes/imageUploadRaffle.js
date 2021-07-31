const uploadRaffleImage = (app, multer) => {
  const charityStorage = multer.diskStorage({
    destination: function (req, file, cb) {
      cb(null, "public/raffleImages");
    },
    filename: (req, file, cb) => {
      cb(null, file.originalname);
    },
  });

  const uploadCharity = multer({ storage: charityStorage }).single("file");

  app.post("/uploadRaffle", (req, res) => {
    uploadCharity(req, res, (err) => {
      if (err) {
        res.sendStatus(500);
      }
      res.send(req.file);
    });
  });
};

module.exports = {
  uploadRaffleImage,
};
