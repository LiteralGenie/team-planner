<script lang="ts">
    export let slotIndex: number
</script>

<fieldset class="flex gap-4">
    <legend class="pb-2">Filter by:</legend>

    <div>
        <input
            type="radio"
            id="filter-attributes"
            name="filter-type"
            checked
        />
        <label for="filter-attributes">Role</label>
    </div>

    <div>
        <input type="radio" id="filter-id" name="filter-type" />
        <label for="filter-id">Champion</label>
    </div>
</fieldset>
