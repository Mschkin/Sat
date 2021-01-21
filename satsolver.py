import numpy as np
from scipy.optimize import fmin_cg
from copy import deepcopy


def generate_evaluate_plains(plain_encoding):
    b = (len(plain_encoding)-1)/len(plain_encoding)**.5

    def evaluate_plain(x):
        distance = 0
        for cor in plain_encoding:
            # the extra minus is because it is NOT this corner
            if cor > 0:
                distance += -x[cor-1]
            if cor < 0:
                distance -= -x[-cor-1]
        distance /= len(plain_encoding)**0.5
        # print(b/len(plain_encoding)**0.5,distance)
        # return (-distance-b)**3
        return np.exp((distance-b))
    return evaluate_plain


def load_cnf_shallow(path, sol):
    plain_encoding = [i[:-1].split(' ') for i in open(path)]
    #print([[i for i in j[:-1]] for j in plain_encoding[1:]])
    plain_encoding = [[int(i) for i in j[:-1]] for j in plain_encoding[1:]]
    plain_encoding, varnum, sol = insert_sure_vals(plain_encoding, sol)
    x = np.zeros(varnum)
    w_list = np.zeros((len(plain_encoding), len(x)))
    bs = np.zeros(len(plain_encoding))
    for i in range(len(plain_encoding)):
        for j in plain_encoding[i]:
            if j > 0:
                w_list[i, j-1] = -1/len(plain_encoding[i])**.5
            if j < 0:
                w_list[i, -j-1] = 1/len(plain_encoding[i])**.5
        bs[i] = (len(plain_encoding)-1)/len(plain_encoding[i])**.5
    return w_list, bs, x, sol


def shallow_ascent(w_list, bs, x, sol):
    print(np.shape(w_list), len(x))
    weights = np.ones_like(bs)
    w_list = np.vstack((w_list, np.eye(len(x)), -np.eye(len(x))))
    bs = np.array(list(bs)+[1]*2*len(x))
    weights = np.array(list(weights)+[.000001]*2*len(x))
    x1 = deepcopy(x)
    x2 = deepcopy(x)
    hes = yield_Hessian(x, w_list, bs, weights)
    eivals, eivecs = np.linalg.eigh(hes)
    x1_dir = eivecs[np.argmin(eivals)]
    x1 += x1_dir/1000
    x2_dir = deepcopy(-1*x1_dir)
    x2 += x2_dir/1000
    old_x2_dir = deepcopy(x2_dir)
    for i in range(5000):
        hes = yield_Hessian(x1, w_list, bs, weights)
        eivals, eivecs = np.linalg.eigh(hes)
        # alternative use the direction which has lowest scalarproduct with the grdient
        # find out why it runs through walls at step 3549
        x1_dir = np.sign(
            x1_dir@eivecs[np.argmin(eivals)])*eivecs[np.argmin(eivals)]
        if not np.allclose(1, np.linalg.norm(x1_dir)):
            raise Exception(np.linalg.norm(x1_dir))
        x1 += x1_dir/1000
        hes = yield_Hessian(x2, w_list, bs, weights)
        eivals, eivecs = np.linalg.eigh(hes)
        print(eivals[:10])
        print('directionchange: ', old_x2_dir@x2_dir)
        old_x2_dir = deepcopy(x2_dir)
        x2_dir = np.sign(
            x2_dir@eivecs[np.argmin(eivals)])*eivecs[np.argmin(eivals)]
        if not np.allclose(1, np.linalg.norm(x2_dir)):
            raise Exception(np.linalg.norm(x2_dir))
        x2 += x2_dir/1000
        print(i, np.linalg.norm(sol-x1), max(np.abs(x1)) -
              1, np.linalg.norm(sol-x2), max(np.abs(x2))-1)

    print(x1)
    print(np.argmax(np.abs(x1)))


def yield_Hessian(x, w_list, bs, weights):
    return np.einsum('ij,ik,i->jk', w_list, w_list, weights/(w_list@x+bs)**2)
    # return np.einsum('ij,ik->jk',w_list,w_list)


