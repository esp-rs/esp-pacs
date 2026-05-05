#[doc = "Register `W0` reader"]
pub type R = crate::R<W0_SPEC>;
#[doc = "Register `W0` writer"]
pub type W = crate::W<W0_SPEC>;
#[doc = "Field `LP_REG_BUF0` reader - data buffer"]
pub type LP_REG_BUF0_R = crate::FieldReader<u32>;
#[doc = "Field `LP_REG_BUF0` writer - data buffer"]
pub type LP_REG_BUF0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - data buffer"]
    #[inline(always)]
    pub fn lp_reg_buf0(&self) -> LP_REG_BUF0_R {
        LP_REG_BUF0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("W0")
            .field("lp_reg_buf0", &self.lp_reg_buf0())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - data buffer"]
    #[inline(always)]
    pub fn lp_reg_buf0(&mut self) -> LP_REG_BUF0_W<'_, W0_SPEC> {
        LP_REG_BUF0_W::new(self, 0)
    }
}
#[doc = "SPI CPU-controlled buffer0\n\nYou can [`read`](crate::Reg::read) this register and get [`w0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`w0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct W0_SPEC;
impl crate::RegisterSpec for W0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`w0::R`](R) reader structure"]
impl crate::Readable for W0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`w0::W`](W) writer structure"]
impl crate::Writable for W0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets W0 to value 0"]
impl crate::Resettable for W0_SPEC {}
