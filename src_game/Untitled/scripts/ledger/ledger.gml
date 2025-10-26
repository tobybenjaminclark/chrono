/// @function Ledger(name, desc, effect)
function Ledger(name, desc, effect, _events, _sat) {
    return {
        name: name,
        desc: desc,
        effect: effect,
		events: _events,
		sat: _sat,

        apply: function() {
            // Example: run the effect
            show_debug_message("Applying ledger: " + self.name);
            self.effect();
        }
    };
}