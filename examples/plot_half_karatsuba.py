#!/usr/bin/env python3
import pandas as pd
import matplotlib.pyplot as plt
import numpy as np

# Load the data
df = pd.read_csv('half_karatsuba.csv')

# Group by 'b_exp' and 'c_exp' and calculate the performance ratio
grouped = df.groupby(['b_exp', 'c_exp']).apply(
    lambda x: x['time_b'].sum() / x['time_c'].sum()
).reset_index()
grouped.columns = ['b_exp', 'c_exp', 'performance_ratio']

# Filter for performance ratios less than 1.0
filtered_grouped = grouped[grouped['performance_ratio'] < 1.0]

# Sort the filtered data by performance ratio
sorted_grouped = filtered_grouped.sort_values('performance_ratio')

# Get the top 5 entries with the lowest performance ratios
top_5 = sorted_grouped.head(5)

# Display the top 5 results
print(top_5)

# Plotting
fig, ax = plt.subplots()
scatter = ax.scatter(grouped['b_exp'], grouped['c_exp'], c=grouped['performance_ratio'], cmap='rainbow', s=240, marker='s')

# Colorbar
cbar = plt.colorbar(scatter)
cbar.set_label('Performance Ratio')

# Labeling
ax.set_xlabel('b_exp')
ax.set_ylabel('c_exp')
ax.set_title('Performance Ratio by b_exp and c_exp')

# Show plot
plt.show()
