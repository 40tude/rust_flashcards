<!-- 
<p align="center">
<img src="../static/md/assets/img1.png" alt="attention" width="577"/>
</p>

$$ E = mc^2 $$

#### Code snippet  

```python
# -----------------------------------------------------------------------------
def preprocessor(df):
    # drop
    df.drop(columns="Unnamed: 7", inplace=True)
    df.drop_duplicates(inplace=True)

    # format
    df.columns = df.columns.str.lower()
    df.columns = df.columns.str.replace("/", "_")
```


Question : 
Answer   : 

#### Code snippet 

```python
# TODO : add sample code
```

-->



<!-- 
############################################################
## 
############################################################ 
-->

Question : Data Structures - What is an **ArrayList**, what are the use cases, can you implement an ArrayList from scratch in Python? 

Answer   : 

* An **ArrayList** is a dynamic array-based data structure that allows elements to be stored in **contiguous memory locations**, with the capacity to automatically resize as elements are added or removed. 
* Unlike fixed-size arrays, an ArrayList can grow and shrink dynamically, which makes it useful for scenarios where the number of elements is not known in advance. Python's built-in list already functions like an ArrayList.
* Like fixed-size arrays, an ArrayList offers index based addressing (``bob = MyArrayList[42]``)
* Unlike **Linked Lists**, an ArrayList uses a contiguous memory area making copy(), paste(), read(), write()... more efficient. 

#### Key Features
1. Dynamic Sizing: It can grow or shrink in size as needed.
1. Indexing: Provides constant time (**O(1)**) access to elements by index, similar to arrays.
1. Amortized Growth: When the internal array is full, the ArrayList typically resizes itself by allocating a new, larger array and copying the elements over.
1. Insertion: Inserting at the end is efficient, but inserting in the middle or at the beginning can be slow (**O(n)**) because elements need to be shifted.

#### Use Cases
* When you need dynamic resizing.
* If you frequently access elements by index.
* Storing and managing collections of objects where the size may change.
* When you want a list-like structure without worrying about manual resizing.



#### Code snippet 

```python
class ArrayList:
    def __init__(self):
        self.capacity = 1  # Initial capacity of the list
        self.size = 0      # Number of elements in the list
        self.data = [None] * self.capacity  # Internal array to hold the elements

    def __resize(self, new_capacity):
        """Resizes the internal array to a new capacity."""
        new_data = [None] * new_capacity
        for i in range(self.size):
            new_data[i] = self.data[i]
        self.data = new_data
        self.capacity = new_capacity

    def append(self, element):
        """Adds a new element to the end of the list."""
        if self.size == self.capacity:
            # Read this : https://stackoverflow.com/questions/5232198/how-does-the-capacity-of-stdvector-grow-automatically-what-is-the-rate
            self.__resize(2 * self.capacity)  
        self.data[self.size] = element
        self.size += 1

    def get(self, index):
        """Gets the element at a given index."""
        if 0 <= index < self.size:
            return self.data[index]
        else:
            raise IndexError("Index out of bounds")

    def remove(self, index):
        """Removes the element at a given index and shifts the rest."""
        if 0 <= index < self.size:
            for i in range(index, self.size - 1):
                self.data[i] = self.data[i + 1]
            self.data[self.size - 1] = None
            self.size -= 1

            # Optionally shrink the capacity if the size is small enough
            if self.size <= self.capacity // 4:
                self.__resize(self.capacity // 2)
        else:
            raise IndexError("Index out of bounds")

    def __str__(self):
        """String representation of the ArrayList."""
        return str([self.data[i] for i in range(self.size)])

# Usage
arr_list = ArrayList()
arr_list.append(10)
arr_list.append(20)
arr_list.append(30)
print("ArrayList after appending:", arr_list)

print("Element at index 1:", arr_list.get(1))

arr_list.remove(1)
print("ArrayList after removing element at index 1:", arr_list)

```


<!-- 
############################################################
## 
############################################################ 
-->

Question : Data Structures - Can you show me how to implement a **Linked List** in Python ?
Answer   : 

* A **Linked List** is a data structure where elements (nodes) are stored in a linear sequence, but each element points to the next one in the list. 
* Each node consists of two parts: 
    1. the data 
    1. a reference (or pointer) to the next node. 
* Fast to insert at the head
* Slow to insert at the tail

Here's how to implement a basic singly linked list in Python

#### Code snippet 

```python
class Node:
    def __init__(self, data):
        self.data = data  # Store data
        self.next = None  # Initialize next as None

class LinkedList:
    def __init__(self):
        self.head = None  # Initialize the head of the list as None

    # Method to insert a new node at the end of the list
    def append(self, data):
        new_node = Node(data)
        if self.head is None:  # If the list is empty, make the new node the head
            self.head = new_node
            return
        last = self.head
        while last.next:  # Traverse to the last node
            last = last.next
        last.next = new_node  # Link the last node to the new node

    # Method to print the linked list
    def print_list(self):
        current = self.head
        while current:  # Traverse the list and print each node's data
            print(current.data, end=" -> ")
            current = current.next
        print("None")  # Indicate the end of the list

    # Method to insert a new node at the beginning of the list
    def prepend(self, data):
        new_node = Node(data)
        new_node.next = self.head  # Make the new node point to the current head
        self.head = new_node  # Move the head to point to the new node

    # Method to delete a node by value
    def delete(self, key):
        current = self.head

        # If the node to be deleted is the head
        if current and current.data == key:
            self.head = current.next  # Move head to the next node
            current = None  # Free the old head node
            return

        # Search for the node to be deleted, keeping track of the previous node
        prev = None
        while current and current.data != key:
            prev = current
            current = current.next

        if current is None:  # If the key was not found
            return

        prev.next = current.next  # Unlink the node from the list
        current = None  # Free the node

# Create an empty linked list
llist = LinkedList()

# Append some nodes to the linked list
llist.append(10)
llist.append(20)
llist.append(30)

# Print the linked list
llist.print_list()  # Output: 10 -> 20 -> 30 -> None

# Prepend a node to the beginning
llist.prepend(5)
llist.print_list()  # Output: 5 -> 10 -> 20 -> 30 -> None

# Delete a node
llist.delete(20)
llist.print_list()  # Output: 5 -> 10 -> 30 -> None

```



