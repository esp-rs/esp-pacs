#[doc = "Register `CONFIG` reader"]
pub type R = crate::R<CONFIG_SPEC>;
#[doc = "Register `CONFIG` writer"]
pub type W = crate::W<CONFIG_SPEC>;
#[doc = "Field `DM_TRIGGER_ENA` reader - Configure whether or not enable cpu trigger action.\\\\1: enable\\\\0:disable\\\\"]
pub type DM_TRIGGER_ENA_R = crate::BitReader;
#[doc = "Field `DM_TRIGGER_ENA` writer - Configure whether or not enable cpu trigger action.\\\\1: enable\\\\0:disable\\\\"]
pub type DM_TRIGGER_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESET_ENA` reader - Configure whether or not enable trace cpu haverest, when enabeld, if cpu have reset, the encoder will output a packet to report the address of the last instruction, and upon reset deassertion, the encoder start again.\\\\1: enabeld\\\\0: disabled\\\\"]
pub type RESET_ENA_R = crate::BitReader;
#[doc = "Field `RESET_ENA` writer - Configure whether or not enable trace cpu haverest, when enabeld, if cpu have reset, the encoder will output a packet to report the address of the last instruction, and upon reset deassertion, the encoder start again.\\\\1: enabeld\\\\0: disabled\\\\"]
pub type RESET_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HALT_ENA` reader - Configure whether or not enable trace cpu is halted, when enabeld, if the cpu halted, the encoder will output a packet to report the address of the last instruction, and upon halted deassertion, the encoder start again.When disabled, encoder will not report the last address before halted and first address after halted, cpu halted information will not be tracked. \\\\1: enabeld\\\\0: disabled\\\\"]
pub type HALT_ENA_R = crate::BitReader;
#[doc = "Field `HALT_ENA` writer - Configure whether or not enable trace cpu is halted, when enabeld, if the cpu halted, the encoder will output a packet to report the address of the last instruction, and upon halted deassertion, the encoder start again.When disabled, encoder will not report the last address before halted and first address after halted, cpu halted information will not be tracked. \\\\1: enabeld\\\\0: disabled\\\\"]
pub type HALT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALL_ENA` reader - Configure whether or not enable stall cpu. When enabled, when the fifo almost full, the cpu will be stalled until the packets is able to write to fifo.\\\\1: enabled.\\\\0: disabled\\\\"]
pub type STALL_ENA_R = crate::BitReader;
#[doc = "Field `STALL_ENA` writer - Configure whether or not enable stall cpu. When enabled, when the fifo almost full, the cpu will be stalled until the packets is able to write to fifo.\\\\1: enabled.\\\\0: disabled\\\\"]
pub type STALL_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FULL_ADDRESS` reader - Configure whether or not enable full-address mode.\\\\1: full address mode.\\\\0: delta address mode\\\\"]
pub type FULL_ADDRESS_R = crate::BitReader;
#[doc = "Field `FULL_ADDRESS` writer - Configure whether or not enable full-address mode.\\\\1: full address mode.\\\\0: delta address mode\\\\"]
pub type FULL_ADDRESS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMPLICIT_EXCEPT` reader - Configure whether or not enabel implicit exception mode. When enabled,, do not sent exception address, only exception cause in exception packets.\\\\1: enabled\\\\0: disabled\\\\"]
pub type IMPLICIT_EXCEPT_R = crate::BitReader;
#[doc = "Field `IMPLICIT_EXCEPT` writer - Configure whether or not enabel implicit exception mode. When enabled,, do not sent exception address, only exception cause in exception packets.\\\\1: enabled\\\\0: disabled\\\\"]
pub type IMPLICIT_EXCEPT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configure whether or not enable cpu trigger action.\\\\1: enable\\\\0:disable\\\\"]
    #[inline(always)]
    pub fn dm_trigger_ena(&self) -> DM_TRIGGER_ENA_R {
        DM_TRIGGER_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configure whether or not enable trace cpu haverest, when enabeld, if cpu have reset, the encoder will output a packet to report the address of the last instruction, and upon reset deassertion, the encoder start again.\\\\1: enabeld\\\\0: disabled\\\\"]
    #[inline(always)]
    pub fn reset_ena(&self) -> RESET_ENA_R {
        RESET_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Configure whether or not enable trace cpu is halted, when enabeld, if the cpu halted, the encoder will output a packet to report the address of the last instruction, and upon halted deassertion, the encoder start again.When disabled, encoder will not report the last address before halted and first address after halted, cpu halted information will not be tracked. \\\\1: enabeld\\\\0: disabled\\\\"]
    #[inline(always)]
    pub fn halt_ena(&self) -> HALT_ENA_R {
        HALT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Configure whether or not enable stall cpu. When enabled, when the fifo almost full, the cpu will be stalled until the packets is able to write to fifo.\\\\1: enabled.\\\\0: disabled\\\\"]
    #[inline(always)]
    pub fn stall_ena(&self) -> STALL_ENA_R {
        STALL_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Configure whether or not enable full-address mode.\\\\1: full address mode.\\\\0: delta address mode\\\\"]
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
    #[doc = "Bit 0 - Configure whether or not enable cpu trigger action.\\\\1: enable\\\\0:disable\\\\"]
    #[inline(always)]
    pub fn dm_trigger_ena(&mut self) -> DM_TRIGGER_ENA_W<CONFIG_SPEC> {
        DM_TRIGGER_ENA_W::new(self, 0)
    }
    #[doc = "Bit 1 - Configure whether or not enable trace cpu haverest, when enabeld, if cpu have reset, the encoder will output a packet to report the address of the last instruction, and upon reset deassertion, the encoder start again.\\\\1: enabeld\\\\0: disabled\\\\"]
    #[inline(always)]
    pub fn reset_ena(&mut self) -> RESET_ENA_W<CONFIG_SPEC> {
        RESET_ENA_W::new(self, 1)
    }
    #[doc = "Bit 2 - Configure whether or not enable trace cpu is halted, when enabeld, if the cpu halted, the encoder will output a packet to report the address of the last instruction, and upon halted deassertion, the encoder start again.When disabled, encoder will not report the last address before halted and first address after halted, cpu halted information will not be tracked. \\\\1: enabeld\\\\0: disabled\\\\"]
    #[inline(always)]
    pub fn halt_ena(&mut self) -> HALT_ENA_W<CONFIG_SPEC> {
        HALT_ENA_W::new(self, 2)
    }
    #[doc = "Bit 3 - Configure whether or not enable stall cpu. When enabled, when the fifo almost full, the cpu will be stalled until the packets is able to write to fifo.\\\\1: enabled.\\\\0: disabled\\\\"]
    #[inline(always)]
    pub fn stall_ena(&mut self) -> STALL_ENA_W<CONFIG_SPEC> {
        STALL_ENA_W::new(self, 3)
    }
    #[doc = "Bit 4 - Configure whether or not enable full-address mode.\\\\1: full address mode.\\\\0: delta address mode\\\\"]
    #[inline(always)]
    pub fn full_address(&mut self) -> FULL_ADDRESS_W<CONFIG_SPEC> {
        FULL_ADDRESS_W::new(self, 4)
    }
    #[doc = "Bit 5 - Configure whether or not enabel implicit exception mode. When enabled,, do not sent exception address, only exception cause in exception packets.\\\\1: enabled\\\\0: disabled\\\\"]
    #[inline(always)]
    pub fn implicit_except(&mut self) -> IMPLICIT_EXCEPT_W<CONFIG_SPEC> {
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONFIG to value 0"]
impl crate::Resettable for CONFIG_SPEC {
    const RESET_VALUE: u32 = 0;
}
