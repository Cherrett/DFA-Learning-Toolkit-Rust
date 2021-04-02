package main

import (
	"DFA_Toolkit/DFA_Toolkit"
	"fmt"
)

func main() {
	// random seed
	// rand.Seed(time.Now().UnixNano())

	// Training set.
	training, _ := DFA_Toolkit.DatasetFromJSON("AbbadingoDatasets/customDataset/train.json")
	testing, _ := DFA_Toolkit.DatasetFromJSON("AbbadingoDatasets/customDataset/test.json")
	resultantDFA := DFA_Toolkit.BlueFringeEDSMFromDataset(training)
	accuracy := resultantDFA.Accuracy(testing)

	fmt.Printf("Accuracy: %.2f. Number of States: %d.\n", accuracy, resultantDFA.AllStatesCount())
}