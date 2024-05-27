#[doc = "Register `GEN%s` reader"]
pub type R = crate::R<GEN_SPEC>;
#[doc = "Register `GEN%s` writer"]
pub type W = crate::W<GEN_SPEC>;
#[doc = "Field `UTEZ` reader - "]
pub type UTEZ_R = crate::FieldReader;
#[doc = "Field `UTEZ` writer - "]
pub type UTEZ_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `UTEP` reader - "]
pub type UTEP_R = crate::FieldReader;
#[doc = "Field `UTEP` writer - "]
pub type UTEP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `UTEA` reader - "]
pub type UTEA_R = crate::FieldReader;
#[doc = "Field `UTEA` writer - "]
pub type UTEA_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `UTEB` reader - "]
pub type UTEB_R = crate::FieldReader;
#[doc = "Field `UTEB` writer - "]
pub type UTEB_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `UT0` reader - "]
pub type UT0_R = crate::FieldReader;
#[doc = "Field `UT0` writer - "]
pub type UT0_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `UT1` reader - "]
pub type UT1_R = crate::FieldReader;
#[doc = "Field `UT1` writer - "]
pub type UT1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DTEZ` reader - "]
pub type DTEZ_R = crate::FieldReader;
#[doc = "Field `DTEZ` writer - "]
pub type DTEZ_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DTEP` reader - "]
pub type DTEP_R = crate::FieldReader;
#[doc = "Field `DTEP` writer - "]
pub type DTEP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DTEA` reader - "]
pub type DTEA_R = crate::FieldReader;
#[doc = "Field `DTEA` writer - "]
pub type DTEA_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DTEB` reader - "]
pub type DTEB_R = crate::FieldReader;
#[doc = "Field `DTEB` writer - "]
pub type DTEB_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DT0` reader - "]
pub type DT0_R = crate::FieldReader;
#[doc = "Field `DT0` writer - "]
pub type DT0_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DT1` reader - "]
pub type DT1_R = crate::FieldReader;
#[doc = "Field `DT1` writer - "]
pub type DT1_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn utez(&self) -> UTEZ_R {
        UTEZ_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn utep(&self) -> UTEP_R {
        UTEP_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn utea(&self) -> UTEA_R {
        UTEA_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn uteb(&self) -> UTEB_R {
        UTEB_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn ut0(&self) -> UT0_R {
        UT0_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn ut1(&self) -> UT1_R {
        UT1_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn dtez(&self) -> DTEZ_R {
        DTEZ_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn dtep(&self) -> DTEP_R {
        DTEP_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn dtea(&self) -> DTEA_R {
        DTEA_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn dteb(&self) -> DTEB_R {
        DTEB_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn dt0(&self) -> DT0_R {
        DT0_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn dt1(&self) -> DT1_R {
        DT1_R::new(((self.bits >> 22) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GEN")
            .field("utez", &self.utez())
            .field("utep", &self.utep())
            .field("utea", &self.utea())
            .field("uteb", &self.uteb())
            .field("ut0", &self.ut0())
            .field("ut1", &self.ut1())
            .field("dtez", &self.dtez())
            .field("dtep", &self.dtep())
            .field("dtea", &self.dtea())
            .field("dteb", &self.dteb())
            .field("dt0", &self.dt0())
            .field("dt1", &self.dt1())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    #[must_use]
    pub fn utez(&mut self) -> UTEZ_W<GEN_SPEC> {
        UTEZ_W::new(self, 0)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    #[must_use]
    pub fn utep(&mut self) -> UTEP_W<GEN_SPEC> {
        UTEP_W::new(self, 2)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn utea(&mut self) -> UTEA_W<GEN_SPEC> {
        UTEA_W::new(self, 4)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    #[must_use]
    pub fn uteb(&mut self) -> UTEB_W<GEN_SPEC> {
        UTEB_W::new(self, 6)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    #[must_use]
    pub fn ut0(&mut self) -> UT0_W<GEN_SPEC> {
        UT0_W::new(self, 8)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    #[must_use]
    pub fn ut1(&mut self) -> UT1_W<GEN_SPEC> {
        UT1_W::new(self, 10)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    #[must_use]
    pub fn dtez(&mut self) -> DTEZ_W<GEN_SPEC> {
        DTEZ_W::new(self, 12)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    #[must_use]
    pub fn dtep(&mut self) -> DTEP_W<GEN_SPEC> {
        DTEP_W::new(self, 14)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    #[must_use]
    pub fn dtea(&mut self) -> DTEA_W<GEN_SPEC> {
        DTEA_W::new(self, 16)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    #[must_use]
    pub fn dteb(&mut self) -> DTEB_W<GEN_SPEC> {
        DTEB_W::new(self, 18)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    #[must_use]
    pub fn dt0(&mut self) -> DT0_W<GEN_SPEC> {
        DT0_W::new(self, 20)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    #[must_use]
    pub fn dt1(&mut self) -> DT1_W<GEN_SPEC> {
        DT1_W::new(self, 22)
    }
}
#[doc = "Actions triggered by events on PWMx%s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gen::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gen::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GEN_SPEC;
impl crate::RegisterSpec for GEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gen::R`](R) reader structure"]
impl crate::Readable for GEN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gen::W`](W) writer structure"]
impl crate::Writable for GEN_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GEN%s to value 0"]
impl crate::Resettable for GEN_SPEC {
    const RESET_VALUE: u32 = 0;
}
