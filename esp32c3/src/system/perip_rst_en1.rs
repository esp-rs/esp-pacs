#[doc = "Register `PERIP_RST_EN1` reader"]
pub struct R(crate::R<PERIP_RST_EN1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERIP_RST_EN1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERIP_RST_EN1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERIP_RST_EN1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PERIP_RST_EN1` writer"]
pub struct W(crate::W<PERIP_RST_EN1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PERIP_RST_EN1_SPEC>;
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
impl From<crate::W<PERIP_RST_EN1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PERIP_RST_EN1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRYPTO_AES_RST` reader - reg_crypto_aes_rst"]
pub type CRYPTO_AES_RST_R = crate::BitReader;
#[doc = "Field `CRYPTO_AES_RST` writer - reg_crypto_aes_rst"]
pub type CRYPTO_AES_RST_W<'a, const O: u8> = crate::BitWriter<'a, PERIP_RST_EN1_SPEC, O>;
#[doc = "Field `CRYPTO_SHA_RST` reader - reg_crypto_sha_rst"]
pub type CRYPTO_SHA_RST_R = crate::BitReader;
#[doc = "Field `CRYPTO_SHA_RST` writer - reg_crypto_sha_rst"]
pub type CRYPTO_SHA_RST_W<'a, const O: u8> = crate::BitWriter<'a, PERIP_RST_EN1_SPEC, O>;
#[doc = "Field `CRYPTO_RSA_RST` reader - reg_crypto_rsa_rst"]
pub type CRYPTO_RSA_RST_R = crate::BitReader;
#[doc = "Field `CRYPTO_RSA_RST` writer - reg_crypto_rsa_rst"]
pub type CRYPTO_RSA_RST_W<'a, const O: u8> = crate::BitWriter<'a, PERIP_RST_EN1_SPEC, O>;
#[doc = "Field `CRYPTO_DS_RST` reader - reg_crypto_ds_rst"]
pub type CRYPTO_DS_RST_R = crate::BitReader;
#[doc = "Field `CRYPTO_DS_RST` writer - reg_crypto_ds_rst"]
pub type CRYPTO_DS_RST_W<'a, const O: u8> = crate::BitWriter<'a, PERIP_RST_EN1_SPEC, O>;
#[doc = "Field `CRYPTO_HMAC_RST` reader - reg_crypto_hmac_rst"]
pub type CRYPTO_HMAC_RST_R = crate::BitReader;
#[doc = "Field `CRYPTO_HMAC_RST` writer - reg_crypto_hmac_rst"]
pub type CRYPTO_HMAC_RST_W<'a, const O: u8> = crate::BitWriter<'a, PERIP_RST_EN1_SPEC, O>;
#[doc = "Field `DMA_RST` reader - reg_dma_rst"]
pub type DMA_RST_R = crate::BitReader;
#[doc = "Field `DMA_RST` writer - reg_dma_rst"]
pub type DMA_RST_W<'a, const O: u8> = crate::BitWriter<'a, PERIP_RST_EN1_SPEC, O>;
#[doc = "Field `SDIO_HOST_RST` reader - reg_sdio_host_rst"]
pub type SDIO_HOST_RST_R = crate::BitReader;
#[doc = "Field `SDIO_HOST_RST` writer - reg_sdio_host_rst"]
pub type SDIO_HOST_RST_W<'a, const O: u8> = crate::BitWriter<'a, PERIP_RST_EN1_SPEC, O>;
#[doc = "Field `LCD_CAM_RST` reader - reg_lcd_cam_rst"]
pub type LCD_CAM_RST_R = crate::BitReader;
#[doc = "Field `LCD_CAM_RST` writer - reg_lcd_cam_rst"]
pub type LCD_CAM_RST_W<'a, const O: u8> = crate::BitWriter<'a, PERIP_RST_EN1_SPEC, O>;
#[doc = "Field `UART2_RST` reader - reg_uart2_rst"]
pub type UART2_RST_R = crate::BitReader;
#[doc = "Field `UART2_RST` writer - reg_uart2_rst"]
pub type UART2_RST_W<'a, const O: u8> = crate::BitWriter<'a, PERIP_RST_EN1_SPEC, O>;
#[doc = "Field `TSENS_RST` reader - reg_tsens_rst"]
pub type TSENS_RST_R = crate::BitReader;
#[doc = "Field `TSENS_RST` writer - reg_tsens_rst"]
pub type TSENS_RST_W<'a, const O: u8> = crate::BitWriter<'a, PERIP_RST_EN1_SPEC, O>;
impl R {
    #[doc = "Bit 1 - reg_crypto_aes_rst"]
    #[inline(always)]
    pub fn crypto_aes_rst(&self) -> CRYPTO_AES_RST_R {
        CRYPTO_AES_RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - reg_crypto_sha_rst"]
    #[inline(always)]
    pub fn crypto_sha_rst(&self) -> CRYPTO_SHA_RST_R {
        CRYPTO_SHA_RST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reg_crypto_rsa_rst"]
    #[inline(always)]
    pub fn crypto_rsa_rst(&self) -> CRYPTO_RSA_RST_R {
        CRYPTO_RSA_RST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - reg_crypto_ds_rst"]
    #[inline(always)]
    pub fn crypto_ds_rst(&self) -> CRYPTO_DS_RST_R {
        CRYPTO_DS_RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - reg_crypto_hmac_rst"]
    #[inline(always)]
    pub fn crypto_hmac_rst(&self) -> CRYPTO_HMAC_RST_R {
        CRYPTO_HMAC_RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - reg_dma_rst"]
    #[inline(always)]
    pub fn dma_rst(&self) -> DMA_RST_R {
        DMA_RST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - reg_sdio_host_rst"]
    #[inline(always)]
    pub fn sdio_host_rst(&self) -> SDIO_HOST_RST_R {
        SDIO_HOST_RST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - reg_lcd_cam_rst"]
    #[inline(always)]
    pub fn lcd_cam_rst(&self) -> LCD_CAM_RST_R {
        LCD_CAM_RST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - reg_uart2_rst"]
    #[inline(always)]
    pub fn uart2_rst(&self) -> UART2_RST_R {
        UART2_RST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - reg_tsens_rst"]
    #[inline(always)]
    pub fn tsens_rst(&self) -> TSENS_RST_R {
        TSENS_RST_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERIP_RST_EN1")
            .field(
                "crypto_aes_rst",
                &format_args!("{}", self.crypto_aes_rst().bit()),
            )
            .field(
                "crypto_sha_rst",
                &format_args!("{}", self.crypto_sha_rst().bit()),
            )
            .field(
                "crypto_rsa_rst",
                &format_args!("{}", self.crypto_rsa_rst().bit()),
            )
            .field(
                "crypto_ds_rst",
                &format_args!("{}", self.crypto_ds_rst().bit()),
            )
            .field(
                "crypto_hmac_rst",
                &format_args!("{}", self.crypto_hmac_rst().bit()),
            )
            .field("dma_rst", &format_args!("{}", self.dma_rst().bit()))
            .field(
                "sdio_host_rst",
                &format_args!("{}", self.sdio_host_rst().bit()),
            )
            .field("lcd_cam_rst", &format_args!("{}", self.lcd_cam_rst().bit()))
            .field("uart2_rst", &format_args!("{}", self.uart2_rst().bit()))
            .field("tsens_rst", &format_args!("{}", self.tsens_rst().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PERIP_RST_EN1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 1 - reg_crypto_aes_rst"]
    #[inline(always)]
    #[must_use]
    pub fn crypto_aes_rst(&mut self) -> CRYPTO_AES_RST_W<1> {
        CRYPTO_AES_RST_W::new(self)
    }
    #[doc = "Bit 2 - reg_crypto_sha_rst"]
    #[inline(always)]
    #[must_use]
    pub fn crypto_sha_rst(&mut self) -> CRYPTO_SHA_RST_W<2> {
        CRYPTO_SHA_RST_W::new(self)
    }
    #[doc = "Bit 3 - reg_crypto_rsa_rst"]
    #[inline(always)]
    #[must_use]
    pub fn crypto_rsa_rst(&mut self) -> CRYPTO_RSA_RST_W<3> {
        CRYPTO_RSA_RST_W::new(self)
    }
    #[doc = "Bit 4 - reg_crypto_ds_rst"]
    #[inline(always)]
    #[must_use]
    pub fn crypto_ds_rst(&mut self) -> CRYPTO_DS_RST_W<4> {
        CRYPTO_DS_RST_W::new(self)
    }
    #[doc = "Bit 5 - reg_crypto_hmac_rst"]
    #[inline(always)]
    #[must_use]
    pub fn crypto_hmac_rst(&mut self) -> CRYPTO_HMAC_RST_W<5> {
        CRYPTO_HMAC_RST_W::new(self)
    }
    #[doc = "Bit 6 - reg_dma_rst"]
    #[inline(always)]
    #[must_use]
    pub fn dma_rst(&mut self) -> DMA_RST_W<6> {
        DMA_RST_W::new(self)
    }
    #[doc = "Bit 7 - reg_sdio_host_rst"]
    #[inline(always)]
    #[must_use]
    pub fn sdio_host_rst(&mut self) -> SDIO_HOST_RST_W<7> {
        SDIO_HOST_RST_W::new(self)
    }
    #[doc = "Bit 8 - reg_lcd_cam_rst"]
    #[inline(always)]
    #[must_use]
    pub fn lcd_cam_rst(&mut self) -> LCD_CAM_RST_W<8> {
        LCD_CAM_RST_W::new(self)
    }
    #[doc = "Bit 9 - reg_uart2_rst"]
    #[inline(always)]
    #[must_use]
    pub fn uart2_rst(&mut self) -> UART2_RST_W<9> {
        UART2_RST_W::new(self)
    }
    #[doc = "Bit 10 - reg_tsens_rst"]
    #[inline(always)]
    #[must_use]
    pub fn tsens_rst(&mut self) -> TSENS_RST_W<10> {
        TSENS_RST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "peripheral reset register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [perip_rst_en1](index.html) module"]
pub struct PERIP_RST_EN1_SPEC;
impl crate::RegisterSpec for PERIP_RST_EN1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [perip_rst_en1::R](R) reader structure"]
impl crate::Readable for PERIP_RST_EN1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [perip_rst_en1::W](W) writer structure"]
impl crate::Writable for PERIP_RST_EN1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PERIP_RST_EN1 to value 0x01fe"]
impl crate::Resettable for PERIP_RST_EN1_SPEC {
    const RESET_VALUE: Self::Ux = 0x01fe;
}
