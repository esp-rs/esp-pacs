#[doc = "Register `TIMER_LOAD` reader"]
pub type R = crate::R<TIMER_LOAD_SPEC>;
#[doc = "Register `TIMER_LOAD` writer"]
pub type W = crate::W<TIMER_LOAD_SPEC>;
#[doc = "Field `VALUE` reader - "]
pub type VALUE_R = crate::FieldReader;
#[doc = "Field `VALUE` writer - "]
pub type VALUE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMER_LOAD")
            .field("value", &format_args!("{}", self.value().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TIMER_LOAD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn value(&mut self) -> VALUE_W<TIMER_LOAD_SPEC, 0> {
        VALUE_W::new(self)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer_load::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer_load::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMER_LOAD_SPEC;
impl crate::RegisterSpec for TIMER_LOAD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer_load::R`](R) reader structure"]
impl crate::Readable for TIMER_LOAD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timer_load::W`](W) writer structure"]
impl crate::Writable for TIMER_LOAD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TIMER_LOAD to value 0"]
impl crate::Resettable for TIMER_LOAD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
