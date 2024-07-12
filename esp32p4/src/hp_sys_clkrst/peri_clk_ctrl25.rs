#[doc = "Register `PERI_CLK_CTRL25` reader"]
pub type R = crate::R<PERI_CLK_CTRL25_SPEC>;
#[doc = "Register `PERI_CLK_CTRL25` writer"]
pub type W = crate::W<PERI_CLK_CTRL25_SPEC>;
#[doc = "Field `PVT_PERI_GROUP_CLK_DIV_NUM` reader - Reserved"]
pub type PVT_PERI_GROUP_CLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `PVT_PERI_GROUP_CLK_DIV_NUM` writer - Reserved"]
pub type PVT_PERI_GROUP_CLK_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `PVT_PERI_GROUP1_CLK_EN` reader - Reserved"]
pub type PVT_PERI_GROUP1_CLK_EN_R = crate::BitReader;
#[doc = "Field `PVT_PERI_GROUP1_CLK_EN` writer - Reserved"]
pub type PVT_PERI_GROUP1_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PVT_PERI_GROUP2_CLK_EN` reader - Reserved"]
pub type PVT_PERI_GROUP2_CLK_EN_R = crate::BitReader;
#[doc = "Field `PVT_PERI_GROUP2_CLK_EN` writer - Reserved"]
pub type PVT_PERI_GROUP2_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PVT_PERI_GROUP3_CLK_EN` reader - Reserved"]
pub type PVT_PERI_GROUP3_CLK_EN_R = crate::BitReader;
#[doc = "Field `PVT_PERI_GROUP3_CLK_EN` writer - Reserved"]
pub type PVT_PERI_GROUP3_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PVT_PERI_GROUP4_CLK_EN` reader - Reserved"]
pub type PVT_PERI_GROUP4_CLK_EN_R = crate::BitReader;
#[doc = "Field `PVT_PERI_GROUP4_CLK_EN` writer - Reserved"]
pub type PVT_PERI_GROUP4_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRYPTO_CLK_SRC_SEL` reader - Reserved"]
pub type CRYPTO_CLK_SRC_SEL_R = crate::FieldReader;
#[doc = "Field `CRYPTO_CLK_SRC_SEL` writer - Reserved"]
pub type CRYPTO_CLK_SRC_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CRYPTO_AES_CLK_EN` reader - Reserved"]
pub type CRYPTO_AES_CLK_EN_R = crate::BitReader;
#[doc = "Field `CRYPTO_AES_CLK_EN` writer - Reserved"]
pub type CRYPTO_AES_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRYPTO_DS_CLK_EN` reader - Reserved"]
pub type CRYPTO_DS_CLK_EN_R = crate::BitReader;
#[doc = "Field `CRYPTO_DS_CLK_EN` writer - Reserved"]
pub type CRYPTO_DS_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRYPTO_ECC_CLK_EN` reader - Reserved"]
pub type CRYPTO_ECC_CLK_EN_R = crate::BitReader;
#[doc = "Field `CRYPTO_ECC_CLK_EN` writer - Reserved"]
pub type CRYPTO_ECC_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRYPTO_HMAC_CLK_EN` reader - Reserved"]
pub type CRYPTO_HMAC_CLK_EN_R = crate::BitReader;
#[doc = "Field `CRYPTO_HMAC_CLK_EN` writer - Reserved"]
pub type CRYPTO_HMAC_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRYPTO_RSA_CLK_EN` reader - Reserved"]
pub type CRYPTO_RSA_CLK_EN_R = crate::BitReader;
#[doc = "Field `CRYPTO_RSA_CLK_EN` writer - Reserved"]
pub type CRYPTO_RSA_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRYPTO_SEC_CLK_EN` reader - Reserved"]
pub type CRYPTO_SEC_CLK_EN_R = crate::BitReader;
#[doc = "Field `CRYPTO_SEC_CLK_EN` writer - Reserved"]
pub type CRYPTO_SEC_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRYPTO_SHA_CLK_EN` reader - Reserved"]
pub type CRYPTO_SHA_CLK_EN_R = crate::BitReader;
#[doc = "Field `CRYPTO_SHA_CLK_EN` writer - Reserved"]
pub type CRYPTO_SHA_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRYPTO_ECDSA_CLK_EN` reader - Reserved"]
pub type CRYPTO_ECDSA_CLK_EN_R = crate::BitReader;
#[doc = "Field `CRYPTO_ECDSA_CLK_EN` writer - Reserved"]
pub type CRYPTO_ECDSA_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRYPTO_KM_CLK_EN` reader - Reserved"]
pub type CRYPTO_KM_CLK_EN_R = crate::BitReader;
#[doc = "Field `CRYPTO_KM_CLK_EN` writer - Reserved"]
pub type CRYPTO_KM_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISP_CLK_SRC_SEL` reader - Reserved"]
pub type ISP_CLK_SRC_SEL_R = crate::FieldReader;
#[doc = "Field `ISP_CLK_SRC_SEL` writer - Reserved"]
pub type ISP_CLK_SRC_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ISP_CLK_EN` reader - Reserved"]
pub type ISP_CLK_EN_R = crate::BitReader;
#[doc = "Field `ISP_CLK_EN` writer - Reserved"]
pub type ISP_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Reserved"]
    #[inline(always)]
    pub fn pvt_peri_group_clk_div_num(&self) -> PVT_PERI_GROUP_CLK_DIV_NUM_R {
        PVT_PERI_GROUP_CLK_DIV_NUM_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - Reserved"]
    #[inline(always)]
    pub fn pvt_peri_group1_clk_en(&self) -> PVT_PERI_GROUP1_CLK_EN_R {
        PVT_PERI_GROUP1_CLK_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Reserved"]
    #[inline(always)]
    pub fn pvt_peri_group2_clk_en(&self) -> PVT_PERI_GROUP2_CLK_EN_R {
        PVT_PERI_GROUP2_CLK_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Reserved"]
    #[inline(always)]
    pub fn pvt_peri_group3_clk_en(&self) -> PVT_PERI_GROUP3_CLK_EN_R {
        PVT_PERI_GROUP3_CLK_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Reserved"]
    #[inline(always)]
    pub fn pvt_peri_group4_clk_en(&self) -> PVT_PERI_GROUP4_CLK_EN_R {
        PVT_PERI_GROUP4_CLK_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:13 - Reserved"]
    #[inline(always)]
    pub fn crypto_clk_src_sel(&self) -> CRYPTO_CLK_SRC_SEL_R {
        CRYPTO_CLK_SRC_SEL_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - Reserved"]
    #[inline(always)]
    pub fn crypto_aes_clk_en(&self) -> CRYPTO_AES_CLK_EN_R {
        CRYPTO_AES_CLK_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Reserved"]
    #[inline(always)]
    pub fn crypto_ds_clk_en(&self) -> CRYPTO_DS_CLK_EN_R {
        CRYPTO_DS_CLK_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Reserved"]
    #[inline(always)]
    pub fn crypto_ecc_clk_en(&self) -> CRYPTO_ECC_CLK_EN_R {
        CRYPTO_ECC_CLK_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Reserved"]
    #[inline(always)]
    pub fn crypto_hmac_clk_en(&self) -> CRYPTO_HMAC_CLK_EN_R {
        CRYPTO_HMAC_CLK_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Reserved"]
    #[inline(always)]
    pub fn crypto_rsa_clk_en(&self) -> CRYPTO_RSA_CLK_EN_R {
        CRYPTO_RSA_CLK_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Reserved"]
    #[inline(always)]
    pub fn crypto_sec_clk_en(&self) -> CRYPTO_SEC_CLK_EN_R {
        CRYPTO_SEC_CLK_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Reserved"]
    #[inline(always)]
    pub fn crypto_sha_clk_en(&self) -> CRYPTO_SHA_CLK_EN_R {
        CRYPTO_SHA_CLK_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Reserved"]
    #[inline(always)]
    pub fn crypto_ecdsa_clk_en(&self) -> CRYPTO_ECDSA_CLK_EN_R {
        CRYPTO_ECDSA_CLK_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Reserved"]
    #[inline(always)]
    pub fn crypto_km_clk_en(&self) -> CRYPTO_KM_CLK_EN_R {
        CRYPTO_KM_CLK_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 23:24 - Reserved"]
    #[inline(always)]
    pub fn isp_clk_src_sel(&self) -> ISP_CLK_SRC_SEL_R {
        ISP_CLK_SRC_SEL_R::new(((self.bits >> 23) & 3) as u8)
    }
    #[doc = "Bit 25 - Reserved"]
    #[inline(always)]
    pub fn isp_clk_en(&self) -> ISP_CLK_EN_R {
        ISP_CLK_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERI_CLK_CTRL25")
            .field(
                "pvt_peri_group_clk_div_num",
                &self.pvt_peri_group_clk_div_num(),
            )
            .field("pvt_peri_group1_clk_en", &self.pvt_peri_group1_clk_en())
            .field("pvt_peri_group2_clk_en", &self.pvt_peri_group2_clk_en())
            .field("pvt_peri_group3_clk_en", &self.pvt_peri_group3_clk_en())
            .field("pvt_peri_group4_clk_en", &self.pvt_peri_group4_clk_en())
            .field("crypto_clk_src_sel", &self.crypto_clk_src_sel())
            .field("crypto_aes_clk_en", &self.crypto_aes_clk_en())
            .field("crypto_ds_clk_en", &self.crypto_ds_clk_en())
            .field("crypto_ecc_clk_en", &self.crypto_ecc_clk_en())
            .field("crypto_hmac_clk_en", &self.crypto_hmac_clk_en())
            .field("crypto_rsa_clk_en", &self.crypto_rsa_clk_en())
            .field("crypto_sec_clk_en", &self.crypto_sec_clk_en())
            .field("crypto_sha_clk_en", &self.crypto_sha_clk_en())
            .field("crypto_ecdsa_clk_en", &self.crypto_ecdsa_clk_en())
            .field("crypto_km_clk_en", &self.crypto_km_clk_en())
            .field("isp_clk_src_sel", &self.isp_clk_src_sel())
            .field("isp_clk_en", &self.isp_clk_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn pvt_peri_group_clk_div_num(
        &mut self,
    ) -> PVT_PERI_GROUP_CLK_DIV_NUM_W<PERI_CLK_CTRL25_SPEC> {
        PVT_PERI_GROUP_CLK_DIV_NUM_W::new(self, 0)
    }
    #[doc = "Bit 8 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn pvt_peri_group1_clk_en(&mut self) -> PVT_PERI_GROUP1_CLK_EN_W<PERI_CLK_CTRL25_SPEC> {
        PVT_PERI_GROUP1_CLK_EN_W::new(self, 8)
    }
    #[doc = "Bit 9 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn pvt_peri_group2_clk_en(&mut self) -> PVT_PERI_GROUP2_CLK_EN_W<PERI_CLK_CTRL25_SPEC> {
        PVT_PERI_GROUP2_CLK_EN_W::new(self, 9)
    }
    #[doc = "Bit 10 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn pvt_peri_group3_clk_en(&mut self) -> PVT_PERI_GROUP3_CLK_EN_W<PERI_CLK_CTRL25_SPEC> {
        PVT_PERI_GROUP3_CLK_EN_W::new(self, 10)
    }
    #[doc = "Bit 11 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn pvt_peri_group4_clk_en(&mut self) -> PVT_PERI_GROUP4_CLK_EN_W<PERI_CLK_CTRL25_SPEC> {
        PVT_PERI_GROUP4_CLK_EN_W::new(self, 11)
    }
    #[doc = "Bits 12:13 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn crypto_clk_src_sel(&mut self) -> CRYPTO_CLK_SRC_SEL_W<PERI_CLK_CTRL25_SPEC> {
        CRYPTO_CLK_SRC_SEL_W::new(self, 12)
    }
    #[doc = "Bit 14 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn crypto_aes_clk_en(&mut self) -> CRYPTO_AES_CLK_EN_W<PERI_CLK_CTRL25_SPEC> {
        CRYPTO_AES_CLK_EN_W::new(self, 14)
    }
    #[doc = "Bit 15 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn crypto_ds_clk_en(&mut self) -> CRYPTO_DS_CLK_EN_W<PERI_CLK_CTRL25_SPEC> {
        CRYPTO_DS_CLK_EN_W::new(self, 15)
    }
    #[doc = "Bit 16 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn crypto_ecc_clk_en(&mut self) -> CRYPTO_ECC_CLK_EN_W<PERI_CLK_CTRL25_SPEC> {
        CRYPTO_ECC_CLK_EN_W::new(self, 16)
    }
    #[doc = "Bit 17 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn crypto_hmac_clk_en(&mut self) -> CRYPTO_HMAC_CLK_EN_W<PERI_CLK_CTRL25_SPEC> {
        CRYPTO_HMAC_CLK_EN_W::new(self, 17)
    }
    #[doc = "Bit 18 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn crypto_rsa_clk_en(&mut self) -> CRYPTO_RSA_CLK_EN_W<PERI_CLK_CTRL25_SPEC> {
        CRYPTO_RSA_CLK_EN_W::new(self, 18)
    }
    #[doc = "Bit 19 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn crypto_sec_clk_en(&mut self) -> CRYPTO_SEC_CLK_EN_W<PERI_CLK_CTRL25_SPEC> {
        CRYPTO_SEC_CLK_EN_W::new(self, 19)
    }
    #[doc = "Bit 20 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn crypto_sha_clk_en(&mut self) -> CRYPTO_SHA_CLK_EN_W<PERI_CLK_CTRL25_SPEC> {
        CRYPTO_SHA_CLK_EN_W::new(self, 20)
    }
    #[doc = "Bit 21 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn crypto_ecdsa_clk_en(&mut self) -> CRYPTO_ECDSA_CLK_EN_W<PERI_CLK_CTRL25_SPEC> {
        CRYPTO_ECDSA_CLK_EN_W::new(self, 21)
    }
    #[doc = "Bit 22 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn crypto_km_clk_en(&mut self) -> CRYPTO_KM_CLK_EN_W<PERI_CLK_CTRL25_SPEC> {
        CRYPTO_KM_CLK_EN_W::new(self, 22)
    }
    #[doc = "Bits 23:24 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn isp_clk_src_sel(&mut self) -> ISP_CLK_SRC_SEL_W<PERI_CLK_CTRL25_SPEC> {
        ISP_CLK_SRC_SEL_W::new(self, 23)
    }
    #[doc = "Bit 25 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn isp_clk_en(&mut self) -> ISP_CLK_EN_W<PERI_CLK_CTRL25_SPEC> {
        ISP_CLK_EN_W::new(self, 25)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`peri_clk_ctrl25::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_clk_ctrl25::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PERI_CLK_CTRL25_SPEC;
impl crate::RegisterSpec for PERI_CLK_CTRL25_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`peri_clk_ctrl25::R`](R) reader structure"]
impl crate::Readable for PERI_CLK_CTRL25_SPEC {}
#[doc = "`write(|w| ..)` method takes [`peri_clk_ctrl25::W`](W) writer structure"]
impl crate::Writable for PERI_CLK_CTRL25_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PERI_CLK_CTRL25 to value 0x007f_c000"]
impl crate::Resettable for PERI_CLK_CTRL25_SPEC {
    const RESET_VALUE: u32 = 0x007f_c000;
}
