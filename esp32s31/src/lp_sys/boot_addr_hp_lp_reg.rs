#[doc = "Register `BOOT_ADDR_HP_LP_REG` reader"]
pub type R = crate::R<BOOT_ADDR_HP_LP_REG_SPEC>;
#[doc = "Register `BOOT_ADDR_HP_LP_REG` writer"]
pub type W = crate::W<BOOT_ADDR_HP_LP_REG_SPEC>;
#[doc = "Field `BOOT_ADDR_HP_LP` reader - "]
pub type BOOT_ADDR_HP_LP_R = crate::FieldReader<u32>;
#[doc = "Field `BOOT_ADDR_HP_LP` writer - "]
pub type BOOT_ADDR_HP_LP_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn boot_addr_hp_lp(&self) -> BOOT_ADDR_HP_LP_R {
        BOOT_ADDR_HP_LP_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BOOT_ADDR_HP_LP_REG")
            .field("boot_addr_hp_lp", &self.boot_addr_hp_lp())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn boot_addr_hp_lp(&mut self) -> BOOT_ADDR_HP_LP_W<'_, BOOT_ADDR_HP_LP_REG_SPEC> {
        BOOT_ADDR_HP_LP_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`boot_addr_hp_lp_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`boot_addr_hp_lp_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BOOT_ADDR_HP_LP_REG_SPEC;
impl crate::RegisterSpec for BOOT_ADDR_HP_LP_REG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`boot_addr_hp_lp_reg::R`](R) reader structure"]
impl crate::Readable for BOOT_ADDR_HP_LP_REG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`boot_addr_hp_lp_reg::W`](W) writer structure"]
impl crate::Writable for BOOT_ADDR_HP_LP_REG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BOOT_ADDR_HP_LP_REG to value 0"]
impl crate::Resettable for BOOT_ADDR_HP_LP_REG_SPEC {}
