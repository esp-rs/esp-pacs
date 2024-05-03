#[doc = "Register `DBIAS_CMD3` reader"]
pub type R = crate::R<DBIAS_CMD3_SPEC>;
#[doc = "Register `DBIAS_CMD3` writer"]
pub type W = crate::W<DBIAS_CMD3_SPEC>;
#[doc = "Field `DBIAS_CMD3` reader - needs field desc"]
pub type DBIAS_CMD3_R = crate::FieldReader<u32>;
#[doc = "Field `DBIAS_CMD3` writer - needs field desc"]
pub type DBIAS_CMD3_W<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    #[doc = "Bits 0:16 - needs field desc"]
    #[inline(always)]
    pub fn dbias_cmd3(&self) -> DBIAS_CMD3_R {
        DBIAS_CMD3_R::new(self.bits & 0x0001_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBIAS_CMD3")
            .field("dbias_cmd3", &self.dbias_cmd3().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DBIAS_CMD3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:16 - needs field desc"]
    #[inline(always)]
    #[must_use]
    pub fn dbias_cmd3(&mut self) -> DBIAS_CMD3_W<DBIAS_CMD3_SPEC> {
        DBIAS_CMD3_W::new(self, 0)
    }
}
#[doc = "needs desc\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbias_cmd3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbias_cmd3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DBIAS_CMD3_SPEC;
impl crate::RegisterSpec for DBIAS_CMD3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbias_cmd3::R`](R) reader structure"]
impl crate::Readable for DBIAS_CMD3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dbias_cmd3::W`](W) writer structure"]
impl crate::Writable for DBIAS_CMD3_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DBIAS_CMD3 to value 0"]
impl crate::Resettable for DBIAS_CMD3_SPEC {
    const RESET_VALUE: u32 = 0;
}
