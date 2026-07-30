#[doc = "Register `DCM_BOOST_CTRL` reader"]
pub type R = crate::R<DCM_BOOST_CTRL_SPEC>;
#[doc = "Register `DCM_BOOST_CTRL` writer"]
pub type W = crate::W<DCM_BOOST_CTRL_SPEC>;
#[doc = "Field `DCDC_BOOST_CCM_CTRLEN` reader - need_des"]
pub type DCDC_BOOST_CCM_CTRLEN_R = crate::BitReader;
#[doc = "Field `DCDC_BOOST_CCM_CTRLEN` writer - need_des"]
pub type DCDC_BOOST_CCM_CTRLEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCDC_BOOST_CCM_ENB` reader - need_des"]
pub type DCDC_BOOST_CCM_ENB_R = crate::BitReader;
#[doc = "Field `DCDC_BOOST_CCM_ENB` writer - need_des"]
pub type DCDC_BOOST_CCM_ENB_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCDC_BOOST_EN` reader - need_des"]
pub type DCDC_BOOST_EN_R = crate::BitReader;
#[doc = "Field `DCDC_BOOST_EN` writer - need_des"]
pub type DCDC_BOOST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCDC_BOOST_DREG` reader - need_des"]
pub type DCDC_BOOST_DREG_R = crate::FieldReader;
#[doc = "Field `DCDC_BOOST_DREG` writer - need_des"]
pub type DCDC_BOOST_DREG_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bit 24 - need_des"]
    #[inline(always)]
    pub fn dcdc_boost_ccm_ctrlen(&self) -> DCDC_BOOST_CCM_CTRLEN_R {
        DCDC_BOOST_CCM_CTRLEN_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - need_des"]
    #[inline(always)]
    pub fn dcdc_boost_ccm_enb(&self) -> DCDC_BOOST_CCM_ENB_R {
        DCDC_BOOST_CCM_ENB_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - need_des"]
    #[inline(always)]
    pub fn dcdc_boost_en(&self) -> DCDC_BOOST_EN_R {
        DCDC_BOOST_EN_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:31 - need_des"]
    #[inline(always)]
    pub fn dcdc_boost_dreg(&self) -> DCDC_BOOST_DREG_R {
        DCDC_BOOST_DREG_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCM_BOOST_CTRL")
            .field("dcdc_boost_ccm_ctrlen", &self.dcdc_boost_ccm_ctrlen())
            .field("dcdc_boost_ccm_enb", &self.dcdc_boost_ccm_enb())
            .field("dcdc_boost_en", &self.dcdc_boost_en())
            .field("dcdc_boost_dreg", &self.dcdc_boost_dreg())
            .finish()
    }
}
impl W {
    #[doc = "Bit 24 - need_des"]
    #[inline(always)]
    pub fn dcdc_boost_ccm_ctrlen(&mut self) -> DCDC_BOOST_CCM_CTRLEN_W<'_, DCM_BOOST_CTRL_SPEC> {
        DCDC_BOOST_CCM_CTRLEN_W::new(self, 24)
    }
    #[doc = "Bit 25 - need_des"]
    #[inline(always)]
    pub fn dcdc_boost_ccm_enb(&mut self) -> DCDC_BOOST_CCM_ENB_W<'_, DCM_BOOST_CTRL_SPEC> {
        DCDC_BOOST_CCM_ENB_W::new(self, 25)
    }
    #[doc = "Bit 26 - need_des"]
    #[inline(always)]
    pub fn dcdc_boost_en(&mut self) -> DCDC_BOOST_EN_W<'_, DCM_BOOST_CTRL_SPEC> {
        DCDC_BOOST_EN_W::new(self, 26)
    }
    #[doc = "Bits 27:31 - need_des"]
    #[inline(always)]
    pub fn dcdc_boost_dreg(&mut self) -> DCDC_BOOST_DREG_W<'_, DCM_BOOST_CTRL_SPEC> {
        DCDC_BOOST_DREG_W::new(self, 27)
    }
}
#[doc = "DCM boost control register\n\nYou can [`read`](crate::Reg::read) this register and get [`dcm_boost_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcm_boost_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCM_BOOST_CTRL_SPEC;
impl crate::RegisterSpec for DCM_BOOST_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcm_boost_ctrl::R`](R) reader structure"]
impl crate::Readable for DCM_BOOST_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dcm_boost_ctrl::W`](W) writer structure"]
impl crate::Writable for DCM_BOOST_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DCM_BOOST_CTRL to value 0x3b80_0000"]
impl crate::Resettable for DCM_BOOST_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x3b80_0000;
}
