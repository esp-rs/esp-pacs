#[doc = "Register `CPLL_DIV` reader"]
pub type R = crate::R<CPLL_DIV_SPEC>;
#[doc = "Register `CPLL_DIV` writer"]
pub type W = crate::W<CPLL_DIV_SPEC>;
#[doc = "Field `CPLL_REF_DIV` reader - cpll ref div value"]
pub type CPLL_REF_DIV_R = crate::FieldReader;
#[doc = "Field `CPLL_REF_DIV` writer - cpll ref div value"]
pub type CPLL_REF_DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CPLL_FB_DIV` reader - cpll fb div value"]
pub type CPLL_FB_DIV_R = crate::FieldReader;
#[doc = "Field `CPLL_FB_DIV` writer - cpll fb div value"]
pub type CPLL_FB_DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:3 - cpll ref div value"]
    #[inline(always)]
    pub fn cpll_ref_div(&self) -> CPLL_REF_DIV_R {
        CPLL_REF_DIV_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:11 - cpll fb div value"]
    #[inline(always)]
    pub fn cpll_fb_div(&self) -> CPLL_FB_DIV_R {
        CPLL_FB_DIV_R::new(((self.bits >> 4) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CPLL_DIV")
            .field("cpll_ref_div", &self.cpll_ref_div())
            .field("cpll_fb_div", &self.cpll_fb_div())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - cpll ref div value"]
    #[inline(always)]
    pub fn cpll_ref_div(&mut self) -> CPLL_REF_DIV_W<'_, CPLL_DIV_SPEC> {
        CPLL_REF_DIV_W::new(self, 0)
    }
    #[doc = "Bits 4:11 - cpll fb div value"]
    #[inline(always)]
    pub fn cpll_fb_div(&mut self) -> CPLL_FB_DIV_W<'_, CPLL_DIV_SPEC> {
        CPLL_FB_DIV_W::new(self, 4)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`cpll_div::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpll_div::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CPLL_DIV_SPEC;
impl crate::RegisterSpec for CPLL_DIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpll_div::R`](R) reader structure"]
impl crate::Readable for CPLL_DIV_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cpll_div::W`](W) writer structure"]
impl crate::Writable for CPLL_DIV_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CPLL_DIV to value 0xf2"]
impl crate::Resettable for CPLL_DIV_SPEC {
    const RESET_VALUE: u32 = 0xf2;
}
