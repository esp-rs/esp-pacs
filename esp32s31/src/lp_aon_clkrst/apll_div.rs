#[doc = "Register `APLL_DIV` reader"]
pub type R = crate::R<APLL_DIV_SPEC>;
#[doc = "Register `APLL_DIV` writer"]
pub type W = crate::W<APLL_DIV_SPEC>;
#[doc = "Field `APLL_REF_DIV` reader - apll ref div value"]
pub type APLL_REF_DIV_R = crate::FieldReader;
#[doc = "Field `APLL_REF_DIV` writer - apll ref div value"]
pub type APLL_REF_DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `APLL_OUT_DIV` reader - apll out div value"]
pub type APLL_OUT_DIV_R = crate::FieldReader;
#[doc = "Field `APLL_OUT_DIV` writer - apll out div value"]
pub type APLL_OUT_DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:2 - apll ref div value"]
    #[inline(always)]
    pub fn apll_ref_div(&self) -> APLL_REF_DIV_R {
        APLL_REF_DIV_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:7 - apll out div value"]
    #[inline(always)]
    pub fn apll_out_div(&self) -> APLL_OUT_DIV_R {
        APLL_OUT_DIV_R::new(((self.bits >> 3) & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APLL_DIV")
            .field("apll_ref_div", &self.apll_ref_div())
            .field("apll_out_div", &self.apll_out_div())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - apll ref div value"]
    #[inline(always)]
    pub fn apll_ref_div(&mut self) -> APLL_REF_DIV_W<'_, APLL_DIV_SPEC> {
        APLL_REF_DIV_W::new(self, 0)
    }
    #[doc = "Bits 3:7 - apll out div value"]
    #[inline(always)]
    pub fn apll_out_div(&mut self) -> APLL_OUT_DIV_W<'_, APLL_DIV_SPEC> {
        APLL_OUT_DIV_W::new(self, 3)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`apll_div::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apll_div::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APLL_DIV_SPEC;
impl crate::RegisterSpec for APLL_DIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apll_div::R`](R) reader structure"]
impl crate::Readable for APLL_DIV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apll_div::W`](W) writer structure"]
impl crate::Writable for APLL_DIV_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APLL_DIV to value 0x85"]
impl crate::Resettable for APLL_DIV_SPEC {
    const RESET_VALUE: u32 = 0x85;
}
