#include<iostream>
using namespace std;


bool esPrimo(int n) 
{
    if(n <= 1 || n%2 == 0) return false;
    int t = 3;

    while(t*t <= n) 
    {
        if(n%t == 0) return false;
        t+=2;
    } 
    return true;
}


int main(int argc, char* argv[]) {

    int n;
    cin>>n;

    cout<<esPrimo(n)<<'\n';

    // cout<<"Hola Mundo!\n";

    return 0;
}

//g++ test.cpp -o executable