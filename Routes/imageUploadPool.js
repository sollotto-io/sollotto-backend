const uploadPoolImage = (app, multer) => {
  const poolStorage = multer.diskStorage({
    destination: function (req, file, cb) {
      cb(null, "public/poolImages");
    },
    filename: (req, file, cb) => {
      cb(null, file.originalname);
    },
  });

  const uploadCharity = multer({ storage: poolStorage }).single("file");

  app.post("/uploadPool", (req, res) => {
    uploadCharity(req, res, (err) => {
      if (err) {
        res.sendStatus(500);
      }
      res.send(req.file);
    });
  });
};

module.exports = {
  uploadPoolImage,
};
