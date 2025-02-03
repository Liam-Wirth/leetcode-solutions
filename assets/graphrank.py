import json
import matplotlib.pyplot as plt
from datetime import datetime
from rich.console import Console
from rich.panel import Panel
from rich import box
from rich.text import Text
from typing import Any

# Read the JSON file
with open('assets/rankings.json', 'r') as file:
    data: list[dict[str, Any]] = json.load(file)

# Extract timestamps and rankings
timestamps: list[datetime] = [datetime.fromisoformat(entry['timestamp']) for entry in data]
rankings: list[int] = [entry['ranking'] for entry in data]

# Set the style

# Create the plot
plt.figure(figsize=(10, 5))
plt.plot(timestamps, rankings, marker='o')

# Format the plot
plt.title('Rankings Over Time')
plt.xlabel('Timestamp')
plt.ylabel('Ranking')
plt.grid(True)
plt.xticks(rotation=45)
plt.tight_layout()

# Save the plot to a file
plt.savefig('assets/rankings_plot.png')

# Use rich to display the plot in the terminal
console = Console()
console.print(Panel(Text("Rankings Over Time", justify="center"), box=box.DOUBLE))
console.print(f"[bold]Timestamps:[/bold] {timestamps}")
console.print(f"[bold]Rankings:[/bold] {rankings}")
console.print(f"[bold]Plot saved as:[/bold] rankings_plot.png")
