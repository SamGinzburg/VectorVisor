package main

import jsoniter "github.com/json-iterator/tinygo"

type Payload_json struct {
}
func (json Payload_json) Type() interface{} {
  var val Payload
  return val
}
func (json Payload_json) Unmarshal(iter *jsoniter.Iterator, out interface{}) {
  Payload_json_unmarshal(iter, out.(*Payload))
}
func (json Payload_json) Marshal(stream *jsoniter.Stream, val interface{}) {
  Payload_json_marshal(stream, val.(Payload))
}
func Payload_json_unmarshal(iter *jsoniter.Iterator, out *Payload) {
  more := iter.ReadObjectHead()
  for more {
    field := iter.ReadObjectField()
    if !Payload_json_unmarshal_field(iter, field, out) {
      iter.Skip()
    }
    more = iter.ReadObjectMore()
  }
}
func Payload_json_unmarshal_field(iter *jsoniter.Iterator, field string, out *Payload) bool {
  switch {
  case field == `tweets`:
    Payload_array1_json_unmarshal(iter, &(*out).Tweets)
    return true
  }
  return false
}
func Payload_array1_json_unmarshal (iter *jsoniter.Iterator, out *[]string) {
  i := 0
  val := *out
  more := iter.ReadArrayHead()
  for more {
    if i == len(val) {
      val = append(val, make([]string, 4)...)
    }
    iter.ReadString(&val[i])
    i++
    more = iter.ReadArrayMore()
  }
  if i == 0 {
    *out = []string{}
  } else {
    *out = val[:i]
  }
}
func Payload_json_marshal(stream *jsoniter.Stream, val Payload) {
    stream.WriteObjectHead()
    Payload_json_marshal_field(stream, val)
    stream.WriteObjectTail()
}
func Payload_json_marshal_field(stream *jsoniter.Stream, val Payload) {
    stream.WriteObjectField(`tweets`)
    Payload_array2_json_marshal(stream, val.Tweets)
    stream.WriteMore()
}
func Payload_array2_json_marshal (stream *jsoniter.Stream, val []string) {
  if len(val) == 0 {
    stream.WriteEmptyArray()
  } else {
    stream.WriteArrayHead()
    for i, elem := range val {
      if i != 0 { stream.WriteMore() }
    stream.WriteString(elem)
    }
    stream.WriteArrayTail()
  }
}
