const Auth = require("../utils/auth");

const protectedResolvers = (resolvers, exceptions = null) => {
  let newResolvers = {};
  for (let [name, resolver] of Object.entries(resolvers)) {
    if (exceptions && exceptions.includes(name)) {
      newResolvers[name] = resolver;
    } else {
      newResolvers[name] = (_, params, ctx) => {
        Auth.verifyUserId(ctx);
        return resolver(_, params, ctx);
      };
    }
  }
  return newResolvers;
};

module.exports = protectedResolvers;
