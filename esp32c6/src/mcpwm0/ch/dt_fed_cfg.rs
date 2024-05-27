///Register `DT_FED_CFG` reader
pub type R = crate::R<DT_FED_CFG_SPEC>;
///Register `DT_FED_CFG` writer
pub type W = crate::W<DT_FED_CFG_SPEC>;
///Field `FED` reader - Shadow register for FED
pub type FED_R = crate::FieldReader<u16>;
///Field `FED` writer - Shadow register for FED
pub type FED_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    ///Bits 0:15 - Shadow register for FED
    #[inline(always)]
    pub fn fed(&self) -> FED_R {
        FED_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DT_FED_CFG")
            .field("fed", &self.fed())
            .finish()
    }
}
impl W {
    ///Bits 0:15 - Shadow register for FED
    #[inline(always)]
    #[must_use]
    pub fn fed(&mut self) -> FED_W<DT_FED_CFG_SPEC> {
        FED_W::new(self, 0)
    }
}
/**Shadow register for falling edge delay (FED).

You can [`read`](crate::generic::Reg::read) this register and get [`dt_fed_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt_fed_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DT_FED_CFG_SPEC;
impl crate::RegisterSpec for DT_FED_CFG_SPEC {
    type Ux = u32;
}
///`read()` method returns [`dt_fed_cfg::R`](R) reader structure
impl crate::Readable for DT_FED_CFG_SPEC {}
///`write(|w| ..)` method takes [`dt_fed_cfg::W`](W) writer structure
impl crate::Writable for DT_FED_CFG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DT_FED_CFG to value 0
impl crate::Resettable for DT_FED_CFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
