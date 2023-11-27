#[doc = "Register `DBI_CFG` reader"]
pub type R = crate::R<DBI_CFG_SPEC>;
#[doc = "Register `DBI_CFG` writer"]
pub type W = crate::W<DBI_CFG_SPEC>;
#[doc = "Field `IN_DBI_CONF` reader - NA"]
pub type IN_DBI_CONF_R = crate::FieldReader;
#[doc = "Field `IN_DBI_CONF` writer - NA"]
pub type IN_DBI_CONF_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `OUT_DBI_CONF` reader - NA"]
pub type OUT_DBI_CONF_R = crate::FieldReader;
#[doc = "Field `OUT_DBI_CONF` writer - NA"]
pub type OUT_DBI_CONF_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `LUT_SIZE_CONF` reader - NA"]
pub type LUT_SIZE_CONF_R = crate::FieldReader;
#[doc = "Field `LUT_SIZE_CONF` writer - NA"]
pub type LUT_SIZE_CONF_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:3 - NA"]
    #[inline(always)]
    pub fn in_dbi_conf(&self) -> IN_DBI_CONF_R {
        IN_DBI_CONF_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - NA"]
    #[inline(always)]
    pub fn out_dbi_conf(&self) -> OUT_DBI_CONF_R {
        OUT_DBI_CONF_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - NA"]
    #[inline(always)]
    pub fn lut_size_conf(&self) -> LUT_SIZE_CONF_R {
        LUT_SIZE_CONF_R::new(((self.bits >> 16) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DBI_CFG")
            .field(
                "in_dbi_conf",
                &format_args!("{}", self.in_dbi_conf().bits()),
            )
            .field(
                "out_dbi_conf",
                &format_args!("{}", self.out_dbi_conf().bits()),
            )
            .field(
                "lut_size_conf",
                &format_args!("{}", self.lut_size_conf().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DBI_CFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:3 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn in_dbi_conf(&mut self) -> IN_DBI_CONF_W<DBI_CFG_SPEC> {
        IN_DBI_CONF_W::new(self, 0)
    }
    #[doc = "Bits 8:11 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn out_dbi_conf(&mut self) -> OUT_DBI_CONF_W<DBI_CFG_SPEC> {
        OUT_DBI_CONF_W::new(self, 8)
    }
    #[doc = "Bits 16:17 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn lut_size_conf(&mut self) -> LUT_SIZE_CONF_W<DBI_CFG_SPEC> {
        LUT_SIZE_CONF_W::new(self, 16)
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
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbi_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbi_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DBI_CFG_SPEC;
impl crate::RegisterSpec for DBI_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbi_cfg::R`](R) reader structure"]
impl crate::Readable for DBI_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dbi_cfg::W`](W) writer structure"]
impl crate::Writable for DBI_CFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DBI_CFG to value 0"]
impl crate::Resettable for DBI_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
