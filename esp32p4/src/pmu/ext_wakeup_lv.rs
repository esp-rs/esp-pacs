#[doc = "Register `EXT_WAKEUP_LV` reader"]
pub type R = crate::R<EXT_WAKEUP_LV_SPEC>;
#[doc = "Register `EXT_WAKEUP_LV` writer"]
pub type W = crate::W<EXT_WAKEUP_LV_SPEC>;
#[doc = "Field `EXT_WAKEUP_LV` reader - need_des"]
pub type EXT_WAKEUP_LV_R = crate::FieldReader<u32>;
#[doc = "Field `EXT_WAKEUP_LV` writer - need_des"]
pub type EXT_WAKEUP_LV_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn ext_wakeup_lv(&self) -> EXT_WAKEUP_LV_R {
        EXT_WAKEUP_LV_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXT_WAKEUP_LV")
            .field("ext_wakeup_lv", &self.ext_wakeup_lv())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn ext_wakeup_lv(&mut self) -> EXT_WAKEUP_LV_W<EXT_WAKEUP_LV_SPEC> {
        EXT_WAKEUP_LV_W::new(self, 0)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ext_wakeup_lv::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_wakeup_lv::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXT_WAKEUP_LV_SPEC;
impl crate::RegisterSpec for EXT_WAKEUP_LV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ext_wakeup_lv::R`](R) reader structure"]
impl crate::Readable for EXT_WAKEUP_LV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ext_wakeup_lv::W`](W) writer structure"]
impl crate::Writable for EXT_WAKEUP_LV_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXT_WAKEUP_LV to value 0"]
impl crate::Resettable for EXT_WAKEUP_LV_SPEC {
    const RESET_VALUE: u32 = 0;
}
