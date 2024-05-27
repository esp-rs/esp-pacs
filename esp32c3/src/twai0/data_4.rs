///Register `DATA_4` reader
pub type R = crate::R<DATA_4_SPEC>;
///Register `DATA_4` writer
pub type W = crate::W<DATA_4_SPEC>;
///Field `TX_BYTE_4` reader - In reset mode, it is acceptance code register 4 with R/W Permission. In operation mode, it stores the 4th byte of the data to be transmitted or received. In operation mode, writing writes to the transmit buffer while reading reads from the receive buffer.
pub type TX_BYTE_4_R = crate::FieldReader;
///Field `TX_BYTE_4` writer - In reset mode, it is acceptance code register 4 with R/W Permission. In operation mode, it stores the 4th byte of the data to be transmitted or received. In operation mode, writing writes to the transmit buffer while reading reads from the receive buffer.
pub type TX_BYTE_4_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - In reset mode, it is acceptance code register 4 with R/W Permission. In operation mode, it stores the 4th byte of the data to be transmitted or received. In operation mode, writing writes to the transmit buffer while reading reads from the receive buffer.
    #[inline(always)]
    pub fn tx_byte_4(&self) -> TX_BYTE_4_R {
        TX_BYTE_4_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DATA_4")
            .field("tx_byte_4", &self.tx_byte_4())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - In reset mode, it is acceptance code register 4 with R/W Permission. In operation mode, it stores the 4th byte of the data to be transmitted or received. In operation mode, writing writes to the transmit buffer while reading reads from the receive buffer.
    #[inline(always)]
    #[must_use]
    pub fn tx_byte_4(&mut self) -> TX_BYTE_4_W<DATA_4_SPEC> {
        TX_BYTE_4_W::new(self, 0)
    }
}
/**Data register 4

You can [`read`](crate::generic::Reg::read) this register and get [`data_4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data_4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DATA_4_SPEC;
impl crate::RegisterSpec for DATA_4_SPEC {
    type Ux = u32;
}
///`read()` method returns [`data_4::R`](R) reader structure
impl crate::Readable for DATA_4_SPEC {}
///`write(|w| ..)` method takes [`data_4::W`](W) writer structure
impl crate::Writable for DATA_4_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DATA_4 to value 0
impl crate::Resettable for DATA_4_SPEC {
    const RESET_VALUE: u32 = 0;
}
