{
  adapters = {
    http = { type = "http"; baseUrl = "https://api.example.com"; };
  };

  tasks = {
    fetchData = {
      adapter = "http";
      endpoint = "/data";
      method = "GET";
      dependsOn = [];
    };

    processData = {
      command = "python process.py";
      dependsOn = ["fetchData"];
    };
  };
}

