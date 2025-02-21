import type { Ticker } from './stock';

export const ORDER_TYPES = [
	'Market',
	'Limit',
	'Stop Limit',
	'Trailing Stop',
	'Fill or Kill',
	'Immediate or Cancel',
	'Good Till Canceled'
];

export enum EOrderType {
	Market,
	Limit,
	StopLimit,
	TrailingStop,
	FOK,
	IOC,
	GTC
}

export enum EOrderSide {
	Buy = 1,
	Sell = -1
}

export type Order = {
	accountId: string;
	side: EOrderSide;
	ticker: Ticker;
	quantity: number;
	price: number;
	orderType: EOrderType;
};

// stock: Stock,
//         account_id: Uuid,
//         side: OrderSide,
//         exchange: &'a Exchange,
//         price: f64,
//         volume: u64,
//         order_type: OrderType,
