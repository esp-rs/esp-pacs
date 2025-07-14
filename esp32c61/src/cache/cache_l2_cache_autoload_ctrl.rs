#[doc = "Register `CACHE_L2_CACHE_AUTOLOAD_CTRL` reader"]
pub type R = crate::R<CACHE_L2_CACHE_AUTOLOAD_CTRL_SPEC>;
#[doc = "Field `CACHE_L2_CACHE_AUTOLOAD_ENA` reader - The bit is used to enable and disable autoload operation on L2-Cache. 1: enable, 0: disable."]
pub type CACHE_L2_CACHE_AUTOLOAD_ENA_R = crate::BitReader;
#[doc = "Field `CACHE_L2_CACHE_AUTOLOAD_DONE` reader - The bit is used to indicate whether autoload operation on L2-Cache is finished or not. 0: not finished. 1: finished."]
pub type CACHE_L2_CACHE_AUTOLOAD_DONE_R = crate::BitReader;
#[doc = "Field `CACHE_L2_CACHE_AUTOLOAD_ORDER` reader - The bit is used to configure the direction of autoload operation on L2-Cache. 0: ascending. 1: descending."]
pub type CACHE_L2_CACHE_AUTOLOAD_ORDER_R = crate::BitReader;
#[doc = "Field `CACHE_L2_CACHE_AUTOLOAD_TRIGGER_MODE` reader - The field is used to configure trigger mode of autoload operation on L2-Cache. 0/3: miss-trigger, 1: hit-trigger, 2: miss-hit-trigger."]
pub type CACHE_L2_CACHE_AUTOLOAD_TRIGGER_MODE_R = crate::FieldReader;
#[doc = "Field `CACHE_L2_CACHE_AUTOLOAD_SCT0_ENA` reader - The bit is used to enable the first section for autoload operation on L2-Cache."]
pub type CACHE_L2_CACHE_AUTOLOAD_SCT0_ENA_R = crate::BitReader;
#[doc = "Field `CACHE_L2_CACHE_AUTOLOAD_SCT1_ENA` reader - The bit is used to enable the second section for autoload operation on L2-Cache."]
pub type CACHE_L2_CACHE_AUTOLOAD_SCT1_ENA_R = crate::BitReader;
#[doc = "Field `CACHE_L2_CACHE_AUTOLOAD_SCT2_ENA` reader - The bit is used to enable the third section for autoload operation on L2-Cache."]
pub type CACHE_L2_CACHE_AUTOLOAD_SCT2_ENA_R = crate::BitReader;
#[doc = "Field `CACHE_L2_CACHE_AUTOLOAD_SCT3_ENA` reader - The bit is used to enable the fourth section for autoload operation on L2-Cache."]
pub type CACHE_L2_CACHE_AUTOLOAD_SCT3_ENA_R = crate::BitReader;
#[doc = "Field `CACHE_L2_CACHE_AUTOLOAD_RGID` reader - The bit is used to set the gid of l2 cache autoload."]
pub type CACHE_L2_CACHE_AUTOLOAD_RGID_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - The bit is used to enable and disable autoload operation on L2-Cache. 1: enable, 0: disable."]
    #[inline(always)]
    pub fn cache_l2_cache_autoload_ena(&self) -> CACHE_L2_CACHE_AUTOLOAD_ENA_R {
        CACHE_L2_CACHE_AUTOLOAD_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to indicate whether autoload operation on L2-Cache is finished or not. 0: not finished. 1: finished."]
    #[inline(always)]
    pub fn cache_l2_cache_autoload_done(&self) -> CACHE_L2_CACHE_AUTOLOAD_DONE_R {
        CACHE_L2_CACHE_AUTOLOAD_DONE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The bit is used to configure the direction of autoload operation on L2-Cache. 0: ascending. 1: descending."]
    #[inline(always)]
    pub fn cache_l2_cache_autoload_order(&self) -> CACHE_L2_CACHE_AUTOLOAD_ORDER_R {
        CACHE_L2_CACHE_AUTOLOAD_ORDER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - The field is used to configure trigger mode of autoload operation on L2-Cache. 0/3: miss-trigger, 1: hit-trigger, 2: miss-hit-trigger."]
    #[inline(always)]
    pub fn cache_l2_cache_autoload_trigger_mode(&self) -> CACHE_L2_CACHE_AUTOLOAD_TRIGGER_MODE_R {
        CACHE_L2_CACHE_AUTOLOAD_TRIGGER_MODE_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 8 - The bit is used to enable the first section for autoload operation on L2-Cache."]
    #[inline(always)]
    pub fn cache_l2_cache_autoload_sct0_ena(&self) -> CACHE_L2_CACHE_AUTOLOAD_SCT0_ENA_R {
        CACHE_L2_CACHE_AUTOLOAD_SCT0_ENA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The bit is used to enable the second section for autoload operation on L2-Cache."]
    #[inline(always)]
    pub fn cache_l2_cache_autoload_sct1_ena(&self) -> CACHE_L2_CACHE_AUTOLOAD_SCT1_ENA_R {
        CACHE_L2_CACHE_AUTOLOAD_SCT1_ENA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The bit is used to enable the third section for autoload operation on L2-Cache."]
    #[inline(always)]
    pub fn cache_l2_cache_autoload_sct2_ena(&self) -> CACHE_L2_CACHE_AUTOLOAD_SCT2_ENA_R {
        CACHE_L2_CACHE_AUTOLOAD_SCT2_ENA_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The bit is used to enable the fourth section for autoload operation on L2-Cache."]
    #[inline(always)]
    pub fn cache_l2_cache_autoload_sct3_ena(&self) -> CACHE_L2_CACHE_AUTOLOAD_SCT3_ENA_R {
        CACHE_L2_CACHE_AUTOLOAD_SCT3_ENA_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15 - The bit is used to set the gid of l2 cache autoload."]
    #[inline(always)]
    pub fn cache_l2_cache_autoload_rgid(&self) -> CACHE_L2_CACHE_AUTOLOAD_RGID_R {
        CACHE_L2_CACHE_AUTOLOAD_RGID_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_L2_CACHE_AUTOLOAD_CTRL")
            .field(
                "cache_l2_cache_autoload_ena",
                &self.cache_l2_cache_autoload_ena(),
            )
            .field(
                "cache_l2_cache_autoload_done",
                &self.cache_l2_cache_autoload_done(),
            )
            .field(
                "cache_l2_cache_autoload_order",
                &self.cache_l2_cache_autoload_order(),
            )
            .field(
                "cache_l2_cache_autoload_trigger_mode",
                &self.cache_l2_cache_autoload_trigger_mode(),
            )
            .field(
                "cache_l2_cache_autoload_sct0_ena",
                &self.cache_l2_cache_autoload_sct0_ena(),
            )
            .field(
                "cache_l2_cache_autoload_sct1_ena",
                &self.cache_l2_cache_autoload_sct1_ena(),
            )
            .field(
                "cache_l2_cache_autoload_sct2_ena",
                &self.cache_l2_cache_autoload_sct2_ena(),
            )
            .field(
                "cache_l2_cache_autoload_sct3_ena",
                &self.cache_l2_cache_autoload_sct3_ena(),
            )
            .field(
                "cache_l2_cache_autoload_rgid",
                &self.cache_l2_cache_autoload_rgid(),
            )
            .finish()
    }
}
#[doc = "L2 Cache autoload-operation control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_l2_cache_autoload_ctrl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_L2_CACHE_AUTOLOAD_CTRL_SPEC;
impl crate::RegisterSpec for CACHE_L2_CACHE_AUTOLOAD_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cache_l2_cache_autoload_ctrl::R`](R) reader structure"]
impl crate::Readable for CACHE_L2_CACHE_AUTOLOAD_CTRL_SPEC {}
#[doc = "`reset()` method sets CACHE_L2_CACHE_AUTOLOAD_CTRL to value 0x02"]
impl crate::Resettable for CACHE_L2_CACHE_AUTOLOAD_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x02;
}
