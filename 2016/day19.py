class Node:
    def __init__(self, value: int, next = None) -> None:
        self.value = value
        self.next = next

nodes = [
    Node(i + 1) for i in range(3014387)
]

for i, node in enumerate(nodes):
    node.next = nodes[(i + 1) % len(nodes)]

current_node = nodes[0]

while current_node.next != current_node:
    current_node.next = current_node.next.next
    current_node = current_node.next

print(current_node.value)
