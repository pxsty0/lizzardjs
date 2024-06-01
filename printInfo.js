() => {
  let device = process.os;

  console.log(`işletim sistemi: ${device.type()}`);
  console.log(`işletim sistemi versiyonu: ${device.version()}`);
  console.log(`işletim sistemi bit : ${device.bitness()}`);
};
