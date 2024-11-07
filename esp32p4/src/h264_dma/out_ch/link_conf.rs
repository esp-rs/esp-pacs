#[doc = "Register `LINK_CONF` reader"]
pub type R = crate::R<LINK_CONF_SPEC>;
#[doc = "Register `LINK_CONF` writer"]
pub type W = crate::W<LINK_CONF_SPEC>;
#[doc = "Field `OUTLINK_STOP` reader - Set this bit to stop dealing with the outlink descriptors."]
pub type OUTLINK_STOP_R = crate::BitReader;
#[doc = "Field `OUTLINK_STOP` writer - Set this bit to stop dealing with the outlink descriptors."]
pub type OUTLINK_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTLINK_START` reader - Set this bit to start dealing with the outlink descriptors."]
pub type OUTLINK_START_R = crate::BitReader;
#[doc = "Field `OUTLINK_START` writer - Set this bit to start dealing with the outlink descriptors."]
pub type OUTLINK_START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTLINK_RESTART` reader - Set this bit to restart a new outlink from the last address."]
pub type OUTLINK_RESTART_R = crate::BitReader;
#[doc = "Field `OUTLINK_RESTART` writer - Set this bit to restart a new outlink from the last address."]
pub type OUTLINK_RESTART_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTLINK_PARK` reader - 1: the outlink descriptor's FSM is in idle state. 0: the outlink descriptor's FSM is working."]
pub type OUTLINK_PARK_R = crate::BitReader;
impl R {
    #[doc = "Bit 20 - Set this bit to stop dealing with the outlink descriptors."]
    #[inline(always)]
    pub fn outlink_stop(&self) -> OUTLINK_STOP_R {
        OUTLINK_STOP_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Set this bit to start dealing with the outlink descriptors."]
    #[inline(always)]
    pub fn outlink_start(&self) -> OUTLINK_START_R {
        OUTLINK_START_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Set this bit to restart a new outlink from the last address."]
    #[inline(always)]
    pub fn outlink_restart(&self) -> OUTLINK_RESTART_R {
        OUTLINK_RESTART_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 1: the outlink descriptor's FSM is in idle state. 0: the outlink descriptor's FSM is working."]
    #[inline(always)]
    pub fn outlink_park(&self) -> OUTLINK_PARK_R {
        OUTLINK_PARK_R::new(((self.bits >> 23) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LINK_CONF")
            .field("outlink_stop", &self.outlink_stop())
            .field("outlink_start", &self.outlink_start())
            .field("outlink_restart", &self.outlink_restart())
            .field("outlink_park", &self.outlink_park())
            .finish()
    }
}
impl W {
    #[doc = "Bit 20 - Set this bit to stop dealing with the outlink descriptors."]
    #[inline(always)]
    pub fn outlink_stop(&mut self) -> OUTLINK_STOP_W<LINK_CONF_SPEC> {
        OUTLINK_STOP_W::new(self, 20)
    }
    #[doc = "Bit 21 - Set this bit to start dealing with the outlink descriptors."]
    #[inline(always)]
    pub fn outlink_start(&mut self) -> OUTLINK_START_W<LINK_CONF_SPEC> {
        OUTLINK_START_W::new(self, 21)
    }
    #[doc = "Bit 22 - Set this bit to restart a new outlink from the last address."]
    #[inline(always)]
    pub fn outlink_restart(&mut self) -> OUTLINK_RESTART_W<LINK_CONF_SPEC> {
        OUTLINK_RESTART_W::new(self, 22)
    }
}
#[doc = "TX CHx out_link dscr ctrl register\n\nYou can [`read`](crate::Reg::read) this register and get [`link_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`link_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LINK_CONF_SPEC;
impl crate::RegisterSpec for LINK_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`link_conf::R`](R) reader structure"]
impl crate::Readable for LINK_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`link_conf::W`](W) writer structure"]
impl crate::Writable for LINK_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LINK_CONF to value 0x0080_0000"]
impl crate::Resettable for LINK_CONF_SPEC {
    const RESET_VALUE: u32 = 0x0080_0000;
}
