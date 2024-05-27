///Register `PERIP_CLK_EN1` reader
pub type R = crate::R<PERIP_CLK_EN1_SPEC>;
///Register `PERIP_CLK_EN1` writer
pub type W = crate::W<PERIP_CLK_EN1_SPEC>;
///Field `PERI_BACKUP_CLK_EN` reader - Set 1 to enable BACKUP clock
pub type PERI_BACKUP_CLK_EN_R = crate::BitReader;
///Field `PERI_BACKUP_CLK_EN` writer - Set 1 to enable BACKUP clock
pub type PERI_BACKUP_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRYPTO_AES_CLK_EN` reader - Set 1 to enable AES clock
pub type CRYPTO_AES_CLK_EN_R = crate::BitReader;
///Field `CRYPTO_AES_CLK_EN` writer - Set 1 to enable AES clock
pub type CRYPTO_AES_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRYPTO_SHA_CLK_EN` reader - Set 1 to enable SHA clock
pub type CRYPTO_SHA_CLK_EN_R = crate::BitReader;
///Field `CRYPTO_SHA_CLK_EN` writer - Set 1 to enable SHA clock
pub type CRYPTO_SHA_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRYPTO_RSA_CLK_EN` reader - Set 1 to enable RSA clock
pub type CRYPTO_RSA_CLK_EN_R = crate::BitReader;
///Field `CRYPTO_RSA_CLK_EN` writer - Set 1 to enable RSA clock
pub type CRYPTO_RSA_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRYPTO_DS_CLK_EN` reader - Set 1 to enable DS clock
pub type CRYPTO_DS_CLK_EN_R = crate::BitReader;
///Field `CRYPTO_DS_CLK_EN` writer - Set 1 to enable DS clock
pub type CRYPTO_DS_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CRYPTO_HMAC_CLK_EN` reader - Set 1 to enable HMAC clock
pub type CRYPTO_HMAC_CLK_EN_R = crate::BitReader;
///Field `CRYPTO_HMAC_CLK_EN` writer - Set 1 to enable HMAC clock
pub type CRYPTO_HMAC_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `DMA_CLK_EN` reader - Set 1 to enable DMA clock
pub type DMA_CLK_EN_R = crate::BitReader;
///Field `DMA_CLK_EN` writer - Set 1 to enable DMA clock
pub type DMA_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SDIO_HOST_CLK_EN` reader - Set 1 to enable SDIO_HOST clock
pub type SDIO_HOST_CLK_EN_R = crate::BitReader;
///Field `SDIO_HOST_CLK_EN` writer - Set 1 to enable SDIO_HOST clock
pub type SDIO_HOST_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `LCD_CAM_CLK_EN` reader - Set 1 to enable LCD_CAM clock
pub type LCD_CAM_CLK_EN_R = crate::BitReader;
///Field `LCD_CAM_CLK_EN` writer - Set 1 to enable LCD_CAM clock
pub type LCD_CAM_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `UART2_CLK_EN` reader - Set 1 to enable UART2 clock
pub type UART2_CLK_EN_R = crate::BitReader;
///Field `UART2_CLK_EN` writer - Set 1 to enable UART2 clock
pub type UART2_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USB_DEVICE_CLK_EN` reader - Set 1 to enable USB_DEVICE clock
pub type USB_DEVICE_CLK_EN_R = crate::BitReader;
///Field `USB_DEVICE_CLK_EN` writer - Set 1 to enable USB_DEVICE clock
pub type USB_DEVICE_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Set 1 to enable BACKUP clock
    #[inline(always)]
    pub fn peri_backup_clk_en(&self) -> PERI_BACKUP_CLK_EN_R {
        PERI_BACKUP_CLK_EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Set 1 to enable AES clock
    #[inline(always)]
    pub fn crypto_aes_clk_en(&self) -> CRYPTO_AES_CLK_EN_R {
        CRYPTO_AES_CLK_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Set 1 to enable SHA clock
    #[inline(always)]
    pub fn crypto_sha_clk_en(&self) -> CRYPTO_SHA_CLK_EN_R {
        CRYPTO_SHA_CLK_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Set 1 to enable RSA clock
    #[inline(always)]
    pub fn crypto_rsa_clk_en(&self) -> CRYPTO_RSA_CLK_EN_R {
        CRYPTO_RSA_CLK_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Set 1 to enable DS clock
    #[inline(always)]
    pub fn crypto_ds_clk_en(&self) -> CRYPTO_DS_CLK_EN_R {
        CRYPTO_DS_CLK_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Set 1 to enable HMAC clock
    #[inline(always)]
    pub fn crypto_hmac_clk_en(&self) -> CRYPTO_HMAC_CLK_EN_R {
        CRYPTO_HMAC_CLK_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Set 1 to enable DMA clock
    #[inline(always)]
    pub fn dma_clk_en(&self) -> DMA_CLK_EN_R {
        DMA_CLK_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Set 1 to enable SDIO_HOST clock
    #[inline(always)]
    pub fn sdio_host_clk_en(&self) -> SDIO_HOST_CLK_EN_R {
        SDIO_HOST_CLK_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Set 1 to enable LCD_CAM clock
    #[inline(always)]
    pub fn lcd_cam_clk_en(&self) -> LCD_CAM_CLK_EN_R {
        LCD_CAM_CLK_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - Set 1 to enable UART2 clock
    #[inline(always)]
    pub fn uart2_clk_en(&self) -> UART2_CLK_EN_R {
        UART2_CLK_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - Set 1 to enable USB_DEVICE clock
    #[inline(always)]
    pub fn usb_device_clk_en(&self) -> USB_DEVICE_CLK_EN_R {
        USB_DEVICE_CLK_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERIP_CLK_EN1")
            .field("peri_backup_clk_en", &self.peri_backup_clk_en())
            .field("crypto_aes_clk_en", &self.crypto_aes_clk_en())
            .field("crypto_sha_clk_en", &self.crypto_sha_clk_en())
            .field("crypto_rsa_clk_en", &self.crypto_rsa_clk_en())
            .field("crypto_ds_clk_en", &self.crypto_ds_clk_en())
            .field("crypto_hmac_clk_en", &self.crypto_hmac_clk_en())
            .field("dma_clk_en", &self.dma_clk_en())
            .field("sdio_host_clk_en", &self.sdio_host_clk_en())
            .field("lcd_cam_clk_en", &self.lcd_cam_clk_en())
            .field("uart2_clk_en", &self.uart2_clk_en())
            .field("usb_device_clk_en", &self.usb_device_clk_en())
            .finish()
    }
}
impl W {
    ///Bit 0 - Set 1 to enable BACKUP clock
    #[inline(always)]
    #[must_use]
    pub fn peri_backup_clk_en(&mut self) -> PERI_BACKUP_CLK_EN_W<PERIP_CLK_EN1_SPEC> {
        PERI_BACKUP_CLK_EN_W::new(self, 0)
    }
    ///Bit 1 - Set 1 to enable AES clock
    #[inline(always)]
    #[must_use]
    pub fn crypto_aes_clk_en(&mut self) -> CRYPTO_AES_CLK_EN_W<PERIP_CLK_EN1_SPEC> {
        CRYPTO_AES_CLK_EN_W::new(self, 1)
    }
    ///Bit 2 - Set 1 to enable SHA clock
    #[inline(always)]
    #[must_use]
    pub fn crypto_sha_clk_en(&mut self) -> CRYPTO_SHA_CLK_EN_W<PERIP_CLK_EN1_SPEC> {
        CRYPTO_SHA_CLK_EN_W::new(self, 2)
    }
    ///Bit 3 - Set 1 to enable RSA clock
    #[inline(always)]
    #[must_use]
    pub fn crypto_rsa_clk_en(&mut self) -> CRYPTO_RSA_CLK_EN_W<PERIP_CLK_EN1_SPEC> {
        CRYPTO_RSA_CLK_EN_W::new(self, 3)
    }
    ///Bit 4 - Set 1 to enable DS clock
    #[inline(always)]
    #[must_use]
    pub fn crypto_ds_clk_en(&mut self) -> CRYPTO_DS_CLK_EN_W<PERIP_CLK_EN1_SPEC> {
        CRYPTO_DS_CLK_EN_W::new(self, 4)
    }
    ///Bit 5 - Set 1 to enable HMAC clock
    #[inline(always)]
    #[must_use]
    pub fn crypto_hmac_clk_en(&mut self) -> CRYPTO_HMAC_CLK_EN_W<PERIP_CLK_EN1_SPEC> {
        CRYPTO_HMAC_CLK_EN_W::new(self, 5)
    }
    ///Bit 6 - Set 1 to enable DMA clock
    #[inline(always)]
    #[must_use]
    pub fn dma_clk_en(&mut self) -> DMA_CLK_EN_W<PERIP_CLK_EN1_SPEC> {
        DMA_CLK_EN_W::new(self, 6)
    }
    ///Bit 7 - Set 1 to enable SDIO_HOST clock
    #[inline(always)]
    #[must_use]
    pub fn sdio_host_clk_en(&mut self) -> SDIO_HOST_CLK_EN_W<PERIP_CLK_EN1_SPEC> {
        SDIO_HOST_CLK_EN_W::new(self, 7)
    }
    ///Bit 8 - Set 1 to enable LCD_CAM clock
    #[inline(always)]
    #[must_use]
    pub fn lcd_cam_clk_en(&mut self) -> LCD_CAM_CLK_EN_W<PERIP_CLK_EN1_SPEC> {
        LCD_CAM_CLK_EN_W::new(self, 8)
    }
    ///Bit 9 - Set 1 to enable UART2 clock
    #[inline(always)]
    #[must_use]
    pub fn uart2_clk_en(&mut self) -> UART2_CLK_EN_W<PERIP_CLK_EN1_SPEC> {
        UART2_CLK_EN_W::new(self, 9)
    }
    ///Bit 10 - Set 1 to enable USB_DEVICE clock
    #[inline(always)]
    #[must_use]
    pub fn usb_device_clk_en(&mut self) -> USB_DEVICE_CLK_EN_W<PERIP_CLK_EN1_SPEC> {
        USB_DEVICE_CLK_EN_W::new(self, 10)
    }
}
/**peripheral clock configuration regsiter 1

You can [`read`](crate::generic::Reg::read) this register and get [`perip_clk_en1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`perip_clk_en1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PERIP_CLK_EN1_SPEC;
impl crate::RegisterSpec for PERIP_CLK_EN1_SPEC {
    type Ux = u32;
}
///`read()` method returns [`perip_clk_en1::R`](R) reader structure
impl crate::Readable for PERIP_CLK_EN1_SPEC {}
///`write(|w| ..)` method takes [`perip_clk_en1::W`](W) writer structure
impl crate::Writable for PERIP_CLK_EN1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets PERIP_CLK_EN1 to value 0x0600
impl crate::Resettable for PERIP_CLK_EN1_SPEC {
    const RESET_VALUE: u32 = 0x0600;
}
