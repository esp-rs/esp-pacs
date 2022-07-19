#[doc = "Register `CORE_0_INTR_RAW` reader"]
pub struct R(crate::R<CORE_0_INTR_RAW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_0_INTR_RAW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_0_INTR_RAW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_0_INTR_RAW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CORE_0_AREA_DRAM0_0_RD_RAW` reader - sp underlow monitor interrupt status register"]
pub type CORE_0_AREA_DRAM0_0_RD_RAW_R = crate::BitReader<bool>;
#[doc = "Field `CORE_0_AREA_DRAM0_0_WR_RAW` reader - sp overflow monitor interupt status register"]
pub type CORE_0_AREA_DRAM0_0_WR_RAW_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bit 0 - sp underlow monitor interrupt status register"]
    #[inline(always)]
    pub fn core_0_area_dram0_0_rd_raw(&self) -> CORE_0_AREA_DRAM0_0_RD_RAW_R {
        CORE_0_AREA_DRAM0_0_RD_RAW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - sp overflow monitor interupt status register"]
    #[inline(always)]
    pub fn core_0_area_dram0_0_wr_raw(&self) -> CORE_0_AREA_DRAM0_0_WR_RAW_R {
        CORE_0_AREA_DRAM0_0_WR_RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "core0 monitor interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_0_intr_raw](index.html) module"]
pub struct CORE_0_INTR_RAW_SPEC;
impl crate::RegisterSpec for CORE_0_INTR_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_0_intr_raw::R](R) reader structure"]
impl crate::Readable for CORE_0_INTR_RAW_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CORE_0_INTR_RAW to value 0"]
impl crate::Resettable for CORE_0_INTR_RAW_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
