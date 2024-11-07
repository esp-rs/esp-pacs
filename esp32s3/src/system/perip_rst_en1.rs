#[doc = "Register `PERIP_RST_EN1` reader"]
pub type R = crate::R<PERIP_RST_EN1_SPEC>;
#[doc = "Register `PERIP_RST_EN1` writer"]
pub type W = crate::W<PERIP_RST_EN1_SPEC>;
#[doc = "Field `PERI_BACKUP_RST` reader - Set 1 to let BACKUP reset"]
pub type PERI_BACKUP_RST_R = crate::BitReader;
#[doc = "Field `PERI_BACKUP_RST` writer - Set 1 to let BACKUP reset"]
pub type PERI_BACKUP_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRYPTO_AES_RST` reader - Set 1 to let CRYPTO_AES reset"]
pub type CRYPTO_AES_RST_R = crate::BitReader;
#[doc = "Field `CRYPTO_AES_RST` writer - Set 1 to let CRYPTO_AES reset"]
pub type CRYPTO_AES_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRYPTO_SHA_RST` reader - Set 1 to let CRYPTO_SHA reset"]
pub type CRYPTO_SHA_RST_R = crate::BitReader;
#[doc = "Field `CRYPTO_SHA_RST` writer - Set 1 to let CRYPTO_SHA reset"]
pub type CRYPTO_SHA_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRYPTO_RSA_RST` reader - Set 1 to let CRYPTO_RSA reset"]
pub type CRYPTO_RSA_RST_R = crate::BitReader;
#[doc = "Field `CRYPTO_RSA_RST` writer - Set 1 to let CRYPTO_RSA reset"]
pub type CRYPTO_RSA_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRYPTO_DS_RST` reader - Set 1 to let CRYPTO_DS reset"]
pub type CRYPTO_DS_RST_R = crate::BitReader;
#[doc = "Field `CRYPTO_DS_RST` writer - Set 1 to let CRYPTO_DS reset"]
pub type CRYPTO_DS_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRYPTO_HMAC_RST` reader - Set 1 to let CRYPTO_HMAC reset"]
pub type CRYPTO_HMAC_RST_R = crate::BitReader;
#[doc = "Field `CRYPTO_HMAC_RST` writer - Set 1 to let CRYPTO_HMAC reset"]
pub type CRYPTO_HMAC_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_RST` reader - Set 1 to let DMA reset"]
pub type DMA_RST_R = crate::BitReader;
#[doc = "Field `DMA_RST` writer - Set 1 to let DMA reset"]
pub type DMA_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIO_HOST_RST` reader - Set 1 to let SDIO_HOST reset"]
pub type SDIO_HOST_RST_R = crate::BitReader;
#[doc = "Field `SDIO_HOST_RST` writer - Set 1 to let SDIO_HOST reset"]
pub type SDIO_HOST_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCD_CAM_RST` reader - Set 1 to let LCD_CAM reset"]
pub type LCD_CAM_RST_R = crate::BitReader;
#[doc = "Field `LCD_CAM_RST` writer - Set 1 to let LCD_CAM reset"]
pub type LCD_CAM_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART2_RST` reader - Set 1 to let UART2 reset"]
pub type UART2_RST_R = crate::BitReader;
#[doc = "Field `UART2_RST` writer - Set 1 to let UART2 reset"]
pub type UART2_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_DEVICE_RST` reader - Set 1 to let USB_DEVICE reset"]
pub type USB_DEVICE_RST_R = crate::BitReader;
#[doc = "Field `USB_DEVICE_RST` writer - Set 1 to let USB_DEVICE reset"]
pub type USB_DEVICE_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set 1 to let BACKUP reset"]
    #[inline(always)]
    pub fn peri_backup_rst(&self) -> PERI_BACKUP_RST_R {
        PERI_BACKUP_RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set 1 to let CRYPTO_AES reset"]
    #[inline(always)]
    pub fn crypto_aes_rst(&self) -> CRYPTO_AES_RST_R {
        CRYPTO_AES_RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set 1 to let CRYPTO_SHA reset"]
    #[inline(always)]
    pub fn crypto_sha_rst(&self) -> CRYPTO_SHA_RST_R {
        CRYPTO_SHA_RST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set 1 to let CRYPTO_RSA reset"]
    #[inline(always)]
    pub fn crypto_rsa_rst(&self) -> CRYPTO_RSA_RST_R {
        CRYPTO_RSA_RST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set 1 to let CRYPTO_DS reset"]
    #[inline(always)]
    pub fn crypto_ds_rst(&self) -> CRYPTO_DS_RST_R {
        CRYPTO_DS_RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Set 1 to let CRYPTO_HMAC reset"]
    #[inline(always)]
    pub fn crypto_hmac_rst(&self) -> CRYPTO_HMAC_RST_R {
        CRYPTO_HMAC_RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Set 1 to let DMA reset"]
    #[inline(always)]
    pub fn dma_rst(&self) -> DMA_RST_R {
        DMA_RST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Set 1 to let SDIO_HOST reset"]
    #[inline(always)]
    pub fn sdio_host_rst(&self) -> SDIO_HOST_RST_R {
        SDIO_HOST_RST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Set 1 to let LCD_CAM reset"]
    #[inline(always)]
    pub fn lcd_cam_rst(&self) -> LCD_CAM_RST_R {
        LCD_CAM_RST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Set 1 to let UART2 reset"]
    #[inline(always)]
    pub fn uart2_rst(&self) -> UART2_RST_R {
        UART2_RST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Set 1 to let USB_DEVICE reset"]
    #[inline(always)]
    pub fn usb_device_rst(&self) -> USB_DEVICE_RST_R {
        USB_DEVICE_RST_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERIP_RST_EN1")
            .field("peri_backup_rst", &self.peri_backup_rst())
            .field("crypto_aes_rst", &self.crypto_aes_rst())
            .field("crypto_sha_rst", &self.crypto_sha_rst())
            .field("crypto_rsa_rst", &self.crypto_rsa_rst())
            .field("crypto_ds_rst", &self.crypto_ds_rst())
            .field("crypto_hmac_rst", &self.crypto_hmac_rst())
            .field("dma_rst", &self.dma_rst())
            .field("sdio_host_rst", &self.sdio_host_rst())
            .field("lcd_cam_rst", &self.lcd_cam_rst())
            .field("uart2_rst", &self.uart2_rst())
            .field("usb_device_rst", &self.usb_device_rst())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to let BACKUP reset"]
    #[inline(always)]
    pub fn peri_backup_rst(&mut self) -> PERI_BACKUP_RST_W<PERIP_RST_EN1_SPEC> {
        PERI_BACKUP_RST_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set 1 to let CRYPTO_AES reset"]
    #[inline(always)]
    pub fn crypto_aes_rst(&mut self) -> CRYPTO_AES_RST_W<PERIP_RST_EN1_SPEC> {
        CRYPTO_AES_RST_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set 1 to let CRYPTO_SHA reset"]
    #[inline(always)]
    pub fn crypto_sha_rst(&mut self) -> CRYPTO_SHA_RST_W<PERIP_RST_EN1_SPEC> {
        CRYPTO_SHA_RST_W::new(self, 2)
    }
    #[doc = "Bit 3 - Set 1 to let CRYPTO_RSA reset"]
    #[inline(always)]
    pub fn crypto_rsa_rst(&mut self) -> CRYPTO_RSA_RST_W<PERIP_RST_EN1_SPEC> {
        CRYPTO_RSA_RST_W::new(self, 3)
    }
    #[doc = "Bit 4 - Set 1 to let CRYPTO_DS reset"]
    #[inline(always)]
    pub fn crypto_ds_rst(&mut self) -> CRYPTO_DS_RST_W<PERIP_RST_EN1_SPEC> {
        CRYPTO_DS_RST_W::new(self, 4)
    }
    #[doc = "Bit 5 - Set 1 to let CRYPTO_HMAC reset"]
    #[inline(always)]
    pub fn crypto_hmac_rst(&mut self) -> CRYPTO_HMAC_RST_W<PERIP_RST_EN1_SPEC> {
        CRYPTO_HMAC_RST_W::new(self, 5)
    }
    #[doc = "Bit 6 - Set 1 to let DMA reset"]
    #[inline(always)]
    pub fn dma_rst(&mut self) -> DMA_RST_W<PERIP_RST_EN1_SPEC> {
        DMA_RST_W::new(self, 6)
    }
    #[doc = "Bit 7 - Set 1 to let SDIO_HOST reset"]
    #[inline(always)]
    pub fn sdio_host_rst(&mut self) -> SDIO_HOST_RST_W<PERIP_RST_EN1_SPEC> {
        SDIO_HOST_RST_W::new(self, 7)
    }
    #[doc = "Bit 8 - Set 1 to let LCD_CAM reset"]
    #[inline(always)]
    pub fn lcd_cam_rst(&mut self) -> LCD_CAM_RST_W<PERIP_RST_EN1_SPEC> {
        LCD_CAM_RST_W::new(self, 8)
    }
    #[doc = "Bit 9 - Set 1 to let UART2 reset"]
    #[inline(always)]
    pub fn uart2_rst(&mut self) -> UART2_RST_W<PERIP_RST_EN1_SPEC> {
        UART2_RST_W::new(self, 9)
    }
    #[doc = "Bit 10 - Set 1 to let USB_DEVICE reset"]
    #[inline(always)]
    pub fn usb_device_rst(&mut self) -> USB_DEVICE_RST_W<PERIP_RST_EN1_SPEC> {
        USB_DEVICE_RST_W::new(self, 10)
    }
}
#[doc = "peripheral reset configuration regsiter 1\n\nYou can [`read`](crate::Reg::read) this register and get [`perip_rst_en1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`perip_rst_en1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PERIP_RST_EN1_SPEC;
impl crate::RegisterSpec for PERIP_RST_EN1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`perip_rst_en1::R`](R) reader structure"]
impl crate::Readable for PERIP_RST_EN1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`perip_rst_en1::W`](W) writer structure"]
impl crate::Writable for PERIP_RST_EN1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PERIP_RST_EN1 to value 0x01fe"]
impl crate::Resettable for PERIP_RST_EN1_SPEC {
    const RESET_VALUE: u32 = 0x01fe;
}
