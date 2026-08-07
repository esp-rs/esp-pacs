#[doc = "Register `EXT_WAKEUP_CNTL` reader"]
pub type R = crate::R<EXT_WAKEUP_CNTL_SPEC>;
#[doc = "Register `EXT_WAKEUP_CNTL` writer"]
pub type W = crate::W<EXT_WAKEUP_CNTL_SPEC>;
#[doc = "Field `EXT_WAKEUP_SEL` reader - enable io0~15 bit map use to external wakeup 1: enable 0: disable"]
pub type EXT_WAKEUP_SEL_R = crate::FieldReader<u16>;
#[doc = "Field `EXT_WAKEUP_SEL` writer - enable io0~15 bit map use to external wakeup 1: enable 0: disable"]
pub type EXT_WAKEUP_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `EXT_WAKEUP_LV` reader - select external wakeup io level 1: io high level wakeup 0: io low level wakeup"]
pub type EXT_WAKEUP_LV_R = crate::FieldReader<u16>;
#[doc = "Field `EXT_WAKEUP_LV` writer - select external wakeup io level 1: io high level wakeup 0: io low level wakeup"]
pub type EXT_WAKEUP_LV_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - enable io0~15 bit map use to external wakeup 1: enable 0: disable"]
    #[inline(always)]
    pub fn ext_wakeup_sel(&self) -> EXT_WAKEUP_SEL_R {
        EXT_WAKEUP_SEL_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - select external wakeup io level 1: io high level wakeup 0: io low level wakeup"]
    #[inline(always)]
    pub fn ext_wakeup_lv(&self) -> EXT_WAKEUP_LV_R {
        EXT_WAKEUP_LV_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXT_WAKEUP_CNTL")
            .field("ext_wakeup_sel", &self.ext_wakeup_sel())
            .field("ext_wakeup_lv", &self.ext_wakeup_lv())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - enable io0~15 bit map use to external wakeup 1: enable 0: disable"]
    #[inline(always)]
    pub fn ext_wakeup_sel(&mut self) -> EXT_WAKEUP_SEL_W<'_, EXT_WAKEUP_CNTL_SPEC> {
        EXT_WAKEUP_SEL_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - select external wakeup io level 1: io high level wakeup 0: io low level wakeup"]
    #[inline(always)]
    pub fn ext_wakeup_lv(&mut self) -> EXT_WAKEUP_LV_W<'_, EXT_WAKEUP_CNTL_SPEC> {
        EXT_WAKEUP_LV_W::new(self, 16)
    }
}
#[doc = "configure alwayson external io wakeup\n\nYou can [`read`](crate::Reg::read) this register and get [`ext_wakeup_cntl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ext_wakeup_cntl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXT_WAKEUP_CNTL_SPEC;
impl crate::RegisterSpec for EXT_WAKEUP_CNTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ext_wakeup_cntl::R`](R) reader structure"]
impl crate::Readable for EXT_WAKEUP_CNTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ext_wakeup_cntl::W`](W) writer structure"]
impl crate::Writable for EXT_WAKEUP_CNTL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EXT_WAKEUP_CNTL to value 0"]
impl crate::Resettable for EXT_WAKEUP_CNTL_SPEC {}
