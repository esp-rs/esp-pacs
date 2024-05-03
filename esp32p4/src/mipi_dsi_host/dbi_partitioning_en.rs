#[doc = "Register `DBI_PARTITIONING_EN` reader"]
pub type R = crate::R<DBI_PARTITIONING_EN_SPEC>;
#[doc = "Register `DBI_PARTITIONING_EN` writer"]
pub type W = crate::W<DBI_PARTITIONING_EN_SPEC>;
#[doc = "Field `PARTITIONING_EN` reader - NA"]
pub type PARTITIONING_EN_R = crate::BitReader;
#[doc = "Field `PARTITIONING_EN` writer - NA"]
pub type PARTITIONING_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn partitioning_en(&self) -> PARTITIONING_EN_R {
        PARTITIONING_EN_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBI_PARTITIONING_EN")
            .field("partitioning_en", &self.partitioning_en().bit())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DBI_PARTITIONING_EN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn partitioning_en(&mut self) -> PARTITIONING_EN_W<DBI_PARTITIONING_EN_SPEC> {
        PARTITIONING_EN_W::new(self, 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbi_partitioning_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbi_partitioning_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DBI_PARTITIONING_EN_SPEC;
impl crate::RegisterSpec for DBI_PARTITIONING_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbi_partitioning_en::R`](R) reader structure"]
impl crate::Readable for DBI_PARTITIONING_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dbi_partitioning_en::W`](W) writer structure"]
impl crate::Writable for DBI_PARTITIONING_EN_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DBI_PARTITIONING_EN to value 0"]
impl crate::Resettable for DBI_PARTITIONING_EN_SPEC {
    const RESET_VALUE: u32 = 0;
}
