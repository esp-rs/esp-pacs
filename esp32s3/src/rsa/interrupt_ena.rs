#[doc = "Register `INTERRUPT_ENA` reader"]
pub type R = crate::R<INTERRUPT_ENA_SPEC>;
#[doc = "Register `INTERRUPT_ENA` writer"]
pub type W = crate::W<INTERRUPT_ENA_SPEC>;
#[doc = "Field `INTERRUPT_ENA` reader - Set this bit to 1 to enable the RSA interrupt. This option is enabled by default."]
pub type INTERRUPT_ENA_R = crate::BitReader;
#[doc = "Field `INTERRUPT_ENA` writer - Set this bit to 1 to enable the RSA interrupt. This option is enabled by default."]
pub type INTERRUPT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set this bit to 1 to enable the RSA interrupt. This option is enabled by default."]
    #[inline(always)]
    pub fn interrupt_ena(&self) -> INTERRUPT_ENA_R {
        INTERRUPT_ENA_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTERRUPT_ENA")
            .field(
                "interrupt_ena",
                &format_args!("{}", self.interrupt_ena().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INTERRUPT_ENA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to 1 to enable the RSA interrupt. This option is enabled by default."]
    #[inline(always)]
    #[must_use]
    pub fn interrupt_ena(&mut self) -> INTERRUPT_ENA_W<INTERRUPT_ENA_SPEC> {
        INTERRUPT_ENA_W::new(self, 0)
    }
}
#[doc = "RSA interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`interrupt_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`interrupt_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTERRUPT_ENA_SPEC;
impl crate::RegisterSpec for INTERRUPT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`interrupt_ena::R`](R) reader structure"]
impl crate::Readable for INTERRUPT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`interrupt_ena::W`](W) writer structure"]
impl crate::Writable for INTERRUPT_ENA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTERRUPT_ENA to value 0"]
impl crate::Resettable for INTERRUPT_ENA_SPEC {
    const RESET_VALUE: u32 = 0;
}
