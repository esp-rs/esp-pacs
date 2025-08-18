#[doc = "Register `SLP_WAKEUP_CNTL8` reader"]
pub type R = crate::R<SLP_WAKEUP_CNTL8_SPEC>;
#[doc = "Register `SLP_WAKEUP_CNTL8` writer"]
pub type W = crate::W<SLP_WAKEUP_CNTL8_SPEC>;
#[doc = "Field `LP_LITE_WAKEUP_ENA` reader - need_des"]
pub type LP_LITE_WAKEUP_ENA_R = crate::BitReader;
#[doc = "Field `LP_LITE_WAKEUP_ENA` writer - need_des"]
pub type LP_LITE_WAKEUP_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn lp_lite_wakeup_ena(&self) -> LP_LITE_WAKEUP_ENA_R {
        LP_LITE_WAKEUP_ENA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLP_WAKEUP_CNTL8")
            .field("lp_lite_wakeup_ena", &self.lp_lite_wakeup_ena())
            .finish()
    }
}
impl W {
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn lp_lite_wakeup_ena(&mut self) -> LP_LITE_WAKEUP_ENA_W<'_, SLP_WAKEUP_CNTL8_SPEC> {
        LP_LITE_WAKEUP_ENA_W::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`slp_wakeup_cntl8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slp_wakeup_cntl8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLP_WAKEUP_CNTL8_SPEC;
impl crate::RegisterSpec for SLP_WAKEUP_CNTL8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slp_wakeup_cntl8::R`](R) reader structure"]
impl crate::Readable for SLP_WAKEUP_CNTL8_SPEC {}
#[doc = "`write(|w| ..)` method takes [`slp_wakeup_cntl8::W`](W) writer structure"]
impl crate::Writable for SLP_WAKEUP_CNTL8_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SLP_WAKEUP_CNTL8 to value 0"]
impl crate::Resettable for SLP_WAKEUP_CNTL8_SPEC {}
