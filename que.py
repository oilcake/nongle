files = [i for i in range(14)]


class Performer:
    """performer watches that no note being played twice"""

    index: int
    len: int
    window: int

    def __init__(self, files: list[int]) -> None:
        self.len = len(files)
        self.index = 0
        self.window = 5

    def next_id_in_que(self) -> None:
        self.index += 1
        self.index = self.index % self.window
        print(f'que state {self.index}')

    def find_sample_by_velocity(self, velocity: int) -> None:
        self.next_id_in_que()
        overdraft = (velocity + self.index) // self.len
        file_id = velocity + self.index - (self.window * overdraft)
        print(f'trying to access index {file_id}')
        print(files[file_id])


p = Performer(files)
for i in range(9):
    p.find_sample_by_velocity(12)
