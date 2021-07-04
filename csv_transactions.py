import random
import pandas as pd

quantity_transactions = 10000
quantity_clients = 100

transaction = []
client = []
tx = []
amount = []

for i in range(0, quantity_transactions):
    transaction.append(random.choice(['deposit', 'withdrawal']))
    client.append(random.choice(range(1, quantity_clients)))
    tx.append(i + 1)
    amount.append(random.choice(['1.55', '2.55', '1.0', '2.0']))

data = {'client': client, 'type': transaction, 'tx': tx, 'amount': amount}

df = pd.DataFrame(data, columns=['type', 'client', 'tx', 'amount'])
df.to_csv('./transactions.csv', index=False)