<!-- 
############################################################
## 
############################################################ 
-->

Question : Data Structures - Can you show me how to implement a **HashTable** in Python ?

Answer   : 

* A **HashTable** (also known as a hash map or dictionary) is a data structure that stores key-value pairs. 
* It allows for fast access, insertion, and deletion of elements by using a hash function to convert keys into indices, where the values are stored in an underlying array. 
* The key feature of a hashtable is its ability to perform these operations in constant time, **O(1)** on average, although collisions can make the worst-case time complexity **O(n)**.

#### Key Components 
1. **Hash Function:** A function that takes a key and returns an index (array position). A good hash function should distribute keys uniformly across the array.
1. **Collision Handling:** When two keys hash to the same index, a collision occurs. Common methods to handle collisions are:
    * Chaining: Use linked lists at each array index to store multiple key-value pairs.
    * Open Addressing: Probe the array to find the next available slot.

#### Use Cases for HashTables
* Caching: Storing and retrieving frequently accessed data quickly.
* Databases: Implementing fast lookup operations in databases.
* Sets: To store unique elements with fast insertion and lookup.
* Symbol Table: In compilers and interpreters to store variable names and their associated information.
* Counting Frequency: Counting occurrences of elements in datasets.

#### Code snippet 

```python
class HashTable:
    def __init__(self, size):
        self.size = size
        self.table = [None] * size

    def _hash(self, key):
        return hash(key) % self.size

    def insert(self, key, value):
        index = self._hash(key)
        if self.table[index] is None:
            self.table[index] = []
        for pair in self.table[index]:
            if pair[0] == key:
                pair[1] = value
                return
        self.table[index].append([key, value])

    def get(self, key):
        index = self._hash(key)
        if self.table[index] is not None:
            for pair in self.table[index]:
                if pair[0] == key:
                    return pair[1]
        return None

    def delete(self, key):
        index = self._hash(key)
        if self.table[index] is not None:
            for i, pair in enumerate(self.table[index]):
                if pair[0] == key:
                    del self.table[index][i]
                    return True
        return False

# Usage
hash_table = HashTable(10)
hash_table.insert("clé1", "valeur1")
hash_table.insert("clé2", "valeur2")
print(hash_table.get("clé1"))  # valeur1
hash_table.delete("clé1")
print(hash_table.get("clé1"))  # None

```

<!-- ```python
class HashTable:
    def __init__(self, size=10):
        self.size = size
        self.table = [[] for _ in range(self.size)]  # Create an array of empty lists
    
    def hash_function(self, key):
        """Generate a hash for a given key."""
        return hash(key) % self.size
    
    def insert(self, key, value):
        """Insert or update a key-value pair in the hashtable."""
        hash_index = self.hash_function(key)
        # Check if the key exists, if yes, update its value
        for idx, element in enumerate(self.table[hash_index]):
            if element[0] == key:
                self.table[hash_index][idx] = (key, value)
                return
        # If the key doesn't exist, append the new key-value pair
        self.table[hash_index].append((key, value))
    
    def get(self, key):
        """Retrieve the value associated with a given key."""
        hash_index = self.hash_function(key)
        for element in self.table[hash_index]:
            if element[0] == key:
                return element[1]
        raise KeyError(f"Key {key} not found.")
    
    def remove(self, key):
        """Remove a key-value pair from the hashtable."""
        hash_index = self.hash_function(key)
        for idx, element in enumerate(self.table[hash_index]):
            if element[0] == key:
                del self.table[hash_index][idx]
                return
        raise KeyError(f"Key {key} not found.")

    def display(self):
        """Display the hashtable's content."""
        for idx, chain in enumerate(self.table):
            print(f"Index {idx}: {chain}")

# Usage example
ht = HashTable()
ht.insert("apple", 10)
ht.insert("banana", 20)
ht.insert("orange", 30)
ht.display()

print("Get 'banana':", ht.get("banana"))

ht.remove("banana")
ht.display()

# Uncomment this to see the KeyError
# ht.get("banana")

``` -->



<!-- 
############################################################
## 
############################################################ 
-->

Question : Data Structures - What is a **Stack**, what are the use cases, can you implement a Stack in Python?

Answer   : 

* A **Stack** is a linear data structure that follows the **Last In First Out (LIFO)** principle
* The last element inserted is the first one to be removed
* Imagine a vertical stack of plates. One person places the last washed plate on top of the stack, while another person removes the top plate to dry it.

#### Use Cases
* Undo functionality in text editors or software (most recent action is undone first).
* Expression evaluation (postfix, prefix notations).
* Backtracking algorithms like depth-first search (DFS).
* Browser history (the most recently visited page is stored at the top).

