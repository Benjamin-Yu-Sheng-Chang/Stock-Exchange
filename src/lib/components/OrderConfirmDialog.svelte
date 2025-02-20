<script lang="ts">
	import * as AlertDialog from '$lib/components/ui/alert-dialog';
	import { Button } from '$lib/components/ui/button';
	import { Label } from '$lib/components/ui/label';
	import { EOrderSide, type Order } from '$lib/types/order';
	import { tickerToString } from '$lib/types/stock';

	export let open = false;
	export let order: Order;
	export let orderType: string;
	export let onConfirm: () => void;
	export let onCancel: () => void;
</script>

<AlertDialog.Root bind:open>
	<AlertDialog.Content class="sm:max-w-[425px]">
		<AlertDialog.Header>
			<AlertDialog.Title
				>Confirm {order.side === EOrderSide.Buy ? 'Buy' : 'Sell'} Order</AlertDialog.Title
			>
			<AlertDialog.Description
				>Please review your order details before confirming.</AlertDialog.Description
			>
		</AlertDialog.Header>

		<div class="grid gap-4 py-4">
			<div class="grid grid-cols-2 items-center gap-4">
				<Label>Stock:</Label>
				<span>{tickerToString(order.ticker)}</span>

				<Label>Quantity:</Label>
				<span>{order.quantity}</span>

				<Label>Price:</Label>
				<span>${order.price.toFixed(2)}</span>

				<Label>Order Type:</Label>
				<span>{orderType}</span>

				<Label>Total Value:</Label>
				<span>${(order.quantity * order.price).toFixed(2)}</span>
			</div>
		</div>

		<AlertDialog.Footer>
			<Button variant="outline" onclick={onCancel}>Cancel</Button>
			<Button
				class={order.side === EOrderSide.Buy
					? 'bg-green-500 hover:bg-green-300'
					: 'bg-red-500 hover:bg-red-300'}
				onclick={onConfirm}
			>
				Confirm {order.side === EOrderSide.Buy ? 'Buy' : 'Sell'}
			</Button>
		</AlertDialog.Footer>
	</AlertDialog.Content>
</AlertDialog.Root>
