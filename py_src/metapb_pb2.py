# Generated by the protocol buffer compiler.  DO NOT EDIT!
# source: metapb.proto

import sys
_b=sys.version_info[0]<3 and (lambda x:x) or (lambda x:x.encode('latin1'))
from google.protobuf.internal import enum_type_wrapper
from google.protobuf import descriptor as _descriptor
from google.protobuf import message as _message
from google.protobuf import reflection as _reflection
from google.protobuf import symbol_database as _symbol_database
from google.protobuf import descriptor_pb2
# @@protoc_insertion_point(imports)

_sym_db = _symbol_database.Default()




DESCRIPTOR = _descriptor.FileDescriptor(
  name='metapb.proto',
  package='metapb',
  syntax='proto2',
  serialized_pb=_b('\n\x0cmetapb.proto\x12\x06metapb\"-\n\x07\x43luster\x12\n\n\x02id\x18\x01 \x01(\x04\x12\x16\n\x0emax_peer_count\x18\x02 \x01(\r\"(\n\nStoreLabel\x12\x0b\n\x03key\x18\x01 \x01(\t\x12\r\n\x05value\x18\x02 \x01(\t\"k\n\x05Store\x12\n\n\x02id\x18\x01 \x01(\x04\x12\x0f\n\x07\x61\x64\x64ress\x18\x02 \x01(\t\x12!\n\x05state\x18\x03 \x01(\x0e\x32\x12.metapb.StoreState\x12\"\n\x06labels\x18\x04 \x03(\x0b\x32\x12.metapb.StoreLabel\"0\n\x0bRegionEpoch\x12\x10\n\x08\x63onf_ver\x18\x01 \x01(\x04\x12\x0f\n\x07version\x18\x02 \x01(\x04\"\x8e\x01\n\x06Region\x12\n\n\x02id\x18\x01 \x01(\x04\x12\x11\n\tstart_key\x18\x02 \x01(\x0c\x12\x0f\n\x07\x65nd_key\x18\x03 \x01(\x0c\x12)\n\x0cregion_epoch\x18\x04 \x01(\x0b\x32\x13.metapb.RegionEpoch\x12\x1b\n\x05peers\x18\x05 \x03(\x0b\x32\x0c.metapb.Peer\x12\x0c\n\x04term\x18\x06 \x01(\x04\"$\n\x04Peer\x12\n\n\x02id\x18\x01 \x01(\x04\x12\x10\n\x08store_id\x18\x02 \x01(\x04*0\n\nStoreState\x12\x06\n\x02Up\x10\x00\x12\x0b\n\x07Offline\x10\x01\x12\r\n\tTombstone\x10\x02')
)
_sym_db.RegisterFileDescriptor(DESCRIPTOR)

_STORESTATE = _descriptor.EnumDescriptor(
  name='StoreState',
  full_name='metapb.StoreState',
  filename=None,
  file=DESCRIPTOR,
  values=[
    _descriptor.EnumValueDescriptor(
      name='Up', index=0, number=0,
      options=None,
      type=None),
    _descriptor.EnumValueDescriptor(
      name='Offline', index=1, number=1,
      options=None,
      type=None),
    _descriptor.EnumValueDescriptor(
      name='Tombstone', index=2, number=2,
      options=None,
      type=None),
  ],
  containing_type=None,
  options=None,
  serialized_start=455,
  serialized_end=503,
)
_sym_db.RegisterEnumDescriptor(_STORESTATE)

StoreState = enum_type_wrapper.EnumTypeWrapper(_STORESTATE)
Up = 0
Offline = 1
Tombstone = 2



_CLUSTER = _descriptor.Descriptor(
  name='Cluster',
  full_name='metapb.Cluster',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  fields=[
    _descriptor.FieldDescriptor(
      name='id', full_name='metapb.Cluster.id', index=0,
      number=1, type=4, cpp_type=4, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      options=None),
    _descriptor.FieldDescriptor(
      name='max_peer_count', full_name='metapb.Cluster.max_peer_count', index=1,
      number=2, type=13, cpp_type=3, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      options=None),
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
  ],
  options=None,
  is_extendable=False,
  syntax='proto2',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=24,
  serialized_end=69,
)