#### Common operations
1. ``push()``: Adds an element to the top of the stack.
1. ``pop()``: Removes the element from the top of the stack.
1. ``peek()``: Returns the "value" of the element at the top of the stack without removing it.
1. ``isEmpty()``: Checks if the stack is empty.

#### Code snippet 

```python
class Stack:
    def __init__(self):
        self.stack = []

    # Push an element to the stack
    def push(self, element):
        self.stack.append(element)

    # Pop an element from the stack
    def pop(self):
        if not self.is_empty():
            return self.stack.pop()
        return "Stack is empty!"

    # Peek at the top element
    def peek(self):
        if not self.is_empty():
            return self.stack[-1]
        return "Stack is empty!"

    # Check if the stack is empty
    def is_empty(self):
        return len(self.stack) == 0

    # Get the size of the stack
    def size(self):
        return len(self.stack)

# Usage
stack = Stack()
stack.push(1)
stack.push(2)
stack.push(3)

print(stack.pop())       # Output: 3
print(stack.peek())      # Output: 2
print(stack.is_empty())  # Output: False

```





<!-- 
############################################################
## 
############################################################ 
-->

Question : Data Structures - What is a **Queue**, what are the use cases, can you implement a Queue in Python?

Answer   : 

* A **Queue** is a linear data structure that follows the **First In First Out (FIFO)** principle
* The first element inserted is the first one to be removed.
* Imagine a horizontal line of people waiting outside a movie theater. The person at the head of the line will be the first to enter the cinema.


#### Use Cases
* Job scheduling in operating systems (jobs are processed in the order they arrive).
* Breadth-first search (BFS) in graph traversal.
* Printer task scheduling (tasks are handled in the order they are submitted).
* Real-time systems (e.g., customer support queues).

#### Common operations
1. ``enqueue()``: Adds an element to the end of the queue.
1. ``dequeue()``: Removes an element from the front of the queue.
1. ``peek()``: Returns the "value" of the element at the front of the queue without removing it.
1. ``isEmpty()``: Checks if the queue is empty.



#### Code snippet 

Queue implementation using `collections.deque` (more efficient than using a list):

```python
from collections import deque

class Queue:
    def __init__(self):
        self.queue = deque()

    # Enqueue an element
    def enqueue(self, element):
        self.queue.append(element)

    # Dequeue an element
    def dequeue(self):
        if not self.is_empty():
            return self.queue.popleft()
        return "Queue is empty!"

    # Peek at the front element
    def peek(self):
        if not self.is_empty():
            return self.queue[0]
        return "Queue is empty!"

    # Check if the queue is empty
    def is_empty(self):
        return len(self.queue) == 0

    # Get the size of the queue
    def size(self):
        return len(self.queue)


queue = Queue()
queue.enqueue(1)
queue.enqueue(2)
queue.enqueue(3)

print(queue.dequeue())   # Output: 1
print(queue.peek())      # Output: 2
print(queue.is_empty())  # Output: False

```





<!-- 
############################################################
## 
############################################################ 
-->

Question : Data Structures - What is a **binary search**? What are the use cases? Can you show how to implement it in Python?

Answer   : 

* Binary search is an efficient algorithm for finding an item from a **sorted** list of items. 
* It works by repeatedly dividing the search interval in half. 
* If the value of the search key is less than the item in the middle of the interval, the search continues on the left half
* otherwise, it continues on the right half. 
* This method eliminates half of the remaining elements each time, resulting in a time complexity of **O($\log_2(n)$)**.

#### Use Cases of Binary Search
1. Finding elements in a sorted array: Binary search is ideal when the list is already sorted, as it significantly reduces search time compared to linear search.
2. Searching for a target value in databases: Binary search can be used in indexing databases where records are sorted based on keys.
3. Searching for boundaries: Binary search is often used to find specific boundaries, such as the smallest or largest value that satisfies a given condition.
4. Optimization problems: It is often applied in situations that require finding optimal solutions, like in games, where you find the best strategy using binary search techniques.

#### How binary search works
1. Start with the entire list and find the middle element.
2. Compare the middle element with the target value:
    * If the middle element is equal to the target, return its position.
    * If the middle element is greater than the target, repeat the search in the left half of the list.
    * If the middle element is less than the target, repeat the search in the right half of the list.
3. Repeat the process until the target is found or the search interval is empty.


#### Code snippet 

```python
def binary_search(arr, target):
    left, right = 0, len(arr) - 1
    
    while left <= right:
        mid = left + (right - left) // 2  # Avoids overflow compared to (left + right) // 2
        
        # Check if target is present at mid
        if arr[mid] == target:
            return mid  # Target found, return index
        elif arr[mid] < target:
            left = mid + 1  # Target is in the right half
        else:
            right = mid - 1  # Target is in the left half
    
    return -1  # Target not found

# Usage
arr = [1, 3, 5, 7, 9, 11, 13, 15]
target = 7
result = binary_search(arr, target)

if result != -1:
    print(f"Element found at index {result}")
else:
    print("Element not found")
```

#### Explanation
* `left` and `right` represent the boundaries of the search.
* The middle index (`mid`) is calculated at each step, and the value at that index is compared with the target.
* Depending on the comparison, either the left or right boundary is updated to "narrow down" the search space.
* The process continues until the element is found or the search space becomes empty.

