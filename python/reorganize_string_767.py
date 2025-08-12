"""
conditions where it is simply impossible to do it
"aaab" reasoning:
there is no way to be able to space out the a's because they take up more than (maybe the number is half?) of the string



so a fail case I'm Identifying would be "amt of any char c is > half len(s)"




"""
from queue import PriorityQueue


def organize(s: str) -> str:
    q = PriorityQueue()
    q.put(('a', 4))
    q.put(('b', 1))
    print(q)
    while q:
        print(q.get())




        
    return ""

organize('test')