_STORELABEL = _descriptor.Descriptor(
  name='StoreLabel',
  full_name='metapb.StoreLabel',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  fields=[
    _descriptor.FieldDescriptor(
      name='key', full_name='metapb.StoreLabel.key', index=0,
      number=1, type=9, cpp_type=9, label=1,
      has_default_value=False, default_value=_b("").decode('utf-8'),
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      options=None),
    _descriptor.FieldDescriptor(
      name='value', full_name='metapb.StoreLabel.value', index=1,
      number=2, type=9, cpp_type=9, label=1,
      has_default_value=False, default_value=_b("").decode('utf-8'),
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      options=None),
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
  ],
  options=None,
  is_extendable=False,
  syntax='proto2',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=71,
  serialized_end=111,
)


_STORE = _descriptor.Descriptor(
  name='Store',
  full_name='metapb.Store',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  fields=[
    _descriptor.FieldDescriptor(
      name='id', full_name='metapb.Store.id', index=0,
      number=1, type=4, cpp_type=4, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      options=None),
    _descriptor.FieldDescriptor(
      name='address', full_name='metapb.Store.address', index=1,
      number=2, type=9, cpp_type=9, label=1,
      has_default_value=False, default_value=_b("").decode('utf-8'),
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      options=None),
    _descriptor.FieldDescriptor(
      name='state', full_name='metapb.Store.state', index=2,
      number=3, type=14, cpp_type=8, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      options=None),
    _descriptor.FieldDescriptor(
      name='labels', full_name='metapb.Store.labels', index=3,
      number=4, type=11, cpp_type=10, label=3,
      has_default_value=False, default_value=[],
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      options=None),
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
  ],
  options=None,
  is_extendable=False,
  syntax='proto2',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=113,
  serialized_end=220,
)


_REGIONEPOCH = _descriptor.Descriptor(
  name='RegionEpoch',
  full_name='metapb.RegionEpoch',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  fields=[
    _descriptor.FieldDescriptor(
      name='conf_ver', full_name='metapb.RegionEpoch.conf_ver', index=0,
      number=1, type=4, cpp_type=4, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      options=None),
    _descriptor.FieldDescriptor(
      name='version', full_name='metapb.RegionEpoch.version', index=1,
      number=2, type=4, cpp_type=4, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      options=None),
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
  ],
  options=None,
  is_extendable=False,
  syntax='proto2',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=222,
  serialized_end=270,
)


_REGION = _descriptor.Descriptor(
  name='Region',
  full_name='metapb.Region',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  fields=[
    _descriptor.FieldDescriptor(
      name='id', full_name='metapb.Region.id', index=0,
      number=1, type=4, cpp_type=4, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      options=None),
    _descriptor.FieldDescriptor(
      name='start_key', full_name='metapb.Region.start_key', index=1,
      number=2, type=12, cpp_type=9, label=1,
      has_default_value=False, default_value=_b(""),
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      options=None),
    _descriptor.FieldDescriptor(
      name='end_key', full_name='metapb.Region.end_key', index=2,
      number=3, type=12, cpp_type=9, label=1,
      has_default_value=False, default_value=_b(""),
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      options=None),
    _descriptor.FieldDescriptor(
      name='region_epoch', full_name='metapb.Region.region_epoch', index=3,
      number=4, type=11, cpp_type=10, label=1,
      has_default_value=False, default_value=None,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      options=None),
    _descriptor.FieldDescriptor(
      name='peers', full_name='metapb.Region.peers', index=4,
      number=5, type=11, cpp_type=10, label=3,
      has_default_value=False, default_value=[],
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      options=None),
    _descriptor.FieldDescriptor(
      name='term', full_name='metapb.Region.term', index=5,
      number=6, type=4, cpp_type=4, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      options=None),
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
  ],
  options=None,
  is_extendable=False,
  syntax='proto2',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=273,
  serialized_end=415,
)


