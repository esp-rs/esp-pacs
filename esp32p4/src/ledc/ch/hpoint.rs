///Register `HPOINT` reader
pub type R = crate::R<HPOINT_SPEC>;
///Register `HPOINT` writer
pub type W = crate::W<HPOINT_SPEC>;
///Field `HPOINT` reader - Configures high point of signal output on channel %s. The output value changes to high when the selected timers has reached the value specified by this register.
pub type HPOINT_R = crate::FieldReader<u32>;
///Field `HPOINT` writer - Configures high point of signal output on channel %s. The output value changes to high when the selected timers has reached the value specified by this register.
pub type HPOINT_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    ///Bits 0:19 - Configures high point of signal output on channel %s. The output value changes to high when the selected timers has reached the value specified by this register.
    #[inline(always)]
    pub fn hpoint(&self) -> HPOINT_R {
        HPOINT_R::new(self.bits & 0x000f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HPOINT")
            .field("hpoint", &self.hpoint())
            .finish()
    }
}
impl W {
    ///Bits 0:19 - Configures high point of signal output on channel %s. The output value changes to high when the selected timers has reached the value specified by this register.
    #[inline(always)]
    #[must_use]
    pub fn hpoint(&mut self) -> HPOINT_W<HPOINT_SPEC> {
        HPOINT_W::new(self, 0)
    }
}
/**High point register for channel 0

You can [`read`](crate::generic::Reg::read) this register and get [`hpoint::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hpoint::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct HPOINT_SPEC;
impl crate::RegisterSpec for HPOINT_SPEC {
    type Ux = u32;
}
///`read()` method returns [`hpoint::R`](R) reader structure
impl crate::Readable for HPOINT_SPEC {}
///`write(|w| ..)` method takes [`hpoint::W`](W) writer structure
impl crate::Writable for HPOINT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets HPOINT to value 0
impl crate::Resettable for HPOINT_SPEC {
    const RESET_VALUE: u32 = 0;
}
