import raft_cmdpb_pb2
import metapb_pb2
import socket
import sys
import struct
import msgpb_pb2
import uuid

server_array = [(1, 4, 20161), (2, 5, 20162), (3, 6, 20163)]
index = 1
#message Peer {      
#    optional uint64 id          = 1 [(gogoproto.nullable) = false]; 
#    optional uint64 store_id    = 2 [(gogoproto.nullable) = false];
#}
peer = metapb_pb2.Peer()
peer.store_id = server_array[index][0]
peer.id = server_array[index][1]

#message RegionEpoch {
#    // Conf change version, auto increment when add or remove peer
#    optional uint64 conf_ver	= 1 [(gogoproto.nullable) = false];
#    // Region version, auto increment when split or merge
#    optional uint64 version     = 2 [(gogoproto.nullable) = false];
#}
region_epoch = metapb_pb2.RegionEpoch()
region_epoch.version = 1
region_epoch.conf_ver = 1

#message RaftRequestHeader {
#    optional uint64 region_id                   = 1;
#    optional metapb.Peer peer                   = 2;
#    optional bool read_quorum                   = 3;
#    optional bytes uuid                         = 4;
#    optional metapb.RegionEpoch region_epoch    = 5;
#    optional uint64 term                        = 6;
#}

header = raft_cmdpb_pb2.RaftRequestHeader()
header.region_id = 100
header.peer.MergeFrom(peer)
header.uuid = uuid.uuid1().bytes
header.region_epoch.MergeFrom(region_epoch)
header.term = 6

print header
#enum CmdType {
#    Invalid     = 0;
#    Get         = 1;
#    Put         = 3;
#    Delete      = 4;
#    Snap        = 5;
#}
#message GetRequest {
#    optional string cf  = 1;
#    optional bytes  key = 2;
#}
get_req = raft_cmdpb_pb2.GetRequest()
get_req.key = "zhaizhiqiang".encode("utf8")

#message PutRequest {
#    optional string cf    = 1;
#    optional bytes  key   = 2;
#    optional bytes  value = 3;
#}

put_req = raft_cmdpb_pb2.PutRequest()
put_req.key = "zhaizhiqiang".encode("utf8")
put_req.value = "wangqing08".encode("utf8")

#message DeleteRequest {
#    optional string cf  = 1;
#    optional bytes  key = 2;
#}

del_req = raft_cmdpb_pb2.DeleteRequest()
del_req.key = "zhaizhiqiang".encode("utf8")

#message Request {
#    optional CmdType        cmd_type    = 1;
#    optional GetRequest     get         = 2;
#    optional PutRequest     put         = 4;
#    optional DeleteRequest  delete      = 5;
#    optional SnapRequest    snap        = 6;
#}
req = raft_cmdpb_pb2.Request()

#1.Get
req.cmd_type = raft_cmdpb_pb2.Get
req.get.MergeFrom(get_req)

#2.Put
#req.cmd_type = raft_cmdpb_pb2.Put
#req.put.MergeFrom(put_req)

#3.Delete
#req.cmd_type = raft_cmdpb_pb2.Delete
#req.delete.MergeFrom(del_req)

#message RaftCmdRequest {
#    optional RaftRequestHeader header     = 1;
#    repeated Request requests             = 2; 
#    optional AdminRequest admin_request   = 3; 
#    optional StatusRequest status_request = 4;
#}
cmd = raft_cmdpb_pb2.RaftCmdRequest()
cmd.header.MergeFrom(header)
cmd_req = cmd.requests.add()
cmd_req.MergeFrom(req)


#|0xdaf4(2 bytes magic value) | 0x01(version 2 bytes) | msg_len(4 bytes) | msg_id(8 bytes) |
message = msgpb_pb2.Message()
message.msg_type = msgpb_pb2.Cmd
message.cmd_req.MergeFrom(cmd)

print message

body_bytes = message.SerializeToString()


magic_num = 56052
version = 0x01
msg_len = message.ByteSize()
msg_id = 0x1
header_bytes = struct.pack('!2HIq', magic_num, version, msg_len, msg_id)


host = "127.0.0.1"
port = server_array[index][2]

#port = 20160

ip_port = (host, port)
sk = socket.socket()
sk.connect(ip_port)

sk.sendall(header_bytes)
sk.sendall(body_bytes)
server_reply = sk.recv(1024)

resp_header_bytes = server_reply[0:16]
resp_magic_num, resp_version, resp_msg_len, resp_msg_id = struct.unpack('!2HIq', resp_header_bytes)
print("response, magic_num:[%d] version:[%d] msg_len:[%d] msg_id:[%d]"%(resp_magic_num, resp_version, resp_msg_len, resp_msg_id))
resp_body_bytes = server_reply[16:]
print len(resp_body_bytes)

resp_message = msgpb_pb2.Message()
resp_message.ParseFromString(resp_body_bytes)
print resp_message

sk.close()
