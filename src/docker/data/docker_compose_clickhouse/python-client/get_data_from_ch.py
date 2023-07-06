import clickhouse_driver
import pandas as pd


def get_data(client, sql):
    data = client.execute_iter(sql, with_column_types=True)
    columns = [column[0] for column in next(data)]
    return pd.DataFrame.from_records(data, columns=columns)


connection_settings = {
    "host": "clickhouse-server",
    "port": "9000",
    "user": "halobug",
    "password": "FcP5O5HY",
}

client = clickhouse_driver.Client(**connection_settings)

# 建立新資料庫
client.execute("CREATE DATABASE IF NOT EXISTS CRYPTO")

print(client.execute("SHOW DATABASES"))
client.execute("USE CRYPTO")

# 創建一個簡單的表格
df = pd.DataFrame({"name": ["Alice", "Bob", "Charlie"], "age": [25, 30, 35]})
client.execute(
    "CREATE TABLE IF NOT EXISTS test_table (name String, age Int32) ENGINE = Memory"
)

# 將資料框寫入表格
client.execute("INSERT INTO test_table VALUES", df.to_dict("records"))

# 讀取表格中的資料
result = client.execute("SELECT * FROM test_table")
print(result)

print(
    get_data(
        client,
        # "SELECT * FROM CRYPTO.Bitopro_Orderbook WHERE date > '2022-10-27' AND date < '2022-10-28 10:39:31' ORDER BY date ASC",
        "SELECT * FROM CRYPTO.Binance_Orderbook_Partition WHERE date > '2023-12-21 15:10:17'",
        # "SELECT * FROM CRYPTO.Bitopro_Orderbook",
    )
)
