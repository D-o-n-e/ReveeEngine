import os

os.system("maturin build")
os.system("pip install C:\\Users\*****\\Documents\\ReveeEngine\\target\\wheels\\ReveeEngine-0.1.0-cp310-none-win_amd64.whl --force-reinstall")
os.system("python python/main.py")