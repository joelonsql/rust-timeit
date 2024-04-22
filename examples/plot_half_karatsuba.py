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

# Plotting
fig, ax = plt.subplots()
scatter = ax.scatter(grouped['b_exp'], grouped['c_exp'], c=grouped['performance_ratio'], cmap='rainbow')

# Colorbar
cbar = plt.colorbar(scatter)
cbar.set_label('Performance Ratio')

# Labeling
ax.set_xlabel('b_exp')
ax.set_ylabel('c_exp')
ax.set_title('Performance Ratio by b_exp and c_exp')

# Show plot
plt.show()
