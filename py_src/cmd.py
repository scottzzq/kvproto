# -*- coding:utf-8 -*-  
import raft_cmdpb_pb2
import metapb_pb2
import socket
import sys
import struct
import msgpb_pb2
import uuid
import pdpb_pb2

# 1、请求pd获取Region的信息
# 2、根据返回的Region的store_id, 请求pd获取Region的Leader地址
# 3、构造请求发送给Leader
############################连接pd
pd_host = "127.0.0.1"
pd_port = 2379

pd_ip_port = (pd_host, pd_port)
pd_sk = socket.socket()
pd_sk.connect(pd_ip_port)
pd_sk.sendall("GET /pd/rpc HTTP/1.0\r\n\r\n")

##################################请求pd header###############
# message RequestHeader {
#     // 16 bytes, to distinguish request.  
#     optional bytes uuid                = 1;
#     optional uint64 cluster_id         = 2 [(gogoproto.nullable) = false];
# }
pd_header = pdpb_pb2.RequestHeader()
pd_header.uuid = uuid.uuid1().bytes
pd_header.cluster_id = 1024

# message GetRegionByIDRequest {
#     optional uint64 region_id      = 1 [(gogoproto.nullable) = false];
# }
# message GetRegionResponse {
#     optional metapb.Region region   = 1;
#     optional metapb.Peer leader     = 2;
# }
pd_get_region_req = pdpb_pb2.GetRegionByIDRequest()
pd_get_region_req.region_id = 2

# enum CommandType {
#     Invalid             = 0;
#     Tso                 = 1;
#     Bootstrap           = 2;
#     IsBootstrapped      = 3;
#     AllocId             = 4;
#     GetStore            = 5;
#     PutStore            = 6;
#     AskSplit            = 7;
#     GetRegion           = 8;
#     RegionHeartbeat     = 9;
#     GetClusterConfig    = 10;
#     PutClusterConfig    = 11;
#     StoreHeartbeat      = 12;
#     ReportSplit         = 13;
#     GetRegionByID       = 14;
#     GetPDMembers        = 15;
# }
# message Request {
#     optional RequestHeader header                           = 1;
#     optional CommandType cmd_type                           = 2 [(gogoproto.nullable) = false];
#     optional TsoRequest tso                                 = 3;
#     optional BootstrapRequest bootstrap                     = 4;
#     optional IsBootstrappedRequest is_bootstrapped          = 5;
#     optional AllocIdRequest alloc_id                        = 6;
#     optional GetStoreRequest get_store                      = 7;
#     optional PutStoreRequest put_store                      = 8;
#     optional AskSplitRequest ask_split                      = 9;
#     optional GetRegionRequest get_region                    = 10;
#     optional RegionHeartbeatRequest region_heartbeat        = 11;
#     optional GetClusterConfigRequest get_cluster_config     = 12;
#     optional PutClusterConfigRequest put_cluster_config     = 13;
#     optional StoreHeartbeatRequest store_heartbeat          = 14;
#     optional ReportSplitRequest report_split                = 15;
#     optional GetRegionByIDRequest get_region_by_id          = 16;
#     optional GetPDMembersRequest get_pd_members             = 17;
# }
#######################获取Region的Leader
pd_req = pdpb_pb2.Request()
pd_req.cmd_type = pdpb_pb2.GetRegionByID
pd_req.header.MergeFrom(pd_header)
pd_req.get_region_by_id.MergeFrom(pd_get_region_req)

message = msgpb_pb2.Message()
message.msg_type = msgpb_pb2.PdReq
message.pd_req.MergeFrom(pd_req)
body_bytes = message.SerializeToString()

magic_num = 56052
version = 0x01
msg_len = message.ByteSize()
msg_id = 0x1
header_bytes = struct.pack('!2HIq', magic_num, version, msg_len, msg_id)

pd_sk.sendall(header_bytes)
pd_sk.sendall(body_bytes)

server_reply = pd_sk.recv(1024)
resp_header_bytes = server_reply[0:16]
resp_magic_num, resp_version, resp_msg_len, resp_msg_id = struct.unpack('!2HIq', resp_header_bytes)
resp_body_bytes = server_reply[16:]

