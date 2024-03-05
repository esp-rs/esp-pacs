#[doc = "Register `CONFIG1` reader"]
pub type R = crate::R<CONFIG1_SPEC>;
#[doc = "Register `CONFIG1` writer"]
pub type W = crate::W<CONFIG1_SPEC>;
#[doc = "Field `WDT_STG0_HOLD` reader - need_des"]
pub type WDT_STG0_HOLD_R = crate::FieldReader<u32>;
#[doc = "Field `WDT_STG0_HOLD` writer - need_des"]
pub type WDT_STG0_HOLD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn wdt_stg0_hold(&self) -> WDT_STG0_HOLD_R {
        WDT_STG0_HOLD_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONFIG1")
            .field(
                "wdt_stg0_hold",
                &format_args!("{}", self.wdt_stg0_hold().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CONFIG1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_stg0_hold(&mut self) -> WDT_STG0_HOLD_W<CONFIG1_SPEC> {
        WDT_STG0_HOLD_W::new(self, 0)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`config1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONFIG1_SPEC;
impl crate::RegisterSpec for CONFIG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config1::R`](R) reader structure"]
impl crate::Readable for CONFIG1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`config1::W`](W) writer structure"]
impl crate::Writable for CONFIG1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONFIG1 to value 0x0003_0d40"]
impl crate::Resettable for CONFIG1_SPEC {
    const RESET_VALUE: u32 = 0x0003_0d40;
}
