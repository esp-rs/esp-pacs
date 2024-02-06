#[doc = "Register `IDPARTNO` reader"]
pub type R = crate::R<IDPARTNO_SPEC>;
#[doc = "Register `IDPARTNO` writer"]
pub type W = crate::W<IDPARTNO_SPEC>;
#[doc = "Field `PARTNO` reader - NA"]
pub type PARTNO_R = crate::FieldReader<u32>;
#[doc = "Field `PARTNO` writer - NA"]
pub type PARTNO_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - NA"]
    #[inline(always)]
    pub fn partno(&self) -> PARTNO_R {
        PARTNO_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IDPARTNO")
            .field("partno", &format_args!("{}", self.partno().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IDPARTNO_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn partno(&mut self) -> PARTNO_W<IDPARTNO_SPEC> {
        PARTNO_W::new(self, 0)
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
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idpartno::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idpartno::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IDPARTNO_SPEC;
impl crate::RegisterSpec for IDPARTNO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idpartno::R`](R) reader structure"]
impl crate::Readable for IDPARTNO_SPEC {}
#[doc = "`write(|w| ..)` method takes [`idpartno::W`](W) writer structure"]
impl crate::Writable for IDPARTNO_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IDPARTNO to value 0"]
impl crate::Resettable for IDPARTNO_SPEC {
    const RESET_VALUE: u32 = 0;
}
