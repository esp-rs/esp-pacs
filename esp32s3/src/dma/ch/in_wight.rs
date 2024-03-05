#[doc = "Register `IN_WIGHT` reader"]
pub type R = crate::R<IN_WIGHT_SPEC>;
#[doc = "Register `IN_WIGHT` writer"]
pub type W = crate::W<IN_WIGHT_SPEC>;
#[doc = "Field `RX_WEIGHT` reader - The weight of Rx channel 0."]
pub type RX_WEIGHT_R = crate::FieldReader;
#[doc = "Field `RX_WEIGHT` writer - The weight of Rx channel 0."]
pub type RX_WEIGHT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 8:11 - The weight of Rx channel 0."]
    #[inline(always)]
    pub fn rx_weight(&self) -> RX_WEIGHT_R {
        RX_WEIGHT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN_WIGHT")
            .field("rx_weight", &format_args!("{}", self.rx_weight().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IN_WIGHT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 8:11 - The weight of Rx channel 0."]
    #[inline(always)]
    #[must_use]
    pub fn rx_weight(&mut self) -> RX_WEIGHT_W<IN_WIGHT_SPEC> {
        RX_WEIGHT_W::new(self, 8)
    }
}
#[doc = "Weight register of Rx channel 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_wight::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_wight::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IN_WIGHT_SPEC;
impl crate::RegisterSpec for IN_WIGHT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_wight::R`](R) reader structure"]
impl crate::Readable for IN_WIGHT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`in_wight::W`](W) writer structure"]
impl crate::Writable for IN_WIGHT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IN_WIGHT to value 0x0f00"]
impl crate::Resettable for IN_WIGHT_SPEC {
    const RESET_VALUE: u32 = 0x0f00;
}
