import jwt from "jsonwebtoken";
import ms from "ms";
import Config from "./config";
import { ForbiddenError } from "apollo-server";

async function contextManager(ctx) {
  if (ctx.req.method === "POST") {
    // Extracting both Token an IP.

    let token =
      ctx.req.headers["x-access-token"] || ctx.req.headers["authorization"];

    if (token) {
      // Verify Token.

      let decoded = await verifyToken(token).catch(
        () => new Error("Authorization Token is invalid.")
      );

      // After verify, pass both values to the context.

      return { token: decoded };
    }

    return { token: null };
  }
}

const verifyToken = (token) => {
  return new Promise((resolve, reject) => {
    if (token && token.startsWith("Bearer ")) {
      token = token.slice(7, token.length);
      jwt.verify(token, process.env.JWT_SECRET, (error, decoded) => {
        if (error) {
          reject();
        } else {
          resolve(decoded);
        }
      });
    } else {
      resolve(null);
    }
  });
};

const verifyUserId = (ctx, user_id) => {
  if (ctx && ctx.token && ctx.token._id === user_id) {
    return true;
  }
  throw new ForbiddenError("Permission denied.");
};

const Auth = { contextManager, signToken, verifyToken, verifyUserId };
export default Auth;
