#[doc = "Register `RD_REPEAT_DATA2` reader"]
pub type R = crate::R<RD_REPEAT_DATA2_SPEC>;
#[doc = "Field `KEY_PURPOSE_0` reader - Represents the purpose of Key0."]
pub type KEY_PURPOSE_0_R = crate::FieldReader;
#[doc = "Field `KEY_PURPOSE_1` reader - Represents the purpose of Key1."]
pub type KEY_PURPOSE_1_R = crate::FieldReader;
#[doc = "Field `KEY_PURPOSE_2` reader - Represents the purpose of Key2."]
pub type KEY_PURPOSE_2_R = crate::FieldReader;
#[doc = "Field `KEY_PURPOSE_3` reader - Represents the purpose of Key3."]
pub type KEY_PURPOSE_3_R = crate::FieldReader;
#[doc = "Field `KEY_PURPOSE_4` reader - Represents the purpose of Key4."]
pub type KEY_PURPOSE_4_R = crate::FieldReader;
#[doc = "Field `ECC_FORCE_CONST_TIME` reader - Represents whether permanently turn on ECC const-time mode. \\\\ 1: turn on\\\\ 0: turn off\\\\"]
pub type ECC_FORCE_CONST_TIME_R = crate::BitReader;
#[doc = "Field `ECDSA_DISABLE_SOFT_K` reader - Represents whether permanently turn off ECDSA software set KEY.\\\\ 1: turn off\\\\ 0: turn on\\\\"]
pub type ECDSA_DISABLE_SOFT_K_R = crate::BitReader;
#[doc = "Field `SEC_DPA_LEVEL` reader - Represents the spa secure level by configuring the clock random divide mode."]
pub type SEC_DPA_LEVEL_R = crate::FieldReader;
#[doc = "Field `XTS_DPA_CLK_ENABLE` reader - Represents whether to enable xts clock anti-dpa attack function.\\\\0: Disabled. \\\\1: Enabled\\\\"]
pub type XTS_DPA_CLK_ENABLE_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:4 - Represents the purpose of Key0."]
    #[inline(always)]
    pub fn key_purpose_0(&self) -> KEY_PURPOSE_0_R {
        KEY_PURPOSE_0_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - Represents the purpose of Key1."]
    #[inline(always)]
    pub fn key_purpose_1(&self) -> KEY_PURPOSE_1_R {
        KEY_PURPOSE_1_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - Represents the purpose of Key2."]
    #[inline(always)]
    pub fn key_purpose_2(&self) -> KEY_PURPOSE_2_R {
        KEY_PURPOSE_2_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - Represents the purpose of Key3."]
    #[inline(always)]
    pub fn key_purpose_3(&self) -> KEY_PURPOSE_3_R {
        KEY_PURPOSE_3_R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24 - Represents the purpose of Key4."]
    #[inline(always)]
    pub fn key_purpose_4(&self) -> KEY_PURPOSE_4_R {
        KEY_PURPOSE_4_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bit 25 - Represents whether permanently turn on ECC const-time mode. \\\\ 1: turn on\\\\ 0: turn off\\\\"]
    #[inline(always)]
    pub fn ecc_force_const_time(&self) -> ECC_FORCE_CONST_TIME_R {
        ECC_FORCE_CONST_TIME_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Represents whether permanently turn off ECDSA software set KEY.\\\\ 1: turn off\\\\ 0: turn on\\\\"]
    #[inline(always)]
    pub fn ecdsa_disable_soft_k(&self) -> ECDSA_DISABLE_SOFT_K_R {
        ECDSA_DISABLE_SOFT_K_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:28 - Represents the spa secure level by configuring the clock random divide mode."]
    #[inline(always)]
    pub fn sec_dpa_level(&self) -> SEC_DPA_LEVEL_R {
        SEC_DPA_LEVEL_R::new(((self.bits >> 27) & 3) as u8)
    }
    #[doc = "Bit 29 - Represents whether to enable xts clock anti-dpa attack function.\\\\0: Disabled. \\\\1: Enabled\\\\"]
    #[inline(always)]
    pub fn xts_dpa_clk_enable(&self) -> XTS_DPA_CLK_ENABLE_R {
        XTS_DPA_CLK_ENABLE_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_REPEAT_DATA2")
            .field("key_purpose_0", &self.key_purpose_0())
            .field("key_purpose_1", &self.key_purpose_1())
            .field("key_purpose_2", &self.key_purpose_2())
            .field("key_purpose_3", &self.key_purpose_3())
            .field("key_purpose_4", &self.key_purpose_4())
            .field("ecc_force_const_time", &self.ecc_force_const_time())
            .field("ecdsa_disable_soft_k", &self.ecdsa_disable_soft_k())
            .field("sec_dpa_level", &self.sec_dpa_level())
            .field("xts_dpa_clk_enable", &self.xts_dpa_clk_enable())
            .finish()
    }
}
#[doc = "Represents rd_repeat_data\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_repeat_data2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_REPEAT_DATA2_SPEC;
impl crate::RegisterSpec for RD_REPEAT_DATA2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_repeat_data2::R`](R) reader structure"]
impl crate::Readable for RD_REPEAT_DATA2_SPEC {}
#[doc = "`reset()` method sets RD_REPEAT_DATA2 to value 0"]
impl crate::Resettable for RD_REPEAT_DATA2_SPEC {}
