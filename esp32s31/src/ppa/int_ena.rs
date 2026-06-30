#[doc = "Register `INT_ENA` reader"]
pub type R = crate::R<INT_ENA_SPEC>;
#[doc = "Register `INT_ENA` writer"]
pub type W = crate::W<INT_ENA_SPEC>;
#[doc = "Field `SRM_EOF_INT_ENA` reader - The interrupt enable bit for the PPA_SRM_EOF_INT interrupt."]
pub type SRM_EOF_INT_ENA_R = crate::BitReader;
#[doc = "Field `SRM_EOF_INT_ENA` writer - The interrupt enable bit for the PPA_SRM_EOF_INT interrupt."]
pub type SRM_EOF_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLEND_EOF_INT_ENA` reader - The interrupt enable bit for the PPA_BLEND_EOF_INT interrupt."]
pub type BLEND_EOF_INT_ENA_R = crate::BitReader;
#[doc = "Field `BLEND_EOF_INT_ENA` writer - The interrupt enable bit for the PPA_BLEND_EOF_INT interrupt."]
pub type BLEND_EOF_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRM_PARAM_CFG_ERR_INT_ENA` reader - The interrupt enable bit for the PPA_SRM_PARAM_CFG_ERR_INT interrupt."]
pub type SRM_PARAM_CFG_ERR_INT_ENA_R = crate::BitReader;
#[doc = "Field `SRM_PARAM_CFG_ERR_INT_ENA` writer - The interrupt enable bit for the PPA_SRM_PARAM_CFG_ERR_INT interrupt."]
pub type SRM_PARAM_CFG_ERR_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLEND_PARAM_CFG_ERR_INT_ENA` reader - The interrupt enable bit for the PPA_BLEND_PARAM_CFG_ERR_INT interrupt."]
pub type BLEND_PARAM_CFG_ERR_INT_ENA_R = crate::BitReader;
#[doc = "Field `BLEND_PARAM_CFG_ERR_INT_ENA` writer - The interrupt enable bit for the PPA_BLEND_PARAM_CFG_ERR_INT interrupt."]
pub type BLEND_PARAM_CFG_ERR_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The interrupt enable bit for the PPA_SRM_EOF_INT interrupt."]
    #[inline(always)]
    pub fn srm_eof_int_ena(&self) -> SRM_EOF_INT_ENA_R {
        SRM_EOF_INT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The interrupt enable bit for the PPA_BLEND_EOF_INT interrupt."]
    #[inline(always)]
    pub fn blend_eof_int_ena(&self) -> BLEND_EOF_INT_ENA_R {
        BLEND_EOF_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The interrupt enable bit for the PPA_SRM_PARAM_CFG_ERR_INT interrupt."]
    #[inline(always)]
    pub fn srm_param_cfg_err_int_ena(&self) -> SRM_PARAM_CFG_ERR_INT_ENA_R {
        SRM_PARAM_CFG_ERR_INT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The interrupt enable bit for the PPA_BLEND_PARAM_CFG_ERR_INT interrupt."]
    #[inline(always)]
    pub fn blend_param_cfg_err_int_ena(&self) -> BLEND_PARAM_CFG_ERR_INT_ENA_R {
        BLEND_PARAM_CFG_ERR_INT_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENA")
            .field("srm_eof_int_ena", &self.srm_eof_int_ena())
            .field("blend_eof_int_ena", &self.blend_eof_int_ena())
            .field(
                "srm_param_cfg_err_int_ena",
                &self.srm_param_cfg_err_int_ena(),
            )
            .field(
                "blend_param_cfg_err_int_ena",
                &self.blend_param_cfg_err_int_ena(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - The interrupt enable bit for the PPA_SRM_EOF_INT interrupt."]
    #[inline(always)]
    pub fn srm_eof_int_ena(&mut self) -> SRM_EOF_INT_ENA_W<'_, INT_ENA_SPEC> {
        SRM_EOF_INT_ENA_W::new(self, 0)
    }
    #[doc = "Bit 1 - The interrupt enable bit for the PPA_BLEND_EOF_INT interrupt."]
    #[inline(always)]
    pub fn blend_eof_int_ena(&mut self) -> BLEND_EOF_INT_ENA_W<'_, INT_ENA_SPEC> {
        BLEND_EOF_INT_ENA_W::new(self, 1)
    }
    #[doc = "Bit 2 - The interrupt enable bit for the PPA_SRM_PARAM_CFG_ERR_INT interrupt."]
    #[inline(always)]
    pub fn srm_param_cfg_err_int_ena(&mut self) -> SRM_PARAM_CFG_ERR_INT_ENA_W<'_, INT_ENA_SPEC> {
        SRM_PARAM_CFG_ERR_INT_ENA_W::new(self, 2)
    }
    #[doc = "Bit 3 - The interrupt enable bit for the PPA_BLEND_PARAM_CFG_ERR_INT interrupt."]
    #[inline(always)]
    pub fn blend_param_cfg_err_int_ena(
        &mut self,
    ) -> BLEND_PARAM_CFG_ERR_INT_ENA_W<'_, INT_ENA_SPEC> {
        BLEND_PARAM_CFG_ERR_INT_ENA_W::new(self, 3)
    }
}
#[doc = "Interrupt enable bits\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_ena::R`](R) reader structure"]
impl crate::Readable for INT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_ena::W`](W) writer structure"]
impl crate::Writable for INT_ENA_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_ENA to value 0"]
impl crate::Resettable for INT_ENA_SPEC {}
