#[doc = "Register `LP_PERI0_0` reader"]
pub type R = crate::R<LP_PERI0_0_SPEC>;
#[doc = "Register `LP_PERI0_0` writer"]
pub type W = crate::W<LP_PERI0_0_SPEC>;
#[doc = "Field `LP_PERI0_PMS_EXCEPTION_CLR` writer - Configures whether or not to clear lp_peri0 peri_pms_record_reg.\\\\ 0: No clear\\\\ 1: Clear peri_pms_record_reg\\\\"]
pub type LP_PERI0_PMS_EXCEPTION_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_PERI0_PMS_EXCEPTION_DET` reader - Represents whether the lp_peri0 pms has been triggered.\\\\ 0: No triggered\\\\ 1: Has been triggered\\\\"]
pub type LP_PERI0_PMS_EXCEPTION_DET_R = crate::BitReader;
#[doc = "Field `LP_PERI0_PMS_EXCEPTION_ID` reader - Represents the master id when lp_peri0 pms has been triggered.\\\\"]
pub type LP_PERI0_PMS_EXCEPTION_ID_R = crate::FieldReader;
#[doc = "Field `LP_PERI0_PMS_EXCEPTION_MODE` reader - Represents the security mode when lp_peri0 pms has been triggered.\\\\"]
pub type LP_PERI0_PMS_EXCEPTION_MODE_R = crate::FieldReader;
impl R {
    #[doc = "Bit 1 - Represents whether the lp_peri0 pms has been triggered.\\\\ 0: No triggered\\\\ 1: Has been triggered\\\\"]
    #[inline(always)]
    pub fn lp_peri0_pms_exception_det(&self) -> LP_PERI0_PMS_EXCEPTION_DET_R {
        LP_PERI0_PMS_EXCEPTION_DET_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:7 - Represents the master id when lp_peri0 pms has been triggered.\\\\"]
    #[inline(always)]
    pub fn lp_peri0_pms_exception_id(&self) -> LP_PERI0_PMS_EXCEPTION_ID_R {
        LP_PERI0_PMS_EXCEPTION_ID_R::new(((self.bits >> 2) & 0x3f) as u8)
    }
    #[doc = "Bits 8:9 - Represents the security mode when lp_peri0 pms has been triggered.\\\\"]
    #[inline(always)]
    pub fn lp_peri0_pms_exception_mode(&self) -> LP_PERI0_PMS_EXCEPTION_MODE_R {
        LP_PERI0_PMS_EXCEPTION_MODE_R::new(((self.bits >> 8) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_PERI0_0")
            .field(
                "lp_peri0_pms_exception_det",
                &self.lp_peri0_pms_exception_det(),
            )
            .field(
                "lp_peri0_pms_exception_id",
                &self.lp_peri0_pms_exception_id(),
            )
            .field(
                "lp_peri0_pms_exception_mode",
                &self.lp_peri0_pms_exception_mode(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Configures whether or not to clear lp_peri0 peri_pms_record_reg.\\\\ 0: No clear\\\\ 1: Clear peri_pms_record_reg\\\\"]
    #[inline(always)]
    pub fn lp_peri0_pms_exception_clr(
        &mut self,
    ) -> LP_PERI0_PMS_EXCEPTION_CLR_W<'_, LP_PERI0_0_SPEC> {
        LP_PERI0_PMS_EXCEPTION_CLR_W::new(self, 0)
    }
}
#[doc = "LP_PERI0 PMS configuration & info register\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_peri0_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_peri0_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_PERI0_0_SPEC;
impl crate::RegisterSpec for LP_PERI0_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_peri0_0::R`](R) reader structure"]
impl crate::Readable for LP_PERI0_0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lp_peri0_0::W`](W) writer structure"]
impl crate::Writable for LP_PERI0_0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LP_PERI0_0 to value 0"]
impl crate::Resettable for LP_PERI0_0_SPEC {}
