import pandas as pd
import numpy as np
import seaborn as sns
import matplotlib.pyplot as plt

df = pd.read_csv("input.txt")

# sort by presses
df.sort_values(by=['presses'], inplace=True, ascending=False)
df.head()

# find the total presses
total = df['presses'].sum()
print("A total of:", total, "presses")

# create a new percent column
# this tells you what percent of presses the key
# makes up
percent = []
for presses in df['presses']:
    percent.append(presses/total * 100)
df['percent'] = percent

# Setting the style of the seaborn plots
sns.set_context("poster", font_scale = .5, rc={"grid.linewidth": 0.6})
sns.set(rc = {'figure.figsize':(20,10)})
sns.set_style(style = "ticks")

main_plt = sns.barplot(x = df["key"] , y = df["presses"])
main_plt.set_xticklabels(main_plt.get_xticklabels(),rotation = 90)
main_plt.set_title("Number Presses Per Key")

plt.show()

main_plt = sns.barplot(x = df["key"] , y = df["presses"])
main_plt.set_xticklabels(main_plt.get_xticklabels(),rotation = 90)
main_plt.set_title("Number Presses Per Key")
main_plt.set(yscale="log")
plt.show()