use std::net::UdpSocket;

#[derive(Debug)]
#[allow(dead_code)]
enum AddressAndPortError
{
  Ipv6NotSupported,
  InvalidValidAddress,
  Ipv4ReservedAddress,
  InvalidPort,
  ReservedPort,
  UnexpectedError,
  StdError(std::io::Error),
}

impl From<std::io::Error> for AddressAndPortError
{
  fn from(err: std::io::Error) -> AddressAndPortError
  {
    return AddressAndPortError::StdError(err)
  }
}

type AddressAndPortResult = Result<(), AddressAndPortError>;

struct AddressAndPort
{
  address: [u8; 4],
  port: u16
}

#[allow(dead_code)]
impl AddressAndPort
{
  //Methods
  fn update_fields(&mut self, address_: &mut String, port_: &mut String) -> AddressAndPortResult
  {
    if address_.ends_with('\n')
    {
      address_.pop();
    }

    let address_ : Vec<&str> = address_.split('.').collect();
    if address_.len() == 6
    {
      return Err(AddressAndPortError::Ipv6NotSupported)
    }
    else if address_.len() != 4
    {
      return Err(AddressAndPortError::InvalidValidAddress)
    }

    let mut i = 0;
    for byte in address_
    {
      self.address[i] = match byte.parse::<u8>() {Ok(num) => num, Err(_) => return Err(AddressAndPortError::UnexpectedError)};
      i += 1;
    }
    
    if port_.ends_with('\n')
    {
      port_.pop();
    }

    self.port = match port_.parse::<u16>() {Ok(num) => num, Err(_) => return Err(AddressAndPortError::UnexpectedError)};

    Ok(())
  }

  fn socket(&self) -> Result<std::net::SocketAddr, AddressAndPortError>
  {
    let socket = std::net::SocketAddr::new(std::net::IpAddr::V4(std::net::Ipv4Addr::new(self.address[0], self.address[1], self.address[2], self.address[3])), self.port);

    if socket.is_ipv6()
    {
      return Err(AddressAndPortError::Ipv6NotSupported)
    }

    Ok(socket)
  }

  //Related functions/methods
  fn new(address_: &mut String, port_: &mut String) -> AddressAndPort
  {
    let mut address_and_port = AddressAndPort::create_empty();
    let result = address_and_port.update_fields(address_, port_);
    match result
    {
      Err(err) => println!("Updating the empty structure resulted in error: {:?}", err),
      Ok(data) => data,
    }
    
    return address_and_port
  }

  fn create(address_: [u8; 4], port_: u16) -> AddressAndPort
  {
    return AddressAndPort {address: address_, port: port_}
  }

  fn create_empty() -> AddressAndPort
  {
    AddressAndPort{address: [0; 4], port: 0}
  }
}

fn get_required_data_from_user(binding_data: &mut AddressAndPort) -> AddressAndPortResult
{
  let mut binding_addr = String::new();
  println!("Enter the binding address");
  std::io::stdin().read_line(&mut binding_addr)?;

  let mut binding_port = String::new();
  println!("Enter the binding port");
  std::io::stdin().read_line(&mut binding_port)?;

  binding_data.update_fields(&mut binding_addr, &mut binding_port)?;

  Ok(())
}

fn receive_data_using_udp(binding_data: AddressAndPort) -> AddressAndPortResult
{
  let binding_data =  binding_data.socket()?;
  let receiver_socket = UdpSocket::bind(binding_data)?;

  println!("ready to listen!! at {:?}", binding_data);

  let mut buf: [u8; 1024] = [0; 1024];
  loop
  {
    let (length, _sender_socket) = receiver_socket.recv_from(&mut buf)?;

    let buf = &mut buf[..length];
    if buf == "\n".as_bytes()
    {
      break;
    }
    else
    {
      //receiver_socket.send_to(buf, sender_socket)?;
      let mut data = match String::from_utf8(buf.to_vec()){ Ok(d) => d, Err(e) => e.to_string() };
      data.ends_with('\n');
      data.pop();
      println!("{:?}", data);
    }
  }

  Ok(())
}

fn main() 
{
  let mut binding_data = AddressAndPort::create_empty();

  let result = get_required_data_from_user(&mut binding_data);
  if result.is_err()
  {
    println!("Failed to get the data from user. Error is: {:?}", result);
    return ()
  }

  let result = receive_data_using_udp(binding_data);
  if result.is_err()
  {
    println!("Receiving data using UDP resulted in error: {:?}", result);
  }
}