#[doc = "Register `CONFIG3` reader"]
pub type R = crate::R<CONFIG3_SPEC>;
#[doc = "Register `CONFIG3` writer"]
pub type W = crate::W<CONFIG3_SPEC>;
#[doc = "Field `WDT_STG2_HOLD` reader - Configure the timeout time for stage2. \\\\Measurement unit: LP\\_DYN\\_SLOW\\_CLK"]
pub type WDT_STG2_HOLD_R = crate::FieldReader<u32>;
#[doc = "Field `WDT_STG2_HOLD` writer - Configure the timeout time for stage2. \\\\Measurement unit: LP\\_DYN\\_SLOW\\_CLK"]
pub type WDT_STG2_HOLD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Configure the timeout time for stage2. \\\\Measurement unit: LP\\_DYN\\_SLOW\\_CLK"]
    #[inline(always)]
    pub fn wdt_stg2_hold(&self) -> WDT_STG2_HOLD_R {
        WDT_STG2_HOLD_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONFIG3")
            .field("wdt_stg2_hold", &self.wdt_stg2_hold())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Configure the timeout time for stage2. \\\\Measurement unit: LP\\_DYN\\_SLOW\\_CLK"]
    #[inline(always)]
    pub fn wdt_stg2_hold(&mut self) -> WDT_STG2_HOLD_W<'_, CONFIG3_SPEC> {
        WDT_STG2_HOLD_W::new(self, 0)
    }
}
#[doc = "Configure the RWDT timeout of stage2\n\nYou can [`read`](crate::Reg::read) this register and get [`config3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONFIG3_SPEC;
impl crate::RegisterSpec for CONFIG3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config3::R`](R) reader structure"]
impl crate::Readable for CONFIG3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`config3::W`](W) writer structure"]
impl crate::Writable for CONFIG3_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONFIG3 to value 0x0fff"]
impl crate::Resettable for CONFIG3_SPEC {
    const RESET_VALUE: u32 = 0x0fff;
}
