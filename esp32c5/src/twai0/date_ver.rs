#[doc = "Register `DATE_VER` reader"]
pub type R = crate::R<DATE_VER_SPEC>;
#[doc = "Register `DATE_VER` writer"]
pub type W = crate::W<DATE_VER_SPEC>;
#[doc = "Field `DATE_VER` reader - TWAI FD version"]
pub type DATE_VER_R = crate::FieldReader<u32>;
#[doc = "Field `DATE_VER` writer - TWAI FD version"]
pub type DATE_VER_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - TWAI FD version"]
    #[inline(always)]
    pub fn date_ver(&self) -> DATE_VER_R {
        DATE_VER_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DATE_VER")
            .field("date_ver", &self.date_ver())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - TWAI FD version"]
    #[inline(always)]
    pub fn date_ver(&mut self) -> DATE_VER_W<DATE_VER_SPEC> {
        DATE_VER_W::new(self, 0)
    }
}
#[doc = "TWAI FD date version\n\nYou can [`read`](crate::Reg::read) this register and get [`date_ver::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date_ver::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATE_VER_SPEC;
impl crate::RegisterSpec for DATE_VER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`date_ver::R`](R) reader structure"]
impl crate::Readable for DATE_VER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`date_ver::W`](W) writer structure"]
impl crate::Writable for DATE_VER_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DATE_VER to value 0x0231_2150"]
impl crate::Resettable for DATE_VER_SPEC {
    const RESET_VALUE: u32 = 0x0231_2150;
}
