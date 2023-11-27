#[doc = "Register `INT_ENA` reader"]
pub type R = crate::R<INT_ENA_SPEC>;
#[doc = "Register `INT_ENA` writer"]
pub type W = crate::W<INT_ENA_SPEC>;
#[doc = "Field `SR_EOF_INT_ENA` reader - The interrupt enable bit for the PPA_SR_EOF_INT interrupt."]
pub type SR_EOF_INT_ENA_R = crate::BitReader;
#[doc = "Field `SR_EOF_INT_ENA` writer - The interrupt enable bit for the PPA_SR_EOF_INT interrupt."]
pub type SR_EOF_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLEND_EOF_INT_ENA` reader - The interrupt enable bit for the PPA_BLEND_EOF_INT interrupt."]
pub type BLEND_EOF_INT_ENA_R = crate::BitReader;
#[doc = "Field `BLEND_EOF_INT_ENA` writer - The interrupt enable bit for the PPA_BLEND_EOF_INT interrupt."]
pub type BLEND_EOF_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SR_PARAM_CFG_ERR_INT_ENA` reader - The interrupt enable bit for the PPA_SR_RX_YSCAL_ERR_INT interrupt."]
pub type SR_PARAM_CFG_ERR_INT_ENA_R = crate::BitReader;
#[doc = "Field `SR_PARAM_CFG_ERR_INT_ENA` writer - The interrupt enable bit for the PPA_SR_RX_YSCAL_ERR_INT interrupt."]
pub type SR_PARAM_CFG_ERR_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The interrupt enable bit for the PPA_SR_EOF_INT interrupt."]
    #[inline(always)]
    pub fn sr_eof_int_ena(&self) -> SR_EOF_INT_ENA_R {
        SR_EOF_INT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The interrupt enable bit for the PPA_BLEND_EOF_INT interrupt."]
    #[inline(always)]
    pub fn blend_eof_int_ena(&self) -> BLEND_EOF_INT_ENA_R {
        BLEND_EOF_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The interrupt enable bit for the PPA_SR_RX_YSCAL_ERR_INT interrupt."]
    #[inline(always)]
    pub fn sr_param_cfg_err_int_ena(&self) -> SR_PARAM_CFG_ERR_INT_ENA_R {
        SR_PARAM_CFG_ERR_INT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENA")
            .field(
                "sr_eof_int_ena",
                &format_args!("{}", self.sr_eof_int_ena().bit()),
            )
            .field(
                "blend_eof_int_ena",
                &format_args!("{}", self.blend_eof_int_ena().bit()),
            )
            .field(
                "sr_param_cfg_err_int_ena",
                &format_args!("{}", self.sr_param_cfg_err_int_ena().bit()),
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
    #[doc = "Bit 0 - The interrupt enable bit for the PPA_SR_EOF_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn sr_eof_int_ena(&mut self) -> SR_EOF_INT_ENA_W<INT_ENA_SPEC> {
        SR_EOF_INT_ENA_W::new(self, 0)
    }
    #[doc = "Bit 1 - The interrupt enable bit for the PPA_BLEND_EOF_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn blend_eof_int_ena(&mut self) -> BLEND_EOF_INT_ENA_W<INT_ENA_SPEC> {
        BLEND_EOF_INT_ENA_W::new(self, 1)
    }
    #[doc = "Bit 2 - The interrupt enable bit for the PPA_SR_RX_YSCAL_ERR_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn sr_param_cfg_err_int_ena(&mut self) -> SR_PARAM_CFG_ERR_INT_ENA_W<INT_ENA_SPEC> {
        SR_PARAM_CFG_ERR_INT_ENA_W::new(self, 2)
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
#[doc = "Interrupt enable bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
