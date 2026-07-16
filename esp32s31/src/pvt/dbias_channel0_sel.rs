#[doc = "Register `DBIAS_CHANNEL0_SEL` reader"]
pub type R = crate::R<DBIAS_CHANNEL0_SEL_SPEC>;
#[doc = "Register `DBIAS_CHANNEL0_SEL` writer"]
pub type W = crate::W<DBIAS_CHANNEL0_SEL_SPEC>;
#[doc = "Field `DBIAS_CHANNEL0_CFG` reader - needs field desc"]
pub type DBIAS_CHANNEL0_CFG_R = crate::FieldReader<u32>;
#[doc = "Field `DBIAS_CHANNEL0_CFG` writer - needs field desc"]
pub type DBIAS_CHANNEL0_CFG_W<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    #[doc = "Bits 0:16 - needs field desc"]
    #[inline(always)]
    pub fn dbias_channel0_cfg(&self) -> DBIAS_CHANNEL0_CFG_R {
        DBIAS_CHANNEL0_CFG_R::new(self.bits & 0x0001_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBIAS_CHANNEL0_SEL")
            .field("dbias_channel0_cfg", &self.dbias_channel0_cfg())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:16 - needs field desc"]
    #[inline(always)]
    pub fn dbias_channel0_cfg(&mut self) -> DBIAS_CHANNEL0_CFG_W<'_, DBIAS_CHANNEL0_SEL_SPEC> {
        DBIAS_CHANNEL0_CFG_W::new(self, 0)
    }
}
#[doc = "needs desc\n\nYou can [`read`](crate::Reg::read) this register and get [`dbias_channel0_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbias_channel0_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DBIAS_CHANNEL0_SEL_SPEC;
impl crate::RegisterSpec for DBIAS_CHANNEL0_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbias_channel0_sel::R`](R) reader structure"]
impl crate::Readable for DBIAS_CHANNEL0_SEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dbias_channel0_sel::W`](W) writer structure"]
impl crate::Writable for DBIAS_CHANNEL0_SEL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DBIAS_CHANNEL0_SEL to value 0"]
impl crate::Resettable for DBIAS_CHANNEL0_SEL_SPEC {}
