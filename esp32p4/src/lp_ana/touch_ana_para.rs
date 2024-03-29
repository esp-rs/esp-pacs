#[doc = "Register `TOUCH_ANA_PARA` reader"]
pub type R = crate::R<TOUCH_ANA_PARA_SPEC>;
#[doc = "Register `TOUCH_ANA_PARA` writer"]
pub type W = crate::W<TOUCH_ANA_PARA_SPEC>;
#[doc = "Field `TOUCH_TOUCH_BUF_DRV` reader - need_des"]
pub type TOUCH_TOUCH_BUF_DRV_R = crate::FieldReader;
#[doc = "Field `TOUCH_TOUCH_BUF_DRV` writer - need_des"]
pub type TOUCH_TOUCH_BUF_DRV_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TOUCH_TOUCH_EN_CAL` reader - need_des"]
pub type TOUCH_TOUCH_EN_CAL_R = crate::BitReader;
#[doc = "Field `TOUCH_TOUCH_EN_CAL` writer - need_des"]
pub type TOUCH_TOUCH_EN_CAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOUCH_TOUCH_DCAP_CAL` reader - need_des"]
pub type TOUCH_TOUCH_DCAP_CAL_R = crate::FieldReader;
#[doc = "Field `TOUCH_TOUCH_DCAP_CAL` writer - need_des"]
pub type TOUCH_TOUCH_DCAP_CAL_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:2 - need_des"]
    #[inline(always)]
    pub fn touch_touch_buf_drv(&self) -> TOUCH_TOUCH_BUF_DRV_R {
        TOUCH_TOUCH_BUF_DRV_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - need_des"]
    #[inline(always)]
    pub fn touch_touch_en_cal(&self) -> TOUCH_TOUCH_EN_CAL_R {
        TOUCH_TOUCH_EN_CAL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:10 - need_des"]
    #[inline(always)]
    pub fn touch_touch_dcap_cal(&self) -> TOUCH_TOUCH_DCAP_CAL_R {
        TOUCH_TOUCH_DCAP_CAL_R::new(((self.bits >> 4) & 0x7f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TOUCH_ANA_PARA")
            .field(
                "touch_touch_buf_drv",
                &format_args!("{}", self.touch_touch_buf_drv().bits()),
            )
            .field(
                "touch_touch_en_cal",
                &format_args!("{}", self.touch_touch_en_cal().bit()),
            )
            .field(
                "touch_touch_dcap_cal",
                &format_args!("{}", self.touch_touch_dcap_cal().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TOUCH_ANA_PARA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:2 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn touch_touch_buf_drv(&mut self) -> TOUCH_TOUCH_BUF_DRV_W<TOUCH_ANA_PARA_SPEC> {
        TOUCH_TOUCH_BUF_DRV_W::new(self, 0)
    }
    #[doc = "Bit 3 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn touch_touch_en_cal(&mut self) -> TOUCH_TOUCH_EN_CAL_W<TOUCH_ANA_PARA_SPEC> {
        TOUCH_TOUCH_EN_CAL_W::new(self, 3)
    }
    #[doc = "Bits 4:10 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn touch_touch_dcap_cal(&mut self) -> TOUCH_TOUCH_DCAP_CAL_W<TOUCH_ANA_PARA_SPEC> {
        TOUCH_TOUCH_DCAP_CAL_W::new(self, 4)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`touch_ana_para::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_ana_para::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TOUCH_ANA_PARA_SPEC;
impl crate::RegisterSpec for TOUCH_ANA_PARA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`touch_ana_para::R`](R) reader structure"]
impl crate::Readable for TOUCH_ANA_PARA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`touch_ana_para::W`](W) writer structure"]
impl crate::Writable for TOUCH_ANA_PARA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TOUCH_ANA_PARA to value 0"]
impl crate::Resettable for TOUCH_ANA_PARA_SPEC {
    const RESET_VALUE: u32 = 0;
}
