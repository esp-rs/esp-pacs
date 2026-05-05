#[doc = "Register `W10` reader"]
pub type R = crate::R<W10_SPEC>;
#[doc = "Register `W10` writer"]
pub type W = crate::W<W10_SPEC>;
#[doc = "Field `LP_REG_BUF10` reader - data buffer"]
pub type LP_REG_BUF10_R = crate::FieldReader<u32>;
#[doc = "Field `LP_REG_BUF10` writer - data buffer"]
pub type LP_REG_BUF10_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - data buffer"]
    #[inline(always)]
    pub fn lp_reg_buf10(&self) -> LP_REG_BUF10_R {
        LP_REG_BUF10_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("W10")
            .field("lp_reg_buf10", &self.lp_reg_buf10())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - data buffer"]
    #[inline(always)]
    pub fn lp_reg_buf10(&mut self) -> LP_REG_BUF10_W<'_, W10_SPEC> {
        LP_REG_BUF10_W::new(self, 0)
    }
}
#[doc = "SPI CPU-controlled buffer10\n\nYou can [`read`](crate::Reg::read) this register and get [`w10::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`w10::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct W10_SPEC;
impl crate::RegisterSpec for W10_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`w10::R`](R) reader structure"]
impl crate::Readable for W10_SPEC {}
#[doc = "`write(|w| ..)` method takes [`w10::W`](W) writer structure"]
impl crate::Writable for W10_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets W10 to value 0"]
impl crate::Resettable for W10_SPEC {}
