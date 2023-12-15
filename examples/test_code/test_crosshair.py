def s_algebra(x: int, y: int) -> int:
  '''
  post: __return__ != y
  '''
  x = y + 4
  y = 2*x
  return (x*4) + y
    

def b_algebra(x:int, y:int) -> int:
  '''
  post: y != 0
  '''
  x = x + 4
  y = 2*y
  return x / y

def b_ifStmt(x:int, y:int) -> int:
  '''
  post: __return__ != 0
  '''
  x = y + 4
  y = 2 *x
  if x <= 4:
    y = 4
  elif x > 4:
    y = 2
  else:
    y = 0
  return y

def s_ifStmt(x:int, y:int) -> int:
  '''
  post: __return__ != 0
  '''
  x = y + 4
  y = 2 * x
  if x < 4:
    y = 4
  elif x > 4:
    y = 6
  else:
    y = 0
  return y

def b(x: int, y: int) -> int:
  """
  post: __return__ != 8
  """
  x = y * 2
  if x == 6:
     y = y + 3
     if y > 2:
       y = y + 2
  else:
    y = y + 4

  return y


def s2_ifStmt(x:int, y:int) -> int:
  """
  post: __return__ != 10
  """
  if x < 5:
    if x > 5:
      y = x
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

def b_nestedIfLoop(a:int) -> int:
  '''
  post: __return__  == 0
  '''
  i = 0
  j = 3
  while i < 3:
    i = i + 1
    if i == 3:
      j = 0
  return j

def s_nestedIfLoop(a:int) -> int:
  '''
  post: __return__  == 0
  '''
  i = 0
  j = 3
  while i < 3:
    if i == 3:
      j = 0
    i = i + 1
  return j
      
def b_infLoop(n:int) -> int:
  '''
  post: __return__  == n
  '''
  i = 0
  j = 1
  while i <= n:
    j = j * 2
  return i


