#[doc = "Register `HW_CFG` reader"]
pub type R = crate::R<HW_CFG_SPEC>;
#[doc = "Field `ICM_REG_AXI_HWCFG_QOS_SUPPORT` reader - NA"]
pub type ICM_REG_AXI_HWCFG_QOS_SUPPORT_R = crate::BitReader;
#[doc = "Field `ICM_REG_AXI_HWCFG_APB3_SUPPORT` reader - NA"]
pub type ICM_REG_AXI_HWCFG_APB3_SUPPORT_R = crate::BitReader;
#[doc = "Field `ICM_REG_AXI_HWCFG_AXI4_SUPPORT` reader - NA"]
pub type ICM_REG_AXI_HWCFG_AXI4_SUPPORT_R = crate::BitReader;
#[doc = "Field `ICM_REG_AXI_HWCFG_LOCK_EN` reader - NA"]
pub type ICM_REG_AXI_HWCFG_LOCK_EN_R = crate::BitReader;
#[doc = "Field `ICM_REG_AXI_HWCFG_TRUST_ZONE_EN` reader - NA"]
pub type ICM_REG_AXI_HWCFG_TRUST_ZONE_EN_R = crate::BitReader;
#[doc = "Field `ICM_REG_AXI_HWCFG_DECODER_TYPE` reader - NA"]
pub type ICM_REG_AXI_HWCFG_DECODER_TYPE_R = crate::BitReader;
#[doc = "Field `ICM_REG_AXI_HWCFG_REMAP_EN` reader - NA"]
pub type ICM_REG_AXI_HWCFG_REMAP_EN_R = crate::BitReader;
#[doc = "Field `ICM_REG_AXI_HWCFG_BI_DIR_CMD_EN` reader - NA"]
pub type ICM_REG_AXI_HWCFG_BI_DIR_CMD_EN_R = crate::BitReader;
#[doc = "Field `ICM_REG_AXI_HWCFG_LOW_POWER_INF_EN` reader - NA"]
pub type ICM_REG_AXI_HWCFG_LOW_POWER_INF_EN_R = crate::BitReader;
#[doc = "Field `ICM_REG_AXI_HWCFG_AXI_NUM_MASTERS` reader - NA"]
pub type ICM_REG_AXI_HWCFG_AXI_NUM_MASTERS_R = crate::FieldReader;
#[doc = "Field `ICM_REG_AXI_HWCFG_AXI_NUM_SLAVES` reader - NA"]
pub type ICM_REG_AXI_HWCFG_AXI_NUM_SLAVES_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn icm_reg_axi_hwcfg_qos_support(&self) -> ICM_REG_AXI_HWCFG_QOS_SUPPORT_R {
        ICM_REG_AXI_HWCFG_QOS_SUPPORT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn icm_reg_axi_hwcfg_apb3_support(&self) -> ICM_REG_AXI_HWCFG_APB3_SUPPORT_R {
        ICM_REG_AXI_HWCFG_APB3_SUPPORT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    pub fn icm_reg_axi_hwcfg_axi4_support(&self) -> ICM_REG_AXI_HWCFG_AXI4_SUPPORT_R {
        ICM_REG_AXI_HWCFG_AXI4_SUPPORT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    pub fn icm_reg_axi_hwcfg_lock_en(&self) -> ICM_REG_AXI_HWCFG_LOCK_EN_R {
        ICM_REG_AXI_HWCFG_LOCK_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NA"]
    #[inline(always)]
    pub fn icm_reg_axi_hwcfg_trust_zone_en(&self) -> ICM_REG_AXI_HWCFG_TRUST_ZONE_EN_R {
        ICM_REG_AXI_HWCFG_TRUST_ZONE_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - NA"]
    #[inline(always)]
    pub fn icm_reg_axi_hwcfg_decoder_type(&self) -> ICM_REG_AXI_HWCFG_DECODER_TYPE_R {
        ICM_REG_AXI_HWCFG_DECODER_TYPE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - NA"]
    #[inline(always)]
    pub fn icm_reg_axi_hwcfg_remap_en(&self) -> ICM_REG_AXI_HWCFG_REMAP_EN_R {
        ICM_REG_AXI_HWCFG_REMAP_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - NA"]
    #[inline(always)]
    pub fn icm_reg_axi_hwcfg_bi_dir_cmd_en(&self) -> ICM_REG_AXI_HWCFG_BI_DIR_CMD_EN_R {
        ICM_REG_AXI_HWCFG_BI_DIR_CMD_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - NA"]
    #[inline(always)]
    pub fn icm_reg_axi_hwcfg_low_power_inf_en(&self) -> ICM_REG_AXI_HWCFG_LOW_POWER_INF_EN_R {
        ICM_REG_AXI_HWCFG_LOW_POWER_INF_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 12:16 - NA"]
    #[inline(always)]
    pub fn icm_reg_axi_hwcfg_axi_num_masters(&self) -> ICM_REG_AXI_HWCFG_AXI_NUM_MASTERS_R {
        ICM_REG_AXI_HWCFG_AXI_NUM_MASTERS_R::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24 - NA"]
    #[inline(always)]
    pub fn icm_reg_axi_hwcfg_axi_num_slaves(&self) -> ICM_REG_AXI_HWCFG_AXI_NUM_SLAVES_R {
        ICM_REG_AXI_HWCFG_AXI_NUM_SLAVES_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HW_CFG")
            .field(
                "icm_reg_axi_hwcfg_qos_support",
                &self.icm_reg_axi_hwcfg_qos_support(),
            )
            .field(
                "icm_reg_axi_hwcfg_apb3_support",
                &self.icm_reg_axi_hwcfg_apb3_support(),
            )
            .field(
                "icm_reg_axi_hwcfg_axi4_support",
                &self.icm_reg_axi_hwcfg_axi4_support(),
            )
            .field(
                "icm_reg_axi_hwcfg_lock_en",
                &self.icm_reg_axi_hwcfg_lock_en(),
            )
            .field(
                "icm_reg_axi_hwcfg_trust_zone_en",
                &self.icm_reg_axi_hwcfg_trust_zone_en(),
            )
            .field(
                "icm_reg_axi_hwcfg_decoder_type",
                &self.icm_reg_axi_hwcfg_decoder_type(),
            )
            .field(
                "icm_reg_axi_hwcfg_remap_en",
                &self.icm_reg_axi_hwcfg_remap_en(),
            )
            .field(
                "icm_reg_axi_hwcfg_bi_dir_cmd_en",
                &self.icm_reg_axi_hwcfg_bi_dir_cmd_en(),
            )
            .field(
                "icm_reg_axi_hwcfg_low_power_inf_en",
                &self.icm_reg_axi_hwcfg_low_power_inf_en(),
            )
            .field(
                "icm_reg_axi_hwcfg_axi_num_masters",
                &self.icm_reg_axi_hwcfg_axi_num_masters(),
            )
            .field(
                "icm_reg_axi_hwcfg_axi_num_slaves",
                &self.icm_reg_axi_hwcfg_axi_num_slaves(),
            )
            .finish()
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hw_cfg::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HW_CFG_SPEC;
impl crate::RegisterSpec for HW_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hw_cfg::R`](R) reader structure"]
impl crate::Readable for HW_CFG_SPEC {}
#[doc = "`reset()` method sets HW_CFG to value 0x0070_d151"]
impl crate::Resettable for HW_CFG_SPEC {
    const RESET_VALUE: u32 = 0x0070_d151;
}
