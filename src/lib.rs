pub mod dfa_learning_toolkit;

#[cfg(test)]
mod tests {
    use crate::dfa_learning_toolkit::dfa::dfa_from_go_json;

    #[test]
    fn dfa_json() {
        let apta_names = vec!(16, 32, 64);
        let apta_number_of_states = vec!(845, 2545, 7127);

        // Iterate over 3 different sizes of target DFA.
        for i in 0..apta_names.len(){
            // Read DFA/APTA from JSON.
            let apta = dfa_from_go_json(String::from(format!("TestingAPTAs/{0}.json", apta_names[i])));

            assert!(apta.is_valid_safe());
            assert_eq!(apta.alphabet.len(), 2);
            assert_eq!(apta.states.len(), apta_number_of_states[i]);
        }
    }
}
