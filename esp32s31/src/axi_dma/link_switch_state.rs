#[doc = "Register `LINK_SWITCH_STATE` reader"]
pub type R = crate::R<LINK_SWITCH_STATE_SPEC>;
#[doc = "Register `LINK_SWITCH_STATE` writer"]
pub type W = crate::W<LINK_SWITCH_STATE_SPEC>;
#[doc = "Field `CH0` reader - The register that confirm ch dscr switch success"]
pub type CH0_R = crate::BitReader;
#[doc = "Field `CH0` writer - The register that confirm ch dscr switch success"]
pub type CH0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1` reader - The register that confirm ch dscr switch success"]
pub type CH1_R = crate::BitReader;
#[doc = "Field `CH1` writer - The register that confirm ch dscr switch success"]
pub type CH1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2` reader - The register that confirm ch dscr switch success"]
pub type CH2_R = crate::BitReader;
#[doc = "Field `CH2` writer - The register that confirm ch dscr switch success"]
pub type CH2_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The register that confirm ch dscr switch success"]
    #[inline(always)]
    pub fn ch0(&self) -> CH0_R {
        CH0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The register that confirm ch dscr switch success"]
    #[inline(always)]
    pub fn ch1(&self) -> CH1_R {
        CH1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The register that confirm ch dscr switch success"]
    #[inline(always)]
    pub fn ch2(&self) -> CH2_R {
        CH2_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LINK_SWITCH_STATE")
            .field("ch0", &self.ch0())
            .field("ch1", &self.ch1())
            .field("ch2", &self.ch2())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - The register that confirm ch dscr switch success"]
    #[inline(always)]
    pub fn ch0(&mut self) -> CH0_W<'_, LINK_SWITCH_STATE_SPEC> {
        CH0_W::new(self, 0)
    }
    #[doc = "Bit 1 - The register that confirm ch dscr switch success"]
    #[inline(always)]
    pub fn ch1(&mut self) -> CH1_W<'_, LINK_SWITCH_STATE_SPEC> {
        CH1_W::new(self, 1)
    }
    #[doc = "Bit 2 - The register that confirm ch dscr switch success"]
    #[inline(always)]
    pub fn ch2(&mut self) -> CH2_W<'_, LINK_SWITCH_STATE_SPEC> {
        CH2_W::new(self, 2)
    }
}
#[doc = "Version control register\n\nYou can [`read`](crate::Reg::read) this register and get [`link_switch_state::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`link_switch_state::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
