from .fliton_fib_rs import calculate_times, calculate_parameters


class NumpyInterface:
    def __init__(self) -> None:
        self.inventory = {}

    def calc_times(self, distance, traffic_grade):
        result = calculate_times({}, distance, traffic_grade)
        self.inventory['Car time'] = result['times'][0][0]
        self.inventory['Truck time'] = result['times'][1][0]

    def calc_parameters(self, car_time, truck_time):
        result = calculate_parameters({}, car_time, truck_time)
        self.inventory['Distance'] = result["parameters"][0][0]
        self.inventory['Traffic grade'] = result['parameters'][1][0]
