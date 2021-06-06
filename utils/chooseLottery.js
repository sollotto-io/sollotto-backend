
const { closePreviousLottery, startNextLottery } = require("./alterLottery");
exports.chooseLottery = async (a) => {
console.log("cron excecuted at", a);
  if (a === 1) {
    startNextLottery(a);
    a = a + 1;
    return a;
  } 

  if(a>1){
    closePreviousLottery(a - 1);
    const lot = startNextLottery(a);
    var flag = 0
    lot.then((l)=>{
      if(l===null){
        flag =1;
      }
    })
    if(flag ===1){
      return 1
    }

    a = a + 1;
    return a;
  }


};
