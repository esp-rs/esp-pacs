#[doc = "Register `MEM_ACCESS_DBUG1` reader"]
pub struct R(crate::R<MEM_ACCESS_DBUG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MEM_ACCESS_DBUG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MEM_ACCESS_DBUG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MEM_ACCESS_DBUG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `INTERNAL_SRAM_MMU_MISS` reader - "]
pub type INTERNAL_SRAM_MMU_MISS_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ARB_IA` reader - "]
pub type ARB_IA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `PIDGEN_IA` reader - "]
pub type PIDGEN_IA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AHB_ACCESS_DENY` reader - "]
pub type AHB_ACCESS_DENY_R = crate::BitReader<bool>;
#[doc = "Field `AHBLITE_ACCESS_DENY` reader - "]
pub type AHBLITE_ACCESS_DENY_R = crate::BitReader<bool>;
#[doc = "Field `AHBLITE_IA` reader - "]
pub type AHBLITE_IA_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn internal_sram_mmu_miss(&self) -> INTERNAL_SRAM_MMU_MISS_R {
        INTERNAL_SRAM_MMU_MISS_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn arb_ia(&self) -> ARB_IA_R {
        ARB_IA_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn pidgen_ia(&self) -> PIDGEN_IA_R {
        PIDGEN_IA_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ahb_access_deny(&self) -> AHB_ACCESS_DENY_R {
        AHB_ACCESS_DENY_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ahblite_access_deny(&self) -> AHBLITE_ACCESS_DENY_R {
        AHBLITE_ACCESS_DENY_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn ahblite_ia(&self) -> AHBLITE_IA_R {
        AHBLITE_IA_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_access_dbug1](index.html) module"]
pub struct MEM_ACCESS_DBUG1_SPEC;
impl crate::RegisterSpec for MEM_ACCESS_DBUG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mem_access_dbug1::R](R) reader structure"]
impl crate::Readable for MEM_ACCESS_DBUG1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MEM_ACCESS_DBUG1 to value 0"]
impl crate::Resettable for MEM_ACCESS_DBUG1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
