#[doc = "Register `RD_REPEAT_DATA_ERR4` reader"]
pub type R = crate::R<RD_REPEAT_DATA_ERR4_SPEC>;
#[doc = "Field `HUK_GEN_STATE_ERR` reader - Represents the programming error of EFUSE_HUK_GEN_STATE"]
pub type HUK_GEN_STATE_ERR_R = crate::FieldReader<u16>;
#[doc = "Field `XTAL_48M_SEL_ERR` reader - Represents the programming error of EFUSE_XTAL_48M_SEL"]
pub type XTAL_48M_SEL_ERR_R = crate::FieldReader;
#[doc = "Field `XTAL_48M_SEL_MODE_ERR` reader - Represents the programming error of EFUSE_XTAL_48M_SEL_MODE"]
pub type XTAL_48M_SEL_MODE_ERR_R = crate::BitReader;
#[doc = "Field `ECDSA_DISABLE_P192_ERR` reader - Represents the programming error of EFUSE_ECDSA_DISABLE_P192"]
pub type ECDSA_DISABLE_P192_ERR_R = crate::BitReader;
#[doc = "Field `ECC_FORCE_CONST_TIME_ERR` reader - Represents the programming error of EFUSE_ECC_FORCE_CONST_TIME"]
pub type ECC_FORCE_CONST_TIME_ERR_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:8 - Represents the programming error of EFUSE_HUK_GEN_STATE"]
    #[inline(always)]
    pub fn huk_gen_state_err(&self) -> HUK_GEN_STATE_ERR_R {
        HUK_GEN_STATE_ERR_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:11 - Represents the programming error of EFUSE_XTAL_48M_SEL"]
    #[inline(always)]
    pub fn xtal_48m_sel_err(&self) -> XTAL_48M_SEL_ERR_R {
        XTAL_48M_SEL_ERR_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bit 12 - Represents the programming error of EFUSE_XTAL_48M_SEL_MODE"]
    #[inline(always)]
    pub fn xtal_48m_sel_mode_err(&self) -> XTAL_48M_SEL_MODE_ERR_R {
        XTAL_48M_SEL_MODE_ERR_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Represents the programming error of EFUSE_ECDSA_DISABLE_P192"]
    #[inline(always)]
    pub fn ecdsa_disable_p192_err(&self) -> ECDSA_DISABLE_P192_ERR_R {
        ECDSA_DISABLE_P192_ERR_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Represents the programming error of EFUSE_ECC_FORCE_CONST_TIME"]
    #[inline(always)]
    pub fn ecc_force_const_time_err(&self) -> ECC_FORCE_CONST_TIME_ERR_R {
        ECC_FORCE_CONST_TIME_ERR_R::new(((self.bits >> 14) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_REPEAT_DATA_ERR4")
            .field("huk_gen_state_err", &self.huk_gen_state_err())
            .field("xtal_48m_sel_err", &self.xtal_48m_sel_err())
            .field("xtal_48m_sel_mode_err", &self.xtal_48m_sel_mode_err())
            .field("ecdsa_disable_p192_err", &self.ecdsa_disable_p192_err())
            .field("ecc_force_const_time_err", &self.ecc_force_const_time_err())
            .finish()
    }
}
#[doc = "Represents rd_repeat_data_err\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_repeat_data_err4::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_REPEAT_DATA_ERR4_SPEC;
impl crate::RegisterSpec for RD_REPEAT_DATA_ERR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_repeat_data_err4::R`](R) reader structure"]
impl crate::Readable for RD_REPEAT_DATA_ERR4_SPEC {}
#[doc = "`reset()` method sets RD_REPEAT_DATA_ERR4 to value 0"]
impl crate::Resettable for RD_REPEAT_DATA_ERR4_SPEC {
    const RESET_VALUE: u32 = 0;
}
