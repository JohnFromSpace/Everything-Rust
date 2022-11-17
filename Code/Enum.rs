enum IpAddress {
  V4, 
  V6,
}

// we can call this function with either variant of "IpAddress"
fn route(ip_address: IpAddress) {
    
}

fn main() {
  // we can create instances of the "enum" class
  let v_four = IpAddress::V4;
  let v_six = IpAddress::V6;
  
  route(IpAddress::V4);
  route(v_six); // same as "route(IpAddress::V6);"
}
