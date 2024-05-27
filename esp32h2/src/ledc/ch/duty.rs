///Register `DUTY` reader
pub type R = crate::R<DUTY_SPEC>;
///Register `DUTY` writer
pub type W = crate::W<DUTY_SPEC>;
///Field `DUTY` reader - This register is used to change the output duty by controlling the Lpoint. The output value turns to low when the selected timers has reached the Lpoint.
pub type DUTY_R = crate::FieldReader<u32>;
///Field `DUTY` writer - This register is used to change the output duty by controlling the Lpoint. The output value turns to low when the selected timers has reached the Lpoint.
pub type DUTY_W<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
impl R {
    ///Bits 0:24 - This register is used to change the output duty by controlling the Lpoint. The output value turns to low when the selected timers has reached the Lpoint.
    #[inline(always)]
    pub fn duty(&self) -> DUTY_R {
        DUTY_R::new(self.bits & 0x01ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DUTY").field("duty", &self.duty()).finish()
    }
}
impl W {
    ///Bits 0:24 - This register is used to change the output duty by controlling the Lpoint. The output value turns to low when the selected timers has reached the Lpoint.
    #[inline(always)]
    #[must_use]
    pub fn duty(&mut self) -> DUTY_W<DUTY_SPEC> {
        DUTY_W::new(self, 0)
    }
}
/**Initial duty cycle for channel 0

You can [`read`](crate::generic::Reg::read) this register and get [`duty::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`duty::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DUTY_SPEC;
impl crate::RegisterSpec for DUTY_SPEC {
    type Ux = u32;
}
///`read()` method returns [`duty::R`](R) reader structure
impl crate::Readable for DUTY_SPEC {}
///`write(|w| ..)` method takes [`duty::W`](W) writer structure
impl crate::Writable for DUTY_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DUTY to value 0
impl crate::Resettable for DUTY_SPEC {
    const RESET_VALUE: u32 = 0;
}
