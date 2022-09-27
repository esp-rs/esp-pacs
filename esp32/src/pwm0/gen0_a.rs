#[doc = "Register `GEN0_A` reader"]
pub struct R(crate::R<GEN0_A_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GEN0_A_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GEN0_A_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GEN0_A_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GEN0_A` writer"]
pub struct W(crate::W<GEN0_A_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GEN0_A_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<GEN0_A_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GEN0_A_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UTEZ` reader - "]
pub type UTEZ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UTEZ` writer - "]
pub type UTEZ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GEN0_A_SPEC, u8, u8, 2, O>;
#[doc = "Field `UTEP` reader - "]
pub type UTEP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UTEP` writer - "]
pub type UTEP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GEN0_A_SPEC, u8, u8, 2, O>;
#[doc = "Field `UTEA` reader - "]
pub type UTEA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UTEA` writer - "]
pub type UTEA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GEN0_A_SPEC, u8, u8, 2, O>;
#[doc = "Field `UTEB` reader - "]
pub type UTEB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UTEB` writer - "]
pub type UTEB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GEN0_A_SPEC, u8, u8, 2, O>;
#[doc = "Field `UT0` reader - "]
pub type UT0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UT0` writer - "]
pub type UT0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GEN0_A_SPEC, u8, u8, 2, O>;
#[doc = "Field `UT1` reader - "]
pub type UT1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `UT1` writer - "]
pub type UT1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GEN0_A_SPEC, u8, u8, 2, O>;
#[doc = "Field `DTEZ` reader - "]
pub type DTEZ_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DTEZ` writer - "]
pub type DTEZ_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GEN0_A_SPEC, u8, u8, 2, O>;
#[doc = "Field `DTEP` reader - "]
pub type DTEP_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DTEP` writer - "]
pub type DTEP_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GEN0_A_SPEC, u8, u8, 2, O>;
#[doc = "Field `DTEA` reader - "]
pub type DTEA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DTEA` writer - "]
pub type DTEA_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GEN0_A_SPEC, u8, u8, 2, O>;
#[doc = "Field `DTEB` reader - "]
pub type DTEB_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DTEB` writer - "]
pub type DTEB_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GEN0_A_SPEC, u8, u8, 2, O>;
#[doc = "Field `DT0` reader - "]
pub type DT0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DT0` writer - "]
pub type DT0_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GEN0_A_SPEC, u8, u8, 2, O>;
#[doc = "Field `DT1` reader - "]
pub type DT1_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DT1` writer - "]
pub type DT1_W<'a, const O: u8> = crate::FieldWriter<'a, u32, GEN0_A_SPEC, u8, u8, 2, O>;
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
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn utez(&mut self) -> UTEZ_W<0> {
        UTEZ_W::new(self)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn utep(&mut self) -> UTEP_W<2> {
        UTEP_W::new(self)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn utea(&mut self) -> UTEA_W<4> {
        UTEA_W::new(self)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn uteb(&mut self) -> UTEB_W<6> {
        UTEB_W::new(self)
    }
    #[doc = "Bits 8:9"]
    #[inline(always)]
    pub fn ut0(&mut self) -> UT0_W<8> {
        UT0_W::new(self)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn ut1(&mut self) -> UT1_W<10> {
        UT1_W::new(self)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn dtez(&mut self) -> DTEZ_W<12> {
        DTEZ_W::new(self)
    }
    #[doc = "Bits 14:15"]
    #[inline(always)]
    pub fn dtep(&mut self) -> DTEP_W<14> {
        DTEP_W::new(self)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn dtea(&mut self) -> DTEA_W<16> {
        DTEA_W::new(self)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn dteb(&mut self) -> DTEB_W<18> {
        DTEB_W::new(self)
    }
    #[doc = "Bits 20:21"]
    #[inline(always)]
    pub fn dt0(&mut self) -> DT0_W<20> {
        DT0_W::new(self)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn dt1(&mut self) -> DT1_W<22> {
        DT1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gen0_a](index.html) module"]
pub struct GEN0_A_SPEC;
impl crate::RegisterSpec for GEN0_A_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gen0_a::R](R) reader structure"]
impl crate::Readable for GEN0_A_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gen0_a::W](W) writer structure"]
impl crate::Writable for GEN0_A_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GEN0_A to value 0"]
impl crate::Resettable for GEN0_A_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
