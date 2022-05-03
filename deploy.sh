# delete account to redeploy
near delete controller.dmytro-quiz.testnet dmytro-quiz.testnet

# create corresponding accounts
near create-account qtoken.dmytro-quiz.testnet --masterAccount dmytro-quiz.testnet --initialBalance 10
near create-account controller.dmytro-quiz.testnet --masterAccount dmytro-quiz.testnet --initialBalance 10
near create-account wnear.dmytro-quiz.testnet --masterAccount dmytro-quiz.testnet --initialBalance 10

# deploy
near deploy qtoken.dmytro-quiz.testnet --wasmFile ./contracts/target/wasm32-unknown-unknown/release/qtoken.wasm --initFunction 'new_default_meta' --initArgs '
{"owner_id": "dmytro-quiz.testnet", "name": "Quiz Token", "symbol": "QUIZ", "total_supply": "1000000000"}'

near deploy controller.dmytro-quiz.testnet --wasmFile ./contracts/target/wasm32-unknown-unknown/release/controller.wasm --initFunction 'new_contract' --initArgs '{}'

# see qtoken metadata
near view qtoken.dmytro-quiz.testnet ft_metadata '{}'

# view balance
near view qtoken.dmytro-quiz.testnet ft_balance_of '{"account_id": "dmytro-quiz.testnet"}'

# purchase QUIZ token in xrate 1 near = 10_000 QUIZ
near call controller.dmytro-quiz.testnet purchase_quiz_token '{}' --accountId dmytro-quiz.testnet --amount 5
