#[doc = "Register `DBIAS_ACTIVE_TIME` reader"]
pub type R = crate::R<DBIAS_ACTIVE_TIME_SPEC>;
#[doc = "Register `DBIAS_ACTIVE_TIME` writer"]
pub type W = crate::W<DBIAS_ACTIVE_TIME_SPEC>;
#[doc = "Field `DBIAS_ACTIVE_TIME` reader - needs field desc"]
pub type DBIAS_ACTIVE_TIME_R = crate::FieldReader<u32>;
#[doc = "Field `DBIAS_ACTIVE_TIME` writer - needs field desc"]
pub type DBIAS_ACTIVE_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - needs field desc"]
    #[inline(always)]
    pub fn dbias_active_time(&self) -> DBIAS_ACTIVE_TIME_R {
        DBIAS_ACTIVE_TIME_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBIAS_ACTIVE_TIME")
            .field("dbias_active_time", &self.dbias_active_time())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - needs field desc"]
    #[inline(always)]
    pub fn dbias_active_time(&mut self) -> DBIAS_ACTIVE_TIME_W<'_, DBIAS_ACTIVE_TIME_SPEC> {
        DBIAS_ACTIVE_TIME_W::new(self, 0)
    }
}
#[doc = "needs field desc\n\nYou can [`read`](crate::Reg::read) this register and get [`dbias_active_time::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbias_active_time::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DBIAS_ACTIVE_TIME_SPEC;
impl crate::RegisterSpec for DBIAS_ACTIVE_TIME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbias_active_time::R`](R) reader structure"]
impl crate::Readable for DBIAS_ACTIVE_TIME_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dbias_active_time::W`](W) writer structure"]
impl crate::Writable for DBIAS_ACTIVE_TIME_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DBIAS_ACTIVE_TIME to value 0xffff"]
impl crate::Resettable for DBIAS_ACTIVE_TIME_SPEC {
    const RESET_VALUE: u32 = 0xffff;
}
