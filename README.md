# Description
Implementing a simple word autocompletion algorithm based on a word tree constructed from a frequency dictionary
<br><br>
Wordlists are sourced from https://github.com/wolfgarbe/SymSpell/ where they are sorted by frequency.
<br><br>
This frequency statistic is conserved in tree construction through the order of any node's children. 
<br>
The suggestions, being constructed by node-children traversal, hence are automatically ranked by this statistic.
<br><br>
e.g common word like _'and'_, _'after'_, and _'at'_ are more likely to be suggested at the input _'a'_ then less common ones like _'aneurysm'_

# Documentation
The live interface is launched per default. <br>
Simply start typing to see your suggestions.
```backspace``` to delete characters and ```enter``` to stop


# Motivation
Briefly having discussed the employability of such trees for this purpose in a recent CS lesson at school, 
<br>this implementation serves as to further dive in to the concept while trying to better my rust skills on the way