resp_message = msgpb_pb2.Message()
resp_message.ParseFromString(resp_body_bytes)
print resp_message
# message RegionEpoch {
#     // Conf change version, auto increment when add or remove peer
#     optional uint64 conf_ver	= 1 [(gogoproto.nullable) = false];
#     // Region version, auto increment when split or merge
#     optional uint64 version     = 2 [(gogoproto.nullable) = false];
# }
# message Region {
#     optional uint64 id                  = 1 [(gogoproto.nullable) = false];
#     // Region key range [start_key, end_key).
#     optional bytes  start_key           = 2;
#     optional bytes  end_key             = 3;
#     optional RegionEpoch region_epoch   = 4;
#     repeated Peer   peers               = 5;
#     optional uint64 term                = 6;
# }
# message Peer {      
#     optional uint64 id          = 1 [(gogoproto.nullable) = false]; 
#     optional uint64 store_id    = 2 [(gogoproto.nullable) = false];
# }
# message GetRegionResponse {
#     optional metapb.Region region   = 1;
#     optional metapb.Peer leader     = 2;
# }
get_region_by_id_resp = resp_message.pd_resp.get_region_by_id


###################################获取store地址#####################
# message GetStoreRequest {
#     optional uint64 store_id       = 1 [(gogoproto.nullable) = false];
# }
# message GetStoreResponse {
#     optional metapb.Store store     = 1;
# }
# message Store {
#     optional uint64 id          = 1 [(gogoproto.nullable) = false];
#     optional string address     = 2 [(gogoproto.nullable) = false];
#     optional StoreState state   = 3 [(gogoproto.nullable) = false];
#     repeated StoreLabel labels  = 4;
#     // more attributes......
# }
get_store_req = pdpb_pb2.GetStoreRequest()
get_store_req.store_id = get_region_by_id_resp.leader.store_id

pd_req = pdpb_pb2.Request()
pd_req.cmd_type = pdpb_pb2.GetStore
pd_req.header.MergeFrom(pd_header)
pd_req.get_store.MergeFrom(get_store_req)

message = msgpb_pb2.Message()
message.msg_type = msgpb_pb2.PdReq
message.pd_req.MergeFrom(pd_req)
body_bytes = message.SerializeToString()

magic_num = 56052
version = 0x01
msg_len = message.ByteSize()
msg_id = 0x1
header_bytes = struct.pack('!2HIq', magic_num, version, msg_len, msg_id)

pd_sk.sendall(header_bytes)
pd_sk.sendall(body_bytes)

server_reply = pd_sk.recv(1024)
resp_header_bytes = server_reply[0:16]
resp_magic_num, resp_version, resp_msg_len, resp_msg_id = struct.unpack('!2HIq', resp_header_bytes)
resp_body_bytes = server_reply[16:]

resp_message = msgpb_pb2.Message()
resp_message.ParseFromString(resp_body_bytes)
get_store_resp = resp_message.pd_resp.get_store


###################################Raft命令Header####################
peer = get_region_by_id_resp.leader
region_epoch = get_region_by_id_resp.region.region_epoch
term = get_region_by_id_resp.region.term
#message RaftRequestHeader {
#    optional uint64 region_id                   = 1;
#    optional metapb.Peer peer                   = 2;
#    optional bool read_quorum                   = 3;
#    optional bytes uuid                         = 4;
#    optional metapb.RegionEpoch region_epoch    = 5;
#    optional uint64 term                        = 6;
#}

ip_port_str = get_store_resp.store.address
ip_port = (ip_port_str.split(":")[0], int(ip_port_str.split(":")[1]))
sk = socket.socket()
sk.connect(ip_port)

