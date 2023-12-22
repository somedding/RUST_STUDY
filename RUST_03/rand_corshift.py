import time

# 난수 생성을 위한 시드를 전역 변수로 선언
SEED = 0

#start 부터 end 사이의 난수 생성
def rand(start, end):
    global SEED

    #난수 초기화
    if SEED == 0:

        SEED = int(time.time() * 1000)
    
    SEED ^= (SEED << 13) & 0xFFFFFFFF
    SEED ^= (SEED >> 17) & 0xFFFFFFFF
    SEED ^= (SEED << 5) & 0xFFFFFFFF
    return SEED % (end - start + 1) + start


for _ in range(100):
    print(rand(1, 6))