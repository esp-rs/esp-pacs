#[doc = "Register `OUT_LINK_CH%s` reader"]
pub type R = crate::R<OUT_LINK_CH_SPEC>;
#[doc = "Register `OUT_LINK_CH%s` writer"]
pub type W = crate::W<OUT_LINK_CH_SPEC>;
#[doc = "Field `OUTLINK_ADDR_CH` reader - This register stores the 20 least significant bits of the first outlink descriptor's address."]
pub type OUTLINK_ADDR_CH_R = crate::FieldReader<u32>;
#[doc = "Field `OUTLINK_ADDR_CH` writer - This register stores the 20 least significant bits of the first outlink descriptor's address."]
pub type OUTLINK_ADDR_CH_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `OUTLINK_STOP_CH` writer - Set this bit to stop dealing with the outlink descriptors."]
pub type OUTLINK_STOP_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTLINK_START_CH` writer - Set this bit to start dealing with the outlink descriptors."]
pub type OUTLINK_START_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTLINK_RESTART_CH` writer - Set this bit to restart a new outlink from the last address."]
pub type OUTLINK_RESTART_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTLINK_PARK_CH` reader - 1: the outlink descriptor's FSM is in idle state. 0: the outlink descriptor's FSM is working."]
pub type OUTLINK_PARK_CH_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:19 - This register stores the 20 least significant bits of the first outlink descriptor's address."]
    #[inline(always)]
    pub fn outlink_addr_ch(&self) -> OUTLINK_ADDR_CH_R {
        OUTLINK_ADDR_CH_R::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bit 23 - 1: the outlink descriptor's FSM is in idle state. 0: the outlink descriptor's FSM is working."]
    #[inline(always)]
    pub fn outlink_park_ch(&self) -> OUTLINK_PARK_CH_R {
        OUTLINK_PARK_CH_R::new(((self.bits >> 23) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_LINK_CH")
            .field("outlink_addr_ch", &self.outlink_addr_ch())
            .field("outlink_park_ch", &self.outlink_park_ch())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:19 - This register stores the 20 least significant bits of the first outlink descriptor's address."]
    #[inline(always)]
    pub fn outlink_addr_ch(&mut self) -> OUTLINK_ADDR_CH_W<'_, OUT_LINK_CH_SPEC> {
        OUTLINK_ADDR_CH_W::new(self, 0)
    }
    #[doc = "Bit 20 - Set this bit to stop dealing with the outlink descriptors."]
    #[inline(always)]
    pub fn outlink_stop_ch(&mut self) -> OUTLINK_STOP_CH_W<'_, OUT_LINK_CH_SPEC> {
        OUTLINK_STOP_CH_W::new(self, 20)
    }
    #[doc = "Bit 21 - Set this bit to start dealing with the outlink descriptors."]
    #[inline(always)]
    pub fn outlink_start_ch(&mut self) -> OUTLINK_START_CH_W<'_, OUT_LINK_CH_SPEC> {
        OUTLINK_START_CH_W::new(self, 21)
    }
    #[doc = "Bit 22 - Set this bit to restart a new outlink from the last address."]
    #[inline(always)]
    pub fn outlink_restart_ch(&mut self) -> OUTLINK_RESTART_CH_W<'_, OUT_LINK_CH_SPEC> {
        OUTLINK_RESTART_CH_W::new(self, 22)
    }
}
#[doc = "Link descriptor configure and control register of Tx channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`out_link_ch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_link_ch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_LINK_CH_SPEC;
impl crate::RegisterSpec for OUT_LINK_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_link_ch::R`](R) reader structure"]
impl crate::Readable for OUT_LINK_CH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`out_link_ch::W`](W) writer structure"]
impl crate::Writable for OUT_LINK_CH_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OUT_LINK_CH%s to value 0x0080_0000"]
impl crate::Resettable for OUT_LINK_CH_SPEC {
    const RESET_VALUE: u32 = 0x0080_0000;
}
