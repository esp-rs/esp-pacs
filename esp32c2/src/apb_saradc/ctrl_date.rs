#[doc = "Register `CTRL_DATE` reader"]
pub type R = crate::R<CTRL_DATE_SPEC>;
#[doc = "Register `CTRL_DATE` writer"]
pub type W = crate::W<CTRL_DATE_SPEC>;
#[doc = "Field `DATE` reader - Need add description"]
pub type DATE_R = crate::FieldReader<u32>;
#[doc = "Field `DATE` writer - Need add description"]
pub type DATE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Need add description"]
    #[inline(always)]
    pub fn date(&self) -> DATE_R {
        DATE_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CTRL_DATE")
            .field("date", &self.date())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Need add description"]
    #[inline(always)]
    #[must_use]
    pub fn date(&mut self) -> DATE_W<CTRL_DATE_SPEC> {
        DATE_W::new(self, 0)
    }
}
#[doc = "register description\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl_date::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl_date::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_DATE_SPEC;
impl crate::RegisterSpec for CTRL_DATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl_date::R`](R) reader structure"]
impl crate::Readable for CTRL_DATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl_date::W`](W) writer structure"]
impl crate::Writable for CTRL_DATE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL_DATE to value 0x0210_7210"]
impl crate::Resettable for CTRL_DATE_SPEC {
    const RESET_VALUE: u32 = 0x0210_7210;
}
