#[doc = "Register `BUSTOEXTMEM_ENA` reader"]
pub type R = crate::R<BUSTOEXTMEM_ENA_SPEC>;
#[doc = "Register `BUSTOEXTMEM_ENA` writer"]
pub type W = crate::W<BUSTOEXTMEM_ENA_SPEC>;
#[doc = "Field `BUSTOEXTMEM_ENA` reader - Set this bit to enable bus to EDMA."]
pub type BUSTOEXTMEM_ENA_R = crate::BitReader;
#[doc = "Field `BUSTOEXTMEM_ENA` writer - Set this bit to enable bus to EDMA."]
pub type BUSTOEXTMEM_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set this bit to enable bus to EDMA."]
    #[inline(always)]
    pub fn bustoextmem_ena(&self) -> BUSTOEXTMEM_ENA_R {
        BUSTOEXTMEM_ENA_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BUSTOEXTMEM_ENA")
            .field(
                "bustoextmem_ena",
                &format_args!("{}", self.bustoextmem_ena().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BUSTOEXTMEM_ENA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to enable bus to EDMA."]
    #[inline(always)]
    #[must_use]
    pub fn bustoextmem_ena(&mut self) -> BUSTOEXTMEM_ENA_W<BUSTOEXTMEM_ENA_SPEC> {
        BUSTOEXTMEM_ENA_W::new(self, 0)
    }
}
#[doc = "EDMA enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bustoextmem_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bustoextmem_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BUSTOEXTMEM_ENA_SPEC;
impl crate::RegisterSpec for BUSTOEXTMEM_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bustoextmem_ena::R`](R) reader structure"]
impl crate::Readable for BUSTOEXTMEM_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bustoextmem_ena::W`](W) writer structure"]
impl crate::Writable for BUSTOEXTMEM_ENA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BUSTOEXTMEM_ENA to value 0x01"]
impl crate::Resettable for BUSTOEXTMEM_ENA_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
