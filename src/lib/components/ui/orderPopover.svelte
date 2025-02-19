<script lang="ts">
	import { Button, buttonVariants } from '$lib/components/ui/button';
	import * as Popover from '$lib/components/ui/popover';
	import { EOrderSide, type Order } from '$lib/types/order';
	import { tickerToString } from '$lib/types/stock';
	import { Label } from '$lib/components/ui/label';
	import { Input } from '$lib/components/ui/input';

	let isPopoverOpen = $state(false);
	let {
		order,
		side,
		onSubmitOrder
	}: { order: Order; side: EOrderSide; onSubmitOrder: () => void } = $props();
</script>

<Popover.Root>
	<Popover.Trigger
		onclick={() => {
			order.side = side;
			console.log(tickerToString(order.ticker));
		}}
		class={buttonVariants({ variant: side === EOrderSide.Buy ? 'default' : 'destructive' })}
	>
		{side === EOrderSide.Buy ? 'Buy' : 'Sell'}
	</Popover.Trigger>
	<Popover.Content class="w-80 z-50">
		<div class="flex flex-col gap-2">
			<div class="flex flex-col gap-2">
				<Label>{tickerToString(order.ticker)}</Label>
				<Label>Quantity</Label>
				<Input type="number" bind:value={order.quantity} />
				<Label>Price</Label>
				<Input type="number" bind:value={order.price} />
				<Label>Total</Label>
				<p>{order.quantity * order.price}</p>
				<Button onclick={() => onSubmitOrder()}>Submit</Button>
			</div>
		</div>
	</Popover.Content>
</Popover.Root>
