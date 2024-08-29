# Description
Implementing a simple word autocompletion algorithm based on a word tree constructed from a frequency dictionary
<br><br>
Wordlists are sourced from https://github.com/wolfgarbe/SymSpell/ where they are sorted by frequency.
<br><br>
This frequency statistic is conserved in tree construction through the order of any node's children. 
<br>
The suggestions, being constructed by node-children traversal, hence are automatically ranked by this statistic.
<br><br>
e.g common word like 'and', 'after', and 'at' are more likely to be suggested at the input 'a' then less common ones like aneurysm



# Motivation
Briefly having discussed the employability of such trees for this purpose in a recent CS lesson at school, 
<br>this implementation serves as to further dive in to the concept while trying to better my rust skills on the way