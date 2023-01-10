from ctypes import cdll

fiboRust = cdll.LoadLibrary("rust/target/release/fiboRust.dll")
fiboRust.fibo_rust()
fiboRust.teste_msg()


print("Acabou")