#### Time Complexity
* Best case: **O(1)** (when the middle element is the target)
* Worst case: **O($\log_2(n)$)** (when the element is not found, or at the boundaries)















<!-- 
############################################################
## 
############################################################ 
-->

Question : Data Structures - What is a **Binary Search Tree**? What are the use cases? Can you show how to implement it in Python?

Answer   : 

A **Binary Search Tree (BST)** is a data structure that stores elements (typically numbers) in a hierarchical manner. 


#### Use Cases
1. Searching: A BST allows for efficient searching, with an average time complexity of **O($\log_2(n)$)** if the tree is **balanced**.
2. Inserting and Deleting: Insertions and deletions also have an average time complexity of **O($\log_2(n)$)** in a **balanced** BST.
3. Sorted Data Access: BSTs maintain data in a sorted manner, so **in order traversal** of the tree provides the elements in ascending order.
4. Range Queries: BSTs are useful in solving range queries, where we need to find all keys within a certain range.
5. Data Storage: It’s used in databases, file systems, and memory allocation, where quick search, insertion, and deletion operations are required.


Each node in a BST contains the following:

1. **Key** or **Value**: Data the node holds.
1. **Left Child**: Points to the left subtree. It contains nodes with values smaller than the parent node.
1. **Right Child**: Points to the right subtree. It contains nodes with values larger than the parent node.


#### Properties of a Binary Search Tree
1. The left subtree of a node contains only nodes with values less than the node’s value.
2. The right subtree of a node contains only nodes with values greater than the node’s value.
3. Both left and right subtrees must also be binary search trees.


#### Key Functions
* ``insert()``: Adds a new value to the tree at the appropriate location.
* ``search()``: Looks for a specific value in the tree.
* ``inorder_traversal()``: Visits all nodes in **ascending** order (useful for sorting or retrieving sorted elements).



#### Code snippet 

```python
class Node:
    def __init__(self, key):
        self.left = None
        self.right = None
        self.value = key

class BinarySearchTree:
    def __init__(self):
        self.root = None

    def insert(self, key):
        """Insert a new node with the given key."""
        if self.root is None:
            self.root = Node(key)
        else:
            self._insert(self.root, key)

    def _insert(self, current_node, key):
        if key < current_node.value:
            if current_node.left is None:
                current_node.left = Node(key)
            else:
                self._insert(current_node.left, key)
        elif key > current_node.value:
            if current_node.right is None:
                current_node.right = Node(key)
            else:
                self._insert(current_node.right, key)
        # If key == current_node.value, do nothing (no duplicates allowed)

    def search(self, key):
        """Search for a node with the given key."""
        return self._search(self.root, key)

    def _search(self, current_node, key):
        if current_node is None or current_node.value == key:
            return current_node
        elif key < current_node.value:
            return self._search(current_node.left, key)
        else:
            return self._search(current_node.right, key)

    def inorder_traversal(self):
        """Inorder traversal of the tree."""
        return self._inorder_traversal(self.root)

    def _inorder_traversal(self, current_node):
        nodes = []
        if current_node is not None:
            nodes += self._inorder_traversal(current_node.left)     # Visit left subtree first
            nodes.append(current_node.value)                        # Then visit node 
            nodes += self._inorder_traversal(current_node.right)    # Then right subtree
        return nodes

# Usage
bst = BinarySearchTree()
bst.insert(10)
bst.insert(5)
bst.insert(20)
bst.insert(3)
bst.insert(7)

print("Inorder Traversal:", bst.inorder_traversal())  # Outputs: [3, 5, 7, 10, 20]
print("Search 7:", bst.search(7) is not None)  # Outputs: True
print("Search 15:", bst.search(15) is not None)  # Outputs: False
```

One can extend this implementation by adding functions for ``deletion()``, ``balancing()``...






<!-- 
############################################################
## 
############################################################ 
-->

Question : Data Structures - If a BST class already has an ``inorder_traversal()`` method coded that way, how would you code ``preorder_traversal()`` and ``postorder_traversal()`` method?


```python

def inorder_traversal(self):
        """Inorder traversal of the tree."""
        return self._inorder_traversal(self.root)

def _inorder_traversal(self, current_node):
        nodes = []
        if current_node is not None:
            nodes += self._inorder_traversal(current_node.left)
            nodes.append(current_node.value)
            nodes += self._inorder_traversal(current_node.right)
        return nodes
```

Answer   : 

**Inorder Traversal** : Visit the left subtree, then the root, then the right subtree. The order is:

1. Traverse the left subtree.
1. Visit the node.
1. Traverse the right subtree.


**Preorder Traversal** : Visit the root first, then the left subtree, and then the right subtree. The order is:

1. Visit the node.
1. Traverse the left subtree.
1. Traverse the right subtree.

**Postorder Traversal** : Traverse the left subtree first, then the right subtree, and then visit the node. The order is:

1. Traverse the left subtree.
1. Traverse the right subtree.
1. Visit the node.


#### Code snippet 

