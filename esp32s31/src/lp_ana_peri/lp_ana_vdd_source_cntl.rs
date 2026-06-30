#[doc = "Register `LP_ANA_VDD_SOURCE_CNTL` reader"]
pub type R = crate::R<LP_ANA_VDD_SOURCE_CNTL_SPEC>;
#[doc = "Register `LP_ANA_VDD_SOURCE_CNTL` writer"]
pub type W = crate::W<LP_ANA_VDD_SOURCE_CNTL_SPEC>;
#[doc = "Field `LP_ANA_DETMODE_SEL` reader - need_des"]
pub type LP_ANA_DETMODE_SEL_R = crate::FieldReader;
#[doc = "Field `LP_ANA_DETMODE_SEL` writer - need_des"]
pub type LP_ANA_DETMODE_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - need_des"]
    #[inline(always)]
    pub fn lp_ana_detmode_sel(&self) -> LP_ANA_DETMODE_SEL_R {
        LP_ANA_DETMODE_SEL_R::new((self.bits & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_ANA_VDD_SOURCE_CNTL")
            .field("lp_ana_detmode_sel", &self.lp_ana_detmode_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5 - need_des"]
    #[inline(always)]
    pub fn lp_ana_detmode_sel(&mut self) -> LP_ANA_DETMODE_SEL_W<'_, LP_ANA_VDD_SOURCE_CNTL_SPEC> {
        LP_ANA_DETMODE_SEL_W::new(self, 0)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_ana_vdd_source_cntl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_ana_vdd_source_cntl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_ANA_VDD_SOURCE_CNTL_SPEC;
impl crate::RegisterSpec for LP_ANA_VDD_SOURCE_CNTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_ana_vdd_source_cntl::R`](R) reader structure"]
impl crate::Readable for LP_ANA_VDD_SOURCE_CNTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lp_ana_vdd_source_cntl::W`](W) writer structure"]
impl crate::Writable for LP_ANA_VDD_SOURCE_CNTL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LP_ANA_VDD_SOURCE_CNTL to value 0x3f"]
impl crate::Resettable for LP_ANA_VDD_SOURCE_CNTL_SPEC {
    const RESET_VALUE: u32 = 0x3f;
}
