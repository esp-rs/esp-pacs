#[doc = "Register `SAR_COCPU_DEBUG` reader"]
pub struct R(crate::R<SAR_COCPU_DEBUG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR_COCPU_DEBUG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR_COCPU_DEBUG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR_COCPU_DEBUG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `COCPU_PC` reader - ULP-RISCV Program counter"]
pub type COCPU_PC_R = crate::FieldReader<u16, u16>;
#[doc = "Field `COCPU_MEM_VLD` reader - ULP-RISCV memory valid output"]
pub type COCPU_MEM_VLD_R = crate::BitReader<bool>;
#[doc = "Field `COCPU_MEM_RDY` reader - ULP-RISCV memory ready input"]
pub type COCPU_MEM_RDY_R = crate::BitReader<bool>;
#[doc = "Field `COCPU_MEM_WEN` reader - ULP-RISCV memory write enable output"]
pub type COCPU_MEM_WEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `COCPU_MEM_ADDR` reader - ULP-RISCV memory address output"]
pub type COCPU_MEM_ADDR_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:12 - ULP-RISCV Program counter"]
    #[inline(always)]
    pub fn cocpu_pc(&self) -> COCPU_PC_R {
        COCPU_PC_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bit 13 - ULP-RISCV memory valid output"]
    #[inline(always)]
    pub fn cocpu_mem_vld(&self) -> COCPU_MEM_VLD_R {
        COCPU_MEM_VLD_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - ULP-RISCV memory ready input"]
    #[inline(always)]
    pub fn cocpu_mem_rdy(&self) -> COCPU_MEM_RDY_R {
        COCPU_MEM_RDY_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:18 - ULP-RISCV memory write enable output"]
    #[inline(always)]
    pub fn cocpu_mem_wen(&self) -> COCPU_MEM_WEN_R {
        COCPU_MEM_WEN_R::new(((self.bits >> 15) & 0x0f) as u8)
    }
    #[doc = "Bits 19:31 - ULP-RISCV memory address output"]
    #[inline(always)]
    pub fn cocpu_mem_addr(&self) -> COCPU_MEM_ADDR_R {
        COCPU_MEM_ADDR_R::new(((self.bits >> 19) & 0x1fff) as u16)
    }
}
#[doc = "ULP-RISCV debug register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_cocpu_debug](index.html) module"]
pub struct SAR_COCPU_DEBUG_SPEC;
impl crate::RegisterSpec for SAR_COCPU_DEBUG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar_cocpu_debug::R](R) reader structure"]
impl crate::Readable for SAR_COCPU_DEBUG_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SAR_COCPU_DEBUG to value 0"]
impl crate::Resettable for SAR_COCPU_DEBUG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
