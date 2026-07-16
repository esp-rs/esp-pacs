#[doc = "Register `INTR_HP2LP_CONF_0` reader"]
pub type R = crate::R<INTR_HP2LP_CONF_0_SPEC>;
#[doc = "Register `INTR_HP2LP_CONF_0` writer"]
pub type W = crate::W<INTR_HP2LP_CONF_0_SPEC>;
#[doc = "Field `INTR_HP2LP_EN_0` reader - reserved"]
pub type INTR_HP2LP_EN_0_R = crate::FieldReader<u32>;
#[doc = "Field `INTR_HP2LP_EN_0` writer - reserved"]
pub type INTR_HP2LP_EN_0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - reserved"]
    #[inline(always)]
    pub fn intr_hp2lp_en_0(&self) -> INTR_HP2LP_EN_0_R {
        INTR_HP2LP_EN_0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTR_HP2LP_CONF_0")
            .field("intr_hp2lp_en_0", &self.intr_hp2lp_en_0())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - reserved"]
    #[inline(always)]
    pub fn intr_hp2lp_en_0(&mut self) -> INTR_HP2LP_EN_0_W<'_, INTR_HP2LP_CONF_0_SPEC> {
        INTR_HP2LP_EN_0_W::new(self, 0)
    }
}
#[doc = "intr hp2lp configuration register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`intr_hp2lp_conf_0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intr_hp2lp_conf_0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTR_HP2LP_CONF_0_SPEC;
impl crate::RegisterSpec for INTR_HP2LP_CONF_0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_hp2lp_conf_0::R`](R) reader structure"]
impl crate::Readable for INTR_HP2LP_CONF_0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`intr_hp2lp_conf_0::W`](W) writer structure"]
impl crate::Writable for INTR_HP2LP_CONF_0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTR_HP2LP_CONF_0 to value 0xffff_ffff"]
impl crate::Resettable for INTR_HP2LP_CONF_0_SPEC {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
