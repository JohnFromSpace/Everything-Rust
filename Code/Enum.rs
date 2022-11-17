enum IpAddressVersion {
  V4, 
  V6,
}

struct IpAddress {
  version: IpAddressVersion,
}

// we can call this function with either variant of "IpAddress"
fn route(ip_address_version: IpAddressVersion) {}

fn main() {
  // we can create instances of the "enum" class
  let v_four = IpAddressVersion::V4;
  let v_six = IpAddressVersio::V6;
  
  route(IpAddressVersion::V4);
  route(v_six); // same as "route(IpAddressVersion::V6);"
}
