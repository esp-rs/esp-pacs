#[doc = "Register `LP_ANA_VDD_SOURCE_CNTL` reader"]
pub type R = crate::R<LP_ANA_VDD_SOURCE_CNTL_SPEC>;
#[doc = "Register `LP_ANA_VDD_SOURCE_CNTL` writer"]
pub type W = crate::W<LP_ANA_VDD_SOURCE_CNTL_SPEC>;
#[doc = "Field `LP_ANA_DETMODE_SEL` reader - need_des"]
pub type LP_ANA_DETMODE_SEL_R = crate::FieldReader;
#[doc = "Field `LP_ANA_DETMODE_SEL` writer - need_des"]
pub type LP_ANA_DETMODE_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LP_ANA_VGOOD_EVENT_RECORD` reader - need_des"]
pub type LP_ANA_VGOOD_EVENT_RECORD_R = crate::FieldReader;
#[doc = "Field `LP_ANA_VBAT_EVENT_RECORD_CLR` writer - need_des"]
pub type LP_ANA_VBAT_EVENT_RECORD_CLR_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LP_ANA_BOD_SOURCE_ENA` reader - need_des"]
pub type LP_ANA_BOD_SOURCE_ENA_R = crate::FieldReader;
#[doc = "Field `LP_ANA_BOD_SOURCE_ENA` writer - need_des"]
pub type LP_ANA_BOD_SOURCE_ENA_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - need_des"]
    #[inline(always)]
    pub fn lp_ana_detmode_sel(&self) -> LP_ANA_DETMODE_SEL_R {
        LP_ANA_DETMODE_SEL_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - need_des"]
    #[inline(always)]
    pub fn lp_ana_vgood_event_record(&self) -> LP_ANA_VGOOD_EVENT_RECORD_R {
        LP_ANA_VGOOD_EVENT_RECORD_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - need_des"]
    #[inline(always)]
    pub fn lp_ana_bod_source_ena(&self) -> LP_ANA_BOD_SOURCE_ENA_R {
        LP_ANA_BOD_SOURCE_ENA_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_ANA_VDD_SOURCE_CNTL")
            .field(
                "lp_ana_detmode_sel",
                &format_args!("{}", self.lp_ana_detmode_sel().bits()),
            )
            .field(
                "lp_ana_vgood_event_record",
                &format_args!("{}", self.lp_ana_vgood_event_record().bits()),
            )
            .field(
                "lp_ana_bod_source_ena",
                &format_args!("{}", self.lp_ana_bod_source_ena().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LP_ANA_VDD_SOURCE_CNTL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_ana_detmode_sel(&mut self) -> LP_ANA_DETMODE_SEL_W<LP_ANA_VDD_SOURCE_CNTL_SPEC> {
        LP_ANA_DETMODE_SEL_W::new(self, 0)
    }
    #[doc = "Bits 16:23 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_ana_vbat_event_record_clr(
        &mut self,
    ) -> LP_ANA_VBAT_EVENT_RECORD_CLR_W<LP_ANA_VDD_SOURCE_CNTL_SPEC> {
        LP_ANA_VBAT_EVENT_RECORD_CLR_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_ana_bod_source_ena(
        &mut self,
    ) -> LP_ANA_BOD_SOURCE_ENA_W<LP_ANA_VDD_SOURCE_CNTL_SPEC> {
        LP_ANA_BOD_SOURCE_ENA_W::new(self, 24)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_ana_vdd_source_cntl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_ana_vdd_source_cntl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_ANA_VDD_SOURCE_CNTL_SPEC;
impl crate::RegisterSpec for LP_ANA_VDD_SOURCE_CNTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_ana_vdd_source_cntl::R`](R) reader structure"]
impl crate::Readable for LP_ANA_VDD_SOURCE_CNTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lp_ana_vdd_source_cntl::W`](W) writer structure"]
impl crate::Writable for LP_ANA_VDD_SOURCE_CNTL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LP_ANA_VDD_SOURCE_CNTL to value 0x0400_00ff"]
impl crate::Resettable for LP_ANA_VDD_SOURCE_CNTL_SPEC {
    const RESET_VALUE: u32 = 0x0400_00ff;
}
