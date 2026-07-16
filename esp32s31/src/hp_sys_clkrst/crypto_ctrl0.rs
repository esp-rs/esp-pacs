#[doc = "Register `CRYPTO_CTRL0` reader"]
pub type R = crate::R<CRYPTO_CTRL0_SPEC>;
#[doc = "Register `CRYPTO_CTRL0` writer"]
pub type W = crate::W<CRYPTO_CTRL0_SPEC>;
#[doc = "Field `CRYPTO_SYS_CLK_EN` reader - need_des"]
pub type CRYPTO_SYS_CLK_EN_R = crate::BitReader;
#[doc = "Field `CRYPTO_SYS_CLK_EN` writer - need_des"]
pub type CRYPTO_SYS_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRYPTO_RST_EN` reader - need_des"]
pub type CRYPTO_RST_EN_R = crate::BitReader;
#[doc = "Field `CRYPTO_RST_EN` writer - need_des"]
pub type CRYPTO_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRYPTO_SEC_CLK_EN` reader - need_des"]
pub type CRYPTO_SEC_CLK_EN_R = crate::BitReader;
#[doc = "Field `CRYPTO_SEC_CLK_EN` writer - need_des"]
pub type CRYPTO_SEC_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRYPTO_SEC_RST_EN` reader - need_des"]
pub type CRYPTO_SEC_RST_EN_R = crate::BitReader;
#[doc = "Field `CRYPTO_SEC_RST_EN` writer - need_des"]
pub type CRYPTO_SEC_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRYPTO_AES_CLK_EN` reader - need_des"]
pub type CRYPTO_AES_CLK_EN_R = crate::BitReader;
#[doc = "Field `CRYPTO_AES_CLK_EN` writer - need_des"]
pub type CRYPTO_AES_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRYPTO_AES_RST_EN` reader - need_des"]
pub type CRYPTO_AES_RST_EN_R = crate::BitReader;
#[doc = "Field `CRYPTO_AES_RST_EN` writer - need_des"]
pub type CRYPTO_AES_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRYPTO_SHA_CLK_EN` reader - need_des"]
pub type CRYPTO_SHA_CLK_EN_R = crate::BitReader;
#[doc = "Field `CRYPTO_SHA_CLK_EN` writer - need_des"]
pub type CRYPTO_SHA_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRYPTO_SHA_RST_EN` reader - need_des"]
pub type CRYPTO_SHA_RST_EN_R = crate::BitReader;
#[doc = "Field `CRYPTO_SHA_RST_EN` writer - need_des"]
pub type CRYPTO_SHA_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRYPTO_RSA_CLK_EN` reader - need_des"]
pub type CRYPTO_RSA_CLK_EN_R = crate::BitReader;
#[doc = "Field `CRYPTO_RSA_CLK_EN` writer - need_des"]
pub type CRYPTO_RSA_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRYPTO_RSA_RST_EN` reader - need_des"]
pub type CRYPTO_RSA_RST_EN_R = crate::BitReader;
#[doc = "Field `CRYPTO_RSA_RST_EN` writer - need_des"]
pub type CRYPTO_RSA_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRYPTO_DS_CLK_EN` reader - need_des"]
pub type CRYPTO_DS_CLK_EN_R = crate::BitReader;
#[doc = "Field `CRYPTO_DS_CLK_EN` writer - need_des"]
pub type CRYPTO_DS_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRYPTO_DS_RST_EN` reader - need_des"]
pub type CRYPTO_DS_RST_EN_R = crate::BitReader;
#[doc = "Field `CRYPTO_DS_RST_EN` writer - need_des"]
pub type CRYPTO_DS_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRYPTO_ECC_CLK_EN` reader - need_des"]
pub type CRYPTO_ECC_CLK_EN_R = crate::BitReader;
#[doc = "Field `CRYPTO_ECC_CLK_EN` writer - need_des"]
pub type CRYPTO_ECC_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRYPTO_ECC_RST_EN` reader - need_des"]
pub type CRYPTO_ECC_RST_EN_R = crate::BitReader;
#[doc = "Field `CRYPTO_ECC_RST_EN` writer - need_des"]
pub type CRYPTO_ECC_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRYPTO_HMAC_CLK_EN` reader - need_des"]
pub type CRYPTO_HMAC_CLK_EN_R = crate::BitReader;
#[doc = "Field `CRYPTO_HMAC_CLK_EN` writer - need_des"]
pub type CRYPTO_HMAC_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRYPTO_HMAC_RST_EN` reader - need_des"]
pub type CRYPTO_HMAC_RST_EN_R = crate::BitReader;
#[doc = "Field `CRYPTO_HMAC_RST_EN` writer - need_des"]
pub type CRYPTO_HMAC_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRYPTO_ECDSA_CLK_EN` reader - need_des"]
pub type CRYPTO_ECDSA_CLK_EN_R = crate::BitReader;
#[doc = "Field `CRYPTO_ECDSA_CLK_EN` writer - need_des"]
pub type CRYPTO_ECDSA_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRYPTO_ECDSA_RST_EN` reader - need_des"]
pub type CRYPTO_ECDSA_RST_EN_R = crate::BitReader;
#[doc = "Field `CRYPTO_ECDSA_RST_EN` writer - need_des"]
pub type CRYPTO_ECDSA_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRYPTO_CLK_SRC_SEL` reader - need_des"]
pub type CRYPTO_CLK_SRC_SEL_R = crate::FieldReader;
#[doc = "Field `CRYPTO_CLK_SRC_SEL` writer - need_des"]
pub type CRYPTO_CLK_SRC_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CRYPTO_RMA_CLK_EN` reader - need_des"]
pub type CRYPTO_RMA_CLK_EN_R = crate::BitReader;
#[doc = "Field `CRYPTO_RMA_CLK_EN` writer - need_des"]
pub type CRYPTO_RMA_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRYPTO_RMA_RST_EN` reader - need_des"]
pub type CRYPTO_RMA_RST_EN_R = crate::BitReader;
#[doc = "Field `CRYPTO_RMA_RST_EN` writer - need_des"]
pub type CRYPTO_RMA_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn crypto_sys_clk_en(&self) -> CRYPTO_SYS_CLK_EN_R {
        CRYPTO_SYS_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn crypto_rst_en(&self) -> CRYPTO_RST_EN_R {
        CRYPTO_RST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn crypto_sec_clk_en(&self) -> CRYPTO_SEC_CLK_EN_R {
        CRYPTO_SEC_CLK_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - need_des"]
    #[inline(always)]
    pub fn crypto_sec_rst_en(&self) -> CRYPTO_SEC_RST_EN_R {
        CRYPTO_SEC_RST_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - need_des"]
    #[inline(always)]
    pub fn crypto_aes_clk_en(&self) -> CRYPTO_AES_CLK_EN_R {
        CRYPTO_AES_CLK_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - need_des"]
    #[inline(always)]
    pub fn crypto_aes_rst_en(&self) -> CRYPTO_AES_RST_EN_R {
        CRYPTO_AES_RST_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - need_des"]
    #[inline(always)]
    pub fn crypto_sha_clk_en(&self) -> CRYPTO_SHA_CLK_EN_R {
        CRYPTO_SHA_CLK_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - need_des"]
    #[inline(always)]
    pub fn crypto_sha_rst_en(&self) -> CRYPTO_SHA_RST_EN_R {
        CRYPTO_SHA_RST_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - need_des"]
    #[inline(always)]
    pub fn crypto_rsa_clk_en(&self) -> CRYPTO_RSA_CLK_EN_R {
        CRYPTO_RSA_CLK_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - need_des"]
    #[inline(always)]
    pub fn crypto_rsa_rst_en(&self) -> CRYPTO_RSA_RST_EN_R {
        CRYPTO_RSA_RST_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - need_des"]
    #[inline(always)]
    pub fn crypto_ds_clk_en(&self) -> CRYPTO_DS_CLK_EN_R {
        CRYPTO_DS_CLK_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - need_des"]
    #[inline(always)]
    pub fn crypto_ds_rst_en(&self) -> CRYPTO_DS_RST_EN_R {
        CRYPTO_DS_RST_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - need_des"]
    #[inline(always)]
    pub fn crypto_ecc_clk_en(&self) -> CRYPTO_ECC_CLK_EN_R {
        CRYPTO_ECC_CLK_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - need_des"]
    #[inline(always)]
    pub fn crypto_ecc_rst_en(&self) -> CRYPTO_ECC_RST_EN_R {
        CRYPTO_ECC_RST_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - need_des"]
    #[inline(always)]
    pub fn crypto_hmac_clk_en(&self) -> CRYPTO_HMAC_CLK_EN_R {
        CRYPTO_HMAC_CLK_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - need_des"]
    #[inline(always)]
    pub fn crypto_hmac_rst_en(&self) -> CRYPTO_HMAC_RST_EN_R {
        CRYPTO_HMAC_RST_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - need_des"]
    #[inline(always)]
    pub fn crypto_ecdsa_clk_en(&self) -> CRYPTO_ECDSA_CLK_EN_R {
        CRYPTO_ECDSA_CLK_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - need_des"]
    #[inline(always)]
    pub fn crypto_ecdsa_rst_en(&self) -> CRYPTO_ECDSA_RST_EN_R {
        CRYPTO_ECDSA_RST_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 20:21 - need_des"]
    #[inline(always)]
    pub fn crypto_clk_src_sel(&self) -> CRYPTO_CLK_SRC_SEL_R {
        CRYPTO_CLK_SRC_SEL_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - need_des"]
    #[inline(always)]
    pub fn crypto_rma_clk_en(&self) -> CRYPTO_RMA_CLK_EN_R {
        CRYPTO_RMA_CLK_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - need_des"]
    #[inline(always)]
    pub fn crypto_rma_rst_en(&self) -> CRYPTO_RMA_RST_EN_R {
        CRYPTO_RMA_RST_EN_R::new(((self.bits >> 23) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CRYPTO_CTRL0")
            .field("crypto_sys_clk_en", &self.crypto_sys_clk_en())
            .field("crypto_rst_en", &self.crypto_rst_en())
            .field("crypto_sec_clk_en", &self.crypto_sec_clk_en())
            .field("crypto_sec_rst_en", &self.crypto_sec_rst_en())
            .field("crypto_aes_clk_en", &self.crypto_aes_clk_en())
            .field("crypto_aes_rst_en", &self.crypto_aes_rst_en())
            .field("crypto_sha_clk_en", &self.crypto_sha_clk_en())
            .field("crypto_sha_rst_en", &self.crypto_sha_rst_en())
            .field("crypto_rsa_clk_en", &self.crypto_rsa_clk_en())
            .field("crypto_rsa_rst_en", &self.crypto_rsa_rst_en())
            .field("crypto_ds_clk_en", &self.crypto_ds_clk_en())
            .field("crypto_ds_rst_en", &self.crypto_ds_rst_en())
            .field("crypto_ecc_clk_en", &self.crypto_ecc_clk_en())
            .field("crypto_ecc_rst_en", &self.crypto_ecc_rst_en())
            .field("crypto_hmac_clk_en", &self.crypto_hmac_clk_en())
            .field("crypto_hmac_rst_en", &self.crypto_hmac_rst_en())
            .field("crypto_ecdsa_clk_en", &self.crypto_ecdsa_clk_en())
            .field("crypto_ecdsa_rst_en", &self.crypto_ecdsa_rst_en())
            .field("crypto_clk_src_sel", &self.crypto_clk_src_sel())
            .field("crypto_rma_clk_en", &self.crypto_rma_clk_en())
            .field("crypto_rma_rst_en", &self.crypto_rma_rst_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn crypto_sys_clk_en(&mut self) -> CRYPTO_SYS_CLK_EN_W<'_, CRYPTO_CTRL0_SPEC> {
        CRYPTO_SYS_CLK_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn crypto_rst_en(&mut self) -> CRYPTO_RST_EN_W<'_, CRYPTO_CTRL0_SPEC> {
        CRYPTO_RST_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn crypto_sec_clk_en(&mut self) -> CRYPTO_SEC_CLK_EN_W<'_, CRYPTO_CTRL0_SPEC> {
        CRYPTO_SEC_CLK_EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - need_des"]
    #[inline(always)]
    pub fn crypto_sec_rst_en(&mut self) -> CRYPTO_SEC_RST_EN_W<'_, CRYPTO_CTRL0_SPEC> {
        CRYPTO_SEC_RST_EN_W::new(self, 3)
    }
    #[doc = "Bit 4 - need_des"]
    #[inline(always)]
    pub fn crypto_aes_clk_en(&mut self) -> CRYPTO_AES_CLK_EN_W<'_, CRYPTO_CTRL0_SPEC> {
        CRYPTO_AES_CLK_EN_W::new(self, 4)
    }
    #[doc = "Bit 5 - need_des"]
    #[inline(always)]
    pub fn crypto_aes_rst_en(&mut self) -> CRYPTO_AES_RST_EN_W<'_, CRYPTO_CTRL0_SPEC> {
        CRYPTO_AES_RST_EN_W::new(self, 5)
    }
    #[doc = "Bit 6 - need_des"]
    #[inline(always)]
    pub fn crypto_sha_clk_en(&mut self) -> CRYPTO_SHA_CLK_EN_W<'_, CRYPTO_CTRL0_SPEC> {
        CRYPTO_SHA_CLK_EN_W::new(self, 6)
    }
    #[doc = "Bit 7 - need_des"]
    #[inline(always)]
    pub fn crypto_sha_rst_en(&mut self) -> CRYPTO_SHA_RST_EN_W<'_, CRYPTO_CTRL0_SPEC> {
        CRYPTO_SHA_RST_EN_W::new(self, 7)
    }
    #[doc = "Bit 8 - need_des"]
    #[inline(always)]
    pub fn crypto_rsa_clk_en(&mut self) -> CRYPTO_RSA_CLK_EN_W<'_, CRYPTO_CTRL0_SPEC> {
        CRYPTO_RSA_CLK_EN_W::new(self, 8)
    }
    #[doc = "Bit 9 - need_des"]
    #[inline(always)]
    pub fn crypto_rsa_rst_en(&mut self) -> CRYPTO_RSA_RST_EN_W<'_, CRYPTO_CTRL0_SPEC> {
        CRYPTO_RSA_RST_EN_W::new(self, 9)
    }
    #[doc = "Bit 10 - need_des"]
    #[inline(always)]
    pub fn crypto_ds_clk_en(&mut self) -> CRYPTO_DS_CLK_EN_W<'_, CRYPTO_CTRL0_SPEC> {
        CRYPTO_DS_CLK_EN_W::new(self, 10)
    }
    #[doc = "Bit 11 - need_des"]
    #[inline(always)]
    pub fn crypto_ds_rst_en(&mut self) -> CRYPTO_DS_RST_EN_W<'_, CRYPTO_CTRL0_SPEC> {
        CRYPTO_DS_RST_EN_W::new(self, 11)
    }
    #[doc = "Bit 12 - need_des"]
    #[inline(always)]
    pub fn crypto_ecc_clk_en(&mut self) -> CRYPTO_ECC_CLK_EN_W<'_, CRYPTO_CTRL0_SPEC> {
        CRYPTO_ECC_CLK_EN_W::new(self, 12)
    }
    #[doc = "Bit 13 - need_des"]
    #[inline(always)]
    pub fn crypto_ecc_rst_en(&mut self) -> CRYPTO_ECC_RST_EN_W<'_, CRYPTO_CTRL0_SPEC> {
        CRYPTO_ECC_RST_EN_W::new(self, 13)
    }
    #[doc = "Bit 14 - need_des"]
    #[inline(always)]
    pub fn crypto_hmac_clk_en(&mut self) -> CRYPTO_HMAC_CLK_EN_W<'_, CRYPTO_CTRL0_SPEC> {
        CRYPTO_HMAC_CLK_EN_W::new(self, 14)
    }
    #[doc = "Bit 15 - need_des"]
    #[inline(always)]
    pub fn crypto_hmac_rst_en(&mut self) -> CRYPTO_HMAC_RST_EN_W<'_, CRYPTO_CTRL0_SPEC> {
        CRYPTO_HMAC_RST_EN_W::new(self, 15)
    }
    #[doc = "Bit 16 - need_des"]
    #[inline(always)]
    pub fn crypto_ecdsa_clk_en(&mut self) -> CRYPTO_ECDSA_CLK_EN_W<'_, CRYPTO_CTRL0_SPEC> {
        CRYPTO_ECDSA_CLK_EN_W::new(self, 16)
    }
    #[doc = "Bit 17 - need_des"]
    #[inline(always)]
    pub fn crypto_ecdsa_rst_en(&mut self) -> CRYPTO_ECDSA_RST_EN_W<'_, CRYPTO_CTRL0_SPEC> {
        CRYPTO_ECDSA_RST_EN_W::new(self, 17)
    }
    #[doc = "Bits 20:21 - need_des"]
    #[inline(always)]
    pub fn crypto_clk_src_sel(&mut self) -> CRYPTO_CLK_SRC_SEL_W<'_, CRYPTO_CTRL0_SPEC> {
        CRYPTO_CLK_SRC_SEL_W::new(self, 20)
    }
    #[doc = "Bit 22 - need_des"]
    #[inline(always)]
    pub fn crypto_rma_clk_en(&mut self) -> CRYPTO_RMA_CLK_EN_W<'_, CRYPTO_CTRL0_SPEC> {
        CRYPTO_RMA_CLK_EN_W::new(self, 22)
    }
    #[doc = "Bit 23 - need_des"]
    #[inline(always)]
    pub fn crypto_rma_rst_en(&mut self) -> CRYPTO_RMA_RST_EN_W<'_, CRYPTO_CTRL0_SPEC> {
        CRYPTO_RMA_RST_EN_W::new(self, 23)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`crypto_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crypto_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRYPTO_CTRL0_SPEC;
impl crate::RegisterSpec for CRYPTO_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crypto_ctrl0::R`](R) reader structure"]
impl crate::Readable for CRYPTO_CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`crypto_ctrl0::W`](W) writer structure"]
impl crate::Writable for CRYPTO_CTRL0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CRYPTO_CTRL0 to value 0x0041_5555"]
impl crate::Resettable for CRYPTO_CTRL0_SPEC {
    const RESET_VALUE: u32 = 0x0041_5555;
}
