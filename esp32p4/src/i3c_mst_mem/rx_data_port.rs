#[doc = "Register `RX_DATA_PORT` reader"]
pub type R = crate::R<RX_DATA_PORT_SPEC>;
#[doc = "Field `RX_DATA_PORT` reader - Receive Data Port. Receive data is mapped to the Rx-data buffer and receive data is always packed in 4-byte aligned data words. If the length of data transfer is not aligned to 4-bytes boundary, then there will be extra(unused) bytes(the additional data bytes have to be ignored) at the end of the transferred data. The valid data must be identified using the DATA_LENGTH filed in the Response Descriptor."]
pub type RX_DATA_PORT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Receive Data Port. Receive data is mapped to the Rx-data buffer and receive data is always packed in 4-byte aligned data words. If the length of data transfer is not aligned to 4-bytes boundary, then there will be extra(unused) bytes(the additional data bytes have to be ignored) at the end of the transferred data. The valid data must be identified using the DATA_LENGTH filed in the Response Descriptor."]
    #[inline(always)]
    pub fn rx_data_port(&self) -> RX_DATA_PORT_R {
        RX_DATA_PORT_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_DATA_PORT")
            .field(
                "rx_data_port",
                &format_args!("{}", self.rx_data_port().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RX_DATA_PORT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rx_data_port::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_DATA_PORT_SPEC;
impl crate::RegisterSpec for RX_DATA_PORT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_data_port::R`](R) reader structure"]
impl crate::Readable for RX_DATA_PORT_SPEC {}
#[doc = "`reset()` method sets RX_DATA_PORT to value 0"]
impl crate::Resettable for RX_DATA_PORT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
