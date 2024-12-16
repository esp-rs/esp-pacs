#[doc = "Register `PG_GLITCH_CNTL` reader"]
pub type R = crate::R<PG_GLITCH_CNTL_SPEC>;
#[doc = "Register `PG_GLITCH_CNTL` writer"]
pub type W = crate::W<PG_GLITCH_CNTL_SPEC>;
#[doc = "Field `POWER_GLITCH_RESET_ENA` reader - need_des"]
pub type POWER_GLITCH_RESET_ENA_R = crate::BitReader;
#[doc = "Field `POWER_GLITCH_RESET_ENA` writer - need_des"]
pub type POWER_GLITCH_RESET_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn power_glitch_reset_ena(&self) -> POWER_GLITCH_RESET_ENA_R {
        POWER_GLITCH_RESET_ENA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PG_GLITCH_CNTL")
            .field("power_glitch_reset_ena", &self.power_glitch_reset_ena())
            .finish()
    }
}
impl W {
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn power_glitch_reset_ena(&mut self) -> POWER_GLITCH_RESET_ENA_W<PG_GLITCH_CNTL_SPEC> {
        POWER_GLITCH_RESET_ENA_W::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`pg_glitch_cntl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pg_glitch_cntl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PG_GLITCH_CNTL_SPEC;
impl crate::RegisterSpec for PG_GLITCH_CNTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pg_glitch_cntl::R`](R) reader structure"]
impl crate::Readable for PG_GLITCH_CNTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pg_glitch_cntl::W`](W) writer structure"]
impl crate::Writable for PG_GLITCH_CNTL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PG_GLITCH_CNTL to value 0"]
impl crate::Resettable for PG_GLITCH_CNTL_SPEC {
    const RESET_VALUE: u32 = 0;
}
