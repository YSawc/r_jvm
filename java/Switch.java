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
    mySwitch(257);
    mySwitch(100);
  }
}
