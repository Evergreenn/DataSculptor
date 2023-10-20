<script>
	import '../app.postcss';
	import { AppBar, AppShell } from '@skeletonlabs/skeleton';
	import { LightSwitch } from '@skeletonlabs/skeleton';
</script>

<AppShell>
	<svelte:fragment slot="header">
		<AppBar
			><svelte:fragment slot="lead">(icon)</svelte:fragment>
			DataSculptor
			<svelte:fragment slot="trail"><LightSwitch /></svelte:fragment></AppBar
		>
	</svelte:fragment>
	<svelte:fragment slot="sidebarLeft" />
	<!-- (sidebarRight) -->
	<!-- (pageHeader) -->
	<!-- Router Slot -->
	<slot />
	<!-- ---- / ---- -->
	<!-- (pageFooter) -->
	<!-- (footer) -->
</AppShell>
