#[doc = "Register `CONFIG` reader"]
pub type R = crate::R<CONFIG_SPEC>;
#[doc = "Register `CONFIG` writer"]
pub type W = crate::W<CONFIG_SPEC>;
#[doc = "Field `DM_TRIGGER_ENA` reader - Configure whether to enable the trigger signal.\\\\0: Disable\\\\1:enable\\\\"]
pub type DM_TRIGGER_ENA_R = crate::BitReader;
#[doc = "Field `DM_TRIGGER_ENA` writer - Configure whether to enable the trigger signal.\\\\0: Disable\\\\1:enable\\\\"]
pub type DM_TRIGGER_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESET_ENA` reader - Configure whether to reset, when enabeld, if cpu have reset, the encoder will output a packet to report the address of the last instruction, and upon reset deassertion, the encoder start again.\\\\0: Disable\\\\0: Enable\\\\"]
pub type RESET_ENA_R = crate::BitReader;
#[doc = "Field `RESET_ENA` writer - Configure whether to reset, when enabeld, if cpu have reset, the encoder will output a packet to report the address of the last instruction, and upon reset deassertion, the encoder start again.\\\\0: Disable\\\\0: Enable\\\\"]
pub type RESET_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HALT_ENA` reader - Configure whether to enable the halt signal. \\\\1: Disable\\\\1: Enable\\\\"]
pub type HALT_ENA_R = crate::BitReader;
#[doc = "Field `HALT_ENA` writer - Configure whether to enable the halt signal. \\\\1: Disable\\\\1: Enable\\\\"]
pub type HALT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALL_ENA` reader - Configure whether to enable the stall signal. \\\\0: Disable.\\\\1: Enable\\\\"]
pub type STALL_ENA_R = crate::BitReader;
#[doc = "Field `STALL_ENA` writer - Configure whether to enable the stall signal. \\\\0: Disable.\\\\1: Enable\\\\"]
pub type STALL_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FULL_ADDRESS` reader - Configure the address mode.\\\\0: Delta address mode.\\\\1: Full address mode.\\\\"]
pub type FULL_ADDRESS_R = crate::BitReader;
#[doc = "Field `FULL_ADDRESS` writer - Configure the address mode.\\\\0: Delta address mode.\\\\1: Full address mode.\\\\"]
pub type FULL_ADDRESS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMPLICIT_EXCEPT` reader - Configure whether or not enabel implicit exception mode. When enabled,, do not sent exception address, only exception cause in exception packets.\\\\1: enabled\\\\0: disabled\\\\"]
pub type IMPLICIT_EXCEPT_R = crate::BitReader;
#[doc = "Field `IMPLICIT_EXCEPT` writer - Configure whether or not enabel implicit exception mode. When enabled,, do not sent exception address, only exception cause in exception packets.\\\\1: enabled\\\\0: disabled\\\\"]
pub type IMPLICIT_EXCEPT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configure whether to enable the trigger signal.\\\\0: Disable\\\\1:enable\\\\"]
    #[inline(always)]
    pub fn dm_trigger_ena(&self) -> DM_TRIGGER_ENA_R {
        DM_TRIGGER_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configure whether to reset, when enabeld, if cpu have reset, the encoder will output a packet to report the address of the last instruction, and upon reset deassertion, the encoder start again.\\\\0: Disable\\\\0: Enable\\\\"]
    #[inline(always)]
    pub fn reset_ena(&self) -> RESET_ENA_R {
        RESET_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Configure whether to enable the halt signal. \\\\1: Disable\\\\1: Enable\\\\"]
    #[inline(always)]
    pub fn halt_ena(&self) -> HALT_ENA_R {
        HALT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Configure whether to enable the stall signal. \\\\0: Disable.\\\\1: Enable\\\\"]
    #[inline(always)]
    pub fn stall_ena(&self) -> STALL_ENA_R {
        STALL_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Configure the address mode.\\\\0: Delta address mode.\\\\1: Full address mode.\\\\"]
    #[inline(always)]
    pub fn full_address(&self) -> FULL_ADDRESS_R {
        FULL_ADDRESS_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Configure whether or not enabel implicit exception mode. When enabled,, do not sent exception address, only exception cause in exception packets.\\\\1: enabled\\\\0: disabled\\\\"]
    #[inline(always)]
    pub fn implicit_except(&self) -> IMPLICIT_EXCEPT_R {
        IMPLICIT_EXCEPT_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONFIG")
            .field("dm_trigger_ena", &self.dm_trigger_ena())
            .field("reset_ena", &self.reset_ena())
            .field("halt_ena", &self.halt_ena())
            .field("stall_ena", &self.stall_ena())
            .field("full_address", &self.full_address())
            .field("implicit_except", &self.implicit_except())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Configure whether to enable the trigger signal.\\\\0: Disable\\\\1:enable\\\\"]
    #[inline(always)]
    pub fn dm_trigger_ena(&mut self) -> DM_TRIGGER_ENA_W<'_, CONFIG_SPEC> {
        DM_TRIGGER_ENA_W::new(self, 0)
    }
    #[doc = "Bit 1 - Configure whether to reset, when enabeld, if cpu have reset, the encoder will output a packet to report the address of the last instruction, and upon reset deassertion, the encoder start again.\\\\0: Disable\\\\0: Enable\\\\"]
    #[inline(always)]
    pub fn reset_ena(&mut self) -> RESET_ENA_W<'_, CONFIG_SPEC> {
        RESET_ENA_W::new(self, 1)
    }
    #[doc = "Bit 2 - Configure whether to enable the halt signal. \\\\1: Disable\\\\1: Enable\\\\"]
    #[inline(always)]
    pub fn halt_ena(&mut self) -> HALT_ENA_W<'_, CONFIG_SPEC> {
        HALT_ENA_W::new(self, 2)
    }
    #[doc = "Bit 3 - Configure whether to enable the stall signal. \\\\0: Disable.\\\\1: Enable\\\\"]
    #[inline(always)]
    pub fn stall_ena(&mut self) -> STALL_ENA_W<'_, CONFIG_SPEC> {
        STALL_ENA_W::new(self, 3)
    }
    #[doc = "Bit 4 - Configure the address mode.\\\\0: Delta address mode.\\\\1: Full address mode.\\\\"]
    #[inline(always)]
    pub fn full_address(&mut self) -> FULL_ADDRESS_W<'_, CONFIG_SPEC> {
        FULL_ADDRESS_W::new(self, 4)
    }
    #[doc = "Bit 5 - Configure whether or not enabel implicit exception mode. When enabled,, do not sent exception address, only exception cause in exception packets.\\\\1: enabled\\\\0: disabled\\\\"]
    #[inline(always)]
    pub fn implicit_except(&mut self) -> IMPLICIT_EXCEPT_W<'_, CONFIG_SPEC> {
        IMPLICIT_EXCEPT_W::new(self, 5)
    }
}
#[doc = "trace configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONFIG_SPEC;
impl crate::RegisterSpec for CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config::R`](R) reader structure"]
impl crate::Readable for CONFIG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`config::W`](W) writer structure"]
impl crate::Writable for CONFIG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONFIG to value 0"]
impl crate::Resettable for CONFIG_SPEC {}
