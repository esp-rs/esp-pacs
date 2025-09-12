#[doc = "Register `CNTL_DBG_SAR_SEL` reader"]
pub type R = crate::R<CNTL_DBG_SAR_SEL_SPEC>;
#[doc = "Register `CNTL_DBG_SAR_SEL` writer"]
pub type W = crate::W<CNTL_DBG_SAR_SEL_SPEC>;
#[doc = "Field `SAR_DEBUG_SEL` reader - Need add desc"]
pub type SAR_DEBUG_SEL_R = crate::FieldReader;
#[doc = "Field `SAR_DEBUG_SEL` writer - Need add desc"]
pub type SAR_DEBUG_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 27:31 - Need add desc"]
    #[inline(always)]
    pub fn sar_debug_sel(&self) -> SAR_DEBUG_SEL_R {
        SAR_DEBUG_SEL_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CNTL_DBG_SAR_SEL")
            .field("sar_debug_sel", &self.sar_debug_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 27:31 - Need add desc"]
    #[inline(always)]
    pub fn sar_debug_sel(&mut self) -> SAR_DEBUG_SEL_W<'_, CNTL_DBG_SAR_SEL_SPEC> {
        SAR_DEBUG_SEL_W::new(self, 27)
    }
}
#[doc = "register description\n\nYou can [`read`](crate::Reg::read) this register and get [`cntl_dbg_sar_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cntl_dbg_sar_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CNTL_DBG_SAR_SEL_SPEC;
impl crate::RegisterSpec for CNTL_DBG_SAR_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cntl_dbg_sar_sel::R`](R) reader structure"]
impl crate::Readable for CNTL_DBG_SAR_SEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cntl_dbg_sar_sel::W`](W) writer structure"]
impl crate::Writable for CNTL_DBG_SAR_SEL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CNTL_DBG_SAR_SEL to value 0"]
impl crate::Resettable for CNTL_DBG_SAR_SEL_SPEC {}
