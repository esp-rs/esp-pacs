///Register `TIMER6` reader
pub type R = crate::R<TIMER6_SPEC>;
///Register `TIMER6` writer
pub type W = crate::W<TIMER6_SPEC>;
///Field `DG_DCDC_WAIT_TIMER` reader -
pub type DG_DCDC_WAIT_TIMER_R = crate::FieldReader<u16>;
///Field `DG_DCDC_WAIT_TIMER` writer -
pub type DG_DCDC_WAIT_TIMER_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `DG_DCDC_POWERUP_TIMER` reader -
pub type DG_DCDC_POWERUP_TIMER_R = crate::FieldReader;
///Field `DG_DCDC_POWERUP_TIMER` writer -
pub type DG_DCDC_POWERUP_TIMER_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    ///Bits 16:24
    #[inline(always)]
    pub fn dg_dcdc_wait_timer(&self) -> DG_DCDC_WAIT_TIMER_R {
        DG_DCDC_WAIT_TIMER_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    ///Bits 25:31
    #[inline(always)]
    pub fn dg_dcdc_powerup_timer(&self) -> DG_DCDC_POWERUP_TIMER_R {
        DG_DCDC_POWERUP_TIMER_R::new(((self.bits >> 25) & 0x7f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMER6")
            .field("dg_dcdc_wait_timer", &self.dg_dcdc_wait_timer())
            .field("dg_dcdc_powerup_timer", &self.dg_dcdc_powerup_timer())
            .finish()
    }
}
impl W {
    ///Bits 16:24
    #[inline(always)]
    #[must_use]
    pub fn dg_dcdc_wait_timer(&mut self) -> DG_DCDC_WAIT_TIMER_W<TIMER6_SPEC> {
        DG_DCDC_WAIT_TIMER_W::new(self, 16)
    }
    ///Bits 25:31
    #[inline(always)]
    #[must_use]
    pub fn dg_dcdc_powerup_timer(&mut self) -> DG_DCDC_POWERUP_TIMER_W<TIMER6_SPEC> {
        DG_DCDC_POWERUP_TIMER_W::new(self, 25)
    }
}
/**Configure minimal sleep cycles register

You can [`read`](crate::generic::Reg::read) this register and get [`timer6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TIMER6_SPEC;
impl crate::RegisterSpec for TIMER6_SPEC {
    type Ux = u32;
}
///`read()` method returns [`timer6::R`](R) reader structure
impl crate::Readable for TIMER6_SPEC {}
///`write(|w| ..)` method takes [`timer6::W`](W) writer structure
impl crate::Writable for TIMER6_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TIMER6 to value 0x1020_0000
impl crate::Resettable for TIMER6_SPEC {
    const RESET_VALUE: u32 = 0x1020_0000;
}
