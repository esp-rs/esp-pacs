#[doc = "Register `CORE_1_CONTROL_0` reader"]
pub type R = crate::R<CORE_1_CONTROL_0_SPEC>;
#[doc = "Register `CORE_1_CONTROL_0` writer"]
pub type W = crate::W<CORE_1_CONTROL_0_SPEC>;
#[doc = "Field `CONTROL_CORE_1_RUNSTALL` reader - Set 1 to stall core1"]
pub type CONTROL_CORE_1_RUNSTALL_R = crate::BitReader;
#[doc = "Field `CONTROL_CORE_1_RUNSTALL` writer - Set 1 to stall core1"]
pub type CONTROL_CORE_1_RUNSTALL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CONTROL_CORE_1_CLKGATE_EN` reader - Set 1 to open core1 clock"]
pub type CONTROL_CORE_1_CLKGATE_EN_R = crate::BitReader;
#[doc = "Field `CONTROL_CORE_1_CLKGATE_EN` writer - Set 1 to open core1 clock"]
pub type CONTROL_CORE_1_CLKGATE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CONTROL_CORE_1_RESETING` reader - Set 1 to let core1 reset"]
pub type CONTROL_CORE_1_RESETING_R = crate::BitReader;
#[doc = "Field `CONTROL_CORE_1_RESETING` writer - Set 1 to let core1 reset"]
pub type CONTROL_CORE_1_RESETING_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set 1 to stall core1"]
    #[inline(always)]
    pub fn control_core_1_runstall(&self) -> CONTROL_CORE_1_RUNSTALL_R {
        CONTROL_CORE_1_RUNSTALL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set 1 to open core1 clock"]
    #[inline(always)]
    pub fn control_core_1_clkgate_en(&self) -> CONTROL_CORE_1_CLKGATE_EN_R {
        CONTROL_CORE_1_CLKGATE_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set 1 to let core1 reset"]
    #[inline(always)]
    pub fn control_core_1_reseting(&self) -> CONTROL_CORE_1_RESETING_R {
        CONTROL_CORE_1_RESETING_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_1_CONTROL_0")
            .field("control_core_1_runstall", &self.control_core_1_runstall())
            .field(
                "control_core_1_clkgate_en",
                &self.control_core_1_clkgate_en(),
            )
            .field("control_core_1_reseting", &self.control_core_1_reseting())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to stall core1"]
    #[inline(always)]
    #[must_use]
    pub fn control_core_1_runstall(&mut self) -> CONTROL_CORE_1_RUNSTALL_W<CORE_1_CONTROL_0_SPEC> {
        CONTROL_CORE_1_RUNSTALL_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set 1 to open core1 clock"]
    #[inline(always)]
    #[must_use]
    pub fn control_core_1_clkgate_en(
        &mut self,
    ) -> CONTROL_CORE_1_CLKGATE_EN_W<CORE_1_CONTROL_0_SPEC> {
        CONTROL_CORE_1_CLKGATE_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set 1 to let core1 reset"]
    #[inline(always)]
    #[must_use]
    pub fn control_core_1_reseting(&mut self) -> CONTROL_CORE_1_RESETING_W<CORE_1_CONTROL_0_SPEC> {
        CONTROL_CORE_1_RESETING_W::new(self, 2)
    }
}
#[doc = "Core0 control regiter 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_control_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_1_control_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_1_CONTROL_0_SPEC;
impl crate::RegisterSpec for CORE_1_CONTROL_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_1_control_0::R`](R) reader structure"]
impl crate::Readable for CORE_1_CONTROL_0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`core_1_control_0::W`](W) writer structure"]
impl crate::Writable for CORE_1_CONTROL_0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CORE_1_CONTROL_0 to value 0x04"]
impl crate::Resettable for CORE_1_CONTROL_0_SPEC {
    const RESET_VALUE: u32 = 0x04;
}
