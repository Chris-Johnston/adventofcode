from dataclasses import dataclass

input = """
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
"""

@dataclass
class Directory:
    path: str
    child_directories = dict()
    # filename, size
    files = dict()

    def __str__(self) -> str:
        return f'Directory({self.path}, [{",".join(self.child_directories.keys())}], [{",".join(self.files.keys())}]'

    def get_size(self) -> int:
        """
        recursively get the size of the directory
        """
        sum = 0

        # print('dir', self.path, 'contains child dirs', self.child_directories)
        for key, dir in self.child_directories.items():
            # print('child dir under', self.path, 'called', key)
            sum += dir.get_size()
        
        for key, f in self.files.items():
            sum += f
        return sum

def parse_filesystem(commands: str):
    dir_stack = []
    # root_dir = Directory()
    # quicker lookup without traversing from root_dir
    dirs = {
        '/': Directory('/')
    }

    # init '/'


    for line in commands.strip().splitlines():
        if line.startswith("$"):
            # trim "$ "
            line = line[2:]
            # cmd, arg = line.split()
            cmd = ''
            arg = '' # ls can have no args
            splitline = line.split()
            if len(splitline) == 2:
                cmd = splitline[0]
                arg = splitline[1]
            else:
                cmd = splitline[0]

            if cmd == "cd":
                if arg == "..":
                    dir_stack.pop()
                else:
                    dir_stack.append(arg)
                print('CD, stack is now', dir_stack)
            elif cmd == "ls":
                # nop, see assumption in else case
                # also ls where we populate the filesystem
                pass
            else:
                print("unknown command", cmd, arg)
        else:
            # assume this is output from ls
            splitline = line.split()
            file_size = splitline[0]
            file_name = splitline[1]

            if file_size == "dir":
                # dir
                # file_name is a new dir
                parent_path = '/'.join(dir_stack)

                child_path_stack = dir_stack + [file_name]
                child_path = '/'.join(child_path_stack)

                # insert new dir for the child
                if child_path not in dirs:
                    print('adding child_path', child_path, 'under parent', parent_path)

                    d = Directory(child_path)
                    d.child_directories = dict()
                    d.files = dict()

                    print(d)
                    dirs[child_path] = d

                    # but also update the parent dir
                    # print('looking for parent path', parent_path)
                    dirs[parent_path].child_directories[child_path] = d

            else:
                # file
                parent_path = '/'.join(dir_stack)
                print('adding file', file_name, 'to path', parent_path)
                dirs[parent_path].files[file_name] = int(file_size)

    return dirs

input = open('input.txt').read()

filesystem_dict = parse_filesystem(input)

# print(filesystem_dict)

# sum = 0

# for k, v in filesystem_dict.items():
#     size = v.get_size()
#     print('dir', k, 'has size', size)

#     if size <= 100000:
#         sum += size

# print('answer', sum)

# 70000000 total disk space
# 30000000 needed
# 70000000 - 48381165 = 21618835 free
# 8381165 more required to delete 
# 24933642


total_space = 70000000
needed = 30000000

# get current free space
root_size = filesystem_dict['/'].get_size()
current_free_space = total_space - root_size

# get space required to delete
additional_required = needed - current_free_space

dir_sizes = []

# iterate through all dirs to get a list of sizes
# don't actually care about which dir
for k, v in filesystem_dict.items():
    # optimization could have a cache for the lookup,
    # don't care
    s = v.get_size()
    if s < additional_required:
        continue
    dir_sizes.append(s)

dir_sizes = sorted(dir_sizes)
print(dir_sizes)

# get the first value
print('answer', dir_sizes[0])