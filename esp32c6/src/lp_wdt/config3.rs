///Register `CONFIG3` reader
pub type R = crate::R<CONFIG3_SPEC>;
///Register `CONFIG3` writer
pub type W = crate::W<CONFIG3_SPEC>;
///Field `WDT_STG2_HOLD` reader - need_des
pub type WDT_STG2_HOLD_R = crate::FieldReader<u32>;
///Field `WDT_STG2_HOLD` writer - need_des
pub type WDT_STG2_HOLD_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - need_des
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
    ///Bits 0:31 - need_des
    #[inline(always)]
    #[must_use]
    pub fn wdt_stg2_hold(&mut self) -> WDT_STG2_HOLD_W<CONFIG3_SPEC> {
        WDT_STG2_HOLD_W::new(self, 0)
    }
}
/**need_des

You can [`read`](crate::generic::Reg::read) this register and get [`config3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`config3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CONFIG3_SPEC;
impl crate::RegisterSpec for CONFIG3_SPEC {
    type Ux = u32;
}
///`read()` method returns [`config3::R`](R) reader structure
impl crate::Readable for CONFIG3_SPEC {}
///`write(|w| ..)` method takes [`config3::W`](W) writer structure
impl crate::Writable for CONFIG3_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CONFIG3 to value 0x0fff
impl crate::Resettable for CONFIG3_SPEC {
    const RESET_VALUE: u32 = 0x0fff;
}
