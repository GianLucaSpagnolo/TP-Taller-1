use crate::control_packets::mqtt_unsubscribe::unsubscribe_properties::_UnsubcribeProperties;

/// ## UNSUBSCRIBE PACKET (Enviado del cliente al servidor)
/// 
/// ### FIXED HEADER: 2 BYTES
/// 
/// Primer Byte:
/// 4 bits mas significativos: MQTT Control Packet Type 
/// 
/// Segundo Byte:
/// Remaining Length
/// El Remaining Length es el número de bytes que quedan en el paquete después del Fixed Header y 
/// contiene el Variable Header y el Payload.
/// 
/// ### VARIABLE HEADER: 
/// PACKER IDENTIFIER: 2 BYTES
/// 
/// Property lenght: Variable Byte Integer 
/// 
/// Properties: Unsubcribe 
/// 
/// 38 - 0x26: User Property - UTF-8 String Pair
/// 
/// ### PAYLOAD:
/// 
/// Contiene una lista de Topic Filters de los cuales el cliente se quiere
/// desuscribir. El Topic Filter DEBEN ser Strings UTF-8 válidos.
/// 
/// El packet unsubscribe DEBE contener AL MENOS un Topic Filter.
/// Un unsubscribe packet sin PAYLOAD es un Protocol Error.
///
/// 
/// ### Consideraciones 
/// El topic filter incluido en un unsubscribe packet DEBE ser comparado caracter a 
/// caracter con el set actual de Topic Filters guardado por el Servidor 
/// para el Cliente. Si cualquier filtro matchea exactamanete con un Topic Filter que el servidor
/// contenga, entonces esa subscripción DEBE ser eliminada. Caso contrario,
/// no ocurre procesamiento adicional
/// 

pub struct _Unsubcribe{
    pub properties: _UnsubcribeProperties,
}