```python
# Preorder Traversal
def preorder_traversal(self):
    """Preorder traversal of the tree."""
    return self._preorder_traversal(self.root)

def _preorder_traversal(self, current_node):
    nodes = []
    if current_node is not None:
        nodes.append(current_node.value)  # Visit node first
        nodes += self._preorder_traversal(current_node.left)  # Then left subtree
        nodes += self._preorder_traversal(current_node.right)  # Then right subtree
    return nodes

# Postorder Traversal
def postorder_traversal(self):
    """Postorder traversal of the tree."""
    return self._postorder_traversal(self.root)

def _postorder_traversal(self, current_node):
    nodes = []
    if current_node is not None:
        nodes += self._postorder_traversal(current_node.left)  # Traverse left subtree
        nodes += self._postorder_traversal(current_node.right)  # Traverse right subtree
        nodes.append(current_node.value)  # Visit node after both subtrees
    return nodes


# Usage
bst = BinarySearchTree()
bst.insert(10)
bst.insert(5)
bst.insert(20)
bst.insert(3)
bst.insert(7)

print("Inorder Traversal:", bst.inorder_traversal())     # Outputs: [3, 5, 7, 10, 20]
print("Preorder Traversal:", bst.preorder_traversal())   # Outputs: [10, 5, 3, 7, 20]
print("Postorder Traversal:", bst.postorder_traversal()) # Outputs: [3, 7, 5, 20, 10]

```


#### Summary

* **Inorder** traversal: Left subtree → Node → Right subtree
* **Preorder** traversal: Node → Left subtree → Right subtree
* **Postorder** traversal: Left subtree → Right subtree → Node











<!-- 
############################################################
## 
############################################################ 
-->

Question : Data Structures - In the context of BST how do you explain **DFS**, can you show how to implement it in Python ? 

Answer   : 

In a **Binary Search Tree (BST)**, both **Depth First Search (DFS)** and **Breadth First Search (BFS)** are common traversal techniques used to explore or search through the tree's nodes. 

#### Depth First Search (DFS)
* DFS explores as far as possible along each branch before backtracking. 
* It can be implemented in three different ways depending on the order in which you visit nodes:
    * **Pre-order** (visit the current node, then left subtree, then right subtree)
    * **In-order** (visit left subtree, then current node, then right subtree)
    * **Post-order** (visit left subtree, then right subtree, then current node)
* DFS can be implemented using recursion or with a stack.
* Use DFS when you want to explore as far as possible along a branch before backtracking.



#### Code snippet 

```python
class Node:
    def __init__(self, key):
        self.left = None
        self.right = None
        self.val = key

def dfs_inorder(root):
    if root:
        # Traverse the left subtree first
        dfs_inorder(root.left)
        # Visit the node
        print(root.val, end=' ')
        # Traverse the right subtree
        dfs_inorder(root.right)

# Create a sample binary search tree
#       50
#      /  \
#    30   70
#    / \  / \
#   20 40 60 80

root = Node(50)
root.left = Node(30)
root.right = Node(70)
root.left.left = Node(20)
root.left.right = Node(40)
root.right.left = Node(60)
root.right.right = Node(80)

dfs_inorder(root)  # Output: 20 30 40 50 60 70 80
```






<!-- 
############################################################
## 
############################################################ 
-->

Question : Data Structures - In the context of BST how do you explain **BFS**, can you show how to implement it in Python ? 

Answer   : 

In a **Binary Search Tree (BST)**, both **Depth First Search (DFS)** and **Breadth First Search (BFS)** are common traversal techniques used to explore or search through the tree's nodes. 


#### Breadth First Search (BFS)
* BFS explores the tree level by level, starting from the root. 
* It visits all nodes at the present depth before moving on to nodes at the next depth level. 
* This can be implemented using a queue
* Use BFS when you want to explore nodes closer to the root first, and then gradually move outward to deeper nodes

#### Use cases
* Web crawling : It starts from a root webpage and explores all the links (nodes) level by level, ensuring that all the pages linked to the current page are visited before moving on to the next depth level.
* Level Order Traversal of a Binary Tree : Printing nodes level by level in a tree.
Converting a binary tree into a human-readable format by level.
* Shortest Path in an Unweighted Graph or Grid : indeed BFS explores all nodes at the current depth (level) before moving to nodes at the next depth. This ensures that when BFS visits a node, it has found the shortest path to that node. 
    * In social networks, to determine the shortest path (or degrees of separation) between two people
    * Finding the shortest path in a maze where movement costs are uniform



#### Code snippet 

```python
from collections import deque

def bfs(root):
    if root is None:
        return
    
    # Initialize a queue for BFS
    queue = deque([root])
    
    while queue:
        # Dequeue a node from the front of the queue
        node = queue.popleft()
        # Visit the node
        print(node.val, end=' ')
        
        # Enqueue the left child if it exists
        if node.left:
            queue.append(node.left)
        # Enqueue the right child if it exists
        if node.right:
            queue.append(node.right)

# Create a sample binary search tree
#       50
#      /  \
#    30   70
#    / \  / \
#   20 40 60 80

root = Node(50)
root.left = Node(30)
root.right = Node(70)
root.left.left = Node(20)
root.left.right = Node(40)
root.right.left = Node(60)
root.right.right = Node(80)

bfs(root)  # Output: 50 30 70 20 40 60 80
```




<!-- 
############################################################
## 
############################################################ 
-->

Question : Data Structures - Show how to implement a **recursive binary search** in Python?
Answer   : 

#### Code snippet 

