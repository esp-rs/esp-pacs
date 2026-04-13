#[doc = "Register `LINK_SWITCH_STATE` reader"]
pub type R = crate::R<LINK_SWITCH_STATE_SPEC>;
#[doc = "Register `LINK_SWITCH_STATE` writer"]
pub type W = crate::W<LINK_SWITCH_STATE_SPEC>;
#[doc = "Field `LINK_SWITCH_STATE_CH0` reader - "]
pub type LINK_SWITCH_STATE_CH0_R = crate::BitReader;
#[doc = "Field `LINK_SWITCH_STATE_CH0` writer - "]
pub type LINK_SWITCH_STATE_CH0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LINK_SWITCH_STATE_CH1` reader - "]
pub type LINK_SWITCH_STATE_CH1_R = crate::BitReader;
#[doc = "Field `LINK_SWITCH_STATE_CH1` writer - "]
pub type LINK_SWITCH_STATE_CH1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LINK_SWITCH_STATE_CH2` reader - "]
pub type LINK_SWITCH_STATE_CH2_R = crate::BitReader;
#[doc = "Field `LINK_SWITCH_STATE_CH2` writer - "]
pub type LINK_SWITCH_STATE_CH2_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn link_switch_state_ch0(&self) -> LINK_SWITCH_STATE_CH0_R {
        LINK_SWITCH_STATE_CH0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn link_switch_state_ch1(&self) -> LINK_SWITCH_STATE_CH1_R {
        LINK_SWITCH_STATE_CH1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn link_switch_state_ch2(&self) -> LINK_SWITCH_STATE_CH2_R {
        LINK_SWITCH_STATE_CH2_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LINK_SWITCH_STATE")
            .field("link_switch_state_ch0", &self.link_switch_state_ch0())
            .field("link_switch_state_ch1", &self.link_switch_state_ch1())
            .field("link_switch_state_ch2", &self.link_switch_state_ch2())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn link_switch_state_ch0(&mut self) -> LINK_SWITCH_STATE_CH0_W<'_, LINK_SWITCH_STATE_SPEC> {
        LINK_SWITCH_STATE_CH0_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn link_switch_state_ch1(&mut self) -> LINK_SWITCH_STATE_CH1_W<'_, LINK_SWITCH_STATE_SPEC> {
        LINK_SWITCH_STATE_CH1_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn link_switch_state_ch2(&mut self) -> LINK_SWITCH_STATE_CH2_W<'_, LINK_SWITCH_STATE_SPEC> {
        LINK_SWITCH_STATE_CH2_W::new(self, 2)
    }
}
#[doc = "Link descriptor switch state\n\nYou can [`read`](crate::Reg::read) this register and get [`link_switch_state::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`link_switch_state::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LINK_SWITCH_STATE_SPEC;
impl crate::RegisterSpec for LINK_SWITCH_STATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`link_switch_state::R`](R) reader structure"]
impl crate::Readable for LINK_SWITCH_STATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`link_switch_state::W`](W) writer structure"]
impl crate::Writable for LINK_SWITCH_STATE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LINK_SWITCH_STATE to value 0"]
impl crate::Resettable for LINK_SWITCH_STATE_SPEC {}
