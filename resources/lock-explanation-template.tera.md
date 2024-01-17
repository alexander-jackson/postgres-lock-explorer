{{ lock }}

Conflicts with:
{% for conflict in conflicts %}- {{ conflict }}
{% endfor %}
Example queries acquiring this lock type:
{% for example in examples %}- {{ example }}
{% endfor %}
Example queries blocked by this lock type:
{% for example in blocked_examples %}- {{ example }}
{% endfor %}
