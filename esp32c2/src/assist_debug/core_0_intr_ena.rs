#[doc = "Register `CORE_0_INTR_ENA` reader"]
pub struct R(crate::R<CORE_0_INTR_ENA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_0_INTR_ENA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_0_INTR_ENA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_0_INTR_ENA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CORE_0_INTR_ENA` writer"]
pub struct W(crate::W<CORE_0_INTR_ENA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE_0_INTR_ENA_SPEC>;
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
impl From<crate::W<CORE_0_INTR_ENA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE_0_INTR_ENA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORE_0_AREA_DRAM0_0_RD_ENA` reader - enbale sp underlow monitor"]
pub type CORE_0_AREA_DRAM0_0_RD_ENA_R = crate::BitReader<bool>;
#[doc = "Field `CORE_0_AREA_DRAM0_0_RD_ENA` writer - enbale sp underlow monitor"]
pub type CORE_0_AREA_DRAM0_0_RD_ENA_W<'a> =
    crate::BitWriter<'a, u32, CORE_0_INTR_ENA_SPEC, bool, 0>;
#[doc = "Field `CORE_0_AREA_DRAM0_0_WR_ENA` reader - enbale sp overflow monitor"]
pub type CORE_0_AREA_DRAM0_0_WR_ENA_R = crate::BitReader<bool>;
#[doc = "Field `CORE_0_AREA_DRAM0_0_WR_ENA` writer - enbale sp overflow monitor"]
pub type CORE_0_AREA_DRAM0_0_WR_ENA_W<'a> =
    crate::BitWriter<'a, u32, CORE_0_INTR_ENA_SPEC, bool, 1>;
impl R {
    #[doc = "Bit 0 - enbale sp underlow monitor"]
    #[inline(always)]
    pub fn core_0_area_dram0_0_rd_ena(&self) -> CORE_0_AREA_DRAM0_0_RD_ENA_R {
        CORE_0_AREA_DRAM0_0_RD_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - enbale sp overflow monitor"]
    #[inline(always)]
    pub fn core_0_area_dram0_0_wr_ena(&self) -> CORE_0_AREA_DRAM0_0_WR_ENA_R {
        CORE_0_AREA_DRAM0_0_WR_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - enbale sp underlow monitor"]
    #[inline(always)]
    pub fn core_0_area_dram0_0_rd_ena(&mut self) -> CORE_0_AREA_DRAM0_0_RD_ENA_W {
        CORE_0_AREA_DRAM0_0_RD_ENA_W::new(self)
    }
    #[doc = "Bit 1 - enbale sp overflow monitor"]
    #[inline(always)]
    pub fn core_0_area_dram0_0_wr_ena(&mut self) -> CORE_0_AREA_DRAM0_0_WR_ENA_W {
        CORE_0_AREA_DRAM0_0_WR_ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "core0 monitor enable configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_0_intr_ena](index.html) module"]
pub struct CORE_0_INTR_ENA_SPEC;
impl crate::RegisterSpec for CORE_0_INTR_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_0_intr_ena::R](R) reader structure"]
impl crate::Readable for CORE_0_INTR_ENA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [core_0_intr_ena::W](W) writer structure"]
impl crate::Writable for CORE_0_INTR_ENA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CORE_0_INTR_ENA to value 0"]
impl crate::Resettable for CORE_0_INTR_ENA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
