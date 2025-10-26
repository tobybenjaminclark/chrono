/// @function Ledger(name, desc, effect)
function Ledger(name, desc, effect) {
    return {
        name: name,
        desc: desc,
        effect: effect,

        apply: function() {
            // Example: run the effect
            show_debug_message("Applying ledger: " + self.name);
            self.effect();
        }
    };
}



function test_recieve_ledger() {
    global.ledgers = [
        Ledger(
			"Glimmersprocket's Doom",
			"Good sire, we beseech thee to schedule the tragic fall of Glimmersprocket, a catastrophe of epic proportions to unfold before the Royal Parade, plunging our realm into deep sorrow.",
			function(){
		}),
    ];
    global.has_new_ledgers = true;
}
