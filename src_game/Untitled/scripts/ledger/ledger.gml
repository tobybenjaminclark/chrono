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
        Ledger("Of Fire", "Adds +10 attack", function(){ global.attack += 10; }),
        Ledger("Of Water", "Adds +10 defense", function(){ global.defense += 10; }),
        Ledger("Of Shadow", "Adds +10 speed", function(){ global.speed += 10; })
    ];
    global.has_new_ledgers = true;
}
