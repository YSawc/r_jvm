class Fibonacci {
  static int fibonacci(int t) {
    int sum = 0;
    for (int i = 1; i < t; i++) {
      sum += i;
    };
    return sum;
  }

  public static void main(String[] args) {
    int r = fibonacci(10);
    System.out.println("r : " + r);
  }
}
