#[doc = "Register `CH2_SAR0` reader"]
pub type R = crate::R<CH2_SAR0_SPEC>;
#[doc = "Register `CH2_SAR0` writer"]
pub type W = crate::W<CH2_SAR0_SPEC>;
#[doc = "Field `CH2_SAR0` reader - NA"]
pub type CH2_SAR0_R = crate::FieldReader<u32>;
#[doc = "Field `CH2_SAR0` writer - NA"]
pub type CH2_SAR0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - NA"]
    #[inline(always)]
    pub fn ch2_sar0(&self) -> CH2_SAR0_R {
        CH2_SAR0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH2_SAR0")
            .field("ch2_sar0", &format_args!("{}", self.ch2_sar0().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CH2_SAR0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_sar0(&mut self) -> CH2_SAR0_W<CH2_SAR0_SPEC> {
        CH2_SAR0_W::new(self, 0)
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
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2_sar0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2_sar0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH2_SAR0_SPEC;
impl crate::RegisterSpec for CH2_SAR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch2_sar0::R`](R) reader structure"]
impl crate::Readable for CH2_SAR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch2_sar0::W`](W) writer structure"]
impl crate::Writable for CH2_SAR0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH2_SAR0 to value 0"]
impl crate::Resettable for CH2_SAR0_SPEC {
    const RESET_VALUE: u32 = 0;
}
