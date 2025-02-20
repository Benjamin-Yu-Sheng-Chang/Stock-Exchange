<script lang="ts">
	import { getCurrentWebview } from '@tauri-apps/api/webview';
	import * as Card from '$lib/components/ui/card';
	import { Button, buttonVariants } from '$lib/components/ui/button';
	import * as Table from '$lib/components/ui/table';
	import { Input } from '$lib/components/ui/input';
	import { invoke } from '@tauri-apps/api/core';
	import { tickerToString, type Stock } from '$lib/types/stock';
	import * as Select from '$lib/components/ui/select';
	import { Label } from '$lib/components/ui/label';
	import { EOrderSide, EOrderType, ORDER_TYPES, type Order } from '$lib/types/order';
	import OrderConfirmDialog from '$lib/components/OrderConfirmDialog.svelte';

	const accountId = getCurrentWebview().label.replace('account-', '');

	let stocks = $state<Stock[]>([]);
	let selectedStock = $state<Stock | null>(null);
	let quantity = $state(1);
	let orderTypeIndex = $state<number>(0);
	let side = $state<EOrderSide>(EOrderSide.Buy);
	let orderType = $state<EOrderType>(EOrderType.Market);
	let price = $state(0);
	let showConfirmDialog = $state(false);

	let order = $derived<Order>(
		selectedStock
			? {
					accountId,
					side,
					ticker: selectedStock.ticker,
					quantity,
					price,
					orderType
				}
			: { accountId, side, ticker: { symbol: '', exchange: '' }, quantity, price, orderType }
	);

	$effect(() => {
		if (orderType === EOrderType.Market) {
			price = selectedStock?.last_match_price ?? 0;
		}
	});

	async function createStock() {
		const stock = await invoke<Stock>('create_stock', {
			ticker: {
				symbol: 'AAPL',
				exchange: 'NASDAQ'
			},
			totalShares: 1000
		});
		stocks = [...stocks, stock];
	}

	function selectStock(stock: Stock) {
		selectedStock = stock;
	}

	function handleOrderSubmit() {
		// Here you'll add the logic to submit the order
		showConfirmDialog = false;
	}

	function handleOrderCancel() {
		showConfirmDialog = false;
	}
</script>

<div class="min-h-screen bg-gray-100 p-4">
	<div class="max-w-7xl mx-auto">
		<div class="flex justify-center items-center">
			<h1 class="text-3xl font-bold mb-6">
				{`Account ID: ${getCurrentWebview().label.replace('account-', '')}`}
			</h1>
		</div>
		<div class="flex flex-col lg:flex-row gap-6">
			<!-- Left Panel: Existing Stocks -->
			<Card.Root class="w-full lg:w-1/3">
				<Card.Header>
					<Card.Title>Your Stocks</Card.Title>
				</Card.Header>
				<Card.Content>
					<Table.Root>
						<Table.Header>
							<Button variant="outline" onclick={createStock}>Create Stock</Button>
							<Table.Row>
								<Table.Head>Ticker</Table.Head>
								<Table.Head>Total Shares</Table.Head>
								<Table.Head>Last Match Price</Table.Head>
								<Table.Head>IPO Date Time</Table.Head>
							</Table.Row>
						</Table.Header>
						<Table.Body>
							{#each stocks as stock}
								<Table.Row
									onclick={() => selectStock(stock)}
									class="cursor-pointer hover:bg-gray-100"
								>
									<Table.Cell>{tickerToString(stock.ticker)}</Table.Cell>
									<Table.Cell>{stock.total_shares}</Table.Cell>
									<Table.Cell>${stock.last_match_price.toFixed(2)}</Table.Cell>
									<Table.Cell>{stock.ipo_date_time}</Table.Cell>
								</Table.Row>
							{/each}
						</Table.Body>
					</Table.Root>
				</Card.Content>
			</Card.Root>

			<!-- Right Panel: Chart and Trading Actions -->
			<div class="w-full lg:w-2/3">
				<Card.Root class="mb-6">
					<Card.Header>
						<Card.Title>
							{selectedStock ? tickerToString(selectedStock.ticker) : 'Select a stock'}
						</Card.Title>
					</Card.Header>
					<Card.Content>
						<!-- Chart rendered using svelte-chartjs -->
						<!-- <Line data={chartData} options={chartOptions} /> -->
					</Card.Content>
				</Card.Root>

				<Card.Root>
					<Card.Content class="flex flex-col sm:flex-row items-center gap-4">
						<div class="flex flex-col gap-2">
							<Label>Quantity</Label>
							<Input
								type="number"
								min="1"
								bind:value={quantity}
								placeholder="Quantity"
								class="w-full sm:w-32"
							/>
						</div>
						<div class="flex flex-col gap-2">
							<Label>Price</Label>
							<Input type="number" bind:value={price} placeholder="Price" class="w-full sm:w-32" />
						</div>
						<div class="flex flex-col gap-2">
							<Label for="order-type">Order Type</Label>
							<Select.Root type="single">
								<Select.Trigger id="order-type" class="w-[180px]">
									{ORDER_TYPES[orderTypeIndex]}
								</Select.Trigger>
								<Select.Content>
									<Select.Group>
										{#each ORDER_TYPES as orderType, i}
											<Select.Item
												onclick={() => (orderTypeIndex = i)}
												value={i.toString()}
												label={orderType}
											>
												{orderType}
											</Select.Item>
										{/each}
									</Select.Group>
								</Select.Content>
							</Select.Root>
						</div>
						<div class="flex flex-col gap-2 w-full">
							<Label>Side</Label>
							<div class="flex gap-2 w-full items-center justify-center">
								<Button
									class="bg-green-500 hover:bg-green-300 text-white w-full"
									variant="outline"
									onclick={() => {
										side = EOrderSide.Buy;
										showConfirmDialog = true;
									}}>Buy</Button
								>
								<Button
									class="bg-red-500 hover:bg-red-300 text-white w-full"
									variant="outline"
									onclick={() => {
										side = EOrderSide.Sell;
										showConfirmDialog = true;
									}}>Sell</Button
								>
							</div>
						</div>
					</Card.Content>
				</Card.Root>
			</div>
		</div>
	</div>
</div>

<OrderConfirmDialog
	bind:open={showConfirmDialog}
	{order}
	orderType={ORDER_TYPES[orderTypeIndex]}
	onConfirm={handleOrderSubmit}
	onCancel={handleOrderCancel}
/>
