#[doc = "Register `SCL_LOW` reader"]
pub type R = crate::R<SCL_LOW_SPEC>;
#[doc = "Register `SCL_LOW` writer"]
pub type W = crate::W<SCL_LOW_SPEC>;
#[doc = "Field `PERIOD` reader - This register is used to configure how many clock cycles SCL remains low."]
pub type PERIOD_R = crate::FieldReader<u32>;
#[doc = "Field `PERIOD` writer - This register is used to configure how many clock cycles SCL remains low."]
pub type PERIOD_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - This register is used to configure how many clock cycles SCL remains low."]
    #[inline(always)]
    pub fn period(&self) -> PERIOD_R {
        PERIOD_R::new(self.bits & 0x000f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCL_LOW")
            .field("period", &self.period())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:19 - This register is used to configure how many clock cycles SCL remains low."]
    #[inline(always)]
    pub fn period(&mut self) -> PERIOD_W<SCL_LOW_SPEC> {
        PERIOD_W::new(self, 0)
    }
}
#[doc = "Configure the low level width of SCL\n\nYou can [`read`](crate::Reg::read) this register and get [`scl_low::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scl_low::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCL_LOW_SPEC;
impl crate::RegisterSpec for SCL_LOW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scl_low::R`](R) reader structure"]
impl crate::Readable for SCL_LOW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scl_low::W`](W) writer structure"]
impl crate::Writable for SCL_LOW_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCL_LOW to value 0x0100"]
impl crate::Resettable for SCL_LOW_SPEC {
    const RESET_VALUE: u32 = 0x0100;
}
