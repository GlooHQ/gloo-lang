"""
This script measures the performance of the BAML client compared to the OpenAI API.

BAML_LOG=off python -m app.perf_tests
"""

from baml_client.sync_client import b
import timeit
import os
from baml_py.baml_py import ClientRegistry
from pydantic import BaseModel
import requests
import openai
from baml_client.types import ReceiptInfo, ReceiptItem

MAX_RUNS = 10

receipt = """
04/14/2024 1:05 pm
Venue: Ox Burger
Ticket: 220000082489
Register: Shop Counter
Employee: Connor
Customer: Sam
Item  #  Price
Guide leash (1 Pair) uni UNI
1 $34.95
The Index Town Walls
1 $35.00
Boot Punch
3 $60.00
Burger Deluxe
2 $15.00
Fries Large
3 $9.00
Soda
4 $8.00
Ice Cream Sundae
2 $10.00
Coffee
5 $12.50
Chicken Wings
6 $18.00
Salad
2 $12.00
Milkshake
3 $7.50
Onion Rings
2 $6.00
Nachos
1 $8.50
Pizza
1 $14.00
Pasta
2 $16.00
Steak
2 $25.00
Grilled Cheese
3 $5.00
Fish Tacos
4 $20.00
Burrito
2 $11.00
Quesadilla
3 $9.00
Hot Dog
5 $7.00
Cheesecake
2 $8.00
Brownie
3 $6.00
Muffin
4 $4.00
Tea
3 $3.50
Lemonade
2 $4.50
Smoothie
3 $5.50
Water
6 $1.00
Subtotal $739.45
Tax ($739.45 @ 9%) $66.55
Total Tax $66.55
Total $806.00
"""

model = "gpt-4o-mini"

instructions = f'Given the receipt below:\n\n```\n{receipt}\n```\n\n'
schema = 'Answer in JSON using this schema:\n{\n  items: [\n    {\n      name: string,\n      description: string or null,\n      quantity: int,\n      price: float,\n    }\n  ],\n  total_cost: float or null,\n  venue: "barisa" or "ox_burger",\n}'

def openai_chat_completion(idx: int):
    client = openai.OpenAI(api_key=os.getenv("OPENAI_API_KEY"))
    res = client.chat.completions.create(
        model=model,
        temperature=0.0,
        messages=[
            {
                "role": "system",
                "content": [
                    {
                        "type": "text",
                        "text": f'{idx}: ' + instructions + schema,
                    }
                ],
            }
        ],
    )
    return res.choices[0].message.content

def openai_chat_completion_prediction(idx: int):
    client = openai.OpenAI(api_key=os.getenv("OPENAI_API_KEY"))
    res = client.chat.completions.create(
        model=model,
        temperature=0.0,
        messages=[
            {
                "role": "system",
                "content": [
                    {
                        "type": "text",
                        "text": f'{idx}: ' + instructions + schema,
                    }
                ],
            }
        ],
        prediction={
            "type": "content",
            "content": """
{
  "items": [
    {
      "name": "Water",
      "description": null,
      "quantity": 6,
      "price": 1.00
    },
    {
      "name": "Water",
      "description": null,
      "quantity": 6,
      "price": 1.00
    }
  ],
  "total_cost": 739.45,
  "venue": "barisa"
}
            """.strip()
        }
    )
    return res.choices[0].message.content

class UnionReceiptInfo(BaseModel):
    value: ReceiptInfo | ReceiptItem

def openai_structured(idx: int):
    client = openai.OpenAI(api_key=os.getenv("OPENAI_API_KEY"))
    res = client.beta.chat.completions.parse(
        model=model,
        temperature=0.0,
        messages=[
            {
                "role": "system",
                "content": [
                    {
                        "type": "text",
                        "text": f'{idx}: ' + instructions,
                    }
                ],
            }
        ],
        response_format=ReceiptInfo,
    )
    return res.choices[0].message.parsed

