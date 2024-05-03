#[doc = "Register `BUS_FREE_TIME` reader"]
pub type R = crate::R<BUS_FREE_TIME_SPEC>;
#[doc = "Register `BUS_FREE_TIME` writer"]
pub type W = crate::W<BUS_FREE_TIME_SPEC>;
#[doc = "Field `REG_BUS_FREE_TIME` reader - I3C Bus Free Count Value. This field is used only in Master mode. In pure Bus System, this field represents tCAS. In Mixed Bus System, this field is expected to be programmed to tLOW of I2C Timing."]
pub type REG_BUS_FREE_TIME_R = crate::FieldReader<u16>;
#[doc = "Field `REG_BUS_FREE_TIME` writer - I3C Bus Free Count Value. This field is used only in Master mode. In pure Bus System, this field represents tCAS. In Mixed Bus System, this field is expected to be programmed to tLOW of I2C Timing."]
pub type REG_BUS_FREE_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - I3C Bus Free Count Value. This field is used only in Master mode. In pure Bus System, this field represents tCAS. In Mixed Bus System, this field is expected to be programmed to tLOW of I2C Timing."]
    #[inline(always)]
    pub fn reg_bus_free_time(&self) -> REG_BUS_FREE_TIME_R {
        REG_BUS_FREE_TIME_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BUS_FREE_TIME")
            .field("reg_bus_free_time", &self.reg_bus_free_time().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BUS_FREE_TIME_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:15 - I3C Bus Free Count Value. This field is used only in Master mode. In pure Bus System, this field represents tCAS. In Mixed Bus System, this field is expected to be programmed to tLOW of I2C Timing."]
    #[inline(always)]
    #[must_use]
    pub fn reg_bus_free_time(&mut self) -> REG_BUS_FREE_TIME_W<BUS_FREE_TIME_SPEC> {
        REG_BUS_FREE_TIME_W::new(self, 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bus_free_time::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bus_free_time::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BUS_FREE_TIME_SPEC;
impl crate::RegisterSpec for BUS_FREE_TIME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bus_free_time::R`](R) reader structure"]
impl crate::Readable for BUS_FREE_TIME_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bus_free_time::W`](W) writer structure"]
impl crate::Writable for BUS_FREE_TIME_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BUS_FREE_TIME to value 0x05"]
impl crate::Resettable for BUS_FREE_TIME_SPEC {
    const RESET_VALUE: u32 = 0x05;
}
