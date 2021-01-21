import numpy as np
from copy import deepcopy


edges=[[-1,-1,-1],[-1,-1,1],[-1,1,-1],[-1,1,1],[1,-1,-1],[1,-1,1],[1,1,-1]]
planes=[]
for i in range(4):
    for edge in edges:
        planes.append(edge[:i]+[0]+edge[i:])
print(len(planes))
valid_problems=[]
testnumber=100
while len(valid_problems)<testnumber:
    x=''.join([str(i) for i in np.random.randint(0,2,28)])
    excluded_corners=set()
    for n,val in enumerate(x):
        if val=='1':
            c=deepcopy(planes[n])
            c[n//7]+=1
            excluded_corners.add(tuple(c))
            c[n//7]-=2
            excluded_corners.add(tuple(c))
    if len(excluded_corners)==15:
        valid_problems.append(x)
print(valid_problems[0])
print(planes)