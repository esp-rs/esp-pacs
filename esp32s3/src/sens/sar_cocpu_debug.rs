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
#[doc = "Field `SAR_COCPU_PC` reader - cocpu Program counter"]
pub type SAR_COCPU_PC_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SAR_COCPU_MEM_VLD` reader - cocpu mem valid output"]
pub type SAR_COCPU_MEM_VLD_R = crate::BitReader<bool>;
#[doc = "Field `SAR_COCPU_MEM_RDY` reader - cocpu mem ready input"]
pub type SAR_COCPU_MEM_RDY_R = crate::BitReader<bool>;
#[doc = "Field `SAR_COCPU_MEM_WEN` reader - cocpu mem write enable output"]
pub type SAR_COCPU_MEM_WEN_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SAR_COCPU_MEM_ADDR` reader - cocpu mem address output"]
pub type SAR_COCPU_MEM_ADDR_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:12 - cocpu Program counter"]
    #[inline(always)]
    pub fn sar_cocpu_pc(&self) -> SAR_COCPU_PC_R {
        SAR_COCPU_PC_R::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bit 13 - cocpu mem valid output"]
    #[inline(always)]
    pub fn sar_cocpu_mem_vld(&self) -> SAR_COCPU_MEM_VLD_R {
        SAR_COCPU_MEM_VLD_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - cocpu mem ready input"]
    #[inline(always)]
    pub fn sar_cocpu_mem_rdy(&self) -> SAR_COCPU_MEM_RDY_R {
        SAR_COCPU_MEM_RDY_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:18 - cocpu mem write enable output"]
    #[inline(always)]
    pub fn sar_cocpu_mem_wen(&self) -> SAR_COCPU_MEM_WEN_R {
        SAR_COCPU_MEM_WEN_R::new(((self.bits >> 15) & 0x0f) as u8)
    }
    #[doc = "Bits 19:31 - cocpu mem address output"]
    #[inline(always)]
    pub fn sar_cocpu_mem_addr(&self) -> SAR_COCPU_MEM_ADDR_R {
        SAR_COCPU_MEM_ADDR_R::new(((self.bits >> 19) & 0x1fff) as u16)
    }
}
#[doc = "Ulp-riscv debug signal\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_cocpu_debug](index.html) module"]
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
    const RESET_VALUE: Self::Ux = 0;
}