for i in range(1, 1000):
	header = raft_cmdpb_pb2.RaftRequestHeader()
	header.region_id = 2
	header.peer.MergeFrom(peer)
	header.uuid = uuid.uuid1().bytes
	header.region_epoch.MergeFrom(region_epoch)
	header.term = term
	#enum CmdType {
	#    Invalid     = 0;
	#    Get         = 1;
	#    Put         = 3;
	#    Delete      = 4;
	#    Snap        = 5;
	#}
	####################################查询#############################
	#message GetRequest {
	#    optional string cf  = 1;
	#    optional uint64  key = 2;
	#}
	get_req = raft_cmdpb_pb2.GetRequest()
	get_req.key = i
	####################################插入#############################
	#message PutRequest {
	#    optional string cf    = 1;
	#    optional uint64  key   = 2;
	#    optional bytes  value = 3;
	#}
	put_req = raft_cmdpb_pb2.PutRequest()
	put_req.key = i
	put_req.value = ("zhaizhiqiang" + str(i)).encode("utf8")
	#print("key:[%d] value:[%s]" % (i, put_req.value))
	####################################删除############################
	#message DeleteRequest {
	#    optional string cf  = 1;
	#    optional uint64  key = 2;
	#}
	
	del_req = raft_cmdpb_pb2.DeleteRequest()
	del_req.key = 2
	
	###########################构造请求#########################
	#message Request {
	#    optional CmdType        cmd_type    = 1;
	#    optional GetRequest     get         = 2;
	#    optional PutRequest     put         = 4;
	#    optional DeleteRequest  delete      = 5;
	#    optional SnapRequest    snap        = 6;
	#}
	req = raft_cmdpb_pb2.Request()
	
	# #1.Get
	req.cmd_type = raft_cmdpb_pb2.Get
	req.get.MergeFrom(get_req)
	
	# #2.Put
	req.cmd_type = raft_cmdpb_pb2.Put
	req.put.MergeFrom(put_req)
	
	# #3.Delete
	# #req.cmd_type = raft_cmdpb_pb2.Delete
	# #req.delete.MergeFrom(del_req)
	
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
	#Message holds all messages communicating with TiKV.
	
	# enum MessageType {
	#     None         = 0;
	#     Cmd          = 1;
	#     CmdResp      = 2;
	#     Raft         = 3;
	#     KvReq        = 4;
	#     KvResp       = 5;
	#     CopReq       = 6;
	#     CopResp      = 7;
	#     PdReq        = 8;
	#     PdResp       = 9;
	# }
	# message Message {
	#     optional MessageType                 msg_type  = 1 [(gogoproto.nullable) = false];
	#     optional raft_cmdpb.RaftCmdRequest   cmd_req   = 2;
	#     optional raft_cmdpb.RaftCmdResponse  cmd_resp  = 3;
	#     optional raft_serverpb.RaftMessage   raft      = 4;
	#     optional kvrpcpb.Request             kv_req    = 5;
	#     optional kvrpcpb.Response            kv_resp   = 6;
	#     optional coprocessor.Request         cop_req   = 7;
	#     optional coprocessor.Response        cop_resp  = 8;
	#     optional pdpb.Request                pd_req    = 9;
	#     optional pdpb.Response               pd_resp   = 10;
	# }
	message = msgpb_pb2.Message()
	message.msg_type = msgpb_pb2.Cmd
	message.cmd_req.MergeFrom(cmd)
	
	body_bytes = message.SerializeToString()
	
	magic_num = 56052
	version = 0x01
	msg_len = message.ByteSize()
	msg_id = 0x1
	header_bytes = struct.pack('!2HIq', magic_num, version, msg_len, msg_id)
	#print "Request:", message
	sk.sendall(header_bytes)
	sk.sendall(body_bytes)

	server_reply = ''
	while True:
		server_reply += sk.recv(1024)
		if len(server_reply) >= 16:
			break
	
	resp_header_bytes = server_reply[0:16]
	resp_magic_num, resp_version, resp_msg_len, resp_msg_id = struct.unpack('!2HIq', resp_header_bytes)
	while True:
		if resp_msg_len + 16 == len(server_reply):
			break
		server_reply += sk.recv(1024)

	resp_body_bytes = server_reply[16:]

	resp_message = msgpb_pb2.Message()
	resp_message.ParseFromString(resp_body_bytes)
	print resp_message
	
sk.close()
	
