#[doc = "Register `BLEBASETIMECNT` reader"]
pub type R = crate::R<BLEBASETIMECNT_SPEC>;
#[doc = "Register `BLEBASETIMECNT` writer"]
pub type W = crate::W<BLEBASETIMECNT_SPEC>;
#[doc = "Field `BASETIMECNT` reader - Base time counter value"]
pub type BASETIMECNT_R = crate::FieldReader<u32>;
#[doc = "Field `BASETIMECNT` writer - Base time counter value"]
pub type BASETIMECNT_W<'a, REG> = crate::FieldWriter<'a, REG, 27, u32>;
#[doc = "Field `SAMP` reader - Sample the base time counter"]
pub type SAMP_R = crate::BitReader;
#[doc = "Field `SAMP` writer - Sample the base time counter"]
pub type SAMP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:26 - Base time counter value"]
    #[inline(always)]
    pub fn basetimecnt(&self) -> BASETIMECNT_R {
        BASETIMECNT_R::new(self.bits & 0x07ff_ffff)
    }
    #[doc = "Bit 31 - Sample the base time counter"]
    #[inline(always)]
    pub fn samp(&self) -> SAMP_R {
        SAMP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLEBASETIMECNT")
            .field("basetimecnt", &self.basetimecnt())
            .field("samp", &self.samp())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:26 - Base time counter value"]
    #[inline(always)]
    pub fn basetimecnt(&mut self) -> BASETIMECNT_W<'_, BLEBASETIMECNT_SPEC> {
        BASETIMECNT_W::new(self, 0)
    }
    #[doc = "Bit 31 - Sample the base time counter"]
    #[inline(always)]
    pub fn samp(&mut self) -> SAMP_W<'_, BLEBASETIMECNT_SPEC> {
        SAMP_W::new(self, 31)
    }
}
#[doc = "Base time reference counter\n\nYou can [`read`](crate::Reg::read) this register and get [`blebasetimecnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blebasetimecnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLEBASETIMECNT_SPEC;
impl crate::RegisterSpec for BLEBASETIMECNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blebasetimecnt::R`](R) reader structure"]
impl crate::Readable for BLEBASETIMECNT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`blebasetimecnt::W`](W) writer structure"]
impl crate::Writable for BLEBASETIMECNT_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BLEBASETIMECNT to value 0"]
impl crate::Resettable for BLEBASETIMECNT_SPEC {}
