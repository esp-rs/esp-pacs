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
            .field("partitioning_en", &self.partitioning_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn partitioning_en(&mut self) -> PARTITIONING_EN_W<DBI_PARTITIONING_EN_SPEC> {
        PARTITIONING_EN_W::new(self, 0)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`dbi_partitioning_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbi_partitioning_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DBI_PARTITIONING_EN_SPEC;
impl crate::RegisterSpec for DBI_PARTITIONING_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbi_partitioning_en::R`](R) reader structure"]
impl crate::Readable for DBI_PARTITIONING_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dbi_partitioning_en::W`](W) writer structure"]
impl crate::Writable for DBI_PARTITIONING_EN_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DBI_PARTITIONING_EN to value 0"]
impl crate::Resettable for DBI_PARTITIONING_EN_SPEC {}
