#[doc = "Register `CORE_0_INTR_RLS` reader"]
pub struct R(crate::R<CORE_0_INTR_RLS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_0_INTR_RLS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_0_INTR_RLS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_0_INTR_RLS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CORE_0_INTR_RLS` writer"]
pub struct W(crate::W<CORE_0_INTR_RLS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE_0_INTR_RLS_SPEC>;
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
impl From<crate::W<CORE_0_INTR_RLS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE_0_INTR_RLS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORE_0_AREA_DRAM0_0_RD_RLS` reader - enbale sp underlow monitor interrupt"]
pub type CORE_0_AREA_DRAM0_0_RD_RLS_R = crate::BitReader<bool>;
#[doc = "Field `CORE_0_AREA_DRAM0_0_RD_RLS` writer - enbale sp underlow monitor interrupt"]
pub type CORE_0_AREA_DRAM0_0_RD_RLS_W<'a> =
    crate::BitWriter<'a, u32, CORE_0_INTR_RLS_SPEC, bool, 0>;
#[doc = "Field `CORE_0_AREA_DRAM0_0_WR_RLS` reader - enbale sp overflow monitor interrupt"]
pub type CORE_0_AREA_DRAM0_0_WR_RLS_R = crate::BitReader<bool>;
#[doc = "Field `CORE_0_AREA_DRAM0_0_WR_RLS` writer - enbale sp overflow monitor interrupt"]
pub type CORE_0_AREA_DRAM0_0_WR_RLS_W<'a> =
    crate::BitWriter<'a, u32, CORE_0_INTR_RLS_SPEC, bool, 1>;
impl R {
    #[doc = "Bit 0 - enbale sp underlow monitor interrupt"]
    #[inline(always)]
    pub fn core_0_area_dram0_0_rd_rls(&self) -> CORE_0_AREA_DRAM0_0_RD_RLS_R {
        CORE_0_AREA_DRAM0_0_RD_RLS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - enbale sp overflow monitor interrupt"]
    #[inline(always)]
    pub fn core_0_area_dram0_0_wr_rls(&self) -> CORE_0_AREA_DRAM0_0_WR_RLS_R {
        CORE_0_AREA_DRAM0_0_WR_RLS_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - enbale sp underlow monitor interrupt"]
    #[inline(always)]
    pub fn core_0_area_dram0_0_rd_rls(&mut self) -> CORE_0_AREA_DRAM0_0_RD_RLS_W {
        CORE_0_AREA_DRAM0_0_RD_RLS_W::new(self)
    }
    #[doc = "Bit 1 - enbale sp overflow monitor interrupt"]
    #[inline(always)]
    pub fn core_0_area_dram0_0_wr_rls(&mut self) -> CORE_0_AREA_DRAM0_0_WR_RLS_W {
        CORE_0_AREA_DRAM0_0_WR_RLS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "core0 monitor interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_0_intr_rls](index.html) module"]
pub struct CORE_0_INTR_RLS_SPEC;
impl crate::RegisterSpec for CORE_0_INTR_RLS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_0_intr_rls::R](R) reader structure"]
impl crate::Readable for CORE_0_INTR_RLS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [core_0_intr_rls::W](W) writer structure"]
impl crate::Writable for CORE_0_INTR_RLS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CORE_0_INTR_RLS to value 0"]
impl crate::Resettable for CORE_0_INTR_RLS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
