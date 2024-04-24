

// int tribonacci(int n){
//     if (n < 2){
//         return n;
//     }
//
//     //each term is the sum of the three preceding terms right, so
//     int a = 0; //T0
//     int b = 1; //T1
//     int c = 1; //T2
//     int d;     //T3
//
//     while (n>2) {
//         d = a + b + c;
//
//         a=b;
//         b=c;
//         c=d; //? maybe?
//         n-=1;
//     }
//     return c;
// }
int trib(int dp[], int n);
// Dynamic programming solution:
int tribonacci(int n) {
    int dp[38]; // see problem constraints for why I chose 38

    for (int i = 0; i < 38; ++i) {
        dp[i] = -1;
    }

    dp[0] = 0;
    dp[1] = 1;
    dp[2] = 1;
    return trib(dp, n);
}

int trib(int dp[], int n) {
    if (n < 2) {
        return n;
    }
    if (dp[n] != -1) {
        return dp[n];
    }

    dp[n] = trib(dp, n - 1) + trib(dp, n - 2) + trib(dp, n - 3);
    return dp[n];
}

