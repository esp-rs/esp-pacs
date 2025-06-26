#[doc = "Register `OUT_LINK` reader"]
pub type R = crate::R<OUT_LINK_SPEC>;
#[doc = "Register `OUT_LINK` writer"]
pub type W = crate::W<OUT_LINK_SPEC>;
#[doc = "Field `OUTLINK_ADDR` reader - This register is used to specify the least significant 20 bits of the first transmit descriptor’s address."]
pub type OUTLINK_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `OUTLINK_ADDR` writer - This register is used to specify the least significant 20 bits of the first transmit descriptor’s address."]
pub type OUTLINK_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `OUTLINK_STOP` reader - Set this bit to stop DMA from reading transmit descriptors after finishing the current data transaction."]
pub type OUTLINK_STOP_R = crate::BitReader;
#[doc = "Field `OUTLINK_STOP` writer - Set this bit to stop DMA from reading transmit descriptors after finishing the current data transaction."]
pub type OUTLINK_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTLINK_START` reader - Set this bit to start a new transmit descriptor."]
pub type OUTLINK_START_R = crate::BitReader;
#[doc = "Field `OUTLINK_START` writer - Set this bit to start a new transmit descriptor."]
pub type OUTLINK_START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTLINK_RESTART` reader - Set this bit to restart the transmit descriptor from the last address."]
pub type OUTLINK_RESTART_R = crate::BitReader;
#[doc = "Field `OUTLINK_RESTART` writer - Set this bit to restart the transmit descriptor from the last address."]
pub type OUTLINK_RESTART_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "1: the transmit descriptor’s FSM is in idle state. 0: the transmit descriptor’s FSM is working.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OUTLINK_PARK {
    #[doc = "0: The transmit descriptor’s FSM is working."]
    Working = 0,
    #[doc = "1: The transmit descriptor’s FSM is in idle state"]
    Idle = 1,
}
impl From<OUTLINK_PARK> for bool {
    #[inline(always)]
    fn from(variant: OUTLINK_PARK) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `OUTLINK_PARK` reader - 1: the transmit descriptor’s FSM is in idle state. 0: the transmit descriptor’s FSM is working."]
pub type OUTLINK_PARK_R = crate::BitReader<OUTLINK_PARK>;
impl OUTLINK_PARK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OUTLINK_PARK {
        match self.bits {
            false => OUTLINK_PARK::Working,
            true => OUTLINK_PARK::Idle,
        }
    }
    #[doc = "The transmit descriptor’s FSM is working."]
    #[inline(always)]
    pub fn is_working(&self) -> bool {
        *self == OUTLINK_PARK::Working
    }
    #[doc = "The transmit descriptor’s FSM is in idle state"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == OUTLINK_PARK::Idle
    }
}
#[doc = "Field `OUTLINK_PARK` writer - 1: the transmit descriptor’s FSM is in idle state. 0: the transmit descriptor’s FSM is working."]
pub type OUTLINK_PARK_W<'a, REG> = crate::BitWriter<'a, REG, OUTLINK_PARK>;
impl<'a, REG> OUTLINK_PARK_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The transmit descriptor’s FSM is working."]
    #[inline(always)]
    pub fn working(self) -> &'a mut crate::W<REG> {
        self.variant(OUTLINK_PARK::Working)
    }
    #[doc = "The transmit descriptor’s FSM is in idle state"]
    #[inline(always)]
    pub fn idle(self) -> &'a mut crate::W<REG> {
        self.variant(OUTLINK_PARK::Idle)
    }
}
impl R {
    #[doc = "Bits 0:19 - This register is used to specify the least significant 20 bits of the first transmit descriptor’s address."]
    #[inline(always)]
    pub fn outlink_addr(&self) -> OUTLINK_ADDR_R {
        OUTLINK_ADDR_R::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bit 28 - Set this bit to stop DMA from reading transmit descriptors after finishing the current data transaction."]
    #[inline(always)]
    pub fn outlink_stop(&self) -> OUTLINK_STOP_R {
        OUTLINK_STOP_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Set this bit to start a new transmit descriptor."]
    #[inline(always)]
    pub fn outlink_start(&self) -> OUTLINK_START_R {
        OUTLINK_START_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Set this bit to restart the transmit descriptor from the last address."]
    #[inline(always)]
    pub fn outlink_restart(&self) -> OUTLINK_RESTART_R {
        OUTLINK_RESTART_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 1: the transmit descriptor’s FSM is in idle state. 0: the transmit descriptor’s FSM is working."]
    #[inline(always)]
    pub fn outlink_park(&self) -> OUTLINK_PARK_R {
        OUTLINK_PARK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_LINK")
            .field("outlink_addr", &self.outlink_addr())
            .field("outlink_stop", &self.outlink_stop())
            .field("outlink_start", &self.outlink_start())
            .field("outlink_restart", &self.outlink_restart())
            .field("outlink_park", &self.outlink_park())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:19 - This register is used to specify the least significant 20 bits of the first transmit descriptor’s address."]
    #[inline(always)]
    pub fn outlink_addr(&mut self) -> OUTLINK_ADDR_W<OUT_LINK_SPEC> {
        OUTLINK_ADDR_W::new(self, 0)
    }
    #[doc = "Bit 28 - Set this bit to stop DMA from reading transmit descriptors after finishing the current data transaction."]
    #[inline(always)]
    pub fn outlink_stop(&mut self) -> OUTLINK_STOP_W<OUT_LINK_SPEC> {
        OUTLINK_STOP_W::new(self, 28)
    }
    #[doc = "Bit 29 - Set this bit to start a new transmit descriptor."]
    #[inline(always)]
    pub fn outlink_start(&mut self) -> OUTLINK_START_W<OUT_LINK_SPEC> {
        OUTLINK_START_W::new(self, 29)
    }
    #[doc = "Bit 30 - Set this bit to restart the transmit descriptor from the last address."]
    #[inline(always)]
    pub fn outlink_restart(&mut self) -> OUTLINK_RESTART_W<OUT_LINK_SPEC> {
        OUTLINK_RESTART_W::new(self, 30)
    }
    #[doc = "Bit 31 - 1: the transmit descriptor’s FSM is in idle state. 0: the transmit descriptor’s FSM is working."]
    #[inline(always)]
    pub fn outlink_park(&mut self) -> OUTLINK_PARK_W<OUT_LINK_SPEC> {
        OUTLINK_PARK_W::new(self, 31)
    }
}
#[doc = "Link descriptor address and control\n\nYou can [`read`](crate::Reg::read) this register and get [`out_link::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_link::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_LINK_SPEC;
impl crate::RegisterSpec for OUT_LINK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_link::R`](R) reader structure"]
impl crate::Readable for OUT_LINK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`out_link::W`](W) writer structure"]
impl crate::Writable for OUT_LINK_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OUT_LINK to value 0"]
impl crate::Resettable for OUT_LINK_SPEC {}
