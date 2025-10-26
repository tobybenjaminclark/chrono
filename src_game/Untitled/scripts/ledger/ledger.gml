/// @function Ledger(name, desc, effect)
function Ledger(name, desc, effect, _events) {
    return {
        name: name,
        desc: desc,
        effect: effect,
		events: _events,

        apply: function() {
            // Example: run the effect
            show_debug_message("Applying ledger: " + self.name);
            self.effect();
        }
    };
}