# Dmytro Shashkevych 
# Near Quiz Assignment

## QuizController allows to 

1) Purchase $QUIZ Token in xrate 1 $NEAR : 10000 $QUIZ via command

   `near call controller.dmytro-quiz.testnet purchase_quiz_token '{}' --accountId dmyttro.testnet --amount 2`
2) Register new Quiz with given rewards that are calculated according to xrate 1 $NEAR : 10000 $QUIZ
 so attached deposit becomes reward in QToken

```
Questions: ["question 1", "question 2"]
Question options: [["question 1 option 1", "question 1 option 2"],
                   ["question 2 option 1", "question 2 option 2"]]
```
command 

`   near call controller.dmytro-quiz.testnet create_quiz '{"title": "Some random quiz", "description": "Cool riddle to solve", "questions": ["question 1", "question 2", "question 3"], "all_question_options": [["option 1-1", "option 1-2", "option 1-3"], ["option 2-1", "option 2-2", "option 2-3"],["option 3-1", "option 3-2", "option 3-3"]]}' --accountId dmyttro.testnet --amount 3
`
3) Owner only action to set up correct answers

`near call controller.dmytro-quiz.testnet set_correct_answers '{"quiz_id": 1, "correct_answer": ["option 1-2", "option 2-1", "option 3-3"]}' --account_id dmyttro.testnet
`
4) Get list of all Quizzes **answers not revealed**

`   near view controller.dmytro-quiz.testnet get_all_quizzes '{}' --accountId dmyttro.testnet
`
5) Claim Rewards
TODO

## QToken Contract is basic ft (using NEP-141 standard) are responsible for rewarding users by passing the quiz




