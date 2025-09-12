#[doc = "Register `LLP0` reader"]
pub type R = crate::R<LLP0_SPEC>;
#[doc = "Register `LLP0` writer"]
pub type W = crate::W<LLP0_SPEC>;
#[doc = "Field `CH1_LMS` reader - NA"]
pub type CH1_LMS_R = crate::BitReader;
#[doc = "Field `CH1_LMS` writer - NA"]
pub type CH1_LMS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_LOC0` reader - NA"]
pub type CH1_LOC0_R = crate::FieldReader<u32>;
#[doc = "Field `CH1_LOC0` writer - NA"]
pub type CH1_LOC0_W<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn ch1_lms(&self) -> CH1_LMS_R {
        CH1_LMS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 6:31 - NA"]
    #[inline(always)]
    pub fn ch1_loc0(&self) -> CH1_LOC0_R {
        CH1_LOC0_R::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LLP0")
            .field("ch1_lms", &self.ch1_lms())
            .field("ch1_loc0", &self.ch1_loc0())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn ch1_lms(&mut self) -> CH1_LMS_W<'_, LLP0_SPEC> {
        CH1_LMS_W::new(self, 0)
    }
    #[doc = "Bits 6:31 - NA"]
    #[inline(always)]
    pub fn ch1_loc0(&mut self) -> CH1_LOC0_W<'_, LLP0_SPEC> {
        CH1_LOC0_W::new(self, 6)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`llp0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`llp0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LLP0_SPEC;
impl crate::RegisterSpec for LLP0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`llp0::R`](R) reader structure"]
impl crate::Readable for LLP0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`llp0::W`](W) writer structure"]
impl crate::Writable for LLP0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LLP0 to value 0"]
impl crate::Resettable for LLP0_SPEC {}
