#[doc = "Register `VERSION` reader"]
pub type R = crate::R<VERSION_SPEC>;
#[doc = "Register `VERSION` writer"]
pub type W = crate::W<VERSION_SPEC>;
#[doc = "Field `DATE` reader - Version of this register file"]
pub type DATE_R = crate::FieldReader<u32>;
#[doc = "Field `DATE` writer - Version of this register file"]
pub type DATE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 28, O, u32>;
impl R {
    #[doc = "Bits 0:27 - Version of this register file"]
    #[inline(always)]
    pub fn date(&self) -> DATE_R {
        DATE_R::new(self.bits & 0x0fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VERSION")
            .field("date", &format_args!("{}", self.date().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<VERSION_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:27 - Version of this register file"]
    #[inline(always)]
    #[must_use]
    pub fn date(&mut self) -> DATE_W<VERSION_SPEC, 0> {
        DATE_W::new(self)
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
#[doc = "Version register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`version::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`version::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VERSION_SPEC;
impl crate::RegisterSpec for VERSION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`version::R`](R) reader structure"]
impl crate::Readable for VERSION_SPEC {}
#[doc = "`write(|w| ..)` method takes [`version::W`](W) writer structure"]
impl crate::Writable for VERSION_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets VERSION to value 0x0150_9110"]
impl crate::Resettable for VERSION_SPEC {
    const RESET_VALUE: Self::Ux = 0x0150_9110;
}
