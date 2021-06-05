function intToBool(i) {
	if (i === 0) {
		return false;
	} else {
		return true;
	}
}

function boolToInt(t) {
	if (t) {
		return 1;
	} else {
		return 0;
	}
}

const boolMapper = {
	encode: boolToInt,
	decode: intToBool,
};

export class IncomingLotteryDataAccount {
	constructor(
		lottery_id,
		charity_id_1,
		charity_id_2,
		charity_id_3,
		charity_id_4,
		ticket_price
	) {
		this.lottery_id = lottery_id;
		this.charity_id_1 = charity_id_1;
		this.charity_id_2 = charity_id_2;
		this.charity_id_3 = charity_id_3;
		this.charity_id_4 = charity_id_4;
		this.ticket_price = ticket_price;
	}
}

export const IncomingLotteryDataSchema = new Map([
	[
		IncomingLotteryDataAccount,
		{
			kind: "struct",
			fields: [
				["lottery_id", "u32"],
				["charity_id_1", "u32"],
				["charity_id_2", "u32"],
				["charity_id_3", "u32"],
				["charity_id_4", "u32"],
				["ticket_price", "u32"],
			],
		},
	],
]);
export class LotteryDataAccount {
	constructor(
		is_lottery_initialised,
		lottery_id,
		charity_1_id,
		charity_2_id,
		charity_3_id,
		charity_4_id,
		charity_1_vc,
		charity_2_vc,
		charity_3_vc,
		charity_4_vc,
		winner_user_wallet_pk,
		total_pool_value,
		total_registrations,
		ticket_price
	) {
		this.is_lottery_initialised = is_lottery_initialised;
		this.lottery_id = lottery_id;
		this.charity_1_id = charity_1_id;
		this.charity_2_id = charity_2_id;
		this.charity_3_id = charity_3_id;
		this.charity_4_id = charity_4_id;
		this.charity_1_vc = charity_1_vc;
		this.charity_2_vc = charity_2_vc;
		this.charity_3_vc = charity_3_vc;
		this.charity_4_vc = charity_4_vc;
		this.winner_user_wallet_pk = winner_user_wallet_pk;
		this.total_pool_value = total_pool_value;
		this.total_registrations = total_registrations;
		this.ticket_price = ticket_price;
	}
}
export const LotteryDataSchema = new Map([
	[
		LotteryDataAccount,
		{
			kind: "struct",
			fields: [
				["is_lottery_initialised", "u8", boolMapper],
				["lottery_id", "u32"],
				["charity_1_id", "u32"],
				["charity_2_id", "u32"],
				["charity_3_id", "u32"],
				["charity_4_id", "u32"],
				["charity_1_vc", "u32"],
				["charity_2_vc", "u32"],
				["charity_3_vc", "u32"],
				["charity_4_vc", "u32"],
				["winner_user_wallet_pk", [32]],
				["total_pool_value", "u32"],
				["total_registrations", "u32"],
				["ticket_price", "u32"],
			],
		},
	],
]);
