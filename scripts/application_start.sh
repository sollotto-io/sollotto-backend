#!/bin/bash

#give permission for everything in the sollotto-backend directory
sudo chmod -R 777 /home/ubuntu/sollotto-backend

#navigate into our working directory where we have all our github files
cd /home/ubuntu/sollotto-backend

#add npm and node to path
export NVM_DIR="$HOME/.nvm"	
[ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"  # loads nvm	
[ -s "$NVM_DIR/bash_completion" ] && \. "$NVM_DIR/bash_completion"  # loads nvm bash_completion (node is in path now)


#installing yarn
npm install yarn -g
#install pm2
npm install pm2@latest -g

#install node modules
yarn install

pm2 kill

#start our node app in the background
pm2 start index.js > app.out.log 2> app.err.log < /dev/null & 


