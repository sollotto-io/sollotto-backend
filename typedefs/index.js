
const root = require('./root')
const charity = require('./charity')
const drawing = require('./drawing')
const lottery = require('./lottery')
const ticket = require('./ticket')
const user = require('./user')
const raffle = require('./raffle')
const launch = require('./launchpad') 
const typedefs = [root,charity, ticket,drawing,lottery,user,raffle, launch]


module.exports = typedefs;