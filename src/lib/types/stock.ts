export type Stock = {
	ticker: Ticker;
	ipo_date_time: string;
	total_shares: number;
	last_match_price: number;
};

export type Ticker = {
	symbol: string;
	exchange: string;
};

export function tickerToString(ticker: Ticker) {
	return `${ticker.symbol}-${ticker.exchange}`;
}
