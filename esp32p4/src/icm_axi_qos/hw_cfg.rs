#[doc = "Register `HW_CFG` reader"]
pub type R = crate::R<HW_CFG_SPEC>;
#[doc = "Field `QOS_SUPPORT` reader - "]
pub type QOS_SUPPORT_R = crate::BitReader;
#[doc = "Field `APB3_SUPPORT` reader - "]
pub type APB3_SUPPORT_R = crate::BitReader;
#[doc = "Field `AXI4_SUPPORT` reader - "]
pub type AXI4_SUPPORT_R = crate::BitReader;
#[doc = "Field `LOCK_EN` reader - "]
pub type LOCK_EN_R = crate::BitReader;
#[doc = "Field `TRUST_ZONE_EN` reader - "]
pub type TRUST_ZONE_EN_R = crate::BitReader;
#[doc = "Field `DECODER_TYPE` reader - "]
pub type DECODER_TYPE_R = crate::BitReader;
#[doc = "Field `REMAP_EN` reader - "]
pub type REMAP_EN_R = crate::BitReader;
#[doc = "Field `BI_DIR_CMD_EN` reader - "]
pub type BI_DIR_CMD_EN_R = crate::BitReader;
#[doc = "Field `LOW_POWER_INF_EN` reader - "]
pub type LOW_POWER_INF_EN_R = crate::BitReader;
#[doc = "Field `AXI_NUM_MASTERS` reader - "]
pub type AXI_NUM_MASTERS_R = crate::FieldReader;
#[doc = "Field `AXI_NUM_SLAVES` reader - "]
pub type AXI_NUM_SLAVES_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn qos_support(&self) -> QOS_SUPPORT_R {
        QOS_SUPPORT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn apb3_support(&self) -> APB3_SUPPORT_R {
        APB3_SUPPORT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn axi4_support(&self) -> AXI4_SUPPORT_R {
        AXI4_SUPPORT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn lock_en(&self) -> LOCK_EN_R {
        LOCK_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn trust_zone_en(&self) -> TRUST_ZONE_EN_R {
        TRUST_ZONE_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn decoder_type(&self) -> DECODER_TYPE_R {
        DECODER_TYPE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn remap_en(&self) -> REMAP_EN_R {
        REMAP_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn bi_dir_cmd_en(&self) -> BI_DIR_CMD_EN_R {
        BI_DIR_CMD_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn low_power_inf_en(&self) -> LOW_POWER_INF_EN_R {
        LOW_POWER_INF_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 12:16"]
    #[inline(always)]
    pub fn axi_num_masters(&self) -> AXI_NUM_MASTERS_R {
        AXI_NUM_MASTERS_R::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bits 20:24"]
    #[inline(always)]
    pub fn axi_num_slaves(&self) -> AXI_NUM_SLAVES_R {
        AXI_NUM_SLAVES_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HW_CFG")
            .field("qos_support", &self.qos_support())
            .field("apb3_support", &self.apb3_support())
            .field("axi4_support", &self.axi4_support())
            .field("lock_en", &self.lock_en())
            .field("trust_zone_en", &self.trust_zone_en())
            .field("decoder_type", &self.decoder_type())
            .field("remap_en", &self.remap_en())
            .field("bi_dir_cmd_en", &self.bi_dir_cmd_en())
            .field("low_power_inf_en", &self.low_power_inf_en())
            .field("axi_num_masters", &self.axi_num_masters())
            .field("axi_num_slaves", &self.axi_num_slaves())
            .finish()
    }
}
#[doc = "QoS hardware configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`hw_cfg::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HW_CFG_SPEC;
impl crate::RegisterSpec for HW_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hw_cfg::R`](R) reader structure"]
impl crate::Readable for HW_CFG_SPEC {}
#[doc = "`reset()` method sets HW_CFG to value 0"]
impl crate::Resettable for HW_CFG_SPEC {}
