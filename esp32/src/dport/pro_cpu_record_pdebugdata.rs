#[doc = "Register `PRO_CPU_RECORD_PDEBUGDATA` reader"]
pub struct R(crate::R<PRO_CPU_RECORD_PDEBUGDATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRO_CPU_RECORD_PDEBUGDATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRO_CPU_RECORD_PDEBUGDATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRO_CPU_RECORD_PDEBUGDATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRO_CPU_RECORD_PDEBUGDATA` writer"]
pub struct W(crate::W<PRO_CPU_RECORD_PDEBUGDATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRO_CPU_RECORD_PDEBUGDATA_SPEC>;
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
impl From<crate::W<PRO_CPU_RECORD_PDEBUGDATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRO_CPU_RECORD_PDEBUGDATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RECORD_PRO_PDEBUGDATA` reader - "]
pub type RECORD_PRO_PDEBUGDATA_R = crate::FieldReader<u32, u32>;
#[doc = "Field `RECORD_PDEBUGDATA_DEP_OTHER` reader - "]
pub type RECORD_PDEBUGDATA_DEP_OTHER_R = crate::BitReader;
#[doc = "Field `RECORD_PDEBUGDATA_DEP_OTHER` writer - "]
pub type RECORD_PDEBUGDATA_DEP_OTHER_W<'a, const O: u8> =
    crate::BitWriter<'a, PRO_CPU_RECORD_PDEBUGDATA_SPEC, O>;
#[doc = "Field `RECORD_PDEBUGDATA_EXCVEC` reader - "]
pub type RECORD_PDEBUGDATA_EXCVEC_R = crate::FieldReader;
#[doc = "Field `RECORD_PDEBUGDATA_EXCVEC` writer - "]
pub type RECORD_PDEBUGDATA_EXCVEC_W<'a, const O: u8> =
    crate::FieldWriter<'a, PRO_CPU_RECORD_PDEBUGDATA_SPEC, 5, O>;
#[doc = "Field `RECORD_PDEBUGDATA_INSNTYPE_SR` reader - "]
pub type RECORD_PDEBUGDATA_INSNTYPE_SR_R = crate::FieldReader;
#[doc = "Field `RECORD_PDEBUGDATA_INSNTYPE_SR` writer - "]
pub type RECORD_PDEBUGDATA_INSNTYPE_SR_W<'a, const O: u8> =
    crate::FieldWriter<'a, PRO_CPU_RECORD_PDEBUGDATA_SPEC, 8, O>;
#[doc = "Field `RECORD_PDEBUGDATA_INSNTYPE_RER` reader - "]
pub type RECORD_PDEBUGDATA_INSNTYPE_RER_R = crate::BitReader;
#[doc = "Field `RECORD_PDEBUGDATA_INSNTYPE_RER` writer - "]
pub type RECORD_PDEBUGDATA_INSNTYPE_RER_W<'a, const O: u8> =
    crate::BitWriter<'a, PRO_CPU_RECORD_PDEBUGDATA_SPEC, O>;
#[doc = "Field `RECORD_PDEBUGDATA_STALL_BUFF` reader - "]
pub type RECORD_PDEBUGDATA_STALL_BUFF_R = crate::BitReader;
#[doc = "Field `RECORD_PDEBUGDATA_STALL_BUFF` writer - "]
pub type RECORD_PDEBUGDATA_STALL_BUFF_W<'a, const O: u8> =
    crate::BitWriter<'a, PRO_CPU_RECORD_PDEBUGDATA_SPEC, O>;
#[doc = "Field `RECORD_PDEBUGDATA_INSNTYPE_WER` reader - "]
pub type RECORD_PDEBUGDATA_INSNTYPE_WER_R = crate::BitReader;
#[doc = "Field `RECORD_PDEBUGDATA_INSNTYPE_WER` writer - "]
pub type RECORD_PDEBUGDATA_INSNTYPE_WER_W<'a, const O: u8> =
    crate::BitWriter<'a, PRO_CPU_RECORD_PDEBUGDATA_SPEC, O>;
#[doc = "Field `RECORD_PDEBUGDATA_STALL_BUFFCONFL` reader - "]
pub type RECORD_PDEBUGDATA_STALL_BUFFCONFL_R = crate::BitReader;
#[doc = "Field `RECORD_PDEBUGDATA_STALL_BUFFCONFL` writer - "]
pub type RECORD_PDEBUGDATA_STALL_BUFFCONFL_W<'a, const O: u8> =
    crate::BitWriter<'a, PRO_CPU_RECORD_PDEBUGDATA_SPEC, O>;
#[doc = "Field `RECORD_PDEBUGDATA_INSNTYPE_ER` reader - "]
pub type RECORD_PDEBUGDATA_INSNTYPE_ER_R = crate::FieldReader<u16, u16>;
#[doc = "Field `RECORD_PDEBUGDATA_INSNTYPE_ER` writer - "]
pub type RECORD_PDEBUGDATA_INSNTYPE_ER_W<'a, const O: u8> =
    crate::FieldWriter<'a, PRO_CPU_RECORD_PDEBUGDATA_SPEC, 12, O, u16, u16>;
#[doc = "Field `RECORD_PDEBUGDATA_STALL_DCM` reader - "]
pub type RECORD_PDEBUGDATA_STALL_DCM_R = crate::BitReader;
#[doc = "Field `RECORD_PDEBUGDATA_STALL_DCM` writer - "]
pub type RECORD_PDEBUGDATA_STALL_DCM_W<'a, const O: u8> =
    crate::BitWriter<'a, PRO_CPU_RECORD_PDEBUGDATA_SPEC, O>;
#[doc = "Field `RECORD_PDEBUGDATA_STALL_LSU` reader - "]
pub type RECORD_PDEBUGDATA_STALL_LSU_R = crate::BitReader;
#[doc = "Field `RECORD_PDEBUGDATA_STALL_LSU` writer - "]
pub type RECORD_PDEBUGDATA_STALL_LSU_W<'a, const O: u8> =
    crate::BitWriter<'a, PRO_CPU_RECORD_PDEBUGDATA_SPEC, O>;
#[doc = "Field `RECORD_PDEBUGDATA_STALL_ICM` reader - "]
pub type RECORD_PDEBUGDATA_STALL_ICM_R = crate::BitReader;
#[doc = "Field `RECORD_PDEBUGDATA_STALL_ICM` writer - "]
pub type RECORD_PDEBUGDATA_STALL_ICM_W<'a, const O: u8> =
    crate::BitWriter<'a, PRO_CPU_RECORD_PDEBUGDATA_SPEC, O>;
#[doc = "Field `RECORD_PDEBUGDATA_STALL_IRAMBUSY` reader - "]
pub type RECORD_PDEBUGDATA_STALL_IRAMBUSY_R = crate::BitReader;
#[doc = "Field `RECORD_PDEBUGDATA_STALL_IRAMBUSY` writer - "]
pub type RECORD_PDEBUGDATA_STALL_IRAMBUSY_W<'a, const O: u8> =
    crate::BitWriter<'a, PRO_CPU_RECORD_PDEBUGDATA_SPEC, O>;
#[doc = "Field `RECORD_PDEBUGDATA_DEP_LSU` reader - "]
pub type RECORD_PDEBUGDATA_DEP_LSU_R = crate::BitReader;
#[doc = "Field `RECORD_PDEBUGDATA_DEP_LSU` writer - "]
pub type RECORD_PDEBUGDATA_DEP_LSU_W<'a, const O: u8> =
    crate::BitWriter<'a, PRO_CPU_RECORD_PDEBUGDATA_SPEC, O>;
#[doc = "Field `RECORD_PDEBUGDATA_STALL_IPIF` reader - "]
pub type RECORD_PDEBUGDATA_STALL_IPIF_R = crate::BitReader;
#[doc = "Field `RECORD_PDEBUGDATA_STALL_IPIF` writer - "]
pub type RECORD_PDEBUGDATA_STALL_IPIF_W<'a, const O: u8> =
    crate::BitWriter<'a, PRO_CPU_RECORD_PDEBUGDATA_SPEC, O>;
#[doc = "Field `RECORD_PDEBUGDATA_INSNTYPE_RSR` reader - "]
pub type RECORD_PDEBUGDATA_INSNTYPE_RSR_R = crate::BitReader;
#[doc = "Field `RECORD_PDEBUGDATA_INSNTYPE_RSR` writer - "]
pub type RECORD_PDEBUGDATA_INSNTYPE_RSR_W<'a, const O: u8> =
    crate::BitWriter<'a, PRO_CPU_RECORD_PDEBUGDATA_SPEC, O>;
#[doc = "Field `RECORD_PDEBUGDATA_STALL_TIE` reader - "]
pub type RECORD_PDEBUGDATA_STALL_TIE_R = crate::BitReader;
#[doc = "Field `RECORD_PDEBUGDATA_STALL_TIE` writer - "]
pub type RECORD_PDEBUGDATA_STALL_TIE_W<'a, const O: u8> =
    crate::BitWriter<'a, PRO_CPU_RECORD_PDEBUGDATA_SPEC, O>;
#[doc = "Field `RECORD_PDEBUGDATA_INSNTYPE_WSR` reader - "]
pub type RECORD_PDEBUGDATA_INSNTYPE_WSR_R = crate::BitReader;
#[doc = "Field `RECORD_PDEBUGDATA_INSNTYPE_WSR` writer - "]
pub type RECORD_PDEBUGDATA_INSNTYPE_WSR_W<'a, const O: u8> =
    crate::BitWriter<'a, PRO_CPU_RECORD_PDEBUGDATA_SPEC, O>;
#[doc = "Field `RECORD_PDEBUGDATA_STALL_RUN` reader - "]
pub type RECORD_PDEBUGDATA_STALL_RUN_R = crate::BitReader;
#[doc = "Field `RECORD_PDEBUGDATA_STALL_RUN` writer - "]
pub type RECORD_PDEBUGDATA_STALL_RUN_W<'a, const O: u8> =
    crate::BitWriter<'a, PRO_CPU_RECORD_PDEBUGDATA_SPEC, O>;
#[doc = "Field `RECORD_PDEBUGDATA_INSNTYPE_XSR` reader - "]
pub type RECORD_PDEBUGDATA_INSNTYPE_XSR_R = crate::BitReader;
#[doc = "Field `RECORD_PDEBUGDATA_INSNTYPE_XSR` writer - "]
pub type RECORD_PDEBUGDATA_INSNTYPE_XSR_W<'a, const O: u8> =
    crate::BitWriter<'a, PRO_CPU_RECORD_PDEBUGDATA_SPEC, O>;
#[doc = "Field `RECORD_PDEBUGDATA_DEP_STR` reader - "]
pub type RECORD_PDEBUGDATA_DEP_STR_R = crate::BitReader;
#[doc = "Field `RECORD_PDEBUGDATA_DEP_STR` writer - "]
pub type RECORD_PDEBUGDATA_DEP_STR_W<'a, const O: u8> =
    crate::BitWriter<'a, PRO_CPU_RECORD_PDEBUGDATA_SPEC, O>;
#[doc = "Field `RECORD_PDEBUGDATA_DEP` reader - "]
pub type RECORD_PDEBUGDATA_DEP_R = crate::BitReader;
#[doc = "Field `RECORD_PDEBUGDATA_DEP` writer - "]
pub type RECORD_PDEBUGDATA_DEP_W<'a, const O: u8> =
    crate::BitWriter<'a, PRO_CPU_RECORD_PDEBUGDATA_SPEC, O>;
#[doc = "Field `RECORD_PDEBUGDATA_STALL_BPIFETCH` reader - "]
pub type RECORD_PDEBUGDATA_STALL_BPIFETCH_R = crate::BitReader;
#[doc = "Field `RECORD_PDEBUGDATA_STALL_BPIFETCH` writer - "]
pub type RECORD_PDEBUGDATA_STALL_BPIFETCH_W<'a, const O: u8> =
    crate::BitWriter<'a, PRO_CPU_RECORD_PDEBUGDATA_SPEC, O>;
#[doc = "Field `RECORD_PDEBUGDATA_STALL_L32R` reader - "]
pub type RECORD_PDEBUGDATA_STALL_L32R_R = crate::BitReader;
#[doc = "Field `RECORD_PDEBUGDATA_STALL_L32R` writer - "]
pub type RECORD_PDEBUGDATA_STALL_L32R_W<'a, const O: u8> =
    crate::BitWriter<'a, PRO_CPU_RECORD_PDEBUGDATA_SPEC, O>;
#[doc = "Field `RECORD_PDEBUGDATA_STALL_LSPROC` reader - "]
pub type RECORD_PDEBUGDATA_STALL_LSPROC_R = crate::BitReader;
#[doc = "Field `RECORD_PDEBUGDATA_STALL_LSPROC` writer - "]
pub type RECORD_PDEBUGDATA_STALL_LSPROC_W<'a, const O: u8> =
    crate::BitWriter<'a, PRO_CPU_RECORD_PDEBUGDATA_SPEC, O>;
#[doc = "Field `RECORD_PDEBUGDATA_STALL_BPLOAD` reader - "]
pub type RECORD_PDEBUGDATA_STALL_BPLOAD_R = crate::BitReader;
#[doc = "Field `RECORD_PDEBUGDATA_STALL_BPLOAD` writer - "]
pub type RECORD_PDEBUGDATA_STALL_BPLOAD_W<'a, const O: u8> =
    crate::BitWriter<'a, PRO_CPU_RECORD_PDEBUGDATA_SPEC, O>;
#[doc = "Field `RECORD_PDEBUGDATA_DEP_MEMW` reader - "]
pub type RECORD_PDEBUGDATA_DEP_MEMW_R = crate::BitReader;
#[doc = "Field `RECORD_PDEBUGDATA_DEP_MEMW` writer - "]
pub type RECORD_PDEBUGDATA_DEP_MEMW_W<'a, const O: u8> =
    crate::BitWriter<'a, PRO_CPU_RECORD_PDEBUGDATA_SPEC, O>;
#[doc = "Field `RECORD_PDEBUGDATA_EXCCAUSE` reader - "]
pub type RECORD_PDEBUGDATA_EXCCAUSE_R = crate::FieldReader;
#[doc = "Field `RECORD_PDEBUGDATA_EXCCAUSE` writer - "]
pub type RECORD_PDEBUGDATA_EXCCAUSE_W<'a, const O: u8> =
    crate::FieldWriter<'a, PRO_CPU_RECORD_PDEBUGDATA_SPEC, 6, O>;
#[doc = "Field `RECORD_PDEBUGDATA_STALL_BANKCONFL` reader - "]
pub type RECORD_PDEBUGDATA_STALL_BANKCONFL_R = crate::BitReader;
#[doc = "Field `RECORD_PDEBUGDATA_STALL_BANKCONFL` writer - "]
pub type RECORD_PDEBUGDATA_STALL_BANKCONFL_W<'a, const O: u8> =
    crate::BitWriter<'a, PRO_CPU_RECORD_PDEBUGDATA_SPEC, O>;
#[doc = "Field `RECORD_PDEBUGDATA_DEP_HALT` reader - "]
pub type RECORD_PDEBUGDATA_DEP_HALT_R = crate::BitReader;
#[doc = "Field `RECORD_PDEBUGDATA_DEP_HALT` writer - "]
pub type RECORD_PDEBUGDATA_DEP_HALT_W<'a, const O: u8> =
    crate::BitWriter<'a, PRO_CPU_RECORD_PDEBUGDATA_SPEC, O>;
#[doc = "Field `RECORD_PDEBUGDATA_STALL_ITERMUL` reader - "]
pub type RECORD_PDEBUGDATA_STALL_ITERMUL_R = crate::BitReader;
#[doc = "Field `RECORD_PDEBUGDATA_STALL_ITERMUL` writer - "]
pub type RECORD_PDEBUGDATA_STALL_ITERMUL_W<'a, const O: u8> =
    crate::BitWriter<'a, PRO_CPU_RECORD_PDEBUGDATA_SPEC, O>;
#[doc = "Field `RECORD_PDEBUGDATA_STALL_ITERDIV` reader - "]
pub type RECORD_PDEBUGDATA_STALL_ITERDIV_R = crate::BitReader;
#[doc = "Field `RECORD_PDEBUGDATA_STALL_ITERDIV` writer - "]
pub type RECORD_PDEBUGDATA_STALL_ITERDIV_W<'a, const O: u8> =
    crate::BitWriter<'a, PRO_CPU_RECORD_PDEBUGDATA_SPEC, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn record_pro_pdebugdata(&self) -> RECORD_PRO_PDEBUGDATA_R {
        RECORD_PRO_PDEBUGDATA_R::new(self.bits)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn record_pdebugdata_dep_other(&self) -> RECORD_PDEBUGDATA_DEP_OTHER_R {
        RECORD_PDEBUGDATA_DEP_OTHER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn record_pdebugdata_excvec(&self) -> RECORD_PDEBUGDATA_EXCVEC_R {
        RECORD_PDEBUGDATA_EXCVEC_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn record_pdebugdata_insntype_sr(&self) -> RECORD_PDEBUGDATA_INSNTYPE_SR_R {
        RECORD_PDEBUGDATA_INSNTYPE_SR_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn record_pdebugdata_insntype_rer(&self) -> RECORD_PDEBUGDATA_INSNTYPE_RER_R {
        RECORD_PDEBUGDATA_INSNTYPE_RER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn record_pdebugdata_stall_buff(&self) -> RECORD_PDEBUGDATA_STALL_BUFF_R {
        RECORD_PDEBUGDATA_STALL_BUFF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn record_pdebugdata_insntype_wer(&self) -> RECORD_PDEBUGDATA_INSNTYPE_WER_R {
        RECORD_PDEBUGDATA_INSNTYPE_WER_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn record_pdebugdata_stall_buffconfl(&self) -> RECORD_PDEBUGDATA_STALL_BUFFCONFL_R {
        RECORD_PDEBUGDATA_STALL_BUFFCONFL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 2:13"]
    #[inline(always)]
    pub fn record_pdebugdata_insntype_er(&self) -> RECORD_PDEBUGDATA_INSNTYPE_ER_R {
        RECORD_PDEBUGDATA_INSNTYPE_ER_R::new(((self.bits >> 2) & 0x0fff) as u16)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn record_pdebugdata_stall_dcm(&self) -> RECORD_PDEBUGDATA_STALL_DCM_R {
        RECORD_PDEBUGDATA_STALL_DCM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn record_pdebugdata_stall_lsu(&self) -> RECORD_PDEBUGDATA_STALL_LSU_R {
        RECORD_PDEBUGDATA_STALL_LSU_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn record_pdebugdata_stall_icm(&self) -> RECORD_PDEBUGDATA_STALL_ICM_R {
        RECORD_PDEBUGDATA_STALL_ICM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn record_pdebugdata_stall_irambusy(&self) -> RECORD_PDEBUGDATA_STALL_IRAMBUSY_R {
        RECORD_PDEBUGDATA_STALL_IRAMBUSY_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn record_pdebugdata_dep_lsu(&self) -> RECORD_PDEBUGDATA_DEP_LSU_R {
        RECORD_PDEBUGDATA_DEP_LSU_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn record_pdebugdata_stall_ipif(&self) -> RECORD_PDEBUGDATA_STALL_IPIF_R {
        RECORD_PDEBUGDATA_STALL_IPIF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn record_pdebugdata_insntype_rsr(&self) -> RECORD_PDEBUGDATA_INSNTYPE_RSR_R {
        RECORD_PDEBUGDATA_INSNTYPE_RSR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn record_pdebugdata_stall_tie(&self) -> RECORD_PDEBUGDATA_STALL_TIE_R {
        RECORD_PDEBUGDATA_STALL_TIE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn record_pdebugdata_insntype_wsr(&self) -> RECORD_PDEBUGDATA_INSNTYPE_WSR_R {
        RECORD_PDEBUGDATA_INSNTYPE_WSR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn record_pdebugdata_stall_run(&self) -> RECORD_PDEBUGDATA_STALL_RUN_R {
        RECORD_PDEBUGDATA_STALL_RUN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn record_pdebugdata_insntype_xsr(&self) -> RECORD_PDEBUGDATA_INSNTYPE_XSR_R {
        RECORD_PDEBUGDATA_INSNTYPE_XSR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn record_pdebugdata_dep_str(&self) -> RECORD_PDEBUGDATA_DEP_STR_R {
        RECORD_PDEBUGDATA_DEP_STR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn record_pdebugdata_dep(&self) -> RECORD_PDEBUGDATA_DEP_R {
        RECORD_PDEBUGDATA_DEP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn record_pdebugdata_stall_bpifetch(&self) -> RECORD_PDEBUGDATA_STALL_BPIFETCH_R {
        RECORD_PDEBUGDATA_STALL_BPIFETCH_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn record_pdebugdata_stall_l32r(&self) -> RECORD_PDEBUGDATA_STALL_L32R_R {
        RECORD_PDEBUGDATA_STALL_L32R_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn record_pdebugdata_stall_lsproc(&self) -> RECORD_PDEBUGDATA_STALL_LSPROC_R {
        RECORD_PDEBUGDATA_STALL_LSPROC_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn record_pdebugdata_stall_bpload(&self) -> RECORD_PDEBUGDATA_STALL_BPLOAD_R {
        RECORD_PDEBUGDATA_STALL_BPLOAD_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn record_pdebugdata_dep_memw(&self) -> RECORD_PDEBUGDATA_DEP_MEMW_R {
        RECORD_PDEBUGDATA_DEP_MEMW_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    pub fn record_pdebugdata_exccause(&self) -> RECORD_PDEBUGDATA_EXCCAUSE_R {
        RECORD_PDEBUGDATA_EXCCAUSE_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn record_pdebugdata_stall_bankconfl(&self) -> RECORD_PDEBUGDATA_STALL_BANKCONFL_R {
        RECORD_PDEBUGDATA_STALL_BANKCONFL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn record_pdebugdata_dep_halt(&self) -> RECORD_PDEBUGDATA_DEP_HALT_R {
        RECORD_PDEBUGDATA_DEP_HALT_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn record_pdebugdata_stall_itermul(&self) -> RECORD_PDEBUGDATA_STALL_ITERMUL_R {
        RECORD_PDEBUGDATA_STALL_ITERMUL_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn record_pdebugdata_stall_iterdiv(&self) -> RECORD_PDEBUGDATA_STALL_ITERDIV_R {
        RECORD_PDEBUGDATA_STALL_ITERDIV_R::new(((self.bits >> 19) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_CPU_RECORD_PDEBUGDATA")
            .field(
                "record_pro_pdebugdata",
                &format_args!("{}", self.record_pro_pdebugdata().bits()),
            )
            .field(
                "record_pdebugdata_dep_other",
                &format_args!("{}", self.record_pdebugdata_dep_other().bit()),
            )
            .field(
                "record_pdebugdata_excvec",
                &format_args!("{}", self.record_pdebugdata_excvec().bits()),
            )
            .field(
                "record_pdebugdata_insntype_sr",
                &format_args!("{}", self.record_pdebugdata_insntype_sr().bits()),
            )
            .field(
                "record_pdebugdata_insntype_rer",
                &format_args!("{}", self.record_pdebugdata_insntype_rer().bit()),
            )
            .field(
                "record_pdebugdata_stall_buff",
                &format_args!("{}", self.record_pdebugdata_stall_buff().bit()),
            )
            .field(
                "record_pdebugdata_insntype_wer",
                &format_args!("{}", self.record_pdebugdata_insntype_wer().bit()),
            )
            .field(
                "record_pdebugdata_stall_buffconfl",
                &format_args!("{}", self.record_pdebugdata_stall_buffconfl().bit()),
            )
            .field(
                "record_pdebugdata_insntype_er",
                &format_args!("{}", self.record_pdebugdata_insntype_er().bits()),
            )
            .field(
                "record_pdebugdata_stall_dcm",
                &format_args!("{}", self.record_pdebugdata_stall_dcm().bit()),
            )
            .field(
                "record_pdebugdata_stall_lsu",
                &format_args!("{}", self.record_pdebugdata_stall_lsu().bit()),
            )
            .field(
                "record_pdebugdata_stall_icm",
                &format_args!("{}", self.record_pdebugdata_stall_icm().bit()),
            )
            .field(
                "record_pdebugdata_stall_irambusy",
                &format_args!("{}", self.record_pdebugdata_stall_irambusy().bit()),
            )
            .field(
                "record_pdebugdata_dep_lsu",
                &format_args!("{}", self.record_pdebugdata_dep_lsu().bit()),
            )
            .field(
                "record_pdebugdata_stall_ipif",
                &format_args!("{}", self.record_pdebugdata_stall_ipif().bit()),
            )
            .field(
                "record_pdebugdata_insntype_rsr",
                &format_args!("{}", self.record_pdebugdata_insntype_rsr().bit()),
            )
            .field(
                "record_pdebugdata_stall_tie",
                &format_args!("{}", self.record_pdebugdata_stall_tie().bit()),
            )
            .field(
                "record_pdebugdata_insntype_wsr",
                &format_args!("{}", self.record_pdebugdata_insntype_wsr().bit()),
            )
            .field(
                "record_pdebugdata_stall_run",
                &format_args!("{}", self.record_pdebugdata_stall_run().bit()),
            )
            .field(
                "record_pdebugdata_insntype_xsr",
                &format_args!("{}", self.record_pdebugdata_insntype_xsr().bit()),
            )
            .field(
                "record_pdebugdata_dep_str",
                &format_args!("{}", self.record_pdebugdata_dep_str().bit()),
            )
            .field(
                "record_pdebugdata_dep",
                &format_args!("{}", self.record_pdebugdata_dep().bit()),
            )
            .field(
                "record_pdebugdata_stall_bpifetch",
                &format_args!("{}", self.record_pdebugdata_stall_bpifetch().bit()),
            )
            .field(
                "record_pdebugdata_stall_l32r",
                &format_args!("{}", self.record_pdebugdata_stall_l32r().bit()),
            )
            .field(
                "record_pdebugdata_stall_lsproc",
                &format_args!("{}", self.record_pdebugdata_stall_lsproc().bit()),
            )
            .field(
                "record_pdebugdata_stall_bpload",
                &format_args!("{}", self.record_pdebugdata_stall_bpload().bit()),
            )
            .field(
                "record_pdebugdata_dep_memw",
                &format_args!("{}", self.record_pdebugdata_dep_memw().bit()),
            )
            .field(
                "record_pdebugdata_exccause",
                &format_args!("{}", self.record_pdebugdata_exccause().bits()),
            )
            .field(
                "record_pdebugdata_stall_bankconfl",
                &format_args!("{}", self.record_pdebugdata_stall_bankconfl().bit()),
            )
            .field(
                "record_pdebugdata_dep_halt",
                &format_args!("{}", self.record_pdebugdata_dep_halt().bit()),
            )
            .field(
                "record_pdebugdata_stall_itermul",
                &format_args!("{}", self.record_pdebugdata_stall_itermul().bit()),
            )
            .field(
                "record_pdebugdata_stall_iterdiv",
                &format_args!("{}", self.record_pdebugdata_stall_iterdiv().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PRO_CPU_RECORD_PDEBUGDATA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn record_pdebugdata_dep_other(&mut self) -> RECORD_PDEBUGDATA_DEP_OTHER_W<0> {
        RECORD_PDEBUGDATA_DEP_OTHER_W::new(self)
    }
    #[doc = "Bits 0:4"]
    #[inline(always)]
    #[must_use]
    pub fn record_pdebugdata_excvec(&mut self) -> RECORD_PDEBUGDATA_EXCVEC_W<0> {
        RECORD_PDEBUGDATA_EXCVEC_W::new(self)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn record_pdebugdata_insntype_sr(&mut self) -> RECORD_PDEBUGDATA_INSNTYPE_SR_W<0> {
        RECORD_PDEBUGDATA_INSNTYPE_SR_W::new(self)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn record_pdebugdata_insntype_rer(&mut self) -> RECORD_PDEBUGDATA_INSNTYPE_RER_W<0> {
        RECORD_PDEBUGDATA_INSNTYPE_RER_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn record_pdebugdata_stall_buff(&mut self) -> RECORD_PDEBUGDATA_STALL_BUFF_W<1> {
        RECORD_PDEBUGDATA_STALL_BUFF_W::new(self)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn record_pdebugdata_insntype_wer(&mut self) -> RECORD_PDEBUGDATA_INSNTYPE_WER_W<1> {
        RECORD_PDEBUGDATA_INSNTYPE_WER_W::new(self)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn record_pdebugdata_stall_buffconfl(&mut self) -> RECORD_PDEBUGDATA_STALL_BUFFCONFL_W<2> {
        RECORD_PDEBUGDATA_STALL_BUFFCONFL_W::new(self)
    }
    #[doc = "Bits 2:13"]
    #[inline(always)]
    #[must_use]
    pub fn record_pdebugdata_insntype_er(&mut self) -> RECORD_PDEBUGDATA_INSNTYPE_ER_W<2> {
        RECORD_PDEBUGDATA_INSNTYPE_ER_W::new(self)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn record_pdebugdata_stall_dcm(&mut self) -> RECORD_PDEBUGDATA_STALL_DCM_W<3> {
        RECORD_PDEBUGDATA_STALL_DCM_W::new(self)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn record_pdebugdata_stall_lsu(&mut self) -> RECORD_PDEBUGDATA_STALL_LSU_W<4> {
        RECORD_PDEBUGDATA_STALL_LSU_W::new(self)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    #[must_use]
    pub fn record_pdebugdata_stall_icm(&mut self) -> RECORD_PDEBUGDATA_STALL_ICM_W<6> {
        RECORD_PDEBUGDATA_STALL_ICM_W::new(self)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    #[must_use]
    pub fn record_pdebugdata_stall_irambusy(&mut self) -> RECORD_PDEBUGDATA_STALL_IRAMBUSY_W<7> {
        RECORD_PDEBUGDATA_STALL_IRAMBUSY_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn record_pdebugdata_dep_lsu(&mut self) -> RECORD_PDEBUGDATA_DEP_LSU_W<8> {
        RECORD_PDEBUGDATA_DEP_LSU_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn record_pdebugdata_stall_ipif(&mut self) -> RECORD_PDEBUGDATA_STALL_IPIF_W<8> {
        RECORD_PDEBUGDATA_STALL_IPIF_W::new(self)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    #[must_use]
    pub fn record_pdebugdata_insntype_rsr(&mut self) -> RECORD_PDEBUGDATA_INSNTYPE_RSR_W<8> {
        RECORD_PDEBUGDATA_INSNTYPE_RSR_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn record_pdebugdata_stall_tie(&mut self) -> RECORD_PDEBUGDATA_STALL_TIE_W<9> {
        RECORD_PDEBUGDATA_STALL_TIE_W::new(self)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn record_pdebugdata_insntype_wsr(&mut self) -> RECORD_PDEBUGDATA_INSNTYPE_WSR_W<9> {
        RECORD_PDEBUGDATA_INSNTYPE_WSR_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn record_pdebugdata_stall_run(&mut self) -> RECORD_PDEBUGDATA_STALL_RUN_W<10> {
        RECORD_PDEBUGDATA_STALL_RUN_W::new(self)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    #[must_use]
    pub fn record_pdebugdata_insntype_xsr(&mut self) -> RECORD_PDEBUGDATA_INSNTYPE_XSR_W<10> {
        RECORD_PDEBUGDATA_INSNTYPE_XSR_W::new(self)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    #[must_use]
    pub fn record_pdebugdata_dep_str(&mut self) -> RECORD_PDEBUGDATA_DEP_STR_W<11> {
        RECORD_PDEBUGDATA_DEP_STR_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn record_pdebugdata_dep(&mut self) -> RECORD_PDEBUGDATA_DEP_W<12> {
        RECORD_PDEBUGDATA_DEP_W::new(self)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    #[must_use]
    pub fn record_pdebugdata_stall_bpifetch(&mut self) -> RECORD_PDEBUGDATA_STALL_BPIFETCH_W<12> {
        RECORD_PDEBUGDATA_STALL_BPIFETCH_W::new(self)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    #[must_use]
    pub fn record_pdebugdata_stall_l32r(&mut self) -> RECORD_PDEBUGDATA_STALL_L32R_W<13> {
        RECORD_PDEBUGDATA_STALL_L32R_W::new(self)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    #[must_use]
    pub fn record_pdebugdata_stall_lsproc(&mut self) -> RECORD_PDEBUGDATA_STALL_LSPROC_W<14> {
        RECORD_PDEBUGDATA_STALL_LSPROC_W::new(self)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    #[must_use]
    pub fn record_pdebugdata_stall_bpload(&mut self) -> RECORD_PDEBUGDATA_STALL_BPLOAD_W<15> {
        RECORD_PDEBUGDATA_STALL_BPLOAD_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn record_pdebugdata_dep_memw(&mut self) -> RECORD_PDEBUGDATA_DEP_MEMW_W<16> {
        RECORD_PDEBUGDATA_DEP_MEMW_W::new(self)
    }
    #[doc = "Bits 16:21"]
    #[inline(always)]
    #[must_use]
    pub fn record_pdebugdata_exccause(&mut self) -> RECORD_PDEBUGDATA_EXCCAUSE_W<16> {
        RECORD_PDEBUGDATA_EXCCAUSE_W::new(self)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn record_pdebugdata_stall_bankconfl(&mut self) -> RECORD_PDEBUGDATA_STALL_BANKCONFL_W<16> {
        RECORD_PDEBUGDATA_STALL_BANKCONFL_W::new(self)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    #[must_use]
    pub fn record_pdebugdata_dep_halt(&mut self) -> RECORD_PDEBUGDATA_DEP_HALT_W<17> {
        RECORD_PDEBUGDATA_DEP_HALT_W::new(self)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    #[must_use]
    pub fn record_pdebugdata_stall_itermul(&mut self) -> RECORD_PDEBUGDATA_STALL_ITERMUL_W<18> {
        RECORD_PDEBUGDATA_STALL_ITERMUL_W::new(self)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    #[must_use]
    pub fn record_pdebugdata_stall_iterdiv(&mut self) -> RECORD_PDEBUGDATA_STALL_ITERDIV_W<19> {
        RECORD_PDEBUGDATA_STALL_ITERDIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pro_cpu_record_pdebugdata](index.html) module"]
pub struct PRO_CPU_RECORD_PDEBUGDATA_SPEC;
impl crate::RegisterSpec for PRO_CPU_RECORD_PDEBUGDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pro_cpu_record_pdebugdata::R](R) reader structure"]
impl crate::Readable for PRO_CPU_RECORD_PDEBUGDATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pro_cpu_record_pdebugdata::W](W) writer structure"]
impl crate::Writable for PRO_CPU_RECORD_PDEBUGDATA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRO_CPU_RECORD_PDEBUGDATA to value 0"]
impl crate::Resettable for PRO_CPU_RECORD_PDEBUGDATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