def generate_gradients(plain_encoding):
    b = (len(plain_encoding)-1)/len(plain_encoding)**.5

    def gradient(x, grad):
        distance = 0
        for cor in plain_encoding:
            if cor > 0:
                distance += -x[cor-1]
            if cor < 0:
                distance -= -x[-cor-1]
        distance /= len(plain_encoding)**0.5
        val = np.exp(distance-b)
        for cor in plain_encoding:
            if cor > 0:
                grad[cor-1] += -val
            if cor < 0:
                grad[-cor-1] -= -val
    return gradient


def gradient_descent(x, plain_list, gradient_list):
    for k in range(10000):
        grad = np.zeros_like(x)
        for grad_func in gradient_list:
            grad_func(x, grad)
        x -= grad/10
        cost = 0
        for plain in plain_list:
            cost += plain(x)
        print(cost)
        if k % 100 == 0:
            print(cost, x)


def get_cost_grad(plain_list, gradient_list):
    def get_grad(x):
        grad = np.zeros_like(x)
        for grad_func in gradient_list:
            grad_func(x, grad)
        return grad

    def get_cost(x):
        cost = 0
        for i, plain in enumerate(plain_list):
            cost += plain(x)
            if plain(x) > 1:
                print(i, plain(x))
        return cost
    return get_grad, get_cost


def load_cnf(path):
    plain_encoding = [i[:-1].split(' ') for i in open(path)]
    x = np.zeros(int(plain_encoding[0][2]))
    plain_encoding = [[int(i) for i in j[:-1]] for j in plain_encoding[1:]]
    plain_list = []
    grad_list = []
    for clause in plain_encoding:
        plain_list.append(generate_evaluate_plains(clause))
        grad_list.append(generate_gradients(clause))
    return x, plain_list, grad_list


def insert_sure_vals(encoding, sol):
    print(encoding)
    singles = [i[0] for i in encoding if len(i) == 1]
    while len(singles) > 0:
        for s in singles:
            new_encoding = []
            for clause in encoding:
                if s not in clause and -s not in clause:
                    new_encoding.append(clause)
                elif -s in clause:
                    clause.remove(-s)
                    new_encoding.append(clause)
            encoding = new_encoding
        singles = [i[0] for i in encoding if len(i) == 1]
    translation = {}
    new_encoding = []
    c = 1
    for clause in encoding:
        new_clause = []
        for var in clause:
            if var > 0:
                x = var
            else:
                x = -var
            if x in translation:
                new_clause.append(translation[x]*np.sign(var))
            else:
                translation[x] = c
                c += 1
                new_clause.append(translation[x]*np.sign(var))
        new_encoding.append(new_clause)
    new_sol = [np.sign(int(i))*translation[np.sign(int(i))*int(i)]
               for i in sol.split() if int(i)*np.sign(int(i)) in translation]
    sol = []
    for i in new_sol:
        try:
            sol.append((int(i) > 0)*2-1)
        except:
            pass
    print(new_encoding)
    return new_encoding, c-1, sol


def to_3sat(plain_encoding, varnum):
    new_encoding = [[-(varnum+1), -(varnum+2), -(varnum+3)],
                    [-(varnum+1), -(varnum+2), (varnum+3)],
                    [-(varnum+1), (varnum+2), -(varnum+3)],
                    [-(varnum+1), (varnum+2), (varnum+3)],
                    [(varnum+1), -(varnum+2), -(varnum+3)],
                    [(varnum+1), -(varnum+2), (varnum+3)],
                    [(varnum+1), (varnum+2), -(varnum+3)]]
    allways_false1 = varnum+1
    allways_false2 = varnum+2
    varnum += 4
    for plain in plain_encoding:
        if len(plain) == 3:
            new_encoding.append(plain)
        if len(plain) < 3:
            if len(plain) == 1:
                plain.append(allways_false1)
                plain.append(allways_false2)
                new_encoding.append(plain)
            if len(plain) == 2:
                plain.append(allways_false1)
                new_encoding.append(plain)
        if len(plain) > 3:
            new_encoding.append([plain[0], plain[1], varnum])
            varnum += 1
            for p in plain[2:-2]:
                new_encoding.append([-varnum+1, p, varnum])
                varnum += 1
            new_encoding.append([-varnum+1, plain[-2], plain[-1]])
    return new_encoding, varnum-1


