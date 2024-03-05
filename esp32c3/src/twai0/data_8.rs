#[doc = "Register `DATA_8` reader"]
pub type R = crate::R<DATA_8_SPEC>;
#[doc = "Register `DATA_8` writer"]
pub type W = crate::W<DATA_8_SPEC>;
#[doc = "Field `TX_BYTE_8` reader - In operation mode, it stores the 8th byte of the data to be transmitted or received. In operation mode, writing writes to the transmit buffer while reading reads from the receive buffer."]
pub type TX_BYTE_8_R = crate::FieldReader;
#[doc = "Field `TX_BYTE_8` writer - In operation mode, it stores the 8th byte of the data to be transmitted or received. In operation mode, writing writes to the transmit buffer while reading reads from the receive buffer."]
pub type TX_BYTE_8_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - In operation mode, it stores the 8th byte of the data to be transmitted or received. In operation mode, writing writes to the transmit buffer while reading reads from the receive buffer."]
    #[inline(always)]
    pub fn tx_byte_8(&self) -> TX_BYTE_8_R {
        TX_BYTE_8_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DATA_8")
            .field("tx_byte_8", &format_args!("{}", self.tx_byte_8().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DATA_8_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - In operation mode, it stores the 8th byte of the data to be transmitted or received. In operation mode, writing writes to the transmit buffer while reading reads from the receive buffer."]
    #[inline(always)]
    #[must_use]
    pub fn tx_byte_8(&mut self) -> TX_BYTE_8_W<DATA_8_SPEC> {
        TX_BYTE_8_W::new(self, 0)
    }
}
#[doc = "Data register 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data_8::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data_8::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATA_8_SPEC;
impl crate::RegisterSpec for DATA_8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data_8::R`](R) reader structure"]
impl crate::Readable for DATA_8_SPEC {}
#[doc = "`write(|w| ..)` method takes [`data_8::W`](W) writer structure"]
impl crate::Writable for DATA_8_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DATA_8 to value 0"]
impl crate::Resettable for DATA_8_SPEC {
    const RESET_VALUE: u32 = 0;
}
