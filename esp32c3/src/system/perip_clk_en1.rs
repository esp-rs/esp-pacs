#[doc = "Register `PERIP_CLK_EN1` reader"]
pub type R = crate::R<PERIP_CLK_EN1_SPEC>;
#[doc = "Register `PERIP_CLK_EN1` writer"]
pub type W = crate::W<PERIP_CLK_EN1_SPEC>;
#[doc = "Field `CRYPTO_AES_CLK_EN` reader - reg_crypto_aes_clk_en"]
pub type CRYPTO_AES_CLK_EN_R = crate::BitReader;
#[doc = "Field `CRYPTO_AES_CLK_EN` writer - reg_crypto_aes_clk_en"]
pub type CRYPTO_AES_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRYPTO_SHA_CLK_EN` reader - reg_crypto_sha_clk_en"]
pub type CRYPTO_SHA_CLK_EN_R = crate::BitReader;
#[doc = "Field `CRYPTO_SHA_CLK_EN` writer - reg_crypto_sha_clk_en"]
pub type CRYPTO_SHA_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRYPTO_RSA_CLK_EN` reader - reg_crypto_rsa_clk_en"]
pub type CRYPTO_RSA_CLK_EN_R = crate::BitReader;
#[doc = "Field `CRYPTO_RSA_CLK_EN` writer - reg_crypto_rsa_clk_en"]
pub type CRYPTO_RSA_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRYPTO_DS_CLK_EN` reader - reg_crypto_ds_clk_en"]
pub type CRYPTO_DS_CLK_EN_R = crate::BitReader;
#[doc = "Field `CRYPTO_DS_CLK_EN` writer - reg_crypto_ds_clk_en"]
pub type CRYPTO_DS_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRYPTO_HMAC_CLK_EN` reader - reg_crypto_hmac_clk_en"]
pub type CRYPTO_HMAC_CLK_EN_R = crate::BitReader;
#[doc = "Field `CRYPTO_HMAC_CLK_EN` writer - reg_crypto_hmac_clk_en"]
pub type CRYPTO_HMAC_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_CLK_EN` reader - reg_dma_clk_en"]
pub type DMA_CLK_EN_R = crate::BitReader;
#[doc = "Field `DMA_CLK_EN` writer - reg_dma_clk_en"]
pub type DMA_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIO_HOST_CLK_EN` reader - reg_sdio_host_clk_en"]
pub type SDIO_HOST_CLK_EN_R = crate::BitReader;
#[doc = "Field `SDIO_HOST_CLK_EN` writer - reg_sdio_host_clk_en"]
pub type SDIO_HOST_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCD_CAM_CLK_EN` reader - reg_lcd_cam_clk_en"]
pub type LCD_CAM_CLK_EN_R = crate::BitReader;
#[doc = "Field `LCD_CAM_CLK_EN` writer - reg_lcd_cam_clk_en"]
pub type LCD_CAM_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART2_CLK_EN` reader - reg_uart2_clk_en"]
pub type UART2_CLK_EN_R = crate::BitReader;
#[doc = "Field `UART2_CLK_EN` writer - reg_uart2_clk_en"]
pub type UART2_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSENS_CLK_EN` reader - reg_tsens_clk_en"]
pub type TSENS_CLK_EN_R = crate::BitReader;
#[doc = "Field `TSENS_CLK_EN` writer - reg_tsens_clk_en"]
pub type TSENS_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - reg_crypto_aes_clk_en"]
    #[inline(always)]
    pub fn crypto_aes_clk_en(&self) -> CRYPTO_AES_CLK_EN_R {
        CRYPTO_AES_CLK_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - reg_crypto_sha_clk_en"]
    #[inline(always)]
    pub fn crypto_sha_clk_en(&self) -> CRYPTO_SHA_CLK_EN_R {
        CRYPTO_SHA_CLK_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reg_crypto_rsa_clk_en"]
    #[inline(always)]
    pub fn crypto_rsa_clk_en(&self) -> CRYPTO_RSA_CLK_EN_R {
        CRYPTO_RSA_CLK_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - reg_crypto_ds_clk_en"]
    #[inline(always)]
    pub fn crypto_ds_clk_en(&self) -> CRYPTO_DS_CLK_EN_R {
        CRYPTO_DS_CLK_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - reg_crypto_hmac_clk_en"]
    #[inline(always)]
    pub fn crypto_hmac_clk_en(&self) -> CRYPTO_HMAC_CLK_EN_R {
        CRYPTO_HMAC_CLK_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - reg_dma_clk_en"]
    #[inline(always)]
    pub fn dma_clk_en(&self) -> DMA_CLK_EN_R {
        DMA_CLK_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - reg_sdio_host_clk_en"]
    #[inline(always)]
    pub fn sdio_host_clk_en(&self) -> SDIO_HOST_CLK_EN_R {
        SDIO_HOST_CLK_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - reg_lcd_cam_clk_en"]
    #[inline(always)]
    pub fn lcd_cam_clk_en(&self) -> LCD_CAM_CLK_EN_R {
        LCD_CAM_CLK_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - reg_uart2_clk_en"]
    #[inline(always)]
    pub fn uart2_clk_en(&self) -> UART2_CLK_EN_R {
        UART2_CLK_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - reg_tsens_clk_en"]
    #[inline(always)]
    pub fn tsens_clk_en(&self) -> TSENS_CLK_EN_R {
        TSENS_CLK_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERIP_CLK_EN1")
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
                "tsens_clk_en",
                &format_args!("{}", self.tsens_clk_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PERIP_CLK_EN1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 1 - reg_crypto_aes_clk_en"]
    #[inline(always)]
    #[must_use]
    pub fn crypto_aes_clk_en(&mut self) -> CRYPTO_AES_CLK_EN_W<PERIP_CLK_EN1_SPEC> {
        CRYPTO_AES_CLK_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - reg_crypto_sha_clk_en"]
    #[inline(always)]
    #[must_use]
    pub fn crypto_sha_clk_en(&mut self) -> CRYPTO_SHA_CLK_EN_W<PERIP_CLK_EN1_SPEC> {
        CRYPTO_SHA_CLK_EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - reg_crypto_rsa_clk_en"]
    #[inline(always)]
    #[must_use]
    pub fn crypto_rsa_clk_en(&mut self) -> CRYPTO_RSA_CLK_EN_W<PERIP_CLK_EN1_SPEC> {
        CRYPTO_RSA_CLK_EN_W::new(self, 3)
    }
    #[doc = "Bit 4 - reg_crypto_ds_clk_en"]
    #[inline(always)]
    #[must_use]
    pub fn crypto_ds_clk_en(&mut self) -> CRYPTO_DS_CLK_EN_W<PERIP_CLK_EN1_SPEC> {
        CRYPTO_DS_CLK_EN_W::new(self, 4)
    }
    #[doc = "Bit 5 - reg_crypto_hmac_clk_en"]
    #[inline(always)]
    #[must_use]
    pub fn crypto_hmac_clk_en(&mut self) -> CRYPTO_HMAC_CLK_EN_W<PERIP_CLK_EN1_SPEC> {
        CRYPTO_HMAC_CLK_EN_W::new(self, 5)
    }
    #[doc = "Bit 6 - reg_dma_clk_en"]
    #[inline(always)]
    #[must_use]
    pub fn dma_clk_en(&mut self) -> DMA_CLK_EN_W<PERIP_CLK_EN1_SPEC> {
        DMA_CLK_EN_W::new(self, 6)
    }
    #[doc = "Bit 7 - reg_sdio_host_clk_en"]
    #[inline(always)]
    #[must_use]
    pub fn sdio_host_clk_en(&mut self) -> SDIO_HOST_CLK_EN_W<PERIP_CLK_EN1_SPEC> {
        SDIO_HOST_CLK_EN_W::new(self, 7)
    }
    #[doc = "Bit 8 - reg_lcd_cam_clk_en"]
    #[inline(always)]
    #[must_use]
    pub fn lcd_cam_clk_en(&mut self) -> LCD_CAM_CLK_EN_W<PERIP_CLK_EN1_SPEC> {
        LCD_CAM_CLK_EN_W::new(self, 8)
    }
    #[doc = "Bit 9 - reg_uart2_clk_en"]
    #[inline(always)]
    #[must_use]
    pub fn uart2_clk_en(&mut self) -> UART2_CLK_EN_W<PERIP_CLK_EN1_SPEC> {
        UART2_CLK_EN_W::new(self, 9)
    }
    #[doc = "Bit 10 - reg_tsens_clk_en"]
    #[inline(always)]
    #[must_use]
    pub fn tsens_clk_en(&mut self) -> TSENS_CLK_EN_W<PERIP_CLK_EN1_SPEC> {
        TSENS_CLK_EN_W::new(self, 10)
    }
}
#[doc = "peripheral clock gating register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`perip_clk_en1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`perip_clk_en1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PERIP_CLK_EN1_SPEC;
impl crate::RegisterSpec for PERIP_CLK_EN1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`perip_clk_en1::R`](R) reader structure"]
impl crate::Readable for PERIP_CLK_EN1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`perip_clk_en1::W`](W) writer structure"]
impl crate::Writable for PERIP_CLK_EN1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PERIP_CLK_EN1 to value 0x0200"]
impl crate::Resettable for PERIP_CLK_EN1_SPEC {
    const RESET_VALUE: u32 = 0x0200;
}
