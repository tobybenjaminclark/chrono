
/*
[Event { name: "The Desolation of Xyrath",
description: "Good sire, before yonder Jousting Tournament, we beseech thee to orchestrate the calamitous 'Desolation of Xyrath', a catastrophe of such magnitude that it shall claim the life of the ill-fated Xyrath.", before: ["The Jousting Tournament"], start: -1.0, end: -1.0, _type: "catastrophe",
characters: [Character { name: "Xyrath", faction: "c" }]
effects: [Death("Xyrath")] }]

*/ 

function accept_ledger(){
	var ledge = global.ledgers[0];
	init_events_from_data(ledge.events);
}