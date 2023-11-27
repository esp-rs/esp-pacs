#[doc = "Register `INT_ENA` reader"]
pub type R = crate::R<INT_ENA_SPEC>;
#[doc = "Register `INT_ENA` writer"]
pub type W = crate::W<INT_ENA_SPEC>;
#[doc = "Field `UNDERRUN_INT_ENA` reader - write 1 to enables dpi_underrun_int_st field of MIPI_DSI_BRG_INT_ST_REG controlled by dpi_underrun interrupt signal"]
pub type UNDERRUN_INT_ENA_R = crate::BitReader;
#[doc = "Field `UNDERRUN_INT_ENA` writer - write 1 to enables dpi_underrun_int_st field of MIPI_DSI_BRG_INT_ST_REG controlled by dpi_underrun interrupt signal"]
pub type UNDERRUN_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - write 1 to enables dpi_underrun_int_st field of MIPI_DSI_BRG_INT_ST_REG controlled by dpi_underrun interrupt signal"]
    #[inline(always)]
    pub fn underrun_int_ena(&self) -> UNDERRUN_INT_ENA_R {
        UNDERRUN_INT_ENA_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENA")
            .field(
                "underrun_int_ena",
                &format_args!("{}", self.underrun_int_ena().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_ENA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - write 1 to enables dpi_underrun_int_st field of MIPI_DSI_BRG_INT_ST_REG controlled by dpi_underrun interrupt signal"]
    #[inline(always)]
    #[must_use]
    pub fn underrun_int_ena(&mut self) -> UNDERRUN_INT_ENA_W<INT_ENA_SPEC> {
        UNDERRUN_INT_ENA_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "dsi_bridge interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_ena::R`](R) reader structure"]
impl crate::Readable for INT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_ena::W`](W) writer structure"]
impl crate::Writable for INT_ENA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_ENA to value 0"]
impl crate::Resettable for INT_ENA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
