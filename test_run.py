import os.path as path
import subprocess
import time
import termcolor

if __name__ == '__main__':
    base_path = './target/release'
    binaries = ['full_board', 'linear_board', 'qs1']
    sizes = [40, 40, 40]
    binary_paths = list(map(lambda x: path.join(base_path, x), binaries))
    if not all(map(lambda x: path.exists(x), binary_paths)):
        print('Please run "cargo build --release" first')
    else:
        for name, bin, num in zip(binaries, binary_paths, sizes):
            print(termcolor.colored(f'Algorithm: {name}', 'red'))
            print(termcolor.colored(f'Size: {num} x {num}', 'yellow'))
            start_time = time.time()
            proc = subprocess.run([bin, str(num)], capture_output=True, encoding='UTF-8')
            end_time = time.time()
            print(proc.stdout, end='')
            print(termcolor.colored(f'Time taken: {end_time - start_time:.4}s\n', 'green'))