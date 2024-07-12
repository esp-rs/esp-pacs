#[doc = "Register `DECODER_STATUS2` reader"]
pub type R = crate::R<DECODER_STATUS2_SPEC>;
#[doc = "Field `COMP_BLOCK_NUM` reader - Reserved"]
pub type COMP_BLOCK_NUM_R = crate::FieldReader<u32>;
#[doc = "Field `SCAN_NUM` reader - Reserved"]
pub type SCAN_NUM_R = crate::FieldReader;
#[doc = "Field `RST_CHECK_WAIT` reader - Reserved"]
pub type RST_CHECK_WAIT_R = crate::BitReader;
#[doc = "Field `SCAN_CHECK_WAIT` reader - Reserved"]
pub type SCAN_CHECK_WAIT_R = crate::BitReader;
#[doc = "Field `MCU_IN_PROC` reader - Reserved"]
pub type MCU_IN_PROC_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:25 - Reserved"]
    #[inline(always)]
    pub fn comp_block_num(&self) -> COMP_BLOCK_NUM_R {
        COMP_BLOCK_NUM_R::new(self.bits & 0x03ff_ffff)
    }
    #[doc = "Bits 26:28 - Reserved"]
    #[inline(always)]
    pub fn scan_num(&self) -> SCAN_NUM_R {
        SCAN_NUM_R::new(((self.bits >> 26) & 7) as u8)
    }
    #[doc = "Bit 29 - Reserved"]
    #[inline(always)]
    pub fn rst_check_wait(&self) -> RST_CHECK_WAIT_R {
        RST_CHECK_WAIT_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Reserved"]
    #[inline(always)]
    pub fn scan_check_wait(&self) -> SCAN_CHECK_WAIT_R {
        SCAN_CHECK_WAIT_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Reserved"]
    #[inline(always)]
    pub fn mcu_in_proc(&self) -> MCU_IN_PROC_R {
        MCU_IN_PROC_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DECODER_STATUS2")
            .field("comp_block_num", &self.comp_block_num())
            .field("scan_num", &self.scan_num())
            .field("rst_check_wait", &self.rst_check_wait())
            .field("scan_check_wait", &self.scan_check_wait())
            .field("mcu_in_proc", &self.mcu_in_proc())
            .finish()
    }
}
#[doc = "Trace and Debug registers\n\nYou can [`read`](crate::Reg::read) this register and get [`decoder_status2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DECODER_STATUS2_SPEC;
impl crate::RegisterSpec for DECODER_STATUS2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`decoder_status2::R`](R) reader structure"]
impl crate::Readable for DECODER_STATUS2_SPEC {}
#[doc = "`reset()` method sets DECODER_STATUS2 to value 0"]
impl crate::Resettable for DECODER_STATUS2_SPEC {
    const RESET_VALUE: u32 = 0;
}
