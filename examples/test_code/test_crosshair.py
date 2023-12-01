
def s_algebra(x: int, y: int) -> int:
  '''
  post: __return__ != y
  '''
  x = y + 4
  y = 2*x;
  return (x*3) + y
    

def b_algebra(x:int, y:int) -> int:
  '''
  post: y != 0
  '''
  x = y + 4
  y = 2*x 
  return x / y

def b_ifStmt(x:int, y:int) -> int:
  '''
  post: __return__ != 0
  '''
  y = 2 
  if x <= 4:
    y = 4
  elif x > 4:
    y = 6
  else:
    y = 0
  return y

def s_ifStmt(x:int, y:int) -> int:
  '''
  post: __return__ != 0
  '''
  y = 2 
  if x < 4:
    y = 4
  elif x > 4:
    y = 6
  else:
    y = 0
  return y

def b2_ifStmt(x:int, y:int) -> int:
  '''
  post: y != x + 1
  '''
  y = 1
  if x < 5:
    if x >= 5:
      y = x
    y = y + 1
  return 0

def s2_ifStmt(x:int, y:int) -> int:
  """
  post: __return__ != 10
  """
  if x < 5:
    if x > 5:
      y = x;
    x = y * 2
  return x

def s_loop(n:int) -> int:
  '''
  pre: n > 0
  post: __return__  == n
  '''
  i = 0
  j = 1
  while i < n:
    j = j * 2
    i = i + 1
  return i

def b_loop(n:int) -> int:
  '''
  pre: n > 0
  post: __return__  == n
  '''
  i = 0
  j = 1
  while i <= n:
    j = j * 2
    i = i + 1
  return i
    
def b_infLoop(n:int) -> int:
  '''
  post: __return__  == n
  '''
  i = 0
  j = 1
  while i <= n:
    j = j * 2
  return i
