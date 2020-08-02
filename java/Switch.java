class Switch {
  static int mySwitch (int i) {
    switch(i) {
      case 0: return 0;
      case 1: return 1;
      case 100: return 100;
      case 257: return 257;
      default: return -1;
    }
  }

  public static void main(String[] args) {
    int r1 = mySwitch(100);
    int r2 = mySwitch(257);

    System.out.println("r1 : " + r1);
    System.out.println("r2 : " + r2);
  }
}
