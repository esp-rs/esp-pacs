#[doc = "Register `BLK0_RDATA6` reader"]
pub struct R(crate::R<BLK0_RDATA6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLK0_RDATA6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLK0_RDATA6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLK0_RDATA6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RD_CODING_SCHEME` reader - read for coding_scheme"]
pub type RD_CODING_SCHEME_R = crate::FieldReader;
#[doc = "Field `RD_CONSOLE_DEBUG_DISABLE` reader - read for console_debug_disable"]
pub type RD_CONSOLE_DEBUG_DISABLE_R = crate::BitReader;
#[doc = "Field `RD_DISABLE_SDIO_HOST` reader - "]
pub type RD_DISABLE_SDIO_HOST_R = crate::BitReader;
#[doc = "Field `RD_ABS_DONE_0` reader - read for abstract_done_0"]
pub type RD_ABS_DONE_0_R = crate::BitReader;
#[doc = "Field `RD_ABS_DONE_1` reader - read for abstract_done_1"]
pub type RD_ABS_DONE_1_R = crate::BitReader;
#[doc = "Field `RD_DISABLE_JTAG` reader - read for JTAG_disable"]
pub type RD_DISABLE_JTAG_R = crate::BitReader;
#[doc = "Field `RD_DISABLE_DL_ENCRYPT` reader - read for download_dis_encrypt"]
pub type RD_DISABLE_DL_ENCRYPT_R = crate::BitReader;
#[doc = "Field `RD_DISABLE_DL_DECRYPT` reader - read for download_dis_decrypt"]
pub type RD_DISABLE_DL_DECRYPT_R = crate::BitReader;
#[doc = "Field `RD_DISABLE_DL_CACHE` reader - read for download_dis_cache"]
pub type RD_DISABLE_DL_CACHE_R = crate::BitReader;
#[doc = "Field `RD_KEY_STATUS` reader - read for key_status"]
pub type RD_KEY_STATUS_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:1 - read for coding_scheme"]
    #[inline(always)]
    pub fn rd_coding_scheme(&self) -> RD_CODING_SCHEME_R {
        RD_CODING_SCHEME_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - read for console_debug_disable"]
    #[inline(always)]
    pub fn rd_console_debug_disable(&self) -> RD_CONSOLE_DEBUG_DISABLE_R {
        RD_CONSOLE_DEBUG_DISABLE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn rd_disable_sdio_host(&self) -> RD_DISABLE_SDIO_HOST_R {
        RD_DISABLE_SDIO_HOST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - read for abstract_done_0"]
    #[inline(always)]
    pub fn rd_abs_done_0(&self) -> RD_ABS_DONE_0_R {
        RD_ABS_DONE_0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - read for abstract_done_1"]
    #[inline(always)]
    pub fn rd_abs_done_1(&self) -> RD_ABS_DONE_1_R {
        RD_ABS_DONE_1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - read for JTAG_disable"]
    #[inline(always)]
    pub fn rd_disable_jtag(&self) -> RD_DISABLE_JTAG_R {
        RD_DISABLE_JTAG_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - read for download_dis_encrypt"]
    #[inline(always)]
    pub fn rd_disable_dl_encrypt(&self) -> RD_DISABLE_DL_ENCRYPT_R {
        RD_DISABLE_DL_ENCRYPT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - read for download_dis_decrypt"]
    #[inline(always)]
    pub fn rd_disable_dl_decrypt(&self) -> RD_DISABLE_DL_DECRYPT_R {
        RD_DISABLE_DL_DECRYPT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - read for download_dis_cache"]
    #[inline(always)]
    pub fn rd_disable_dl_cache(&self) -> RD_DISABLE_DL_CACHE_R {
        RD_DISABLE_DL_CACHE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - read for key_status"]
    #[inline(always)]
    pub fn rd_key_status(&self) -> RD_KEY_STATUS_R {
        RD_KEY_STATUS_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK0_RDATA6")
            .field(
                "rd_coding_scheme",
                &format_args!("{}", self.rd_coding_scheme().bits()),
            )
            .field(
                "rd_console_debug_disable",
                &format_args!("{}", self.rd_console_debug_disable().bit()),
            )
            .field(
                "rd_disable_sdio_host",
                &format_args!("{}", self.rd_disable_sdio_host().bit()),
            )
            .field(
                "rd_abs_done_0",
                &format_args!("{}", self.rd_abs_done_0().bit()),
            )
            .field(
                "rd_abs_done_1",
                &format_args!("{}", self.rd_abs_done_1().bit()),
            )
            .field(
                "rd_disable_jtag",
                &format_args!("{}", self.rd_disable_jtag().bit()),
            )
            .field(
                "rd_disable_dl_encrypt",
                &format_args!("{}", self.rd_disable_dl_encrypt().bit()),
            )
            .field(
                "rd_disable_dl_decrypt",
                &format_args!("{}", self.rd_disable_dl_decrypt().bit()),
            )
            .field(
                "rd_disable_dl_cache",
                &format_args!("{}", self.rd_disable_dl_cache().bit()),
            )
            .field(
                "rd_key_status",
                &format_args!("{}", self.rd_key_status().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BLK0_RDATA6_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blk0_rdata6](index.html) module"]
pub struct BLK0_RDATA6_SPEC;
impl crate::RegisterSpec for BLK0_RDATA6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [blk0_rdata6::R](R) reader structure"]
impl crate::Readable for BLK0_RDATA6_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BLK0_RDATA6 to value 0"]
impl crate::Resettable for BLK0_RDATA6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