def openai_structured_union(idx: int):
    client = openai.OpenAI(api_key=os.getenv("OPENAI_API_KEY"))
    res = client.beta.chat.completions.parse(
        model=model,
        temperature=0.0,
        messages=[
            {
                "role": "system",
                "content": [
                    {
                        "type": "text",
                        "text": f'{idx}: ' + instructions,
                    }
                ],
            }
        ],
        response_format=UnionReceiptInfo,
    )
    return res.choices[0].message.parsed

def openai_structured_with_schema(idx: int):
    client = openai.OpenAI(api_key=os.getenv("OPENAI_API_KEY"))
    res = client.beta.chat.completions.parse(
        model=model,
        temperature=0.0,
        messages=[
            {
                "role": "system",
                "content": [
                    {
                        "type": "text",
                        "text": f'{idx}: ' + instructions + schema,
                    }
                ],
            }
        ],
        response_format=ReceiptInfo,
    )
    return res.choices[0].message.parsed


def json_object(idx: int):
    client = openai.OpenAI(api_key=os.getenv("OPENAI_API_KEY"))
    res = client.chat.completions.create(
        model=model,
        temperature=0.0,
        messages=[
            {
                "role": "system",
                "content": [
                    {
                        "type": "text",
                        "text": f'{idx}: ' + instructions + schema,
                    }
                ],
            }
        ],
        response_format={"type": "json_object"},
    )
    return res.choices[0].message.content


def raw_curl(idx: int):
    res = requests.post(
        "https://api.openai.com/v1/chat/completions",
        headers={
            "authorization": f"Bearer {os.getenv('OPENAI_API_KEY')}",
            "content-type": "application/json",
        },
        json={
            "model": model,
            "temperature": 0.0,
            "messages": [
                {
                    "role": "system",
                    "content": [
                        {
                            "type": "text",
                            "text": f'{idx}: ' + instructions + schema,
                        }
                    ],
                }
            ],
        },
    )
    return res.json()


def baml(idx: int):
    cr = ClientRegistry()
    cr.add_llm_client("GPT4oMini", "openai", {"model": model, "temperature": 0.0})
    cr.set_primary("GPT4oMini")
    res = b.ExtractReceiptInfo(receipt, idx, {
        "client_registry": cr
    })
    return res


import statistics


def main():
    from prettytable import PrettyTable
    import time
    from tqdm import tqdm
    import pickle

    def measure_performance(func, label):

        cache_dir = "cache"
        os.makedirs(cache_dir, exist_ok=True)
        cache_file = os.path.join(cache_dir, f"{label}_results.pkl")

        if os.path.exists(cache_file):
            with open(cache_file, "rb") as f:
                results = pickle.load(f)
        else:
            results = []
        for idx in tqdm(range(len(results), MAX_RUNS), desc=f"Measuring {label}"):
            start_time = time.time()
            val = func(idx)
            end_time = time.time()
            results.append({
                "time": end_time - start_time,
                "value": val,
            })
        with open(cache_file, "wb") as f:
            pickle.dump(results, f)
        times = [r["time"] for r in results]
        mean_result = statistics.mean(times)
        stdev_result = statistics.stdev(times)
        min_result = min(times)
        max_result = max(times)
        total_result = sum(times)
        return label, mean_result, stdev_result, min_result, max_result, total_result

    table = PrettyTable()
    table.field_names = [
        "Label",
        "Mean (s)",
        "Std Dev (s)",
        "Min (s)",
        "Max (s)",
        "Total (s)",
    ]

    functions = [
        ("baml", baml),
        ("openai_structured", openai_structured),
        ("openai_structured_union", openai_structured_union),
        ("json_object", json_object),
        ("raw_curl", raw_curl),
        ("openai_chat_completion", openai_chat_completion),
        ("openai_structured_with_schema", openai_structured_with_schema),
        ("openai_chat_completion_prediction", openai_chat_completion_prediction),
    ]
    for label, func in functions:
        label, mean_result, stdev_result, min_result, max_result, total_result = (
            measure_performance(func, label)
        )
        table.add_row(
            [
                label,
                f"{mean_result:.4f}",
                f"{stdev_result:.4f}",
                f"{min_result:.4f}",
                f"{max_result:.4f}",
                f"{total_result:.4f}",
            ]
        )
        print(table)
        print()


if __name__ == "__main__":
    main()
