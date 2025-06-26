#[doc = "Register `TEST_MUX` reader"]
pub type R = crate::R<TEST_MUX_SPEC>;
#[doc = "Register `TEST_MUX` writer"]
pub type W = crate::W<TEST_MUX_SPEC>;
#[doc = "Field `ENT_RTC` reader - ENT_RTC"]
pub type ENT_RTC_R = crate::BitReader;
#[doc = "Field `ENT_RTC` writer - ENT_RTC"]
pub type ENT_RTC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTEST_RTC` reader - DTEST_RTC"]
pub type DTEST_RTC_R = crate::FieldReader;
#[doc = "Field `DTEST_RTC` writer - DTEST_RTC"]
pub type DTEST_RTC_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 29 - ENT_RTC"]
    #[inline(always)]
    pub fn ent_rtc(&self) -> ENT_RTC_R {
        ENT_RTC_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bits 30:31 - DTEST_RTC"]
    #[inline(always)]
    pub fn dtest_rtc(&self) -> DTEST_RTC_R {
        DTEST_RTC_R::new(((self.bits >> 30) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TEST_MUX")
            .field("ent_rtc", &self.ent_rtc())
            .field("dtest_rtc", &self.dtest_rtc())
            .finish()
    }
}
impl W {
    #[doc = "Bit 29 - ENT_RTC"]
    #[inline(always)]
    pub fn ent_rtc(&mut self) -> ENT_RTC_W<TEST_MUX_SPEC> {
        ENT_RTC_W::new(self, 29)
    }
    #[doc = "Bits 30:31 - DTEST_RTC"]
    #[inline(always)]
    pub fn dtest_rtc(&mut self) -> DTEST_RTC_W<TEST_MUX_SPEC> {
        DTEST_RTC_W::new(self, 30)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`test_mux::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`test_mux::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TEST_MUX_SPEC;
impl crate::RegisterSpec for TEST_MUX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`test_mux::R`](R) reader structure"]
impl crate::Readable for TEST_MUX_SPEC {}
#[doc = "`write(|w| ..)` method takes [`test_mux::W`](W) writer structure"]
impl crate::Writable for TEST_MUX_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TEST_MUX to value 0"]
impl crate::Resettable for TEST_MUX_SPEC {}
