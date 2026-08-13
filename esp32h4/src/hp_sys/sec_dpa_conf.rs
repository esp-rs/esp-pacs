#[doc = "Register `SEC_DPA_CONF` reader"]
pub type R = crate::R<SEC_DPA_CONF_SPEC>;
#[doc = "Field `SEC_DPA_LEVEL` reader - Configures whether or not to enable anti-DPA attack. Valid only when HP_SYSTEM_SEC_DPA_CFG_SEL is 0. \\\\ 0: Disable\\\\ 1-3: Enable. The larger the number, the higher the security level, which represents the ability to resist DPA attacks, with increased computational overhead of the hardware crypto-accelerators at the same time. \\\\"]
pub type SEC_DPA_LEVEL_R = crate::FieldReader;
#[doc = "Field `SEC_DPA_CFG_SEL` reader - Configures whether to select HP_SYSTEM_SEC_DPA_LEVEL or EFUSE_SEC_DPA_LEVEL (from eFuse) to control DPA level. \\\\ 0: Select EFUSE_SEC_DPA_LEVEL\\\\ 1: Select HP_SYSTEM_SEC_DPA_LEVEL\\\\"]
pub type SEC_DPA_CFG_SEL_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:1 - Configures whether or not to enable anti-DPA attack. Valid only when HP_SYSTEM_SEC_DPA_CFG_SEL is 0. \\\\ 0: Disable\\\\ 1-3: Enable. The larger the number, the higher the security level, which represents the ability to resist DPA attacks, with increased computational overhead of the hardware crypto-accelerators at the same time. \\\\"]
    #[inline(always)]
    pub fn sec_dpa_level(&self) -> SEC_DPA_LEVEL_R {
        SEC_DPA_LEVEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Configures whether to select HP_SYSTEM_SEC_DPA_LEVEL or EFUSE_SEC_DPA_LEVEL (from eFuse) to control DPA level. \\\\ 0: Select EFUSE_SEC_DPA_LEVEL\\\\ 1: Select HP_SYSTEM_SEC_DPA_LEVEL\\\\"]
    #[inline(always)]
    pub fn sec_dpa_cfg_sel(&self) -> SEC_DPA_CFG_SEL_R {
        SEC_DPA_CFG_SEL_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SEC_DPA_CONF")
            .field("sec_dpa_level", &self.sec_dpa_level())
            .field("sec_dpa_cfg_sel", &self.sec_dpa_cfg_sel())
            .finish()
    }
}
#[doc = "HP anti-DPA security configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_dpa_conf::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SEC_DPA_CONF_SPEC;
impl crate::RegisterSpec for SEC_DPA_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sec_dpa_conf::R`](R) reader structure"]
impl crate::Readable for SEC_DPA_CONF_SPEC {}
#[doc = "`reset()` method sets SEC_DPA_CONF to value 0"]
impl crate::Resettable for SEC_DPA_CONF_SPEC {}
