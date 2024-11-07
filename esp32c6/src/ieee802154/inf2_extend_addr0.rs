#[doc = "Register `INF2_EXTEND_ADDR0` reader"]
pub type R = crate::R<INF2_EXTEND_ADDR0_SPEC>;
#[doc = "Register `INF2_EXTEND_ADDR0` writer"]
pub type W = crate::W<INF2_EXTEND_ADDR0_SPEC>;
#[doc = "Field `MAC_INF2_EXTEND_ADDR0` reader - "]
pub type MAC_INF2_EXTEND_ADDR0_R = crate::FieldReader<u32>;
#[doc = "Field `MAC_INF2_EXTEND_ADDR0` writer - "]
pub type MAC_INF2_EXTEND_ADDR0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn mac_inf2_extend_addr0(&self) -> MAC_INF2_EXTEND_ADDR0_R {
        MAC_INF2_EXTEND_ADDR0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INF2_EXTEND_ADDR0")
            .field("mac_inf2_extend_addr0", &self.mac_inf2_extend_addr0())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn mac_inf2_extend_addr0(&mut self) -> MAC_INF2_EXTEND_ADDR0_W<INF2_EXTEND_ADDR0_SPEC> {
        MAC_INF2_EXTEND_ADDR0_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`inf2_extend_addr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inf2_extend_addr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INF2_EXTEND_ADDR0_SPEC;
impl crate::RegisterSpec for INF2_EXTEND_ADDR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inf2_extend_addr0::R`](R) reader structure"]
impl crate::Readable for INF2_EXTEND_ADDR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`inf2_extend_addr0::W`](W) writer structure"]
impl crate::Writable for INF2_EXTEND_ADDR0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INF2_EXTEND_ADDR0 to value 0"]
impl crate::Resettable for INF2_EXTEND_ADDR0_SPEC {
    const RESET_VALUE: u32 = 0;
}
