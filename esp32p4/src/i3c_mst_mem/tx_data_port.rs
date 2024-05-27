#[doc = "Register `TX_DATA_PORT` reader"]
pub type R = crate::R<TX_DATA_PORT_SPEC>;
#[doc = "Register `TX_DATA_PORT` writer"]
pub type W = crate::W<TX_DATA_PORT_SPEC>;
#[doc = "Field `REG_TX_DATA_PORT` reader - Transmit Data Port. Transmit data is mapped to the Tx-data buffer and transmit data is always packed in 4-byte aligned data words. If the length of data transfer is not aligned to 4-bytes boundary, then there will be extra(unused) bytes(the additional data bytes have to be ignored) at the end of the transferred data. The valid data must be identified using the DATA_LENGTH filed in the Response Descriptor."]
pub type REG_TX_DATA_PORT_R = crate::FieldReader<u32>;
#[doc = "Field `REG_TX_DATA_PORT` writer - Transmit Data Port. Transmit data is mapped to the Tx-data buffer and transmit data is always packed in 4-byte aligned data words. If the length of data transfer is not aligned to 4-bytes boundary, then there will be extra(unused) bytes(the additional data bytes have to be ignored) at the end of the transferred data. The valid data must be identified using the DATA_LENGTH filed in the Response Descriptor."]
pub type REG_TX_DATA_PORT_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Transmit Data Port. Transmit data is mapped to the Tx-data buffer and transmit data is always packed in 4-byte aligned data words. If the length of data transfer is not aligned to 4-bytes boundary, then there will be extra(unused) bytes(the additional data bytes have to be ignored) at the end of the transferred data. The valid data must be identified using the DATA_LENGTH filed in the Response Descriptor."]
    #[inline(always)]
    pub fn reg_tx_data_port(&self) -> REG_TX_DATA_PORT_R {
        REG_TX_DATA_PORT_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TX_DATA_PORT")
            .field("reg_tx_data_port", &self.reg_tx_data_port())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Transmit Data Port. Transmit data is mapped to the Tx-data buffer and transmit data is always packed in 4-byte aligned data words. If the length of data transfer is not aligned to 4-bytes boundary, then there will be extra(unused) bytes(the additional data bytes have to be ignored) at the end of the transferred data. The valid data must be identified using the DATA_LENGTH filed in the Response Descriptor."]
    #[inline(always)]
    #[must_use]
    pub fn reg_tx_data_port(&mut self) -> REG_TX_DATA_PORT_W<TX_DATA_PORT_SPEC> {
        REG_TX_DATA_PORT_W::new(self, 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_data_port::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_data_port::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX_DATA_PORT_SPEC;
impl crate::RegisterSpec for TX_DATA_PORT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_data_port::R`](R) reader structure"]
impl crate::Readable for TX_DATA_PORT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tx_data_port::W`](W) writer structure"]
impl crate::Writable for TX_DATA_PORT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TX_DATA_PORT to value 0"]
impl crate::Resettable for TX_DATA_PORT_SPEC {
    const RESET_VALUE: u32 = 0;
}
