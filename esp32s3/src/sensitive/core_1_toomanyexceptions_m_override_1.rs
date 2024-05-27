#[doc = "Register `CORE_1_TOOMANYEXCEPTIONS_M_OVERRIDE_1` reader"]
pub type R = crate::R<CORE_1_TOOMANYEXCEPTIONS_M_OVERRIDE_1_SPEC>;
#[doc = "Register `CORE_1_TOOMANYEXCEPTIONS_M_OVERRIDE_1` writer"]
pub type W = crate::W<CORE_1_TOOMANYEXCEPTIONS_M_OVERRIDE_1_SPEC>;
#[doc = "Field `CORE_1_TOOMANYEXCEPTIONS_M_OVERRIDE` reader - Set 1 to mask toomanyexception."]
pub type CORE_1_TOOMANYEXCEPTIONS_M_OVERRIDE_R = crate::BitReader;
#[doc = "Field `CORE_1_TOOMANYEXCEPTIONS_M_OVERRIDE` writer - Set 1 to mask toomanyexception."]
pub type CORE_1_TOOMANYEXCEPTIONS_M_OVERRIDE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set 1 to mask toomanyexception."]
    #[inline(always)]
    pub fn core_1_toomanyexceptions_m_override(&self) -> CORE_1_TOOMANYEXCEPTIONS_M_OVERRIDE_R {
        CORE_1_TOOMANYEXCEPTIONS_M_OVERRIDE_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_1_TOOMANYEXCEPTIONS_M_OVERRIDE_1")
            .field(
                "core_1_toomanyexceptions_m_override",
                &self.core_1_toomanyexceptions_m_override(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to mask toomanyexception."]
    #[inline(always)]
    #[must_use]
    pub fn core_1_toomanyexceptions_m_override(
        &mut self,
    ) -> CORE_1_TOOMANYEXCEPTIONS_M_OVERRIDE_W<CORE_1_TOOMANYEXCEPTIONS_M_OVERRIDE_1_SPEC> {
        CORE_1_TOOMANYEXCEPTIONS_M_OVERRIDE_W::new(self, 0)
    }
}
#[doc = "core1 toomanyexception override configuration register 1.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_1_toomanyexceptions_m_override_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_1_toomanyexceptions_m_override_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_1_TOOMANYEXCEPTIONS_M_OVERRIDE_1_SPEC;
impl crate::RegisterSpec for CORE_1_TOOMANYEXCEPTIONS_M_OVERRIDE_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_1_toomanyexceptions_m_override_1::R`](R) reader structure"]
impl crate::Readable for CORE_1_TOOMANYEXCEPTIONS_M_OVERRIDE_1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`core_1_toomanyexceptions_m_override_1::W`](W) writer structure"]
impl crate::Writable for CORE_1_TOOMANYEXCEPTIONS_M_OVERRIDE_1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CORE_1_TOOMANYEXCEPTIONS_M_OVERRIDE_1 to value 0x01"]
impl crate::Resettable for CORE_1_TOOMANYEXCEPTIONS_M_OVERRIDE_1_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
