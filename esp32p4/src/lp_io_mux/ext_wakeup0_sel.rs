#[doc = "Register `EXT_WAKEUP0_SEL` reader"]
pub type R = crate::R<EXT_WAKEUP0_SEL_SPEC>;
#[doc = "Register `EXT_WAKEUP0_SEL` writer"]
pub type W = crate::W<EXT_WAKEUP0_SEL_SPEC>;
#[doc = "Field `REG_XTL_EXT_CTR_SEL` reader - select LP GPIO 0 ~ 15 to control XTAL"]
pub type REG_XTL_EXT_CTR_SEL_R = crate::FieldReader;
#[doc = "Field `REG_XTL_EXT_CTR_SEL` writer - select LP GPIO 0 ~ 15 to control XTAL"]
pub type REG_XTL_EXT_CTR_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `REG_EXT_WAKEUP0_SEL` reader - Reserved"]
pub type REG_EXT_WAKEUP0_SEL_R = crate::FieldReader;
#[doc = "Field `REG_EXT_WAKEUP0_SEL` writer - Reserved"]
pub type REG_EXT_WAKEUP0_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - select LP GPIO 0 ~ 15 to control XTAL"]
    #[inline(always)]
    pub fn reg_xtl_ext_ctr_sel(&self) -> REG_XTL_EXT_CTR_SEL_R {
        REG_XTL_EXT_CTR_SEL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - Reserved"]
    #[inline(always)]
    pub fn reg_ext_wakeup0_sel(&self) -> REG_EXT_WAKEUP0_SEL_R {
        REG_EXT_WAKEUP0_SEL_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXT_WAKEUP0_SEL")
            .field("reg_xtl_ext_ctr_sel", &self.reg_xtl_ext_ctr_sel())
            .field("reg_ext_wakeup0_sel", &self.reg_ext_wakeup0_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - select LP GPIO 0 ~ 15 to control XTAL"]
    #[inline(always)]
    #[must_use]
    pub fn reg_xtl_ext_ctr_sel(&mut self) -> REG_XTL_EXT_CTR_SEL_W<EXT_WAKEUP0_SEL_SPEC> {
        REG_XTL_EXT_CTR_SEL_W::new(self, 0)
    }
    #[doc = "Bits 5:9 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reg_ext_wakeup0_sel(&mut self) -> REG_EXT_WAKEUP0_SEL_W<EXT_WAKEUP0_SEL_SPEC> {
        REG_EXT_WAKEUP0_SEL_W::new(self, 5)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`ext_wakeup0_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ext_wakeup0_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXT_WAKEUP0_SEL_SPEC;
impl crate::RegisterSpec for EXT_WAKEUP0_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ext_wakeup0_sel::R`](R) reader structure"]
impl crate::Readable for EXT_WAKEUP0_SEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ext_wakeup0_sel::W`](W) writer structure"]
impl crate::Writable for EXT_WAKEUP0_SEL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXT_WAKEUP0_SEL to value 0"]
impl crate::Resettable for EXT_WAKEUP0_SEL_SPEC {
    const RESET_VALUE: u32 = 0;
}
