#[doc = "Register `SHARP_FILTER0` reader"]
pub type R = crate::R<SHARP_FILTER0_SPEC>;
#[doc = "Register `SHARP_FILTER0` writer"]
pub type W = crate::W<SHARP_FILTER0_SPEC>;
#[doc = "Field `SHARP_FILTER_COE00` reader - this field configures unsharp masking(usm) filter coefficient"]
pub type SHARP_FILTER_COE00_R = crate::FieldReader;
#[doc = "Field `SHARP_FILTER_COE00` writer - this field configures unsharp masking(usm) filter coefficient"]
pub type SHARP_FILTER_COE00_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SHARP_FILTER_COE01` reader - this field configures usm filter coefficient"]
pub type SHARP_FILTER_COE01_R = crate::FieldReader;
#[doc = "Field `SHARP_FILTER_COE01` writer - this field configures usm filter coefficient"]
pub type SHARP_FILTER_COE01_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `SHARP_FILTER_COE02` reader - this field configures usm filter coefficient"]
pub type SHARP_FILTER_COE02_R = crate::FieldReader;
#[doc = "Field `SHARP_FILTER_COE02` writer - this field configures usm filter coefficient"]
pub type SHARP_FILTER_COE02_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - this field configures unsharp masking(usm) filter coefficient"]
    #[inline(always)]
    pub fn sharp_filter_coe00(&self) -> SHARP_FILTER_COE00_R {
        SHARP_FILTER_COE00_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - this field configures usm filter coefficient"]
    #[inline(always)]
    pub fn sharp_filter_coe01(&self) -> SHARP_FILTER_COE01_R {
        SHARP_FILTER_COE01_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:14 - this field configures usm filter coefficient"]
    #[inline(always)]
    pub fn sharp_filter_coe02(&self) -> SHARP_FILTER_COE02_R {
        SHARP_FILTER_COE02_R::new(((self.bits >> 10) & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SHARP_FILTER0")
            .field("sharp_filter_coe00", &self.sharp_filter_coe00())
            .field("sharp_filter_coe01", &self.sharp_filter_coe01())
            .field("sharp_filter_coe02", &self.sharp_filter_coe02())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - this field configures unsharp masking(usm) filter coefficient"]
    #[inline(always)]
    #[must_use]
    pub fn sharp_filter_coe00(&mut self) -> SHARP_FILTER_COE00_W<SHARP_FILTER0_SPEC> {
        SHARP_FILTER_COE00_W::new(self, 0)
    }
    #[doc = "Bits 5:9 - this field configures usm filter coefficient"]
    #[inline(always)]
    #[must_use]
    pub fn sharp_filter_coe01(&mut self) -> SHARP_FILTER_COE01_W<SHARP_FILTER0_SPEC> {
        SHARP_FILTER_COE01_W::new(self, 5)
    }
    #[doc = "Bits 10:14 - this field configures usm filter coefficient"]
    #[inline(always)]
    #[must_use]
    pub fn sharp_filter_coe02(&mut self) -> SHARP_FILTER_COE02_W<SHARP_FILTER0_SPEC> {
        SHARP_FILTER_COE02_W::new(self, 10)
    }
}
#[doc = "sharp usm config register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sharp_filter0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sharp_filter0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SHARP_FILTER0_SPEC;
impl crate::RegisterSpec for SHARP_FILTER0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sharp_filter0::R`](R) reader structure"]
impl crate::Readable for SHARP_FILTER0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sharp_filter0::W`](W) writer structure"]
impl crate::Writable for SHARP_FILTER0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SHARP_FILTER0 to value 0x0441"]
impl crate::Resettable for SHARP_FILTER0_SPEC {
    const RESET_VALUE: u32 = 0x0441;
}
