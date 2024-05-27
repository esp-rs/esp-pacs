///Register `FSM_WAIT` reader
pub type R = crate::R<FSM_WAIT_SPEC>;
///Register `FSM_WAIT` writer
pub type W = crate::W<FSM_WAIT_SPEC>;
///Field `XPD_WAIT` reader - Need add description
pub type XPD_WAIT_R = crate::FieldReader;
///Field `XPD_WAIT` writer - Need add description
pub type XPD_WAIT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `RSTB_WAIT` reader - Need add description
pub type RSTB_WAIT_R = crate::FieldReader;
///Field `RSTB_WAIT` writer - Need add description
pub type RSTB_WAIT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `STANDBY_WAIT` reader - Need add description
pub type STANDBY_WAIT_R = crate::FieldReader;
///Field `STANDBY_WAIT` writer - Need add description
pub type STANDBY_WAIT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    ///Bits 0:7 - Need add description
    #[inline(always)]
    pub fn xpd_wait(&self) -> XPD_WAIT_R {
        XPD_WAIT_R::new((self.bits & 0xff) as u8)
    }
    ///Bits 8:15 - Need add description
    #[inline(always)]
    pub fn rstb_wait(&self) -> RSTB_WAIT_R {
        RSTB_WAIT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    ///Bits 16:23 - Need add description
    #[inline(always)]
    pub fn standby_wait(&self) -> STANDBY_WAIT_R {
        STANDBY_WAIT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FSM_WAIT")
            .field("xpd_wait", &self.xpd_wait())
            .field("rstb_wait", &self.rstb_wait())
            .field("standby_wait", &self.standby_wait())
            .finish()
    }
}
impl W {
    ///Bits 0:7 - Need add description
    #[inline(always)]
    #[must_use]
    pub fn xpd_wait(&mut self) -> XPD_WAIT_W<FSM_WAIT_SPEC> {
        XPD_WAIT_W::new(self, 0)
    }
    ///Bits 8:15 - Need add description
    #[inline(always)]
    #[must_use]
    pub fn rstb_wait(&mut self) -> RSTB_WAIT_W<FSM_WAIT_SPEC> {
        RSTB_WAIT_W::new(self, 8)
    }
    ///Bits 16:23 - Need add description
    #[inline(always)]
    #[must_use]
    pub fn standby_wait(&mut self) -> STANDBY_WAIT_W<FSM_WAIT_SPEC> {
        STANDBY_WAIT_W::new(self, 16)
    }
}
/**register description

You can [`read`](crate::generic::Reg::read) this register and get [`fsm_wait::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsm_wait::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FSM_WAIT_SPEC;
impl crate::RegisterSpec for FSM_WAIT_SPEC {
    type Ux = u32;
}
///`read()` method returns [`fsm_wait::R`](R) reader structure
impl crate::Readable for FSM_WAIT_SPEC {}
///`write(|w| ..)` method takes [`fsm_wait::W`](W) writer structure
impl crate::Writable for FSM_WAIT_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets FSM_WAIT to value 0x00ff_0808
impl crate::Resettable for FSM_WAIT_SPEC {
    const RESET_VALUE: u32 = 0x00ff_0808;
}
