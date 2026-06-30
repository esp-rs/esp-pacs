#[doc = "Register `CONFIG` reader"]
pub type R = crate::R<CONFIG_SPEC>;
#[doc = "Register `CONFIG` writer"]
pub type W = crate::W<CONFIG_SPEC>;
#[doc = "Field `WORK_MODE` reader - Configures the RMA module. \\\\ 00: REQUEST CERTIFICATE\\\\ 01: VERIFICATION CERTIFICATE\\\\ 10: FAST VERIFICATION\\\\"]
pub type WORK_MODE_R = crate::FieldReader;
#[doc = "Field `WORK_MODE` writer - Configures the RMA module. \\\\ 00: REQUEST CERTIFICATE\\\\ 01: VERIFICATION CERTIFICATE\\\\ 10: FAST VERIFICATION\\\\"]
pub type WORK_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REUSE_JTAG` reader - Whether enable JTAG after RMA pass. \\\\ 0: Do not enable JTAG \\\\ 1: Enable JTAG \\\\"]
pub type REUSE_JTAG_R = crate::BitReader;
#[doc = "Field `REUSE_JTAG` writer - Whether enable JTAG after RMA pass. \\\\ 0: Do not enable JTAG \\\\ 1: Enable JTAG \\\\"]
pub type REUSE_JTAG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REUSE_DOWNLOAD` reader - Whether enable DOWNLOAD_MODE after RMA pass. \\\\ 0: Do not enable DOWNLOAD mode\\\\ 1: Enable DOWNLOAD\\\\"]
pub type REUSE_DOWNLOAD_R = crate::BitReader;
#[doc = "Field `REUSE_DOWNLOAD` writer - Whether enable DOWNLOAD_MODE after RMA pass. \\\\ 0: Do not enable DOWNLOAD mode\\\\ 1: Enable DOWNLOAD\\\\"]
pub type REUSE_DOWNLOAD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_SPI` reader - Whether enable FORCE_SPI_BOOT after RMA pass. \\\\ 0: Do not enable FORCE_SPI_BOOT mode\\\\ 1: Enable FORCE_SPI_BOOT\\\\"]
pub type FORCE_SPI_R = crate::BitReader;
#[doc = "Field `FORCE_SPI` writer - Whether enable FORCE_SPI_BOOT after RMA pass. \\\\ 0: Do not enable FORCE_SPI_BOOT mode\\\\ 1: Enable FORCE_SPI_BOOT\\\\"]
pub type FORCE_SPI_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USE_NONCE` reader - Whether use NONCE. \\\\ 0: Do not use NONCE\\\\ 1: Use NONCE\\\\"]
pub type USE_NONCE_R = crate::BitReader;
#[doc = "Field `USE_NONCE` writer - Whether use NONCE. \\\\ 0: Do not use NONCE\\\\ 1: Use NONCE\\\\"]
pub type USE_NONCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USE_KM` reader - Whether use km to gnerate NONCE. \\\\ 0: Do not use KM to generate NONCE \\\\ 1: Use KM to generate NONCE\\\\"]
pub type USE_KM_R = crate::BitReader;
#[doc = "Field `USE_KM` writer - Whether use km to gnerate NONCE. \\\\ 0: Do not use KM to generate NONCE \\\\ 1: Use KM to generate NONCE\\\\"]
pub type USE_KM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USC_BLOCK_NUM` reader - The length of USC."]
pub type USC_BLOCK_NUM_R = crate::FieldReader;
#[doc = "Field `USC_BLOCK_NUM` writer - The length of USC."]
pub type USC_BLOCK_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:1 - Configures the RMA module. \\\\ 00: REQUEST CERTIFICATE\\\\ 01: VERIFICATION CERTIFICATE\\\\ 10: FAST VERIFICATION\\\\"]
    #[inline(always)]
    pub fn work_mode(&self) -> WORK_MODE_R {
        WORK_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - Whether enable JTAG after RMA pass. \\\\ 0: Do not enable JTAG \\\\ 1: Enable JTAG \\\\"]
    #[inline(always)]
    pub fn reuse_jtag(&self) -> REUSE_JTAG_R {
        REUSE_JTAG_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Whether enable DOWNLOAD_MODE after RMA pass. \\\\ 0: Do not enable DOWNLOAD mode\\\\ 1: Enable DOWNLOAD\\\\"]
    #[inline(always)]
    pub fn reuse_download(&self) -> REUSE_DOWNLOAD_R {
        REUSE_DOWNLOAD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Whether enable FORCE_SPI_BOOT after RMA pass. \\\\ 0: Do not enable FORCE_SPI_BOOT mode\\\\ 1: Enable FORCE_SPI_BOOT\\\\"]
    #[inline(always)]
    pub fn force_spi(&self) -> FORCE_SPI_R {
        FORCE_SPI_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Whether use NONCE. \\\\ 0: Do not use NONCE\\\\ 1: Use NONCE\\\\"]
    #[inline(always)]
    pub fn use_nonce(&self) -> USE_NONCE_R {
        USE_NONCE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Whether use km to gnerate NONCE. \\\\ 0: Do not use KM to generate NONCE \\\\ 1: Use KM to generate NONCE\\\\"]
    #[inline(always)]
    pub fn use_km(&self) -> USE_KM_R {
        USE_KM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:14 - The length of USC."]
    #[inline(always)]
    pub fn usc_block_num(&self) -> USC_BLOCK_NUM_R {
        USC_BLOCK_NUM_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONFIG")
            .field("work_mode", &self.work_mode())
            .field("reuse_jtag", &self.reuse_jtag())
            .field("reuse_download", &self.reuse_download())
            .field("force_spi", &self.force_spi())
            .field("use_nonce", &self.use_nonce())
            .field("use_km", &self.use_km())
            .field("usc_block_num", &self.usc_block_num())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Configures the RMA module. \\\\ 00: REQUEST CERTIFICATE\\\\ 01: VERIFICATION CERTIFICATE\\\\ 10: FAST VERIFICATION\\\\"]
    #[inline(always)]
    pub fn work_mode(&mut self) -> WORK_MODE_W<'_, CONFIG_SPEC> {
        WORK_MODE_W::new(self, 0)
    }
    #[doc = "Bit 3 - Whether enable JTAG after RMA pass. \\\\ 0: Do not enable JTAG \\\\ 1: Enable JTAG \\\\"]
    #[inline(always)]
    pub fn reuse_jtag(&mut self) -> REUSE_JTAG_W<'_, CONFIG_SPEC> {
        REUSE_JTAG_W::new(self, 3)
    }
    #[doc = "Bit 4 - Whether enable DOWNLOAD_MODE after RMA pass. \\\\ 0: Do not enable DOWNLOAD mode\\\\ 1: Enable DOWNLOAD\\\\"]
    #[inline(always)]
    pub fn reuse_download(&mut self) -> REUSE_DOWNLOAD_W<'_, CONFIG_SPEC> {
        REUSE_DOWNLOAD_W::new(self, 4)
    }
    #[doc = "Bit 5 - Whether enable FORCE_SPI_BOOT after RMA pass. \\\\ 0: Do not enable FORCE_SPI_BOOT mode\\\\ 1: Enable FORCE_SPI_BOOT\\\\"]
    #[inline(always)]
    pub fn force_spi(&mut self) -> FORCE_SPI_W<'_, CONFIG_SPEC> {
        FORCE_SPI_W::new(self, 5)
    }
    #[doc = "Bit 6 - Whether use NONCE. \\\\ 0: Do not use NONCE\\\\ 1: Use NONCE\\\\"]
    #[inline(always)]
    pub fn use_nonce(&mut self) -> USE_NONCE_W<'_, CONFIG_SPEC> {
        USE_NONCE_W::new(self, 6)
    }
    #[doc = "Bit 7 - Whether use km to gnerate NONCE. \\\\ 0: Do not use KM to generate NONCE \\\\ 1: Use KM to generate NONCE\\\\"]
    #[inline(always)]
    pub fn use_km(&mut self) -> USE_KM_W<'_, CONFIG_SPEC> {
        USE_KM_W::new(self, 7)
    }
    #[doc = "Bits 8:14 - The length of USC."]
    #[inline(always)]
    pub fn usc_block_num(&mut self) -> USC_BLOCK_NUM_W<'_, CONFIG_SPEC> {
        USC_BLOCK_NUM_W::new(self, 8)
    }
}
#[doc = "Configures RMA algorithm\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONFIG_SPEC;
impl crate::RegisterSpec for CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config::R`](R) reader structure"]
impl crate::Readable for CONFIG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`config::W`](W) writer structure"]
impl crate::Writable for CONFIG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONFIG to value 0"]
impl crate::Resettable for CONFIG_SPEC {}
