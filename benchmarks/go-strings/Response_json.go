package main

import jsoniter "github.com/json-iterator/tinygo"

type Response_json struct {
}
func (json Response_json) Type() interface{} {
  var val Response
  return val
}
func (json Response_json) Unmarshal(iter *jsoniter.Iterator, out interface{}) {
  Response_json_unmarshal(iter, out.(*Response))
}
func (json Response_json) Marshal(stream *jsoniter.Stream, val interface{}) {
  Response_json_marshal(stream, val.(Response))
}
func Response_json_unmarshal(iter *jsoniter.Iterator, out *Response) {
  more := iter.ReadObjectHead()
  for more {
    field := iter.ReadObjectField()
    if !Response_json_unmarshal_field(iter, field, out) {
      iter.Skip()
    }
    more = iter.ReadObjectMore()
  }
}
func Response_json_unmarshal_field(iter *jsoniter.Iterator, field string, out *Response) bool {
  switch {
  case field == `Tokenized`:
    Response_array1_json_unmarshal(iter, &(*out).Tokenized)
    return true
  case field == `Hashtags`:
    Response_array3_json_unmarshal(iter, &(*out).Hashtags)
    return true
  }
  return false
}
func Response_array2_json_unmarshal (iter *jsoniter.Iterator, out *[]string) {
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
func Response_array1_json_unmarshal (iter *jsoniter.Iterator, out *[][]string) {
  i := 0
  val := *out
  more := iter.ReadArrayHead()
  for more {
    if i == len(val) {
      val = append(val, make([][]string, 4)...)
    }
    Response_array2_json_unmarshal(iter, &val[i])
    i++
    more = iter.ReadArrayMore()
  }
  if i == 0 {
    *out = [][]string{}
  } else {
    *out = val[:i]
  }
}
func Response_array4_json_unmarshal (iter *jsoniter.Iterator, out *[]string) {
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
func Response_array3_json_unmarshal (iter *jsoniter.Iterator, out *[][]string) {
  i := 0
  val := *out
  more := iter.ReadArrayHead()
  for more {
    if i == len(val) {
      val = append(val, make([][]string, 4)...)
    }
    Response_array4_json_unmarshal(iter, &val[i])
    i++
    more = iter.ReadArrayMore()
  }
  if i == 0 {
    *out = [][]string{}
  } else {
    *out = val[:i]
  }
}
func Response_json_marshal(stream *jsoniter.Stream, val Response) {
    stream.WriteObjectHead()
    Response_json_marshal_field(stream, val)
    stream.WriteObjectTail()
}
func Response_json_marshal_field(stream *jsoniter.Stream, val Response) {
    stream.WriteObjectField(`Tokenized`)
    Response_array5_json_marshal(stream, val.Tokenized)
    stream.WriteMore()
    stream.WriteObjectField(`Hashtags`)
    Response_array7_json_marshal(stream, val.Hashtags)
    stream.WriteMore()
}
func Response_array6_json_marshal (stream *jsoniter.Stream, val []string) {
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
func Response_array5_json_marshal (stream *jsoniter.Stream, val [][]string) {
  if len(val) == 0 {
    stream.WriteEmptyArray()
  } else {
    stream.WriteArrayHead()
    for i, elem := range val {
      if i != 0 { stream.WriteMore() }
    Response_array6_json_marshal(stream, elem)
    }
    stream.WriteArrayTail()
  }
}
func Response_array8_json_marshal (stream *jsoniter.Stream, val []string) {
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
func Response_array7_json_marshal (stream *jsoniter.Stream, val [][]string) {
  if len(val) == 0 {
    stream.WriteEmptyArray()
  } else {
    stream.WriteArrayHead()
    for i, elem := range val {
      if i != 0 { stream.WriteMore() }
    Response_array8_json_marshal(stream, elem)
    }
    stream.WriteArrayTail()
  }
}
