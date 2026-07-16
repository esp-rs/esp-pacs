#[doc = "Register `RESULT_ST` reader"]
pub type R = crate::R<RESULT_ST_SPEC>;
#[doc = "Field `REUSE_JTAG_ST` reader - The state of reuse jtag register."]
pub type REUSE_JTAG_ST_R = crate::BitReader;
#[doc = "Field `REUSE_DOWNLOAD_ST` reader - The state of reuse download register."]
pub type REUSE_DOWNLOAD_ST_R = crate::BitReader;
#[doc = "Field `FORCE_SPI_BOOT_ST` reader - The state of force spi boot reg register."]
pub type FORCE_SPI_BOOT_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The state of reuse jtag register."]
    #[inline(always)]
    pub fn reuse_jtag_st(&self) -> REUSE_JTAG_ST_R {
        REUSE_JTAG_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The state of reuse download register."]
    #[inline(always)]
    pub fn reuse_download_st(&self) -> REUSE_DOWNLOAD_ST_R {
        REUSE_DOWNLOAD_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The state of force spi boot reg register."]
    #[inline(always)]
    pub fn force_spi_boot_st(&self) -> FORCE_SPI_BOOT_ST_R {
        FORCE_SPI_BOOT_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RESULT_ST")
            .field("reuse_jtag_st", &self.reuse_jtag_st())
            .field("reuse_download_st", &self.reuse_download_st())
            .field("force_spi_boot_st", &self.force_spi_boot_st())
            .finish()
    }
}
#[doc = "RMA result reg.\n\nYou can [`read`](crate::Reg::read) this register and get [`result_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RESULT_ST_SPEC;
impl crate::RegisterSpec for RESULT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`result_st::R`](R) reader structure"]
impl crate::Readable for RESULT_ST_SPEC {}
#[doc = "`reset()` method sets RESULT_ST to value 0"]
impl crate::Resettable for RESULT_ST_SPEC {}
