#[doc = "Register `MEM_CONF2` reader"]
pub type R = crate::R<MEM_CONF2_SPEC>;
#[doc = "Register `MEM_CONF2` writer"]
pub type W = crate::W<MEM_CONF2_SPEC>;
#[doc = "Field `PWR_MEM_MODE_SEL` reader - "]
pub type PWR_MEM_MODE_SEL_R = crate::FieldReader;
#[doc = "Field `PWR_MEM_MODE_SEL` writer - "]
pub type PWR_MEM_MODE_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn pwr_mem_mode_sel(&self) -> PWR_MEM_MODE_SEL_R {
        PWR_MEM_MODE_SEL_R::new((self.bits & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEM_CONF2")
            .field("pwr_mem_mode_sel", &self.pwr_mem_mode_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn pwr_mem_mode_sel(&mut self) -> PWR_MEM_MODE_SEL_W<'_, MEM_CONF2_SPEC> {
        PWR_MEM_MODE_SEL_W::new(self, 0)
    }
}
#[doc = "MEM_CONF2\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_conf2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_conf2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MEM_CONF2_SPEC;
impl crate::RegisterSpec for MEM_CONF2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_conf2::R`](R) reader structure"]
impl crate::Readable for MEM_CONF2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mem_conf2::W`](W) writer structure"]
impl crate::Writable for MEM_CONF2_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MEM_CONF2 to value 0x04"]
impl crate::Resettable for MEM_CONF2_SPEC {
    const RESET_VALUE: u32 = 0x04;
}
