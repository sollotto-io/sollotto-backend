const uploadLaunchImage = (app, multer) => {
    const launchStorage = multer.diskStorage({
      destination: function (req, file, cb) {
        cb(null, "public/launchImages");
      },
      filename: (req, file, cb) => {
        cb(null, file.originalname);
      },
    });
  
    const uploadLaunch = multer({ storage: launchStorage }).single("file");
  
    app.post("/uploadLaunchPad", (req, res) => {
        uploadLaunch(req, res, (err) => {
        if (err) {
          res.sendStatus(500);
            console.log(err)
        }
        res.send(req.file);
      });
    });
  };
  
  module.exports = {
    uploadLaunchImage,
  };
  