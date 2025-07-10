import time

def count_to_billion():
  start_time = time.time()
  
  for i in range(1_000_000_000):
    pass
  
  end_time = time.time()
  elapsed_time = end_time - start_time
  
  print(f"Counting to 1 billion took {elapsed_time:.2f} seconds")

if __name__ == "__main__":
  count_to_billion()