```python
def binary_search_recursive(arr, target, left, right):
    # Base case: if the search space is invalid
    if left > right:
        return -1  # Target not found
    
    # Avoid overflow compared to (left + right) // 2 
    # (// is integer division)
    mid = left + (right - left) // 2  

    # Check if the target is present at mid
    if arr[mid] == target:
        return mid  # Target found
    elif arr[mid] < target:
        # Target is in the right half, so we recurse on the right side
        return binary_search_recursive(arr, target, mid + 1, right)
    else:
        # Target is in the left half, so we recurse on the left side
        return binary_search_recursive(arr, target, left, mid - 1)

# Wrapper function for cleaner usage
def binary_search(arr, target):
    return binary_search_recursive(arr, target, 0, len(arr) - 1)

# Usage
arr = [1, 3, 5, 7, 9, 11, 13, 15]
target = 7
result = binary_search(arr, target)

if result != -1:
    print(f"Element found at index {result}")
else:
    print("Element not found")

```




<!-- 
############################################################
## 
############################################################ 
-->

Question : Data Structures - La **récursivité**... ça vous parle ?
Answer   : 

* Cela consiste à appeler une fonction depuis la fonction elle-même. 
* Utile dans les problèmes qui peuvent être divisés en sous-problèmes à **plus petite échelle**
* **ATTENTION**. Pour intérressante quelle soit d'un point de vue intellectuel ou esthétique, une implémentation récursive n'est pas toujours la plus efficace 

#### Quand utiliser la récursivité ?
* Il existe une condition d'arrêt claire (un cas de base) qui met fin à la récursion.
* Le problème peut être décomposé en sous-problèmes identiques mais à plus petite échelle (pensez aux fractales).


#### Recette de cuisine à suivre

1. **Identifier** la condition d'arrêt
    * C'est la première chose à coder dans la fonction récursive
    * Evite une récursion infinie. 
    * C’est l’étape qui détermine quand la fonction doit arrêter de s'appeler.
    * Souvent un cas simple ou trivial du problème initial

2. **Décomposer** le problème
    * Diviser le problème en problème à **plus petite échelle**, similaires à l'original. 
    * La fonction s'appelle avec des arguments plus simples ou plus réduits.

3. **Combiner** les résultats
    * Optionnel. Combiner les résultats des appels récursifs pour produire le résultat final.

#### Exemples simples de récursion

##### Factorielle

* Cas de base  : `n = 0`, la factorielle de 0 est 1
* Cas récursif : `factorielle(n) = n * factorielle(n - 1)`

###### Code snippet 

```python
def factorielle(n):
    if n == 0:  # Cas de base
        return 1
    else:
        return n * factorielle(n - 1)  # Cas récursif

```

##### Somme des éléments d'une liste

* Cas de base  : Si la liste est vide, la somme est 0.
* Cas récursif : La somme d’une liste c'est la valeur du premier élément plus la somme des éléments restants

###### Code snippet 

```python
def somme(liste):
    if len(liste) == 0:  # Cas de base
        return 0
    else:
        return liste[0] + somme(liste[1:])  # Cas récursif

```

##### Suite de Fibonacci

La suite de Fibonacci est définie par :

* `F(0) = 0`
* `F(1) = 1`
* Si `n > 1` alors `F(n) = F(n-1) + F(n-2)`

On a donc

* Cas de base : `F(0) = 0` et `F(1) = 1`.
* Cas récursif : `F(n) = F(n-1) + F(n-2)`.


###### Code snippet 

```python
def fibonacci(n):
    if n == 0:  # Cas de base 1/2
        return 0
    elif n == 1:  # Cas de base 2/2
        return 1
    else:
        return fibonacci(n-1) + fibonacci(n-2)  # Cas récursif

```

#### Bonnes pratiques

* **Vérifier le cas de base** : Pour éviter une récursion infinie.
* **Limiter la profondeur** : Pour préserver la stack.
* **Memoization** : La mise en cache des résultats intermédiaires peut être plus efficace que la recursivité.




<!-- 
############################################################
## 
############################################################ 
-->

Question : Data Structures - What is **P vs NP** ? What can you say about it ? 
Answer   : 

#### What is P?

**P** stands for "Polynomial time." It refers to problems that we can solve **efficiently** using a computer. When we say a problem is in P, it means that we can find a solution in a reasonable amount of time (specifically, in time that grows like a polynomial function of the size of the input).

For example, consider a simple sorting algorithm like **merge sort**, which has a time complexity of O($n \log(n)$). This is a polynomial time algorithm, which means as the input size increases, the time it takes to sort grows in a manageable way.

##### Note

* In the specific case of merge sort the $\log()$ is $\log_2()$ but with other problems this might **not** be the case. 
* Here $\log()$ should be understood as a "generic" version of $\log()$. 


##### Code snippet 

```python
arr = [5, 2, 9, 1, 5, 6]
sorted_arr = sorted(arr)  # Merge sort in the background (P problem)
print(sorted_arr)
```

#### What is NP?

**NP** stands for "Nondeterministic Polynomial time." It refers to problems where it’s **hard to find** a solution, but if someone gives you a solution, you can **verify** it quickly (in polynomial time).

Think about credit cards. The code is hard to break but if you get the code you can quickly check that it is correct.

Another classic NP problem is the **Traveling Salesman Problem (TSP)**. Given a list of cities and distances between them, you need to find the shortest route that visits all cities exactly once and returns to the starting city. There are no efficient known algorithms to solve this problem for large numbers of cities.

However, if someone gives you a potential solution (a specific route), you can quickly check if it's valid and what the total distance is.

##### Code snippet 

