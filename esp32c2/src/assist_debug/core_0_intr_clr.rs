#[doc = "Register `CORE_0_INTR_CLR` writer"]
pub struct W(crate::W<CORE_0_INTR_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE_0_INTR_CLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CORE_0_INTR_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE_0_INTR_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORE_0_AREA_DRAM0_0_RD_CLR` writer - clr sp underlow monitor interrupt"]
pub type CORE_0_AREA_DRAM0_0_RD_CLR_W<'a> =
    crate::BitWriter<'a, u32, CORE_0_INTR_CLR_SPEC, bool, 0>;
#[doc = "Field `CORE_0_AREA_DRAM0_0_WR_CLR` writer - clr sp overflow monitor interrupt"]
pub type CORE_0_AREA_DRAM0_0_WR_CLR_W<'a> =
    crate::BitWriter<'a, u32, CORE_0_INTR_CLR_SPEC, bool, 1>;
impl W {
    #[doc = "Bit 0 - clr sp underlow monitor interrupt"]
    #[inline(always)]
    pub fn core_0_area_dram0_0_rd_clr(&mut self) -> CORE_0_AREA_DRAM0_0_RD_CLR_W {
        CORE_0_AREA_DRAM0_0_RD_CLR_W::new(self)
    }
    #[doc = "Bit 1 - clr sp overflow monitor interrupt"]
    #[inline(always)]
    pub fn core_0_area_dram0_0_wr_clr(&mut self) -> CORE_0_AREA_DRAM0_0_WR_CLR_W {
        CORE_0_AREA_DRAM0_0_WR_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "core0 monitor interrupt clr register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_0_intr_clr](index.html) module"]
pub struct CORE_0_INTR_CLR_SPEC;
impl crate::RegisterSpec for CORE_0_INTR_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [core_0_intr_clr::W](W) writer structure"]
impl crate::Writable for CORE_0_INTR_CLR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CORE_0_INTR_CLR to value 0"]
impl crate::Resettable for CORE_0_INTR_CLR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
