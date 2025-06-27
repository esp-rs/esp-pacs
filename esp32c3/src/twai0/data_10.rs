#[doc = "Register `DATA_10` reader"]
pub type R = crate::R<DATA_10_SPEC>;
#[doc = "Register `DATA_10` writer"]
pub type W = crate::W<DATA_10_SPEC>;
#[doc = "Field `TX_BYTE_10` reader - In operation mode, it stores the 10th byte of the data to be transmitted or received. In operation mode, writing writes to the transmit buffer while reading reads from the receive buffer."]
pub type TX_BYTE_10_R = crate::FieldReader;
#[doc = "Field `TX_BYTE_10` writer - In operation mode, it stores the 10th byte of the data to be transmitted or received. In operation mode, writing writes to the transmit buffer while reading reads from the receive buffer."]
pub type TX_BYTE_10_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - In operation mode, it stores the 10th byte of the data to be transmitted or received. In operation mode, writing writes to the transmit buffer while reading reads from the receive buffer."]
    #[inline(always)]
    pub fn tx_byte_10(&self) -> TX_BYTE_10_R {
        TX_BYTE_10_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DATA_10")
            .field("tx_byte_10", &self.tx_byte_10())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - In operation mode, it stores the 10th byte of the data to be transmitted or received. In operation mode, writing writes to the transmit buffer while reading reads from the receive buffer."]
    #[inline(always)]
    pub fn tx_byte_10(&mut self) -> TX_BYTE_10_W<DATA_10_SPEC> {
        TX_BYTE_10_W::new(self, 0)
    }
}
#[doc = "Data register 10\n\nYou can [`read`](crate::Reg::read) this register and get [`data_10::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data_10::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATA_10_SPEC;
impl crate::RegisterSpec for DATA_10_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data_10::R`](R) reader structure"]
impl crate::Readable for DATA_10_SPEC {}
#[doc = "`write(|w| ..)` method takes [`data_10::W`](W) writer structure"]
impl crate::Writable for DATA_10_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATA_10 to value 0"]
impl crate::Resettable for DATA_10_SPEC {}