def to_cnf_txt(plain_encoding, varnum, path):
    file = open(path, 'w')
    s = f'p cnf {varnum} {len(plain_encoding)}\n'
    for plain in plain_encoding:
        for p in plain:
            s += f'{p} '
        s += '0\n'
    file.write(s)


plain_encoding = [i[:-1].split(' ') for i in open('src\\test_encoded.cnf')]
varnum = int(plain_encoding[0][2])
plain_encoding = [[int(i) for i in j[:-1]] for j in plain_encoding[1:]]
plain_encoding, varnum = to_3sat(plain_encoding, varnum)
to_cnf_txt(plain_encoding,varnum,'3sat_sodoku.cnf')


# gradient_descent(*load_cnf('test_encoded.cnf'))
# get_grad,get_cost=get_cost_grad(load_cnf('test_encoded.cnf')[1],load_cnf('test_encoded.cnf')[2])
# print(fmin_cg(get_cost,load_cnf('test_encoded.cnf')[0],get_grad))
"""
test='1 2 0'
ev=generate_evaluate_plains(test)
print(ev([0,0]),ev([1,1]),ev([1,-1]),ev([-1,1]),ev([-1,-1]))
load_cnf('tents_encoded.cnf')
"""
correct_sol = """ 1 -2 -3 -4 -5 -6 -7 -8 -9 -10 -11 -12 -13 -14 -15 16 -17 -18 -19 -20 -21 -22
 -23 -24 -25 26 -27 -28 -29 -30 -31 32 -33 -34 -35 -36 -37 -38 -39 40 -41 -42
 -43 -44 -45 -46 -47 -48 -49 -50 51 -52 -53 -54 -55 56 -57 -58 -59 -60 -61
 -62 -63 -64 -65 66 -67 -68 -69 -70 -71 -72 -73 -74 -75 -76 -77 -78 -79 -80
 81 -82 -83 -84 85 -86 -87 -88 -89 -90 -91 92 -93 -94 -95 -96 -97 -98 -99
 -100 -101 -102 -103 -104 -105 -106 -107 108 -109 -110 111 -112 -113 -114
 -115 -116 -117 -118 -119 -120 -121 -122 -123 -124 125 -126 127 -128 -129
 -130 -131 -132 -133 -134 -135 -136 -137 -138 -139 140 -141 -142 -143 -144
 -145 -146 -147 -148 -149 150 -151 -152 -153 -154 -155 -156 -157 -158 -159
 160 -161 -162 -163 -164 -165 -166 167 -168 -169 -170 -171 -172 -173 -174
 -175 -176 177 -178 -179 -180 -181 -182 183 -184 -185 -186 -187 -188 -189
 -190 -191 -192 -193 -194 -195 -196 -197 198 -199 200 -201 -202 -203 -204
 -205 -206 -207 -208 -209 -210 -211 -212 -213 214 -215 -216 217 -218 -219
 -220 -221 -222 -223 -224 -225 -226 -227 -228 -229 -230 -231 -232 233 -234
 -235 -236 -237 238 -239 -240 -241 -242 -243 -244 -245 -246 -247 -248 -249
 -250 -251 252 -253 -254 255 -256 -257 -258 -259 -260 -261 -262 -263 -264
 -265 266 -267 -268 -269 -270 -271 272 -273 -274 -275 -276 -277 -278 -279 280
 -281 -282 -283 -284 -285 -286 -287 -288 -289 -290 -291 292 -293 -294 -295
 -296 -297 -298 -299 -300 -301 -302 303 -304 -305 -306 -307 -308 -309 -310
 -311 -312 313 -314 -315 -316 -317 -318 -319 -320 -321 -322 323 -324 -325
 -326 -327 -328 -329 -330 331 -332 -333 -334 -335 -336 337 -338 -339 -340
 -341 -342 343 -344 -345 -346 -347 -348 -349 -350 -351 -352 -353 -354 -355
 -356 -357 -358 359 -360 -361 -362 -363 -364 -365 366 -367 -368 -369 -370
 -371 -372 -373 374 -375 -376 -377 -378 -379 -380 -381 -382 -383 -384 -385
 -386 387 -388 389 -390 -391 -392 -393 -394 -395 -396 -397 -398 399 -400 -401
 -402 -403 -404 -405 -406 -407 -408 -409 -410 411 -412 -413 -414 -415 -416
 -417 -418 -419 -420 -421 422 -423 -424 425 -426 -427 -428 -429 -430 -431
 -432 -433 -434 -435 -436 -437 -438 439 -440 -441 -442 -443 -444 -445 -446
 -447 -448 -449 450 -451 -452 453 -454 -455 -456 -457 -458 -459 -460 -461
 -462 463 -464 -465 -466 -467 -468 469 -470 -471 -472 -473 -474 -475 -476
 -477 -478 -479 -480 -481 482 -483 -484 -485 -486 -487 488 -489 -490 -491
 -492 -493 -494 -495 -496 -497 -498 -499 500 -501 -502 -503 -504 -505 -506
 -507 -508 -509 510 -511 -512 -513 -514 -515 -516 517 -518 -519 -520 -521
 -522 -523 -524 -525 -526 -527 -528 529 -530 -531 -532 -533 -534 -535 -536
 -537 -538 539 -540 -541 -542 543 -544 -545 -546 -547 -548 -549 -550 -551
 -552 -553 -554 -555 -556 -557 558 559 -560 -561 -562 -563 -564 -565 -566
 -567 -568 -569 -570 -571 -572 -573 -574 575 -576 577 -578 -579 -580 -581
 -582 -583 -584 -585 -586 -587 -588 589 -590 -591 -592 -593 -594 -595 -596
 -597 -598 -599 600 -601 -602 -603 -604 -605 606 -607 -608 -609 -610 -611
 -612 -613 -614 -615 -616 -617 -618 -619 -620 621 -622 -623 -624 -625 -626
 -627 628 -629 -630 -631 -632 -633 -634 635 -636 -637 -638 -639 -640 641 -642
 -643 -644 -645 -646 -647 -648 -649 -650 651 -652 -653 -654 -655 -656 -657
 -658 -659 -660 -661 -662 -663 -664 -665 666 -667 -668 -669 -670 -671 -672
 673 -674 -675 676 -677 -678 -679 -680 -681 -682 -683 -684 -685 -686 -687
 -688 689 -690 -691 -692 -693 -694 695 -696 -697 -698 -699 -700 -701 -702
 -703 -704 -705 -706 -707 -708 -709 710 -711 -712 -713 -714 715 -716 -717
 -718 -719 -720 -721 -722 -723 -724 -725 726 -727 -728 -729"""
correct_sol = """1 -2 -3 4 -5 -6 7 -8 -9 -10 -11 12 -13 -14 15 -16 -17 18 19 -20 -21 22 -23
-24 25 -26 -27 28 -29 -30 -31 32 -33 34 -35 -36 37 -38 39 -40 -41 42 43 -44
45 -46 -47 48 -49 -50 -51 52 -53 -54 55 -56 57 -58 59 60 -61 -62 63 -64 -65
66 -67 -68 69 -70 71 -72 73 -74 75 -76 77 -78 79 -80 -81 82 83 -84 85 -86 87
-88 -89 90 -91 92 93 -94 -95 96 -97 98 -99 -100 101 -102 -103 104 -105 106
-107 108 -109 110 111 -112 -113 114 -115 116 -117 -118 119 -120 121 122 -123
-124 125 -126 -127 128 -129 130 -131 132 133 -134 -135 136 -137 -138 139
-140 141 -142 143 -144 145 -146"""
# shallow_ascent(*(load_cnf_shallow('src\\tents_encoded.cnf',correct_sol)))
# print(get_cost(sol))
