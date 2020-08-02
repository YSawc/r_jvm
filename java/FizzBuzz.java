class FizzBuzz {
  public static void main(String[] args){
    int a = 0;
    int b = 0;
    int c = 0;
    int d = 0;

    for(int i = 1; i <= 100; ++i){
      if(i % 3 == 0 && i % 5 == 0){
        a += 1;
      } else if (i % 5 == 0) {
        b += 1;
      } else if (i % 3 == 0) {
        c += 1;
      } else {
        d += 1;
      }
    }

    System.out.println("a : " + a);
    System.out.println("b : " + b);
    System.out.println("c : " + c);
    System.out.println("d : " + d);
  }
}
