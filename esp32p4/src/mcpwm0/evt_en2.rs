#[doc = "Register `EVT_EN2` reader"]
pub type R = crate::R<EVT_EN2_SPEC>;
#[doc = "Register `EVT_EN2` writer"]
pub type W = crate::W<EVT_EN2_SPEC>;
#[doc = "Field `EVT_OP0_TEE1_EN` reader - Configures whether or not to enable PWM generator0 timer equal OP0_TSTMP_E1_REG event generate.\\\\0: Disable\\\\1: Enable"]
pub type EVT_OP0_TEE1_EN_R = crate::BitReader;
#[doc = "Field `EVT_OP0_TEE1_EN` writer - Configures whether or not to enable PWM generator0 timer equal OP0_TSTMP_E1_REG event generate.\\\\0: Disable\\\\1: Enable"]
pub type EVT_OP0_TEE1_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVT_OP1_TEE1_EN` reader - Configures whether or not to enable PWM generator1 timer equal OP1_TSTMP_E1_REG event generate.\\\\0: Disable\\\\1: Enable"]
pub type EVT_OP1_TEE1_EN_R = crate::BitReader;
#[doc = "Field `EVT_OP1_TEE1_EN` writer - Configures whether or not to enable PWM generator1 timer equal OP1_TSTMP_E1_REG event generate.\\\\0: Disable\\\\1: Enable"]
pub type EVT_OP1_TEE1_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVT_OP2_TEE1_EN` reader - Configures whether or not to enable PWM generator2 timer equal OP2_TSTMP_E1_REG event generate.\\\\0: Disable\\\\1: Enable"]
pub type EVT_OP2_TEE1_EN_R = crate::BitReader;
#[doc = "Field `EVT_OP2_TEE1_EN` writer - Configures whether or not to enable PWM generator2 timer equal OP2_TSTMP_E1_REG event generate.\\\\0: Disable\\\\1: Enable"]
pub type EVT_OP2_TEE1_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVT_OP0_TEE2_EN` reader - Configures whether or not to enable PWM generator0 timer equal OP0_TSTMP_E2_REG event generate.\\\\0: Disable\\\\1: Enable"]
pub type EVT_OP0_TEE2_EN_R = crate::BitReader;
#[doc = "Field `EVT_OP0_TEE2_EN` writer - Configures whether or not to enable PWM generator0 timer equal OP0_TSTMP_E2_REG event generate.\\\\0: Disable\\\\1: Enable"]
pub type EVT_OP0_TEE2_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVT_OP1_TEE2_EN` reader - Configures whether or not to enable PWM generator1 timer equal OP1_TSTMP_E2_REG event generate.\\\\0: Disable\\\\1: Enable"]
pub type EVT_OP1_TEE2_EN_R = crate::BitReader;
#[doc = "Field `EVT_OP1_TEE2_EN` writer - Configures whether or not to enable PWM generator1 timer equal OP1_TSTMP_E2_REG event generate.\\\\0: Disable\\\\1: Enable"]
pub type EVT_OP1_TEE2_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EVT_OP2_TEE2_EN` reader - Configures whether or not to enable PWM generator2 timer equal OP2_TSTMP_E2_REG event generate.\\\\0: Disable\\\\1: Enable"]
pub type EVT_OP2_TEE2_EN_R = crate::BitReader;
#[doc = "Field `EVT_OP2_TEE2_EN` writer - Configures whether or not to enable PWM generator2 timer equal OP2_TSTMP_E2_REG event generate.\\\\0: Disable\\\\1: Enable"]
pub type EVT_OP2_TEE2_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configures whether or not to enable PWM generator0 timer equal OP0_TSTMP_E1_REG event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn evt_op0_tee1_en(&self) -> EVT_OP0_TEE1_EN_R {
        EVT_OP0_TEE1_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configures whether or not to enable PWM generator1 timer equal OP1_TSTMP_E1_REG event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn evt_op1_tee1_en(&self) -> EVT_OP1_TEE1_EN_R {
        EVT_OP1_TEE1_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Configures whether or not to enable PWM generator2 timer equal OP2_TSTMP_E1_REG event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn evt_op2_tee1_en(&self) -> EVT_OP2_TEE1_EN_R {
        EVT_OP2_TEE1_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Configures whether or not to enable PWM generator0 timer equal OP0_TSTMP_E2_REG event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn evt_op0_tee2_en(&self) -> EVT_OP0_TEE2_EN_R {
        EVT_OP0_TEE2_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Configures whether or not to enable PWM generator1 timer equal OP1_TSTMP_E2_REG event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn evt_op1_tee2_en(&self) -> EVT_OP1_TEE2_EN_R {
        EVT_OP1_TEE2_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Configures whether or not to enable PWM generator2 timer equal OP2_TSTMP_E2_REG event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn evt_op2_tee2_en(&self) -> EVT_OP2_TEE2_EN_R {
        EVT_OP2_TEE2_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EVT_EN2")
            .field(
                "evt_op0_tee1_en",
                &format_args!("{}", self.evt_op0_tee1_en().bit()),
            )
            .field(
                "evt_op1_tee1_en",
                &format_args!("{}", self.evt_op1_tee1_en().bit()),
            )
            .field(
                "evt_op2_tee1_en",
                &format_args!("{}", self.evt_op2_tee1_en().bit()),
            )
            .field(
                "evt_op0_tee2_en",
                &format_args!("{}", self.evt_op0_tee2_en().bit()),
            )
            .field(
                "evt_op1_tee2_en",
                &format_args!("{}", self.evt_op1_tee2_en().bit()),
            )
            .field(
                "evt_op2_tee2_en",
                &format_args!("{}", self.evt_op2_tee2_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<EVT_EN2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Configures whether or not to enable PWM generator0 timer equal OP0_TSTMP_E1_REG event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    #[must_use]
    pub fn evt_op0_tee1_en(&mut self) -> EVT_OP0_TEE1_EN_W<EVT_EN2_SPEC> {
        EVT_OP0_TEE1_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Configures whether or not to enable PWM generator1 timer equal OP1_TSTMP_E1_REG event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    #[must_use]
    pub fn evt_op1_tee1_en(&mut self) -> EVT_OP1_TEE1_EN_W<EVT_EN2_SPEC> {
        EVT_OP1_TEE1_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Configures whether or not to enable PWM generator2 timer equal OP2_TSTMP_E1_REG event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    #[must_use]
    pub fn evt_op2_tee1_en(&mut self) -> EVT_OP2_TEE1_EN_W<EVT_EN2_SPEC> {
        EVT_OP2_TEE1_EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - Configures whether or not to enable PWM generator0 timer equal OP0_TSTMP_E2_REG event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    #[must_use]
    pub fn evt_op0_tee2_en(&mut self) -> EVT_OP0_TEE2_EN_W<EVT_EN2_SPEC> {
        EVT_OP0_TEE2_EN_W::new(self, 3)
    }
    #[doc = "Bit 4 - Configures whether or not to enable PWM generator1 timer equal OP1_TSTMP_E2_REG event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    #[must_use]
    pub fn evt_op1_tee2_en(&mut self) -> EVT_OP1_TEE2_EN_W<EVT_EN2_SPEC> {
        EVT_OP1_TEE2_EN_W::new(self, 4)
    }
    #[doc = "Bit 5 - Configures whether or not to enable PWM generator2 timer equal OP2_TSTMP_E2_REG event generate.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    #[must_use]
    pub fn evt_op2_tee2_en(&mut self) -> EVT_OP2_TEE2_EN_W<EVT_EN2_SPEC> {
        EVT_OP2_TEE2_EN_W::new(self, 5)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Event enable register2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evt_en2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evt_en2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EVT_EN2_SPEC;
impl crate::RegisterSpec for EVT_EN2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`evt_en2::R`](R) reader structure"]
impl crate::Readable for EVT_EN2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`evt_en2::W`](W) writer structure"]
impl crate::Writable for EVT_EN2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EVT_EN2 to value 0"]
impl crate::Resettable for EVT_EN2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
