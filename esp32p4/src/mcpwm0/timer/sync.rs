///Register `SYNC` reader
pub type R = crate::R<SYNC_SPEC>;
///Register `SYNC` writer
pub type W = crate::W<SYNC_SPEC>;
///Field `SYNCI_EN` reader - Configures whether or not to enable timer%s reloading with phase on sync input event is enabled.\\0: Disable\\1: Enable
pub type SYNCI_EN_R = crate::BitReader;
///Field `SYNCI_EN` writer - Configures whether or not to enable timer%s reloading with phase on sync input event is enabled.\\0: Disable\\1: Enable
pub type SYNCI_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SW` reader - Configures the generation of software sync. Toggling this bit will trigger a software sync.
pub type SW_R = crate::BitReader;
///Field `SW` writer - Configures the generation of software sync. Toggling this bit will trigger a software sync.
pub type SW_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SYNCO_SEL` reader - Configures the selection of PWM timer%s sync_out.\\0: Sync_in\\1: TEZ\\2: TEP\\3: Invalid, sync_out selects noting
pub type SYNCO_SEL_R = crate::FieldReader;
///Field `SYNCO_SEL` writer - Configures the selection of PWM timer%s sync_out.\\0: Sync_in\\1: TEZ\\2: TEP\\3: Invalid, sync_out selects noting
pub type SYNCO_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
///Field `PHASE` reader - Configures the phase for timer%s reload on sync event.
pub type PHASE_R = crate::FieldReader<u16>;
///Field `PHASE` writer - Configures the phase for timer%s reload on sync event.
pub type PHASE_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
///Field `PHASE_DIRECTION` reader - Configures the PWM timer%s's direction when timer%s mode is up-down mode.\\0: Increase\\1: Decrease
pub type PHASE_DIRECTION_R = crate::BitReader;
///Field `PHASE_DIRECTION` writer - Configures the PWM timer%s's direction when timer%s mode is up-down mode.\\0: Increase\\1: Decrease
pub type PHASE_DIRECTION_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - Configures whether or not to enable timer%s reloading with phase on sync input event is enabled.\\0: Disable\\1: Enable
    #[inline(always)]
    pub fn synci_en(&self) -> SYNCI_EN_R {
        SYNCI_EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Configures the generation of software sync. Toggling this bit will trigger a software sync.
    #[inline(always)]
    pub fn sw(&self) -> SW_R {
        SW_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:3 - Configures the selection of PWM timer%s sync_out.\\0: Sync_in\\1: TEZ\\2: TEP\\3: Invalid, sync_out selects noting
    #[inline(always)]
    pub fn synco_sel(&self) -> SYNCO_SEL_R {
        SYNCO_SEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bits 4:19 - Configures the phase for timer%s reload on sync event.
    #[inline(always)]
    pub fn phase(&self) -> PHASE_R {
        PHASE_R::new(((self.bits >> 4) & 0xffff) as u16)
    }
    ///Bit 20 - Configures the PWM timer%s's direction when timer%s mode is up-down mode.\\0: Increase\\1: Decrease
    #[inline(always)]
    pub fn phase_direction(&self) -> PHASE_DIRECTION_R {
        PHASE_DIRECTION_R::new(((self.bits >> 20) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYNC")
            .field("synci_en", &self.synci_en())
            .field("sw", &self.sw())
            .field("synco_sel", &self.synco_sel())
            .field("phase", &self.phase())
            .field("phase_direction", &self.phase_direction())
            .finish()
    }
}
impl W {
    ///Bit 0 - Configures whether or not to enable timer%s reloading with phase on sync input event is enabled.\\0: Disable\\1: Enable
    #[inline(always)]
    #[must_use]
    pub fn synci_en(&mut self) -> SYNCI_EN_W<SYNC_SPEC> {
        SYNCI_EN_W::new(self, 0)
    }
    ///Bit 1 - Configures the generation of software sync. Toggling this bit will trigger a software sync.
    #[inline(always)]
    #[must_use]
    pub fn sw(&mut self) -> SW_W<SYNC_SPEC> {
        SW_W::new(self, 1)
    }
    ///Bits 2:3 - Configures the selection of PWM timer%s sync_out.\\0: Sync_in\\1: TEZ\\2: TEP\\3: Invalid, sync_out selects noting
    #[inline(always)]
    #[must_use]
    pub fn synco_sel(&mut self) -> SYNCO_SEL_W<SYNC_SPEC> {
        SYNCO_SEL_W::new(self, 2)
    }
    ///Bits 4:19 - Configures the phase for timer%s reload on sync event.
    #[inline(always)]
    #[must_use]
    pub fn phase(&mut self) -> PHASE_W<SYNC_SPEC> {
        PHASE_W::new(self, 4)
    }
    ///Bit 20 - Configures the PWM timer%s's direction when timer%s mode is up-down mode.\\0: Increase\\1: Decrease
    #[inline(always)]
    #[must_use]
    pub fn phase_direction(&mut self) -> PHASE_DIRECTION_W<SYNC_SPEC> {
        PHASE_DIRECTION_W::new(self, 20)
    }
}
/**PWM timer0 sync function configuration register.

You can [`read`](crate::generic::Reg::read) this register and get [`sync::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sync::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SYNC_SPEC;
impl crate::RegisterSpec for SYNC_SPEC {
    type Ux = u32;
}
///`read()` method returns [`sync::R`](R) reader structure
impl crate::Readable for SYNC_SPEC {}
///`write(|w| ..)` method takes [`sync::W`](W) writer structure
impl crate::Writable for SYNC_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SYNC to value 0
impl crate::Resettable for SYNC_SPEC {
    const RESET_VALUE: u32 = 0;
}
