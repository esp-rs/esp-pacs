#[doc = "Register `PERIP_CLK_EN1` reader"]
pub struct R(crate::R<PERIP_CLK_EN1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERIP_CLK_EN1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERIP_CLK_EN1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERIP_CLK_EN1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PERIP_CLK_EN1` writer"]
pub struct W(crate::W<PERIP_CLK_EN1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PERIP_CLK_EN1_SPEC>;
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
impl From<crate::W<PERIP_CLK_EN1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PERIP_CLK_EN1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PERI_BACKUP_CLK_EN` reader - Set 1 to enable BACKUP clock"]
pub type PERI_BACKUP_CLK_EN_R = crate::BitReader;
#[doc = "Field `PERI_BACKUP_CLK_EN` writer - Set 1 to enable BACKUP clock"]
pub type PERI_BACKUP_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, PERIP_CLK_EN1_SPEC, O>;
#[doc = "Field `CRYPTO_AES_CLK_EN` reader - Set 1 to enable AES clock"]
pub type CRYPTO_AES_CLK_EN_R = crate::BitReader;
#[doc = "Field `CRYPTO_AES_CLK_EN` writer - Set 1 to enable AES clock"]
pub type CRYPTO_AES_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, PERIP_CLK_EN1_SPEC, O>;
#[doc = "Field `CRYPTO_SHA_CLK_EN` reader - Set 1 to enable SHA clock"]
pub type CRYPTO_SHA_CLK_EN_R = crate::BitReader;
#[doc = "Field `CRYPTO_SHA_CLK_EN` writer - Set 1 to enable SHA clock"]
pub type CRYPTO_SHA_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, PERIP_CLK_EN1_SPEC, O>;
#[doc = "Field `CRYPTO_RSA_CLK_EN` reader - Set 1 to enable RSA clock"]
pub type CRYPTO_RSA_CLK_EN_R = crate::BitReader;
#[doc = "Field `CRYPTO_RSA_CLK_EN` writer - Set 1 to enable RSA clock"]
pub type CRYPTO_RSA_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, PERIP_CLK_EN1_SPEC, O>;
#[doc = "Field `CRYPTO_DS_CLK_EN` reader - Set 1 to enable DS clock"]
pub type CRYPTO_DS_CLK_EN_R = crate::BitReader;
#[doc = "Field `CRYPTO_DS_CLK_EN` writer - Set 1 to enable DS clock"]
pub type CRYPTO_DS_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, PERIP_CLK_EN1_SPEC, O>;
#[doc = "Field `CRYPTO_HMAC_CLK_EN` reader - Set 1 to enable HMAC clock"]
pub type CRYPTO_HMAC_CLK_EN_R = crate::BitReader;
#[doc = "Field `CRYPTO_HMAC_CLK_EN` writer - Set 1 to enable HMAC clock"]
pub type CRYPTO_HMAC_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, PERIP_CLK_EN1_SPEC, O>;
#[doc = "Field `DMA_CLK_EN` reader - Set 1 to enable DMA clock"]
pub type DMA_CLK_EN_R = crate::BitReader;
#[doc = "Field `DMA_CLK_EN` writer - Set 1 to enable DMA clock"]
pub type DMA_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, PERIP_CLK_EN1_SPEC, O>;
#[doc = "Field `SDIO_HOST_CLK_EN` reader - Set 1 to enable SDIO_HOST clock"]
pub type SDIO_HOST_CLK_EN_R = crate::BitReader;
#[doc = "Field `SDIO_HOST_CLK_EN` writer - Set 1 to enable SDIO_HOST clock"]
pub type SDIO_HOST_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, PERIP_CLK_EN1_SPEC, O>;
#[doc = "Field `LCD_CAM_CLK_EN` reader - Set 1 to enable LCD_CAM clock"]
pub type LCD_CAM_CLK_EN_R = crate::BitReader;
#[doc = "Field `LCD_CAM_CLK_EN` writer - Set 1 to enable LCD_CAM clock"]
pub type LCD_CAM_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, PERIP_CLK_EN1_SPEC, O>;
#[doc = "Field `UART2_CLK_EN` reader - Set 1 to enable UART2 clock"]
pub type UART2_CLK_EN_R = crate::BitReader;
#[doc = "Field `UART2_CLK_EN` writer - Set 1 to enable UART2 clock"]
pub type UART2_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, PERIP_CLK_EN1_SPEC, O>;
#[doc = "Field `USB_DEVICE_CLK_EN` reader - Set 1 to enable USB_DEVICE clock"]
pub type USB_DEVICE_CLK_EN_R = crate::BitReader;
#[doc = "Field `USB_DEVICE_CLK_EN` writer - Set 1 to enable USB_DEVICE clock"]
pub type USB_DEVICE_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, PERIP_CLK_EN1_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Set 1 to enable BACKUP clock"]
    #[inline(always)]
    pub fn peri_backup_clk_en(&self) -> PERI_BACKUP_CLK_EN_R {
        PERI_BACKUP_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set 1 to enable AES clock"]
    #[inline(always)]
    pub fn crypto_aes_clk_en(&self) -> CRYPTO_AES_CLK_EN_R {
        CRYPTO_AES_CLK_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set 1 to enable SHA clock"]
    #[inline(always)]
    pub fn crypto_sha_clk_en(&self) -> CRYPTO_SHA_CLK_EN_R {
        CRYPTO_SHA_CLK_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set 1 to enable RSA clock"]
    #[inline(always)]
    pub fn crypto_rsa_clk_en(&self) -> CRYPTO_RSA_CLK_EN_R {
        CRYPTO_RSA_CLK_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set 1 to enable DS clock"]
    #[inline(always)]
    pub fn crypto_ds_clk_en(&self) -> CRYPTO_DS_CLK_EN_R {
        CRYPTO_DS_CLK_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Set 1 to enable HMAC clock"]
    #[inline(always)]
    pub fn crypto_hmac_clk_en(&self) -> CRYPTO_HMAC_CLK_EN_R {
        CRYPTO_HMAC_CLK_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Set 1 to enable DMA clock"]
    #[inline(always)]
    pub fn dma_clk_en(&self) -> DMA_CLK_EN_R {
        DMA_CLK_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Set 1 to enable SDIO_HOST clock"]
    #[inline(always)]
    pub fn sdio_host_clk_en(&self) -> SDIO_HOST_CLK_EN_R {
        SDIO_HOST_CLK_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Set 1 to enable LCD_CAM clock"]
    #[inline(always)]
    pub fn lcd_cam_clk_en(&self) -> LCD_CAM_CLK_EN_R {
        LCD_CAM_CLK_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Set 1 to enable UART2 clock"]
    #[inline(always)]
    pub fn uart2_clk_en(&self) -> UART2_CLK_EN_R {
        UART2_CLK_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Set 1 to enable USB_DEVICE clock"]
    #[inline(always)]
    pub fn usb_device_clk_en(&self) -> USB_DEVICE_CLK_EN_R {
        USB_DEVICE_CLK_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERIP_CLK_EN1")
            .field(
                "peri_backup_clk_en",
                &format_args!("{}", self.peri_backup_clk_en().bit()),
            )
            .field(
                "crypto_aes_clk_en",
                &format_args!("{}", self.crypto_aes_clk_en().bit()),
            )
            .field(
                "crypto_sha_clk_en",
                &format_args!("{}", self.crypto_sha_clk_en().bit()),
            )
            .field(
                "crypto_rsa_clk_en",
                &format_args!("{}", self.crypto_rsa_clk_en().bit()),
            )
            .field(
                "crypto_ds_clk_en",
                &format_args!("{}", self.crypto_ds_clk_en().bit()),
            )
            .field(
                "crypto_hmac_clk_en",
                &format_args!("{}", self.crypto_hmac_clk_en().bit()),
            )
            .field("dma_clk_en", &format_args!("{}", self.dma_clk_en().bit()))
            .field(
                "sdio_host_clk_en",
                &format_args!("{}", self.sdio_host_clk_en().bit()),
            )
            .field(
                "lcd_cam_clk_en",
                &format_args!("{}", self.lcd_cam_clk_en().bit()),
            )
            .field(
                "uart2_clk_en",
                &format_args!("{}", self.uart2_clk_en().bit()),
            )
            .field(
                "usb_device_clk_en",
                &format_args!("{}", self.usb_device_clk_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PERIP_CLK_EN1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to enable BACKUP clock"]
    #[inline(always)]
    #[must_use]
    pub fn peri_backup_clk_en(&mut self) -> PERI_BACKUP_CLK_EN_W<0> {
        PERI_BACKUP_CLK_EN_W::new(self)
    }
    #[doc = "Bit 1 - Set 1 to enable AES clock"]
    #[inline(always)]
    #[must_use]
    pub fn crypto_aes_clk_en(&mut self) -> CRYPTO_AES_CLK_EN_W<1> {
        CRYPTO_AES_CLK_EN_W::new(self)
    }
    #[doc = "Bit 2 - Set 1 to enable SHA clock"]
    #[inline(always)]
    #[must_use]
    pub fn crypto_sha_clk_en(&mut self) -> CRYPTO_SHA_CLK_EN_W<2> {
        CRYPTO_SHA_CLK_EN_W::new(self)
    }
    #[doc = "Bit 3 - Set 1 to enable RSA clock"]
    #[inline(always)]
    #[must_use]
    pub fn crypto_rsa_clk_en(&mut self) -> CRYPTO_RSA_CLK_EN_W<3> {
        CRYPTO_RSA_CLK_EN_W::new(self)
    }
    #[doc = "Bit 4 - Set 1 to enable DS clock"]
    #[inline(always)]
    #[must_use]
    pub fn crypto_ds_clk_en(&mut self) -> CRYPTO_DS_CLK_EN_W<4> {
        CRYPTO_DS_CLK_EN_W::new(self)
    }
    #[doc = "Bit 5 - Set 1 to enable HMAC clock"]
    #[inline(always)]
    #[must_use]
    pub fn crypto_hmac_clk_en(&mut self) -> CRYPTO_HMAC_CLK_EN_W<5> {
        CRYPTO_HMAC_CLK_EN_W::new(self)
    }
    #[doc = "Bit 6 - Set 1 to enable DMA clock"]
    #[inline(always)]
    #[must_use]
    pub fn dma_clk_en(&mut self) -> DMA_CLK_EN_W<6> {
        DMA_CLK_EN_W::new(self)
    }
    #[doc = "Bit 7 - Set 1 to enable SDIO_HOST clock"]
    #[inline(always)]
    #[must_use]
    pub fn sdio_host_clk_en(&mut self) -> SDIO_HOST_CLK_EN_W<7> {
        SDIO_HOST_CLK_EN_W::new(self)
    }
    #[doc = "Bit 8 - Set 1 to enable LCD_CAM clock"]
    #[inline(always)]
    #[must_use]
    pub fn lcd_cam_clk_en(&mut self) -> LCD_CAM_CLK_EN_W<8> {
        LCD_CAM_CLK_EN_W::new(self)
    }
    #[doc = "Bit 9 - Set 1 to enable UART2 clock"]
    #[inline(always)]
    #[must_use]
    pub fn uart2_clk_en(&mut self) -> UART2_CLK_EN_W<9> {
        UART2_CLK_EN_W::new(self)
    }
    #[doc = "Bit 10 - Set 1 to enable USB_DEVICE clock"]
    #[inline(always)]
    #[must_use]
    pub fn usb_device_clk_en(&mut self) -> USB_DEVICE_CLK_EN_W<10> {
        USB_DEVICE_CLK_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "peripheral clock configuration regsiter 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [perip_clk_en1](index.html) module"]
pub struct PERIP_CLK_EN1_SPEC;
impl crate::RegisterSpec for PERIP_CLK_EN1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [perip_clk_en1::R](R) reader structure"]
impl crate::Readable for PERIP_CLK_EN1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [perip_clk_en1::W](W) writer structure"]
impl crate::Writable for PERIP_CLK_EN1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PERIP_CLK_EN1 to value 0x0600"]
impl crate::Resettable for PERIP_CLK_EN1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0600;
}
