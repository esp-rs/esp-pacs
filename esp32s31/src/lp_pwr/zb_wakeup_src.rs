#[doc = "Register `ZB_WAKEUP_SRC` reader"]
pub type R = crate::R<ZB_WAKEUP_SRC_SPEC>;
#[doc = "Register `ZB_WAKEUP_SRC` writer"]
pub type W = crate::W<ZB_WAKEUP_SRC_SPEC>;
#[doc = "Field `ZB_WAKEUP_SOURCE_EN` reader - wakeup enazb signal for zb"]
pub type ZB_WAKEUP_SOURCE_EN_R = crate::FieldReader<u32>;
#[doc = "Field `ZB_WAKEUP_SOURCE_EN` writer - wakeup enazb signal for zb"]
pub type ZB_WAKEUP_SOURCE_EN_W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bits 0:30 - wakeup enazb signal for zb"]
    #[inline(always)]
    pub fn zb_wakeup_source_en(&self) -> ZB_WAKEUP_SOURCE_EN_R {
        ZB_WAKEUP_SOURCE_EN_R::new(self.bits & 0x7fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ZB_WAKEUP_SRC")
            .field("zb_wakeup_source_en", &self.zb_wakeup_source_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:30 - wakeup enazb signal for zb"]
    #[inline(always)]
    pub fn zb_wakeup_source_en(&mut self) -> ZB_WAKEUP_SOURCE_EN_W<'_, ZB_WAKEUP_SRC_SPEC> {
        ZB_WAKEUP_SOURCE_EN_W::new(self, 0)
    }
}
#[doc = "wakeup source register for zb\n\nYou can [`read`](crate::Reg::read) this register and get [`zb_wakeup_src::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`zb_wakeup_src::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ZB_WAKEUP_SRC_SPEC;
impl crate::RegisterSpec for ZB_WAKEUP_SRC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`zb_wakeup_src::R`](R) reader structure"]
impl crate::Readable for ZB_WAKEUP_SRC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`zb_wakeup_src::W`](W) writer structure"]
impl crate::Writable for ZB_WAKEUP_SRC_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ZB_WAKEUP_SRC to value 0"]
impl crate::Resettable for ZB_WAKEUP_SRC_SPEC {}
