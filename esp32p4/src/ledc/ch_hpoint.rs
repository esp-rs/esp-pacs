#[doc = "Register `CH%s_HPOINT` reader"]
pub type R = crate::R<CH_HPOINT_SPEC>;
#[doc = "Register `CH%s_HPOINT` writer"]
pub type W = crate::W<CH_HPOINT_SPEC>;
#[doc = "Field `HPOINT_CH` reader - Configures high point of signal output on channel %s. The output value changes to high when the selected timers has reached the value specified by this register."]
pub type HPOINT_CH_R = crate::FieldReader<u32>;
#[doc = "Field `HPOINT_CH` writer - Configures high point of signal output on channel %s. The output value changes to high when the selected timers has reached the value specified by this register."]
pub type HPOINT_CH_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - Configures high point of signal output on channel %s. The output value changes to high when the selected timers has reached the value specified by this register."]
    #[inline(always)]
    pub fn hpoint_ch(&self) -> HPOINT_CH_R {
        HPOINT_CH_R::new(self.bits & 0x000f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH_HPOINT")
            .field("hpoint_ch", &format_args!("{}", self.hpoint_ch().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CH_HPOINT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:19 - Configures high point of signal output on channel %s. The output value changes to high when the selected timers has reached the value specified by this register."]
    #[inline(always)]
    #[must_use]
    pub fn hpoint_ch(&mut self) -> HPOINT_CH_W<CH_HPOINT_SPEC> {
        HPOINT_CH_W::new(self, 0)
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
#[doc = "High point register for channel %s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch_hpoint::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch_hpoint::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH_HPOINT_SPEC;
impl crate::RegisterSpec for CH_HPOINT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch_hpoint::R`](R) reader structure"]
impl crate::Readable for CH_HPOINT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch_hpoint::W`](W) writer structure"]
impl crate::Writable for CH_HPOINT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH%s_HPOINT to value 0"]
impl crate::Resettable for CH_HPOINT_SPEC {
    const RESET_VALUE: u32 = 0;
}