_PEER = _descriptor.Descriptor(
  name='Peer',
  full_name='metapb.Peer',
  filename=None,
  file=DESCRIPTOR,
  containing_type=None,
  fields=[
    _descriptor.FieldDescriptor(
      name='id', full_name='metapb.Peer.id', index=0,
      number=1, type=4, cpp_type=4, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      options=None),
    _descriptor.FieldDescriptor(
      name='store_id', full_name='metapb.Peer.store_id', index=1,
      number=2, type=4, cpp_type=4, label=1,
      has_default_value=False, default_value=0,
      message_type=None, enum_type=None, containing_type=None,
      is_extension=False, extension_scope=None,
      options=None),
  ],
  extensions=[
  ],
  nested_types=[],
  enum_types=[
  ],
  options=None,
  is_extendable=False,
  syntax='proto2',
  extension_ranges=[],
  oneofs=[
  ],
  serialized_start=417,
  serialized_end=453,
)

_STORE.fields_by_name['state'].enum_type = _STORESTATE
_STORE.fields_by_name['labels'].message_type = _STORELABEL
_REGION.fields_by_name['region_epoch'].message_type = _REGIONEPOCH
_REGION.fields_by_name['peers'].message_type = _PEER
DESCRIPTOR.message_types_by_name['Cluster'] = _CLUSTER
DESCRIPTOR.message_types_by_name['StoreLabel'] = _STORELABEL
DESCRIPTOR.message_types_by_name['Store'] = _STORE
DESCRIPTOR.message_types_by_name['RegionEpoch'] = _REGIONEPOCH
DESCRIPTOR.message_types_by_name['Region'] = _REGION
DESCRIPTOR.message_types_by_name['Peer'] = _PEER
DESCRIPTOR.enum_types_by_name['StoreState'] = _STORESTATE

Cluster = _reflection.GeneratedProtocolMessageType('Cluster', (_message.Message,), dict(
  DESCRIPTOR = _CLUSTER,
  __module__ = 'metapb_pb2'
  # @@protoc_insertion_point(class_scope:metapb.Cluster)
  ))
_sym_db.RegisterMessage(Cluster)

StoreLabel = _reflection.GeneratedProtocolMessageType('StoreLabel', (_message.Message,), dict(
  DESCRIPTOR = _STORELABEL,
  __module__ = 'metapb_pb2'
  # @@protoc_insertion_point(class_scope:metapb.StoreLabel)
  ))
_sym_db.RegisterMessage(StoreLabel)

Store = _reflection.GeneratedProtocolMessageType('Store', (_message.Message,), dict(
  DESCRIPTOR = _STORE,
  __module__ = 'metapb_pb2'
  # @@protoc_insertion_point(class_scope:metapb.Store)
  ))
_sym_db.RegisterMessage(Store)

RegionEpoch = _reflection.GeneratedProtocolMessageType('RegionEpoch', (_message.Message,), dict(
  DESCRIPTOR = _REGIONEPOCH,
  __module__ = 'metapb_pb2'
  # @@protoc_insertion_point(class_scope:metapb.RegionEpoch)
  ))
_sym_db.RegisterMessage(RegionEpoch)

Region = _reflection.GeneratedProtocolMessageType('Region', (_message.Message,), dict(
  DESCRIPTOR = _REGION,
  __module__ = 'metapb_pb2'
  # @@protoc_insertion_point(class_scope:metapb.Region)
  ))
_sym_db.RegisterMessage(Region)

Peer = _reflection.GeneratedProtocolMessageType('Peer', (_message.Message,), dict(
  DESCRIPTOR = _PEER,
  __module__ = 'metapb_pb2'
  # @@protoc_insertion_point(class_scope:metapb.Peer)
  ))
_sym_db.RegisterMessage(Peer)


# @@protoc_insertion_point(module_scope)
