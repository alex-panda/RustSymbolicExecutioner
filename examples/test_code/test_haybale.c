int s_algebra(int x, int y);
int b_algebra(int x, int y);
int s_ifStmt(int x, int y);
int b_ifStmt(int x, int y);
int b(int x, int y);
int s2_ifStmt(int x, int y);
int s_loop(int n);
int b_loop(int n);
int s_nestedIfLoop(int a);
int b_nestedIfLoop(int a);
int b_infLoop(int n);

int s_algebra(int x, int y) {
    x = y + 4;
    y = 2*x; 
    int w = (x*4) + y;
    //symex - what are the possible values?
    return w;
}

int b_algebra(int x, int y) {
    x = y + 4;
    y = 2*x; 
    int w = x / y;
    //symex - division by zero?
    return w;
}

int b_ifStmt(int x, int y) {
    y = 2;
    if (x <= 4) {
        x = 4;
    }

    else if (x > 4) {
        x = 2;
    }

    else {
        y = 0;
        //symex - is this reachable?
    }
    return y;
}
int s_ifStmt(int x, int y) {
    y = 2;
    if (x < 4) {
        x = 4;
    }

    else if (x > 4) {
        x = 2;
    }

    else {
        y = 0;
        //symex - is this reachable?
    }
    return y;
}

int b(int x, int y) {
    x = y * 2;
    if (x == 6) {
        y = y + 3;
        if (y > 2) {
            y = y + 2;
        }
    } else {
        y = y + 4;
    }
    return y - 8;
}

int s2_ifStmt(int x, int y) {
    if (x < 5) {
        if (x > 5) {
            y = x;
        }
        x = y * 2;
        //symex
    }
    return x - (y*2);
}

int s_loop(int n) {
    int i = 0;
    int j = 1;
    while (i < n) {
        j = j * 2;
        i = i + 1;
    }
    //symex - what is the value of i
	return i - n;
}

int b_loop(int n)  {
    int i = 0;
    int j = 1;
    while (i <= n) {
        j = j * 2;
        i = i + 1;
    }
    //symex - what is the value of i
	return i - n;
}

int s_nestedIfLoop(int a) {
    int i = 0;
    int j = 3;
    while (i < 3) {
        if (i == 3) {
            j = 0;
        }
        i = i + 1;
    }
    return j;
}
int b_nestedIfLoop(int a)  {
    int i = 0;
    int j = 3;
    while (i < 3) {
        i = i + 1;
        if (i == 3) {
            j = 0;
        }
    }
    return j;
}

int b_infLoop(int n) {
    int i = 0;
    int j = 1;
    while (i < n) {
        j = j * 2;
    }
    return i;
}