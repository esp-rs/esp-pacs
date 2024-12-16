#[doc = "Register `TMOUT` reader"]
pub type R = crate::R<TMOUT_SPEC>;
#[doc = "Register `TMOUT` writer"]
pub type W = crate::W<TMOUT_SPEC>;
#[doc = "Field `RESPONSE_TIMEOUT` reader - Response timeout value. Value is specified in terms of number of card output clocks, i.e., sdhost_cclk_out."]
pub type RESPONSE_TIMEOUT_R = crate::FieldReader;
#[doc = "Field `RESPONSE_TIMEOUT` writer - Response timeout value. Value is specified in terms of number of card output clocks, i.e., sdhost_cclk_out."]
pub type RESPONSE_TIMEOUT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DATA_TIMEOUT` reader - Value for card data read timeout. This value is also used for data starvation by host timeout. The timeout counter is started only after the card clock is stopped. This value is specified in number of card output clocks, i.e. sdhost_cclk_out of the selected card. NOTE: The software timer should be used if the timeout value is in the order of 100 ms. In this case, read data timeout interrupt needs to be disabled."]
pub type DATA_TIMEOUT_R = crate::FieldReader<u32>;
#[doc = "Field `DATA_TIMEOUT` writer - Value for card data read timeout. This value is also used for data starvation by host timeout. The timeout counter is started only after the card clock is stopped. This value is specified in number of card output clocks, i.e. sdhost_cclk_out of the selected card. NOTE: The software timer should be used if the timeout value is in the order of 100 ms. In this case, read data timeout interrupt needs to be disabled."]
pub type DATA_TIMEOUT_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - Response timeout value. Value is specified in terms of number of card output clocks, i.e., sdhost_cclk_out."]
    #[inline(always)]
    pub fn response_timeout(&self) -> RESPONSE_TIMEOUT_R {
        RESPONSE_TIMEOUT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - Value for card data read timeout. This value is also used for data starvation by host timeout. The timeout counter is started only after the card clock is stopped. This value is specified in number of card output clocks, i.e. sdhost_cclk_out of the selected card. NOTE: The software timer should be used if the timeout value is in the order of 100 ms. In this case, read data timeout interrupt needs to be disabled."]
    #[inline(always)]
    pub fn data_timeout(&self) -> DATA_TIMEOUT_R {
        DATA_TIMEOUT_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TMOUT")
            .field("response_timeout", &self.response_timeout())
            .field("data_timeout", &self.data_timeout())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Response timeout value. Value is specified in terms of number of card output clocks, i.e., sdhost_cclk_out."]
    #[inline(always)]
    pub fn response_timeout(&mut self) -> RESPONSE_TIMEOUT_W<TMOUT_SPEC> {
        RESPONSE_TIMEOUT_W::new(self, 0)
    }
    #[doc = "Bits 8:31 - Value for card data read timeout. This value is also used for data starvation by host timeout. The timeout counter is started only after the card clock is stopped. This value is specified in number of card output clocks, i.e. sdhost_cclk_out of the selected card. NOTE: The software timer should be used if the timeout value is in the order of 100 ms. In this case, read data timeout interrupt needs to be disabled."]
    #[inline(always)]
    pub fn data_timeout(&mut self) -> DATA_TIMEOUT_W<TMOUT_SPEC> {
        DATA_TIMEOUT_W::new(self, 8)
    }
}
#[doc = "Data and response timeout configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`tmout::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmout::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TMOUT_SPEC;
impl crate::RegisterSpec for TMOUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tmout::R`](R) reader structure"]
impl crate::Readable for TMOUT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tmout::W`](W) writer structure"]
impl crate::Writable for TMOUT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TMOUT to value 0xffff_ff40"]
impl crate::Resettable for TMOUT_SPEC {
    const RESET_VALUE: u32 = 0xffff_ff40;
}
