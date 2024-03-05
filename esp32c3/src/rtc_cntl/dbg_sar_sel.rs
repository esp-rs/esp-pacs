#[doc = "Register `DBG_SAR_SEL` reader"]
pub type R = crate::R<DBG_SAR_SEL_SPEC>;
#[doc = "Register `DBG_SAR_SEL` writer"]
pub type W = crate::W<DBG_SAR_SEL_SPEC>;
#[doc = "Field `SAR_DEBUG_SEL` reader - use for debug"]
pub type SAR_DEBUG_SEL_R = crate::FieldReader;
#[doc = "Field `SAR_DEBUG_SEL` writer - use for debug"]
pub type SAR_DEBUG_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 27:31 - use for debug"]
    #[inline(always)]
    pub fn sar_debug_sel(&self) -> SAR_DEBUG_SEL_R {
        SAR_DEBUG_SEL_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBG_SAR_SEL")
            .field(
                "sar_debug_sel",
                &format_args!("{}", self.sar_debug_sel().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DBG_SAR_SEL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 27:31 - use for debug"]
    #[inline(always)]
    #[must_use]
    pub fn sar_debug_sel(&mut self) -> SAR_DEBUG_SEL_W<DBG_SAR_SEL_SPEC> {
        SAR_DEBUG_SEL_W::new(self, 27)
    }
}
#[doc = "rtc configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbg_sar_sel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbg_sar_sel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DBG_SAR_SEL_SPEC;
impl crate::RegisterSpec for DBG_SAR_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbg_sar_sel::R`](R) reader structure"]
impl crate::Readable for DBG_SAR_SEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dbg_sar_sel::W`](W) writer structure"]
impl crate::Writable for DBG_SAR_SEL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DBG_SAR_SEL to value 0"]
impl crate::Resettable for DBG_SAR_SEL_SPEC {
    const RESET_VALUE: u32 = 0;
}
