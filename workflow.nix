{
  adapters = [ "aws" "gcp" ];
  tasks = [
    { name = "fetch_data"; command = "curl https://example.com/data"; }
    { name = "process_data"; command = "python3 process.py"; }
  ];
}
