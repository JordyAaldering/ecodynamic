def unzip(data):
    real, user, energy, threads = zip(*data)
    real = list(map(lambda x: x / 1e9, real))
    user = list(map(lambda x: x / 1e9, user))
    energy = list(map(lambda y: y / 1e6, energy))
    print(f'Energy: {sum(energy):.2f}J, Runtime: {sum(real):.2f}s')
    return (real, user, energy, threads)

def acc(xs, start=0):
    acc = start
    for x in xs:
        acc += x
        yield acc
