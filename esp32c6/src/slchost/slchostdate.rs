#[doc = "Register `SLCHOSTDATE` reader"]
pub type R = crate::R<SLCHOSTDATE_SPEC>;
#[doc = "Register `SLCHOSTDATE` writer"]
pub type W = crate::W<SLCHOSTDATE_SPEC>;
#[doc = "Field `SLCHOST_DATE` reader - *******Description***********"]
pub type SLCHOST_DATE_R = crate::FieldReader<u32>;
#[doc = "Field `SLCHOST_DATE` writer - *******Description***********"]
pub type SLCHOST_DATE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - *******Description***********"]
    #[inline(always)]
    pub fn slchost_date(&self) -> SLCHOST_DATE_R {
        SLCHOST_DATE_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLCHOSTDATE")
            .field(
                "slchost_date",
                &format_args!("{}", self.slchost_date().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SLCHOSTDATE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - *******Description***********"]
    #[inline(always)]
    #[must_use]
    pub fn slchost_date(&mut self) -> SLCHOST_DATE_W<SLCHOSTDATE_SPEC, 0> {
        SLCHOST_DATE_W::new(self)
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
#[doc = "*******Description***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slchostdate::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slchostdate::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLCHOSTDATE_SPEC;
impl crate::RegisterSpec for SLCHOSTDATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slchostdate::R`](R) reader structure"]
impl crate::Readable for SLCHOSTDATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`slchostdate::W`](W) writer structure"]
impl crate::Writable for SLCHOSTDATE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLCHOSTDATE to value 0x2106_0700"]
impl crate::Resettable for SLCHOSTDATE_SPEC {
    const RESET_VALUE: Self::Ux = 0x2106_0700;
}
