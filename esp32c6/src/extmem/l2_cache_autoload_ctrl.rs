#[doc = "Register `L2_CACHE_AUTOLOAD_CTRL` reader"]
pub struct R(crate::R<L2_CACHE_AUTOLOAD_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<L2_CACHE_AUTOLOAD_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<L2_CACHE_AUTOLOAD_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<L2_CACHE_AUTOLOAD_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `L2_CACHE_AUTOLOAD_ENA` reader - The bit is used to enable and disable autoload operation on L2-Cache. 1: enable, 0: disable."]
pub type L2_CACHE_AUTOLOAD_ENA_R = crate::BitReader;
#[doc = "Field `L2_CACHE_AUTOLOAD_DONE` reader - The bit is used to indicate whether autoload operation on L2-Cache is finished or not. 0: not finished. 1: finished."]
pub type L2_CACHE_AUTOLOAD_DONE_R = crate::BitReader;
#[doc = "Field `L2_CACHE_AUTOLOAD_ORDER` reader - The bit is used to configure the direction of autoload operation on L2-Cache. 0: ascending. 1: descending."]
pub type L2_CACHE_AUTOLOAD_ORDER_R = crate::BitReader;
#[doc = "Field `L2_CACHE_AUTOLOAD_TRIGGER_MODE` reader - The field is used to configure trigger mode of autoload operation on L2-Cache. 0/3: miss-trigger, 1: hit-trigger, 2: miss-hit-trigger."]
pub type L2_CACHE_AUTOLOAD_TRIGGER_MODE_R = crate::FieldReader;
#[doc = "Field `L2_CACHE_AUTOLOAD_SCT0_ENA` reader - The bit is used to enable the first section for autoload operation on L2-Cache."]
pub type L2_CACHE_AUTOLOAD_SCT0_ENA_R = crate::BitReader;
#[doc = "Field `L2_CACHE_AUTOLOAD_SCT1_ENA` reader - The bit is used to enable the second section for autoload operation on L2-Cache."]
pub type L2_CACHE_AUTOLOAD_SCT1_ENA_R = crate::BitReader;
#[doc = "Field `L2_CACHE_AUTOLOAD_SCT2_ENA` reader - The bit is used to enable the third section for autoload operation on L2-Cache."]
pub type L2_CACHE_AUTOLOAD_SCT2_ENA_R = crate::BitReader;
#[doc = "Field `L2_CACHE_AUTOLOAD_SCT3_ENA` reader - The bit is used to enable the fourth section for autoload operation on L2-Cache."]
pub type L2_CACHE_AUTOLOAD_SCT3_ENA_R = crate::BitReader;
#[doc = "Field `L2_CACHE_AUTOLOAD_RGID` reader - The bit is used to set the gid of l2 cache autoload."]
pub type L2_CACHE_AUTOLOAD_RGID_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - The bit is used to enable and disable autoload operation on L2-Cache. 1: enable, 0: disable."]
    #[inline(always)]
    pub fn l2_cache_autoload_ena(&self) -> L2_CACHE_AUTOLOAD_ENA_R {
        L2_CACHE_AUTOLOAD_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to indicate whether autoload operation on L2-Cache is finished or not. 0: not finished. 1: finished."]
    #[inline(always)]
    pub fn l2_cache_autoload_done(&self) -> L2_CACHE_AUTOLOAD_DONE_R {
        L2_CACHE_AUTOLOAD_DONE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The bit is used to configure the direction of autoload operation on L2-Cache. 0: ascending. 1: descending."]
    #[inline(always)]
    pub fn l2_cache_autoload_order(&self) -> L2_CACHE_AUTOLOAD_ORDER_R {
        L2_CACHE_AUTOLOAD_ORDER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - The field is used to configure trigger mode of autoload operation on L2-Cache. 0/3: miss-trigger, 1: hit-trigger, 2: miss-hit-trigger."]
    #[inline(always)]
    pub fn l2_cache_autoload_trigger_mode(&self) -> L2_CACHE_AUTOLOAD_TRIGGER_MODE_R {
        L2_CACHE_AUTOLOAD_TRIGGER_MODE_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 8 - The bit is used to enable the first section for autoload operation on L2-Cache."]
    #[inline(always)]
    pub fn l2_cache_autoload_sct0_ena(&self) -> L2_CACHE_AUTOLOAD_SCT0_ENA_R {
        L2_CACHE_AUTOLOAD_SCT0_ENA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The bit is used to enable the second section for autoload operation on L2-Cache."]
    #[inline(always)]
    pub fn l2_cache_autoload_sct1_ena(&self) -> L2_CACHE_AUTOLOAD_SCT1_ENA_R {
        L2_CACHE_AUTOLOAD_SCT1_ENA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The bit is used to enable the third section for autoload operation on L2-Cache."]
    #[inline(always)]
    pub fn l2_cache_autoload_sct2_ena(&self) -> L2_CACHE_AUTOLOAD_SCT2_ENA_R {
        L2_CACHE_AUTOLOAD_SCT2_ENA_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The bit is used to enable the fourth section for autoload operation on L2-Cache."]
    #[inline(always)]
    pub fn l2_cache_autoload_sct3_ena(&self) -> L2_CACHE_AUTOLOAD_SCT3_ENA_R {
        L2_CACHE_AUTOLOAD_SCT3_ENA_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15 - The bit is used to set the gid of l2 cache autoload."]
    #[inline(always)]
    pub fn l2_cache_autoload_rgid(&self) -> L2_CACHE_AUTOLOAD_RGID_R {
        L2_CACHE_AUTOLOAD_RGID_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("L2_CACHE_AUTOLOAD_CTRL")
            .field(
                "l2_cache_autoload_ena",
                &format_args!("{}", self.l2_cache_autoload_ena().bit()),
            )
            .field(
                "l2_cache_autoload_done",
                &format_args!("{}", self.l2_cache_autoload_done().bit()),
            )
            .field(
                "l2_cache_autoload_order",
                &format_args!("{}", self.l2_cache_autoload_order().bit()),
            )
            .field(
                "l2_cache_autoload_trigger_mode",
                &format_args!("{}", self.l2_cache_autoload_trigger_mode().bits()),
            )
            .field(
                "l2_cache_autoload_sct0_ena",
                &format_args!("{}", self.l2_cache_autoload_sct0_ena().bit()),
            )
            .field(
                "l2_cache_autoload_sct1_ena",
                &format_args!("{}", self.l2_cache_autoload_sct1_ena().bit()),
            )
            .field(
                "l2_cache_autoload_sct2_ena",
                &format_args!("{}", self.l2_cache_autoload_sct2_ena().bit()),
            )
            .field(
                "l2_cache_autoload_sct3_ena",
                &format_args!("{}", self.l2_cache_autoload_sct3_ena().bit()),
            )
            .field(
                "l2_cache_autoload_rgid",
                &format_args!("{}", self.l2_cache_autoload_rgid().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<L2_CACHE_AUTOLOAD_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "L2 Cache autoload-operation control register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [l2_cache_autoload_ctrl](index.html) module"]
pub struct L2_CACHE_AUTOLOAD_CTRL_SPEC;
impl crate::RegisterSpec for L2_CACHE_AUTOLOAD_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [l2_cache_autoload_ctrl::R](R) reader structure"]
impl crate::Readable for L2_CACHE_AUTOLOAD_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets L2_CACHE_AUTOLOAD_CTRL to value 0x02"]
impl crate::Resettable for L2_CACHE_AUTOLOAD_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
