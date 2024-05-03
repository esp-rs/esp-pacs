#[doc = "Register `DPC_CTRL` reader"]
pub type R = crate::R<DPC_CTRL_SPEC>;
#[doc = "Register `DPC_CTRL` writer"]
pub type W = crate::W<DPC_CTRL_SPEC>;
#[doc = "Field `DPC_CHECK_EN` reader - this bit configures the check mode enable. 0: disable, 1: enable"]
pub type DPC_CHECK_EN_R = crate::BitReader;
#[doc = "Field `DPC_CHECK_EN` writer - this bit configures the check mode enable. 0: disable, 1: enable"]
pub type DPC_CHECK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STA_EN` reader - this bit configures the sta dpc enable. 0: disable, 1: enable"]
pub type STA_EN_R = crate::BitReader;
#[doc = "Field `STA_EN` writer - this bit configures the sta dpc enable. 0: disable, 1: enable"]
pub type STA_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DYN_EN` reader - this bit configures the dyn dpc enable. 0: disable, 1: enable"]
pub type DYN_EN_R = crate::BitReader;
#[doc = "Field `DYN_EN` writer - this bit configures the dyn dpc enable. 0: disable, 1: enable"]
pub type DYN_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPC_BLACK_EN` reader - this bit configures input image type select when in check mode, 0: white img, 1: black img"]
pub type DPC_BLACK_EN_R = crate::BitReader;
#[doc = "Field `DPC_BLACK_EN` writer - this bit configures input image type select when in check mode, 0: white img, 1: black img"]
pub type DPC_BLACK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPC_METHOD_SEL` reader - this bit configures dyn dpc method select. 0: simple method, 1: hard method"]
pub type DPC_METHOD_SEL_R = crate::BitReader;
#[doc = "Field `DPC_METHOD_SEL` writer - this bit configures dyn dpc method select. 0: simple method, 1: hard method"]
pub type DPC_METHOD_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPC_CHECK_OD_EN` reader - this bit configures output pixel data when in check mode or not. 0: no data output, 1: data output"]
pub type DPC_CHECK_OD_EN_R = crate::BitReader;
#[doc = "Field `DPC_CHECK_OD_EN` writer - this bit configures output pixel data when in check mode or not. 0: no data output, 1: data output"]
pub type DPC_CHECK_OD_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - this bit configures the check mode enable. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn dpc_check_en(&self) -> DPC_CHECK_EN_R {
        DPC_CHECK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - this bit configures the sta dpc enable. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn sta_en(&self) -> STA_EN_R {
        STA_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - this bit configures the dyn dpc enable. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn dyn_en(&self) -> DYN_EN_R {
        DYN_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - this bit configures input image type select when in check mode, 0: white img, 1: black img"]
    #[inline(always)]
    pub fn dpc_black_en(&self) -> DPC_BLACK_EN_R {
        DPC_BLACK_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - this bit configures dyn dpc method select. 0: simple method, 1: hard method"]
    #[inline(always)]
    pub fn dpc_method_sel(&self) -> DPC_METHOD_SEL_R {
        DPC_METHOD_SEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - this bit configures output pixel data when in check mode or not. 0: no data output, 1: data output"]
    #[inline(always)]
    pub fn dpc_check_od_en(&self) -> DPC_CHECK_OD_EN_R {
        DPC_CHECK_OD_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DPC_CTRL")
            .field("dpc_check_en", &self.dpc_check_en().bit())
            .field("sta_en", &self.sta_en().bit())
            .field("dyn_en", &self.dyn_en().bit())
            .field("dpc_black_en", &self.dpc_black_en().bit())
            .field("dpc_method_sel", &self.dpc_method_sel().bit())
            .field("dpc_check_od_en", &self.dpc_check_od_en().bit())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DPC_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - this bit configures the check mode enable. 0: disable, 1: enable"]
    #[inline(always)]
    #[must_use]
    pub fn dpc_check_en(&mut self) -> DPC_CHECK_EN_W<DPC_CTRL_SPEC> {
        DPC_CHECK_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - this bit configures the sta dpc enable. 0: disable, 1: enable"]
    #[inline(always)]
    #[must_use]
    pub fn sta_en(&mut self) -> STA_EN_W<DPC_CTRL_SPEC> {
        STA_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - this bit configures the dyn dpc enable. 0: disable, 1: enable"]
    #[inline(always)]
    #[must_use]
    pub fn dyn_en(&mut self) -> DYN_EN_W<DPC_CTRL_SPEC> {
        DYN_EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - this bit configures input image type select when in check mode, 0: white img, 1: black img"]
    #[inline(always)]
    #[must_use]
    pub fn dpc_black_en(&mut self) -> DPC_BLACK_EN_W<DPC_CTRL_SPEC> {
        DPC_BLACK_EN_W::new(self, 3)
    }
    #[doc = "Bit 4 - this bit configures dyn dpc method select. 0: simple method, 1: hard method"]
    #[inline(always)]
    #[must_use]
    pub fn dpc_method_sel(&mut self) -> DPC_METHOD_SEL_W<DPC_CTRL_SPEC> {
        DPC_METHOD_SEL_W::new(self, 4)
    }
    #[doc = "Bit 5 - this bit configures output pixel data when in check mode or not. 0: no data output, 1: data output"]
    #[inline(always)]
    #[must_use]
    pub fn dpc_check_od_en(&mut self) -> DPC_CHECK_OD_EN_W<DPC_CTRL_SPEC> {
        DPC_CHECK_OD_EN_W::new(self, 5)
    }
}
#[doc = "DPC mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpc_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dpc_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DPC_CTRL_SPEC;
impl crate::RegisterSpec for DPC_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dpc_ctrl::R`](R) reader structure"]
impl crate::Readable for DPC_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dpc_ctrl::W`](W) writer structure"]
impl crate::Writable for DPC_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DPC_CTRL to value 0x04"]
impl crate::Resettable for DPC_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x04;
}
