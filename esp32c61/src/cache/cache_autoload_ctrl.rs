#[doc = "Register `CACHE_AUTOLOAD_CTRL` reader"]
pub type R = crate::R<CACHE_AUTOLOAD_CTRL_SPEC>;
#[doc = "Register `CACHE_AUTOLOAD_CTRL` writer"]
pub type W = crate::W<CACHE_AUTOLOAD_CTRL_SPEC>;
#[doc = "Field `CACHE_AUTOLOAD_ENA` reader - The bit is used to enable and disable autoload operation on L1-Cache. 1: enable, 0: disable."]
pub type CACHE_AUTOLOAD_ENA_R = crate::BitReader;
#[doc = "Field `CACHE_AUTOLOAD_ENA` writer - The bit is used to enable and disable autoload operation on L1-Cache. 1: enable, 0: disable."]
pub type CACHE_AUTOLOAD_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CACHE_AUTOLOAD_DONE` reader - The bit is used to indicate whether autoload operation on L1-Cache is finished or not. 0: not finished. 1: finished."]
pub type CACHE_AUTOLOAD_DONE_R = crate::BitReader;
#[doc = "Field `CACHE_AUTOLOAD_ORDER` reader - The bit is used to configure the direction of autoload operation on L1-Cache. 0: ascending. 1: descending."]
pub type CACHE_AUTOLOAD_ORDER_R = crate::BitReader;
#[doc = "Field `CACHE_AUTOLOAD_ORDER` writer - The bit is used to configure the direction of autoload operation on L1-Cache. 0: ascending. 1: descending."]
pub type CACHE_AUTOLOAD_ORDER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CACHE_AUTOLOAD_TRIGGER_MODE` reader - The field is used to configure trigger mode of autoload operation on L1-Cache. 0/3: miss-trigger, 1: hit-trigger, 2: miss-hit-trigger."]
pub type CACHE_AUTOLOAD_TRIGGER_MODE_R = crate::FieldReader;
#[doc = "Field `CACHE_AUTOLOAD_TRIGGER_MODE` writer - The field is used to configure trigger mode of autoload operation on L1-Cache. 0/3: miss-trigger, 1: hit-trigger, 2: miss-hit-trigger."]
pub type CACHE_AUTOLOAD_TRIGGER_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CACHE_AUTOLOAD_SCT0_ENA` reader - The bit is used to enable the first section for autoload operation on L1-Cache."]
pub type CACHE_AUTOLOAD_SCT0_ENA_R = crate::BitReader;
#[doc = "Field `CACHE_AUTOLOAD_SCT0_ENA` writer - The bit is used to enable the first section for autoload operation on L1-Cache."]
pub type CACHE_AUTOLOAD_SCT0_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CACHE_AUTOLOAD_SCT1_ENA` reader - The bit is used to enable the second section for autoload operation on L1-Cache."]
pub type CACHE_AUTOLOAD_SCT1_ENA_R = crate::BitReader;
#[doc = "Field `CACHE_AUTOLOAD_SCT1_ENA` writer - The bit is used to enable the second section for autoload operation on L1-Cache."]
pub type CACHE_AUTOLOAD_SCT1_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CACHE_AUTOLOAD_SCT2_ENA` reader - The bit is used to enable the third section for autoload operation on L1-Cache."]
pub type CACHE_AUTOLOAD_SCT2_ENA_R = crate::BitReader;
#[doc = "Field `CACHE_AUTOLOAD_SCT3_ENA` reader - The bit is used to enable the fourth section for autoload operation on L1-Cache."]
pub type CACHE_AUTOLOAD_SCT3_ENA_R = crate::BitReader;
#[doc = "Field `CACHE_AUTOLOAD_RGID` reader - The bit is used to set the gid of l1 cache autoload."]
pub type CACHE_AUTOLOAD_RGID_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - The bit is used to enable and disable autoload operation on L1-Cache. 1: enable, 0: disable."]
    #[inline(always)]
    pub fn cache_autoload_ena(&self) -> CACHE_AUTOLOAD_ENA_R {
        CACHE_AUTOLOAD_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to indicate whether autoload operation on L1-Cache is finished or not. 0: not finished. 1: finished."]
    #[inline(always)]
    pub fn cache_autoload_done(&self) -> CACHE_AUTOLOAD_DONE_R {
        CACHE_AUTOLOAD_DONE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The bit is used to configure the direction of autoload operation on L1-Cache. 0: ascending. 1: descending."]
    #[inline(always)]
    pub fn cache_autoload_order(&self) -> CACHE_AUTOLOAD_ORDER_R {
        CACHE_AUTOLOAD_ORDER_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - The field is used to configure trigger mode of autoload operation on L1-Cache. 0/3: miss-trigger, 1: hit-trigger, 2: miss-hit-trigger."]
    #[inline(always)]
    pub fn cache_autoload_trigger_mode(&self) -> CACHE_AUTOLOAD_TRIGGER_MODE_R {
        CACHE_AUTOLOAD_TRIGGER_MODE_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 8 - The bit is used to enable the first section for autoload operation on L1-Cache."]
    #[inline(always)]
    pub fn cache_autoload_sct0_ena(&self) -> CACHE_AUTOLOAD_SCT0_ENA_R {
        CACHE_AUTOLOAD_SCT0_ENA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The bit is used to enable the second section for autoload operation on L1-Cache."]
    #[inline(always)]
    pub fn cache_autoload_sct1_ena(&self) -> CACHE_AUTOLOAD_SCT1_ENA_R {
        CACHE_AUTOLOAD_SCT1_ENA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The bit is used to enable the third section for autoload operation on L1-Cache."]
    #[inline(always)]
    pub fn cache_autoload_sct2_ena(&self) -> CACHE_AUTOLOAD_SCT2_ENA_R {
        CACHE_AUTOLOAD_SCT2_ENA_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The bit is used to enable the fourth section for autoload operation on L1-Cache."]
    #[inline(always)]
    pub fn cache_autoload_sct3_ena(&self) -> CACHE_AUTOLOAD_SCT3_ENA_R {
        CACHE_AUTOLOAD_SCT3_ENA_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15 - The bit is used to set the gid of l1 cache autoload."]
    #[inline(always)]
    pub fn cache_autoload_rgid(&self) -> CACHE_AUTOLOAD_RGID_R {
        CACHE_AUTOLOAD_RGID_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_AUTOLOAD_CTRL")
            .field("cache_autoload_ena", &self.cache_autoload_ena())
            .field("cache_autoload_done", &self.cache_autoload_done())
            .field("cache_autoload_order", &self.cache_autoload_order())
            .field(
                "cache_autoload_trigger_mode",
                &self.cache_autoload_trigger_mode(),
            )
            .field("cache_autoload_sct0_ena", &self.cache_autoload_sct0_ena())
            .field("cache_autoload_sct1_ena", &self.cache_autoload_sct1_ena())
            .field("cache_autoload_sct2_ena", &self.cache_autoload_sct2_ena())
            .field("cache_autoload_sct3_ena", &self.cache_autoload_sct3_ena())
            .field("cache_autoload_rgid", &self.cache_autoload_rgid())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to enable and disable autoload operation on L1-Cache. 1: enable, 0: disable."]
    #[inline(always)]
    pub fn cache_autoload_ena(&mut self) -> CACHE_AUTOLOAD_ENA_W<'_, CACHE_AUTOLOAD_CTRL_SPEC> {
        CACHE_AUTOLOAD_ENA_W::new(self, 0)
    }
    #[doc = "Bit 2 - The bit is used to configure the direction of autoload operation on L1-Cache. 0: ascending. 1: descending."]
    #[inline(always)]
    pub fn cache_autoload_order(&mut self) -> CACHE_AUTOLOAD_ORDER_W<'_, CACHE_AUTOLOAD_CTRL_SPEC> {
        CACHE_AUTOLOAD_ORDER_W::new(self, 2)
    }
    #[doc = "Bits 3:4 - The field is used to configure trigger mode of autoload operation on L1-Cache. 0/3: miss-trigger, 1: hit-trigger, 2: miss-hit-trigger."]
    #[inline(always)]
    pub fn cache_autoload_trigger_mode(
        &mut self,
    ) -> CACHE_AUTOLOAD_TRIGGER_MODE_W<'_, CACHE_AUTOLOAD_CTRL_SPEC> {
        CACHE_AUTOLOAD_TRIGGER_MODE_W::new(self, 3)
    }
    #[doc = "Bit 8 - The bit is used to enable the first section for autoload operation on L1-Cache."]
    #[inline(always)]
    pub fn cache_autoload_sct0_ena(
        &mut self,
    ) -> CACHE_AUTOLOAD_SCT0_ENA_W<'_, CACHE_AUTOLOAD_CTRL_SPEC> {
        CACHE_AUTOLOAD_SCT0_ENA_W::new(self, 8)
    }
    #[doc = "Bit 9 - The bit is used to enable the second section for autoload operation on L1-Cache."]
    #[inline(always)]
    pub fn cache_autoload_sct1_ena(
        &mut self,
    ) -> CACHE_AUTOLOAD_SCT1_ENA_W<'_, CACHE_AUTOLOAD_CTRL_SPEC> {
        CACHE_AUTOLOAD_SCT1_ENA_W::new(self, 9)
    }
}
#[doc = "L1 Cache autoload-operation control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cache_autoload_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cache_autoload_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CACHE_AUTOLOAD_CTRL_SPEC;
impl crate::RegisterSpec for CACHE_AUTOLOAD_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cache_autoload_ctrl::R`](R) reader structure"]
impl crate::Readable for CACHE_AUTOLOAD_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cache_autoload_ctrl::W`](W) writer structure"]
impl crate::Writable for CACHE_AUTOLOAD_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CACHE_AUTOLOAD_CTRL to value 0x02"]
impl crate::Resettable for CACHE_AUTOLOAD_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x02;
}
