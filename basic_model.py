from unittest import result
import numpy as np


class MatixModel:
    @property
    def weights_matrix(self) -> np.array:
        return np.array([
            [3, 2],
            [1, 4]
        ])

    def calculate_times(self, distance: int, traffic_grade: int) -> dict:
        inputs = np.array([
            [distance],
            [traffic_grade]
        ])

        result = np.dot(self.weights_matrix, inputs)
        return {
            "Car time": result[0][0],
            "Truck time": result[1][0]
        }

    def calculate_parameters(self, car_time: int, truck_time: int) -> dict:
        inputs = np.array([
            [car_time],
            [truck_time]
        ])

        result = np.dot(np.linalg.inv(self.weights_matrix), inputs)
        return {
            "Distance": result[0][0],
            "Traffic grade": result[1][0]
        }


if __name__ == "__main__":
    test = MatixModel()

    times = test.calculate_times(distance=10, traffic_grade=3)
    print(f"Here are the times: {times}")

    parameters = test.calculate_parameters(
        car_time=times["Car time"], truck_time=times["Truck time"])
    print(f"Here are the parameters: {parameters}")
