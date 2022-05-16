from typing import List, Optional
from .fliton_fib_rs import object_interface


class ObjectInterface:
    def __init__(self, number: List[int], numbers: List[List[int]]) -> None:
        self.number = number
        self.numbers = numbers
        self.number_results: Optional[List[int]] = None
        self.numbers_results: Optional[List[List[int]]] = None

    def process(self) -> None:
        object_interface(self)
