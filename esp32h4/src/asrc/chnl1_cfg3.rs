#[doc = "Register `CHNL1_CFG3` writer"]
pub type W = crate::W<CHNL1_CFG3_SPEC>;
#[doc = "Field `CHNL1_RESET` writer - Set this bit to reset channel1."]
pub type CHNL1_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CHNL1_CFG3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to reset channel1."]
    #[inline(always)]
    pub fn chnl1_reset(&mut self) -> CHNL1_RESET_W<'_, CHNL1_CFG3_SPEC> {
        CHNL1_RESET_W::new(self, 0)
    }
}
#[doc = "Control and configuration registers\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chnl1_cfg3::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHNL1_CFG3_SPEC;
impl crate::RegisterSpec for CHNL1_CFG3_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`chnl1_cfg3::W`](W) writer structure"]
impl crate::Writable for CHNL1_CFG3_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHNL1_CFG3 to value 0"]
impl crate::Resettable for CHNL1_CFG3_SPEC {}
