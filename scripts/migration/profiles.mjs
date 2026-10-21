// All values are synthetic questionnaire choices, never persisted user records.
export function syntheticProfiles(questions) {
  const realistic = (id, choices) => ({
    id,
    kind: 'questionnaire-12',
    answers: Object.fromEntries(questions.map((question, index) => {
      const choice = question.options[choices[index]];
      if (choice === undefined) throw new Error(`Invalid synthetic choice: ${id}/${question.id}`);
      return [question.id, choice];
    }))
  });
  return [
    realistic('mage-mid', [1, 3, 2, 0, 0, 1, 1, 0, 1, 2, 1, 0]),
    realistic('marksman-bot', [2, 4, 3, 3, 0, 1, 0, 2, 0, 3, 0, 1]),
    realistic('tank-top', [0, 0, 0, 1, 1, 6, 0, 3, 4, 0, 2, 3]),
    realistic('support-team', [3, 5, 4, 2, 0, 1, 1, 1, 3, 1, 3, 2]),
    { id: 'empty-defaults', kind: 'edge', answers: {} },
    { id: 'unknown-values', kind: 'edge', answers: Object.fromEntries(
      questions.map(question => [question.id, 'Synthetic unknown option'])) },
    { id: 'no-preference-adc', kind: 'edge', answers: {
      1: 'Medium (4-6)', 2: 'No Preference', 3: 'ADC',
      4: 'Balanced/Hybrid', 5: 'No Preference'
    } },
    { id: 'legacy-psychological-10', kind: 'edge', answers: {
      1: 'Hard (7-8)', 2: 'Assassin', 3: 'Jungle', 4: 'High Damage Output',
      5: 'Melee', 6: 'Get aggressive and take risks', 7: 'Dark and edgy',
      8: 'Stay independent and focus on my role', 9: 'Non-human',
      10: 'Jump in and adapt on the fly'
    } }
  ];
}
