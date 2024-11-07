#[doc = "Register `SCL_HIGH` reader"]
pub type R = crate::R<SCL_HIGH_SPEC>;
#[doc = "Register `SCL_HIGH` writer"]
pub type W = crate::W<SCL_HIGH_SPEC>;
#[doc = "Field `PERIOD` reader - This register is used to configure how many cycles SCL remains high."]
pub type PERIOD_R = crate::FieldReader<u32>;
#[doc = "Field `PERIOD` writer - This register is used to configure how many cycles SCL remains high."]
pub type PERIOD_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - This register is used to configure how many cycles SCL remains high."]
    #[inline(always)]
    pub fn period(&self) -> PERIOD_R {
        PERIOD_R::new(self.bits & 0x000f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCL_HIGH")
            .field("period", &self.period())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:19 - This register is used to configure how many cycles SCL remains high."]
    #[inline(always)]
    pub fn period(&mut self) -> PERIOD_W<SCL_HIGH_SPEC> {
        PERIOD_W::new(self, 0)
    }
}
#[doc = "Configure the high level width of SCL\n\nYou can [`read`](crate::Reg::read) this register and get [`scl_high::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scl_high::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCL_HIGH_SPEC;
impl crate::RegisterSpec for SCL_HIGH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scl_high::R`](R) reader structure"]
impl crate::Readable for SCL_HIGH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scl_high::W`](W) writer structure"]
impl crate::Writable for SCL_HIGH_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCL_HIGH to value 0x0100"]
impl crate::Resettable for SCL_HIGH_SPEC {
    const RESET_VALUE: u32 = 0x0100;
}
