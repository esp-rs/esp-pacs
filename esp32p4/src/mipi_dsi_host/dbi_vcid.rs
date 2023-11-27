#[doc = "Register `DBI_VCID` reader"]
pub type R = crate::R<DBI_VCID_SPEC>;
#[doc = "Register `DBI_VCID` writer"]
pub type W = crate::W<DBI_VCID_SPEC>;
#[doc = "Field `DBI_VCID` reader - NA"]
pub type DBI_VCID_R = crate::FieldReader;
#[doc = "Field `DBI_VCID` writer - NA"]
pub type DBI_VCID_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - NA"]
    #[inline(always)]
    pub fn dbi_vcid(&self) -> DBI_VCID_R {
        DBI_VCID_R::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBI_VCID")
            .field("dbi_vcid", &format_args!("{}", self.dbi_vcid().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DBI_VCID_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:1 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn dbi_vcid(&mut self) -> DBI_VCID_W<DBI_VCID_SPEC> {
        DBI_VCID_W::new(self, 0)
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
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbi_vcid::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbi_vcid::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DBI_VCID_SPEC;
impl crate::RegisterSpec for DBI_VCID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbi_vcid::R`](R) reader structure"]
impl crate::Readable for DBI_VCID_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dbi_vcid::W`](W) writer structure"]
impl crate::Writable for DBI_VCID_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DBI_VCID to value 0"]
impl crate::Resettable for DBI_VCID_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
