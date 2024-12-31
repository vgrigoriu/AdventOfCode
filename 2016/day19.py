class Node:
    def __init__(self, value: int, next=None) -> None:
        self.value = value
        self.next = next


nodes = [Node(i + 1) for i in range(3014387)]

for i, node in enumerate(nodes):
    node.next = nodes[(i + 1) % len(nodes)]

current_node = nodes[0]

while current_node.next != current_node:
    current_node.next = current_node.next.next
    current_node = current_node.next

print(current_node.value)

nodes = [Node(i + 1) for i in range(3014387)]

for i, node in enumerate(nodes):
    node.next = nodes[(i + 1) % len(nodes)]

current_node = nodes[0]

nodes_left = len(nodes)
# choose first victim
steps = nodes_left // 2
before_victim = current_node
for _ in range(steps - 1):
    before_victim = before_victim.next

# found first victim
while nodes_left > 1:
    # remove the victim
    before_victim.next = before_victim.next.next

    nodes_left -= 1
    if nodes_left % 2 == 1:
        # one step to the next victim, leave cursor where it is
        pass
    else:
        before_victim = before_victim.next

print(before_victim.value)

# 1 2 3 4 5 6 7 8 9 10
# 6 (10 -> 5)
# 2 3 4 5 7 8 9 10 1
# 7 (odd -> 1 step)
# 3 4 5 8 9 10 1 2
# 9 (even -> 2 steps)
# 4 5 8 10 1 2 3
# 10 (odd -> 1 step)
# 5 8 1 2 3 4
# 2 (even -> 2 steps)
# 8 1 3 4 5
# 3 (odd -> 1 step)
# 1 4 5 8
# 5 (even -> 2 steps)
# 4 8 1
# 8
# 1 4
# 4
# 1
