class FizzBuzz{
    public static void main(String[] args){
      int fb_c = 0;
      int f_c = 0;
      int b_c = 0;
      int o_c = 0;
        for(int i = 1; i <= 100; ++i){
            if(i % 3 == 0 && i % 5 == 0){
              fb_c += 1;
            } else if (i % 5 == 0) {
              f_c += 1;
            } else if (i % 3 == 0) {
              b_c += 1;
            } else {
              o_c += 1;
            }
        }
    }
}