```python
# Given a possible solution to TSP
route = ['A', 'B', 'C', 'D', 'A']
distances = {('A', 'B'): 10, ('B', 'C'): 20, ('C', 'D'): 15, ('D', 'A'): 10}

# Verifying the distance of this route (quick to verify)
def calculate_distance(route):
    total_distance = 0
    for i in range(len(route) - 1):
        total_distance += distances[(route[i], route[i+1])]
    return total_distance

print(calculate_distance(route))  # Easy to verify the route distance
```
Here, checking the route is quick (polynomial time), but finding the best route is extremely hard for large sets of cities.

#### The 1M$ question
The big unsolved question in computer science is: **Are P and NP the same?** In simpler terms, can all NP problems (which are hard to solve but easy to verify) actually be solved efficiently like P problems?

If we prove that **P = NP**, it would mean that for every hard problem we can verify quickly, there is also a fast way to solve it from scratch. This would revolutionize fields like cryptography, optimization, and more.

On the other hand, if **P ≠ NP**, it means that some problems are inherently hard to solve, and there’s no shortcut to finding solutions, even though checking them is easy.

#### Example of a P Problem (Efficient Solution)
Finding the shortest path in a graph using **Dijkstra’s algorithm** is a P problem because we can solve it efficiently.


##### Code snippet 

```python
import heapq

def dijkstra(graph, start):
    queue = [(0, start)]
    distances = {node: float('inf') for node in graph}
    distances[start] = 0
    
    while queue:
        current_distance, current_node = heapq.heappop(queue)
        
        for neighbor, weight in graph[current_node]:
            distance = current_distance + weight
            if distance < distances[neighbor]:
                distances[neighbor] = distance
                heapq.heappush(queue, (distance, neighbor))
    
    return distances

graph = {
    'A': [('B', 1), ('C', 4)],
    'B': [('A', 1), ('C', 2), ('D', 5)],
    'C': [('A', 4), ('B', 2), ('D', 1)],
    'D': [('B', 5), ('C', 1)]
}

print(dijkstra(graph, 'A'))  # P problem: can be solved efficiently
```

#### Example of an NP Problem (Hard to Solve)
Let’s simulate the **Traveling Salesman Problem** (TSP), which is NP-hard. Here, we’re trying all possible permutations of routes (which is very inefficient).

##### Code snippet 

```python
import itertools

cities = ['A', 'B', 'C', 'D']
distances = {
    ('A', 'B'): 10, ('A', 'C'): 20, ('A', 'D'): 15,
    ('B', 'C'): 25, ('B', 'D'): 30,
    ('C', 'D'): 35
}

def calculate_route_distance(route):
    total = 0
    for i in range(len(route) - 1):
        total += distances.get((route[i], route[i+1]), distances.get((route[i+1], route[i]), float('inf')))
    return total

# Trying all possible permutations (brute force)
def tsp(cities):
    shortest_route = None
    min_distance = float('inf')
    
    for perm in itertools.permutations(cities):
        route_distance = calculate_route_distance(perm)
        if route_distance < min_distance:
            min_distance = route_distance
            shortest_route = perm
    
    return shortest_route, min_distance

print(tsp(cities))  # NP problem: inefficient solution for large input
```

#### Summary
* **P** : Problems that can be solved efficiently (like sorting or finding the shortest path) in Python.
* **NP** : Problems where finding a solution is hard, but verifying it is easy (like TSP).
* The question of whether **P = NP** is still open. 






<!-- 
############################################################
## 
############################################################ 
-->

Question : Data Structures - What is a **Trie** data structure, what are the use cases, can you implement an Trie from scratch in Python? 


Answer   : 

A **Trie** (prefix tree) is a specialized tree data structure used to store a dynamic set or associative array where the keys are usually strings. It allows for fast retrieval of words or prefixes, making it useful for tasks like autocomplete and spell checking.

##### Structure
- Each node in a Trie represents a character of the word.
- The root node is typically empty, and all subsequent nodes are linked to one another through edges.
- Each path down the tree corresponds to a prefix or word formed by concatenating the characters along the path.
- A boolean flag (e.g., `is_end_of_word`) is often used at the node level to mark the end of a valid word.

<p align="center">
<img src="../static/md/assets/trie.png" alt="trie" width="577"/>
</p>

