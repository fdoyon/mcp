import interop_pb2 as protobuf
import pandas as pd

class PolledMultiplexer:
    def Poll(self,query):
        pass

def addFloatCol(query,name):
    pass
def createQuery(query):
    pass

def __main__():
query = protobuf.Query()
query.schema.name="ticks"
addFloatCol(query.schema,"bid")
addFloatCol(query.schema,"ask")

mcp = PolledMultiplexer(query)
snapshot = mcp.Poll()
print(f"Initial snapshot :{snapshot}")

while true:
    delta = mcp.Poll()
    snapshot = pd.concat([snapshot,delta])
    print(f"current set:{snapshot}")

