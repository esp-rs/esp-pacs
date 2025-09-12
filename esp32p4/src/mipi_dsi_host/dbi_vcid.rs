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
            .field("dbi_vcid", &self.dbi_vcid())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - NA"]
    #[inline(always)]
    pub fn dbi_vcid(&mut self) -> DBI_VCID_W<'_, DBI_VCID_SPEC> {
        DBI_VCID_W::new(self, 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`dbi_vcid::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbi_vcid::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DBI_VCID_SPEC;
impl crate::RegisterSpec for DBI_VCID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbi_vcid::R`](R) reader structure"]
impl crate::Readable for DBI_VCID_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dbi_vcid::W`](W) writer structure"]
impl crate::Writable for DBI_VCID_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DBI_VCID to value 0"]
impl crate::Resettable for DBI_VCID_SPEC {}