Source : [Wikipedia](https://en.wikipedia.org/wiki/Trie)



##### Key Features

* **Space-efficient for common prefixes**: Instead of storing duplicate prefixes for words, they share common ancestors.
* **Time complexity**: For insertions and lookups, the time complexity is proportional to the length of the word being inserted or searched, i.e., O(m), where m is the length of the word.

##### Use Cases
1. **Autocomplete**: Given a prefix, quickly find all words in the dictionary starting with that prefix.
2. **Spell checking**: Efficiently look up if a word exists in a dictionary.
3. **IP routing (Longest prefix matching)**: Used in routers for finding the longest matching prefix.
4. **Word Games**: Useful in games like Scrabble or Boggle where words are formed by combining letters.



##### Code snippet 

```python
class TrieNode:
    def __init__(self):
        self.children = {}  # Dictionary to store child nodes
        self.is_end_of_word = False  # Flag to mark the end of a word


class Trie:
    def __init__(self):
        self.root = TrieNode()

    def insert(self, word):
        """
        Insert a word into the trie.
        """
        node = self.root
        for char in word:
            if char not in node.children:
                node.children[char] = TrieNode()  # Create a new node if the character doesn't exist
            node = node.children[char]
        node.is_end_of_word = True  # Mark the end of the word

    def search(self, word):
        """
        Search if a word exists in the trie.
        """
        node = self.root
        for char in word:
            if char not in node.children:
                return False
            node = node.children[char]
        return node.is_end_of_word  # Return True only if it's a complete word

    def starts_with(self, prefix):
        """
        Check if any word in the trie starts with the given prefix.
        """
        node = self.root
        for char in prefix:
            if char not in node.children:
                return False
            node = node.children[char]
        return True  # If we can navigate through the entire prefix, return True


# Usage example:
trie = Trie()

# Insert words into the trie
trie.insert("apple")
trie.insert("app")
trie.insert("bat")
trie.insert("ball")

# Search for words
print(trie.search("app"))      # True (word exists)
print(trie.search("appl"))     # False (not a complete word)
print(trie.search("apple"))    # True (word exists)

# Check for prefixes
print(trie.starts_with("app")) # True (prefix exists)
print(trie.starts_with("bat")) # True (prefix exists)
print(trie.starts_with("cat")) # False (no word starts with this prefix)
```

##### Key Methods

* **`insert(word)`**: Inserts a word into the trie.
* **`search(word)`**: Returns `True` if the word is found in the trie, and `False` otherwise.
* **`starts_with(prefix)`**: Returns `True` if there is any word in the trie that starts with the given prefix.

##### Time Complexity

* **Insert**: O(m), where m is the length of the word.
* **Search**: O(m), where m is the length of the word.
* **Prefix Search**: O(m), where m is the length of the prefix.









<!-- 
############################################################
## 
############################################################ 
-->

Question : Data Structures - How would you reverse a linked list ? 

Answer : 

##### Code snippet 

```python

class Node: 
    def __init__(self, data): 
        self.data = data 
        self.next = None
  
class LinkedList: 
    def __init__(self): 
        self.head = None
  
    def prepend(self, new_data): 
        new_node = Node(new_data) 
        new_node.next = self.head 
        self.head = new_node 
    
    # 3 pointers : prev, current, next
    # Move to the "right" the 3 pointers
    #       current.next = previous 
    #       previous = current
    #       current = current.next (but we need to make a copy of current.next in next pointer first)
    def reverse(self): 
        prev = None
        current = self.head 
        while(current is not None): 
            next = current.next     # make a copy to prepare step 3
            current.next = prev     # step 1
            prev = current          # step 2
            current = next          # step 3
        self.head = prev 
  
    def print_list(self):
        current = self.head
        while current:  
            print(current.data, end=" -> ")
            current = current.next
        print("None")  

llist = LinkedList() 
llist.prepend(10) 
llist.prepend(20) 
llist.prepend(30) 
llist.prepend(42) 
  
llist.print_list() 
llist.reverse() 
llist.print_list()
```

* Time Complexity : O(n) 
* Space Complexity : O(n) (next is created in each loop) 




<!-- 
############################################################
## 
############################################################ 
-->

Question : Data Structures - What is a **Merge Sort** algorithm, what are the use cases and can you implement a Merge Sort from scratch in Python? 

Answer   : 

This is a **divide-and-conquer algorithm** that recursively divides an input array into two halves, sorts each half, and then merges the two sorted halves back together. It's an efficient, stable, and comparison-based sorting algorithm with a time complexity of \( O(n \log n) \).

#### Steps of the Merge Sort Algorithm:
1. **Divide**: Split the array into two halves until each sub-array contains a single element or is empty.
2. **Conquer**: Sort each of the two halves.
3. **Merge**: Merge the two sorted halves back into a single sorted array.

#### Complexity and Efficiency
- **Time Complexity**: \( O(n \log n) \) for all cases (worst, average, and best) because it consistently divides the array in half.
- **Space Complexity**: \( O(n) \) because it requires additional space for the temporary arrays during merging.
- **Stability**: Merge Sort is stable, meaning it maintains the relative order of equal elements in the sorted array.

#### Use Cases of Merge Sort
1. **Large Datasets**: Suitable for large datasets because it guarantees \( O(n \log n) \) time complexity, unlike algorithms like Quick Sort that can degrade to \( O(n^2) \) in the worst case.
2. **Linked Lists**: Often used for sorting linked lists, where the cost of random access is high, making merges more efficient.
3. **External Sorting**: Used in external sorting (e.g., sorting data on disk) where merging is efficient for handling large datasets that don't fit in memory.

#### Implementation in Python

```python
def merge_sort(arr):
    if len(arr) <= 1:
        return arr

    # Step 1: Divide the array in half
    mid = len(arr) // 2
    left_half = arr[:mid]
    right_half = arr[mid:]

    # Recursively sort each half
    left_sorted = merge_sort(left_half)
    right_sorted = merge_sort(right_half)

    # Step 3: Merge the sorted halves
    return merge(left_sorted, right_sorted)

def merge(left, right):
    sorted_array = []
    i = j = 0

    # Merge the two halves into a sorted array
    while i < len(left) and j < len(right):
        if left[i] < right[j]:
            sorted_array.append(left[i])
            i += 1
        else:
            sorted_array.append(right[j])
            j += 1

    # Append any remaining elements from both halves
    sorted_array.extend(left[i:])
    sorted_array.extend(right[j:])

    return sorted_array

# Usage
arr = [38, 27, 43, 3, 9, 82, 10]
sorted_arr = merge_sort(arr)
print("Sorted array:", sorted_arr) # [3, 9, 10, 27, 38, 43, 82]
```

