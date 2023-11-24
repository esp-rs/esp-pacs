#[doc = "Register `HOST_SLCHOSTDATE` reader"]
pub type R = crate::R<HOST_SLCHOSTDATE_SPEC>;
#[doc = "Register `HOST_SLCHOSTDATE` writer"]
pub type W = crate::W<HOST_SLCHOSTDATE_SPEC>;
#[doc = "Field `HOST_SLCHOST_DATE` reader - "]
pub type HOST_SLCHOST_DATE_R = crate::FieldReader<u32>;
#[doc = "Field `HOST_SLCHOST_DATE` writer - "]
pub type HOST_SLCHOST_DATE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn host_slchost_date(&self) -> HOST_SLCHOST_DATE_R {
        HOST_SLCHOST_DATE_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HOST_SLCHOSTDATE")
            .field(
                "host_slchost_date",
                &format_args!("{}", self.host_slchost_date().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HOST_SLCHOSTDATE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn host_slchost_date(&mut self) -> HOST_SLCHOST_DATE_W<HOST_SLCHOSTDATE_SPEC> {
        HOST_SLCHOST_DATE_W::new(self, 0)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`host_slchostdate::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_slchostdate::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HOST_SLCHOSTDATE_SPEC;
impl crate::RegisterSpec for HOST_SLCHOSTDATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`host_slchostdate::R`](R) reader structure"]
impl crate::Readable for HOST_SLCHOSTDATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`host_slchostdate::W`](W) writer structure"]
impl crate::Writable for HOST_SLCHOSTDATE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HOST_SLCHOSTDATE to value 0x1602_2500"]
impl crate::Resettable for HOST_SLCHOSTDATE_SPEC {
    const RESET_VALUE: Self::Ux = 0x1602_2500;
}
