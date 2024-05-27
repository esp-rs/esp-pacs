#[doc = "Register `EXT_WAKEUP_CONF` reader"]
pub type R = crate::R<EXT_WAKEUP_CONF_SPEC>;
#[doc = "Register `EXT_WAKEUP_CONF` writer"]
pub type W = crate::W<EXT_WAKEUP_CONF_SPEC>;
#[doc = "Field `EXT_WAKEUP0_LV` reader - 0: external wakeup at low level 1: external wakeup at high level"]
pub type EXT_WAKEUP0_LV_R = crate::BitReader;
#[doc = "Field `EXT_WAKEUP0_LV` writer - 0: external wakeup at low level 1: external wakeup at high level"]
pub type EXT_WAKEUP0_LV_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXT_WAKEUP1_LV` reader - 0: external wakeup at low level 1: external wakeup at high level"]
pub type EXT_WAKEUP1_LV_R = crate::BitReader;
#[doc = "Field `EXT_WAKEUP1_LV` writer - 0: external wakeup at low level 1: external wakeup at high level"]
pub type EXT_WAKEUP1_LV_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 30 - 0: external wakeup at low level 1: external wakeup at high level"]
    #[inline(always)]
    pub fn ext_wakeup0_lv(&self) -> EXT_WAKEUP0_LV_R {
        EXT_WAKEUP0_LV_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 0: external wakeup at low level 1: external wakeup at high level"]
    #[inline(always)]
    pub fn ext_wakeup1_lv(&self) -> EXT_WAKEUP1_LV_R {
        EXT_WAKEUP1_LV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXT_WAKEUP_CONF")
            .field("ext_wakeup0_lv", &self.ext_wakeup0_lv())
            .field("ext_wakeup1_lv", &self.ext_wakeup1_lv())
            .finish()
    }
}
impl W {
    #[doc = "Bit 30 - 0: external wakeup at low level 1: external wakeup at high level"]
    #[inline(always)]
    #[must_use]
    pub fn ext_wakeup0_lv(&mut self) -> EXT_WAKEUP0_LV_W<EXT_WAKEUP_CONF_SPEC> {
        EXT_WAKEUP0_LV_W::new(self, 30)
    }
    #[doc = "Bit 31 - 0: external wakeup at low level 1: external wakeup at high level"]
    #[inline(always)]
    #[must_use]
    pub fn ext_wakeup1_lv(&mut self) -> EXT_WAKEUP1_LV_W<EXT_WAKEUP_CONF_SPEC> {
        EXT_WAKEUP1_LV_W::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ext_wakeup_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_wakeup_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXT_WAKEUP_CONF_SPEC;
impl crate::RegisterSpec for EXT_WAKEUP_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ext_wakeup_conf::R`](R) reader structure"]
impl crate::Readable for EXT_WAKEUP_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ext_wakeup_conf::W`](W) writer structure"]
impl crate::Writable for EXT_WAKEUP_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXT_WAKEUP_CONF to value 0"]
impl crate::Resettable for EXT_WAKEUP_CONF_SPEC {
    const RESET_VALUE: u32 = 0;
}
