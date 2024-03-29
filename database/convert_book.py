#!/usr/bin/env python3
import argparse
import struct

BUFFER_DELIMIT = 1

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('path', help='file path to the opening book')
    parser.add_argument('--max', type=int,
                        help='maximum number of iterations')
    parser.add_argument('--output', default='opening_book.bin',
                        help='specify output file')

    args = parser.parse_args()

    with open(args.path, 'rb') as file:
        # Read header
        data = struct.unpack('6B', file.read(6))
        width, height, max_depth, key_size, value_size, log_size = data

        assert width == 7
        assert height == 6

        size = next_prime(log_size)

        # Read keys and values
        keys = struct.unpack(
            f'{size * key_size}B', file.read(size * key_size))
        values = struct.unpack(
            f'{size * value_size}B', file.read(size * value_size))

    assert size == len(keys), len(keys)
    assert size == len(values), len(keys)

    # Add relevant keys and values to cache
    cache = [[] for _ in range(0, 37)]
    for i, (key, value) in enumerate(zip(keys, values)):
        if key == (i & 0xFF) and value != 0:
            cache[value - 1].append(i)
        if args.max is not None and i > args.max:
            break

    # Write file
    with open(args.output, 'wb+') as file:
        file.write(struct.pack(f'<B', max_depth))
        for moves in cache:
            packed = struct.pack(f'<{len(moves) + 1}I', *moves, BUFFER_DELIMIT)
            file.write(packed)


def next_prime(n):
    size = 2 ** n
    for i in range(size, size * 2):
        is_prime = True
        for j in range(2, i):
            if i % j == 0:
                is_prime = False
                break
        if is_prime:
            return i
    return size

if __name__ == '__main__':
    main()
