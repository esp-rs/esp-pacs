#[doc = "Register `CHNL0_CFG3` writer"]
pub type W = crate::W<CHNL0_CFG3_SPEC>;
#[doc = "Field `CHNL0_RESET` writer - Set this bit to reset channel0."]
pub type CHNL0_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CHNL0_CFG3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to reset channel0."]
    #[inline(always)]
    pub fn chnl0_reset(&mut self) -> CHNL0_RESET_W<'_, CHNL0_CFG3_SPEC> {
        CHNL0_RESET_W::new(self, 0)
    }
}
#[doc = "Control and configuration registers\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chnl0_cfg3::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHNL0_CFG3_SPEC;
impl crate::RegisterSpec for CHNL0_CFG3_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`chnl0_cfg3::W`](W) writer structure"]
impl crate::Writable for CHNL0_CFG3_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHNL0_CFG3 to value 0"]
impl crate::Resettable for CHNL0_CFG3_SPEC {}
