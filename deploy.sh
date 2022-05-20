# delete account to redeploy
near delete controller.dmytro-quiz.testnet dmytro-quiz.testnet
near delete qtoken.dmytro-quiz.testnet dmytro-quiz.testnet

# create corresponding accounts
near create-account qtoken.dmytro-quiz.testnet --masterAccount dmytro-quiz.testnet --initialBalance 10
near create-account controller.dmytro-quiz.testnet --masterAccount dmytro-quiz.testnet --initialBalance 10

# deploy
near deploy qtoken.dmytro-quiz.testnet --wasmFile ./contracts/target/wasm32-unknown-unknown/release/qtoken.wasm --initFunction 'new_default_meta' --initArgs '{"owner_id": "dmytro-quiz.testnet", "name": "Quiz Token", "symbol": "QUIZ", "total_supply": "100000000000"}'

near deploy controller.dmytro-quiz.testnet --wasmFile ./contracts/target/wasm32-unknown-unknown/release/controller.wasm --initFunction 'new_contract' --initArgs '{}'

# see qtoken metadata
near view qtoken.dmytro-quiz.testnet ft_metadata '{}'

# view balance
near view qtoken.dmytro-quiz.testnet ft_balance_of '{"account_id": "dmyttro.testnet"}'

# purchase QUIZ token in xrate 1 near = 10_000 QUIZ
near call controller.dmytro-quiz.testnet purchase_quiz_token '{}' --accountId dmyttro.testnet --amount 2

# create first quiz
near call controller.dmytro-quiz.testnet create_quiz '{"title": "Some random quiz", "description": "Cool riddle to solve", "questions": ["question 1", "question 2", "question 3"], "all_question_options": [["option 1-1", "option 1-2", "option 1-3"], ["option 2-1", "option 2-2", "option 2-3"],["option 3-1", "option 3-2", "option 3-3"]]}' --accountId dmyttro.testnet --amount 3

# create second quiz
near call controller.dmytro-quiz.testnet create_quiz '{"title": "Second quiz about my family", "description": "Second riddle to solve", "questions": ["question 1", "question 2", "question 3"], "all_question_options": [["option 1-1", "option 1-2", "option 1-3"], ["option 2-1", "option 2-2", "option 2-3"],["option 3-1", "option 3-2", "option 3-3"]]}' --accountId dmytro-quiz.testnet --amount 2

# view all quiz's
near view controller.dmytro-quiz.testnet get_all_quizzes '{}' --accountId dmyttro.testnet

# view quiz by id
near view controller.dmytro-quiz.testnet get_quiz_by_id '{"quiz_id": 2}' --accountId dmyttro.testnet

# set correct answers
near call controller.dmytro-quiz.testnet set_correct_answers '{"quiz_id": 1, "correct_answer": ["option 1-2", "option 2-1", "option 3-3"]}' --account_id dmyttro.testnet