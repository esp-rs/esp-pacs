#[doc = "Register `HI` reader"]
pub type R = crate::R<HI_SPEC>;
#[doc = "Register `HI` writer"]
pub type W = crate::W<HI_SPEC>;
#[doc = "Field `HI` reader - timer taget0 high 20 bits"]
pub type HI_R = crate::FieldReader<u32>;
#[doc = "Field `HI` writer - timer taget0 high 20 bits"]
pub type HI_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32, crate::Safe>;
impl R {
    #[doc = "Bits 0:19 - timer taget0 high 20 bits"]
    #[inline(always)]
    pub fn hi(&self) -> HI_R {
        HI_R::new(self.bits & 0x000f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HI").field("hi", &self.hi().bits()).finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HI_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:19 - timer taget0 high 20 bits"]
    #[inline(always)]
    #[must_use]
    pub fn hi(&mut self) -> HI_W<HI_SPEC> {
        HI_W::new(self, 0)
    }
}
#[doc = "system timer comp0 value high register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hi::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hi::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HI_SPEC;
impl crate::RegisterSpec for HI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hi::R`](R) reader structure"]
impl crate::Readable for HI_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hi::W`](W) writer structure"]
impl crate::Writable for HI_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HI to value 0"]
impl crate::Resettable for HI_SPEC {
    const RESET_VALUE: u32 = 0;
}
