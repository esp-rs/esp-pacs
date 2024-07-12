#[doc = "Register `POWER_PD_LPPERI_MASK` reader"]
pub type R = crate::R<POWER_PD_LPPERI_MASK_SPEC>;
#[doc = "Register `POWER_PD_LPPERI_MASK` writer"]
pub type W = crate::W<POWER_PD_LPPERI_MASK_SPEC>;
#[doc = "Field `XPD_LP_PERI_MASK` reader - need_des"]
pub type XPD_LP_PERI_MASK_R = crate::FieldReader;
#[doc = "Field `XPD_LP_PERI_MASK` writer - need_des"]
pub type XPD_LP_PERI_MASK_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `PD_LP_PERI_MASK` reader - need_des"]
pub type PD_LP_PERI_MASK_R = crate::FieldReader;
#[doc = "Field `PD_LP_PERI_MASK` writer - need_des"]
pub type PD_LP_PERI_MASK_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - need_des"]
    #[inline(always)]
    pub fn xpd_lp_peri_mask(&self) -> XPD_LP_PERI_MASK_R {
        XPD_LP_PERI_MASK_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 27:31 - need_des"]
    #[inline(always)]
    pub fn pd_lp_peri_mask(&self) -> PD_LP_PERI_MASK_R {
        PD_LP_PERI_MASK_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("POWER_PD_LPPERI_MASK")
            .field("xpd_lp_peri_mask", &self.xpd_lp_peri_mask())
            .field("pd_lp_peri_mask", &self.pd_lp_peri_mask())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn xpd_lp_peri_mask(&mut self) -> XPD_LP_PERI_MASK_W<POWER_PD_LPPERI_MASK_SPEC> {
        XPD_LP_PERI_MASK_W::new(self, 0)
    }
    #[doc = "Bits 27:31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn pd_lp_peri_mask(&mut self) -> PD_LP_PERI_MASK_W<POWER_PD_LPPERI_MASK_SPEC> {
        PD_LP_PERI_MASK_W::new(self, 27)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`power_pd_lpperi_mask::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`power_pd_lpperi_mask::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct POWER_PD_LPPERI_MASK_SPEC;
impl crate::RegisterSpec for POWER_PD_LPPERI_MASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`power_pd_lpperi_mask::R`](R) reader structure"]
impl crate::Readable for POWER_PD_LPPERI_MASK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`power_pd_lpperi_mask::W`](W) writer structure"]
impl crate::Writable for POWER_PD_LPPERI_MASK_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets POWER_PD_LPPERI_MASK to value 0"]
impl crate::Resettable for POWER_PD_LPPERI_MASK_SPEC {
    const RESET_VALUE: u32 = 0;
}
