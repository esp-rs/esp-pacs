#[doc = "Register `DCM_CTRL` reader"]
pub type R = crate::R<DCM_CTRL_SPEC>;
#[doc = "Register `DCM_CTRL` writer"]
pub type W = crate::W<DCM_CTRL_SPEC>;
#[doc = "Field `DCDC_ON_REQ` writer - SW trigger dcdc on"]
pub type DCDC_ON_REQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCDC_OFF_REQ` writer - SW trigger dcdc off"]
pub type DCDC_OFF_REQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCDC_LIGHTSLP_REQ` writer - SW trigger dcdc enter lightsleep"]
pub type DCDC_LIGHTSLP_REQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCDC_DEEPSLP_REQ` writer - SW trigger dcdc enter deepsleep"]
pub type DCDC_DEEPSLP_REQ_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCDC_DONE_FORCE` reader - need_des"]
pub type DCDC_DONE_FORCE_R = crate::BitReader;
#[doc = "Field `DCDC_DONE_FORCE` writer - need_des"]
pub type DCDC_DONE_FORCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCDC_ON_FORCE_PU` reader - need_des"]
pub type DCDC_ON_FORCE_PU_R = crate::BitReader;
#[doc = "Field `DCDC_ON_FORCE_PU` writer - need_des"]
pub type DCDC_ON_FORCE_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCDC_ON_FORCE_PD` reader - need_des"]
pub type DCDC_ON_FORCE_PD_R = crate::BitReader;
#[doc = "Field `DCDC_ON_FORCE_PD` writer - need_des"]
pub type DCDC_ON_FORCE_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCDC_FB_RES_FORCE_PU` reader - need_des"]
pub type DCDC_FB_RES_FORCE_PU_R = crate::BitReader;
#[doc = "Field `DCDC_FB_RES_FORCE_PU` writer - need_des"]
pub type DCDC_FB_RES_FORCE_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCDC_FB_RES_FORCE_PD` reader - need_des"]
pub type DCDC_FB_RES_FORCE_PD_R = crate::BitReader;
#[doc = "Field `DCDC_FB_RES_FORCE_PD` writer - need_des"]
pub type DCDC_FB_RES_FORCE_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCDC_LS_FORCE_PU` reader - need_des"]
pub type DCDC_LS_FORCE_PU_R = crate::BitReader;
#[doc = "Field `DCDC_LS_FORCE_PU` writer - need_des"]
pub type DCDC_LS_FORCE_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCDC_LS_FORCE_PD` reader - need_des"]
pub type DCDC_LS_FORCE_PD_R = crate::BitReader;
#[doc = "Field `DCDC_LS_FORCE_PD` writer - need_des"]
pub type DCDC_LS_FORCE_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCDC_DS_FORCE_PU` reader - need_des"]
pub type DCDC_DS_FORCE_PU_R = crate::BitReader;
#[doc = "Field `DCDC_DS_FORCE_PU` writer - need_des"]
pub type DCDC_DS_FORCE_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCDC_DS_FORCE_PD` reader - need_des"]
pub type DCDC_DS_FORCE_PD_R = crate::BitReader;
#[doc = "Field `DCDC_DS_FORCE_PD` writer - need_des"]
pub type DCDC_DS_FORCE_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCM_CUR_ST` reader - need_des"]
pub type DCM_CUR_ST_R = crate::FieldReader;
#[doc = "Field `DCDC_EN_AMUX_TEST` reader - Enable analog mux to pull PAD TEST_DCDC voltage signal"]
pub type DCDC_EN_AMUX_TEST_R = crate::BitReader;
#[doc = "Field `DCDC_EN_AMUX_TEST` writer - Enable analog mux to pull PAD TEST_DCDC voltage signal"]
pub type DCDC_EN_AMUX_TEST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 7 - need_des"]
    #[inline(always)]
    pub fn dcdc_done_force(&self) -> DCDC_DONE_FORCE_R {
        DCDC_DONE_FORCE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - need_des"]
    #[inline(always)]
    pub fn dcdc_on_force_pu(&self) -> DCDC_ON_FORCE_PU_R {
        DCDC_ON_FORCE_PU_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - need_des"]
    #[inline(always)]
    pub fn dcdc_on_force_pd(&self) -> DCDC_ON_FORCE_PD_R {
        DCDC_ON_FORCE_PD_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - need_des"]
    #[inline(always)]
    pub fn dcdc_fb_res_force_pu(&self) -> DCDC_FB_RES_FORCE_PU_R {
        DCDC_FB_RES_FORCE_PU_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - need_des"]
    #[inline(always)]
    pub fn dcdc_fb_res_force_pd(&self) -> DCDC_FB_RES_FORCE_PD_R {
        DCDC_FB_RES_FORCE_PD_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - need_des"]
    #[inline(always)]
    pub fn dcdc_ls_force_pu(&self) -> DCDC_LS_FORCE_PU_R {
        DCDC_LS_FORCE_PU_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - need_des"]
    #[inline(always)]
    pub fn dcdc_ls_force_pd(&self) -> DCDC_LS_FORCE_PD_R {
        DCDC_LS_FORCE_PD_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - need_des"]
    #[inline(always)]
    pub fn dcdc_ds_force_pu(&self) -> DCDC_DS_FORCE_PU_R {
        DCDC_DS_FORCE_PU_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - need_des"]
    #[inline(always)]
    pub fn dcdc_ds_force_pd(&self) -> DCDC_DS_FORCE_PD_R {
        DCDC_DS_FORCE_PD_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - need_des"]
    #[inline(always)]
    pub fn dcm_cur_st(&self) -> DCM_CUR_ST_R {
        DCM_CUR_ST_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 29 - Enable analog mux to pull PAD TEST_DCDC voltage signal"]
    #[inline(always)]
    pub fn dcdc_en_amux_test(&self) -> DCDC_EN_AMUX_TEST_R {
        DCDC_EN_AMUX_TEST_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCM_CTRL")
            .field(
                "dcdc_done_force",
                &format_args!("{}", self.dcdc_done_force().bit()),
            )
            .field(
                "dcdc_on_force_pu",
                &format_args!("{}", self.dcdc_on_force_pu().bit()),
            )
            .field(
                "dcdc_on_force_pd",
                &format_args!("{}", self.dcdc_on_force_pd().bit()),
            )
            .field(
                "dcdc_fb_res_force_pu",
                &format_args!("{}", self.dcdc_fb_res_force_pu().bit()),
            )
            .field(
                "dcdc_fb_res_force_pd",
                &format_args!("{}", self.dcdc_fb_res_force_pd().bit()),
            )
            .field(
                "dcdc_ls_force_pu",
                &format_args!("{}", self.dcdc_ls_force_pu().bit()),
            )
            .field(
                "dcdc_ls_force_pd",
                &format_args!("{}", self.dcdc_ls_force_pd().bit()),
            )
            .field(
                "dcdc_ds_force_pu",
                &format_args!("{}", self.dcdc_ds_force_pu().bit()),
            )
            .field(
                "dcdc_ds_force_pd",
                &format_args!("{}", self.dcdc_ds_force_pd().bit()),
            )
            .field("dcm_cur_st", &format_args!("{}", self.dcm_cur_st().bits()))
            .field(
                "dcdc_en_amux_test",
                &format_args!("{}", self.dcdc_en_amux_test().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DCM_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - SW trigger dcdc on"]
    #[inline(always)]
    #[must_use]
    pub fn dcdc_on_req(&mut self) -> DCDC_ON_REQ_W<DCM_CTRL_SPEC> {
        DCDC_ON_REQ_W::new(self, 0)
    }
    #[doc = "Bit 1 - SW trigger dcdc off"]
    #[inline(always)]
    #[must_use]
    pub fn dcdc_off_req(&mut self) -> DCDC_OFF_REQ_W<DCM_CTRL_SPEC> {
        DCDC_OFF_REQ_W::new(self, 1)
    }
    #[doc = "Bit 2 - SW trigger dcdc enter lightsleep"]
    #[inline(always)]
    #[must_use]
    pub fn dcdc_lightslp_req(&mut self) -> DCDC_LIGHTSLP_REQ_W<DCM_CTRL_SPEC> {
        DCDC_LIGHTSLP_REQ_W::new(self, 2)
    }
    #[doc = "Bit 3 - SW trigger dcdc enter deepsleep"]
    #[inline(always)]
    #[must_use]
    pub fn dcdc_deepslp_req(&mut self) -> DCDC_DEEPSLP_REQ_W<DCM_CTRL_SPEC> {
        DCDC_DEEPSLP_REQ_W::new(self, 3)
    }
    #[doc = "Bit 7 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn dcdc_done_force(&mut self) -> DCDC_DONE_FORCE_W<DCM_CTRL_SPEC> {
        DCDC_DONE_FORCE_W::new(self, 7)
    }
    #[doc = "Bit 8 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn dcdc_on_force_pu(&mut self) -> DCDC_ON_FORCE_PU_W<DCM_CTRL_SPEC> {
        DCDC_ON_FORCE_PU_W::new(self, 8)
    }
    #[doc = "Bit 9 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn dcdc_on_force_pd(&mut self) -> DCDC_ON_FORCE_PD_W<DCM_CTRL_SPEC> {
        DCDC_ON_FORCE_PD_W::new(self, 9)
    }
    #[doc = "Bit 10 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn dcdc_fb_res_force_pu(&mut self) -> DCDC_FB_RES_FORCE_PU_W<DCM_CTRL_SPEC> {
        DCDC_FB_RES_FORCE_PU_W::new(self, 10)
    }
    #[doc = "Bit 11 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn dcdc_fb_res_force_pd(&mut self) -> DCDC_FB_RES_FORCE_PD_W<DCM_CTRL_SPEC> {
        DCDC_FB_RES_FORCE_PD_W::new(self, 11)
    }
    #[doc = "Bit 12 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn dcdc_ls_force_pu(&mut self) -> DCDC_LS_FORCE_PU_W<DCM_CTRL_SPEC> {
        DCDC_LS_FORCE_PU_W::new(self, 12)
    }
    #[doc = "Bit 13 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn dcdc_ls_force_pd(&mut self) -> DCDC_LS_FORCE_PD_W<DCM_CTRL_SPEC> {
        DCDC_LS_FORCE_PD_W::new(self, 13)
    }
    #[doc = "Bit 14 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn dcdc_ds_force_pu(&mut self) -> DCDC_DS_FORCE_PU_W<DCM_CTRL_SPEC> {
        DCDC_DS_FORCE_PU_W::new(self, 14)
    }
    #[doc = "Bit 15 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn dcdc_ds_force_pd(&mut self) -> DCDC_DS_FORCE_PD_W<DCM_CTRL_SPEC> {
        DCDC_DS_FORCE_PD_W::new(self, 15)
    }
    #[doc = "Bit 29 - Enable analog mux to pull PAD TEST_DCDC voltage signal"]
    #[inline(always)]
    #[must_use]
    pub fn dcdc_en_amux_test(&mut self) -> DCDC_EN_AMUX_TEST_W<DCM_CTRL_SPEC> {
        DCDC_EN_AMUX_TEST_W::new(self, 29)
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
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcm_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcm_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCM_CTRL_SPEC;
impl crate::RegisterSpec for DCM_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcm_ctrl::R`](R) reader structure"]
impl crate::Readable for DCM_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dcm_ctrl::W`](W) writer structure"]
impl crate::Writable for DCM_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCM_CTRL to value 0x0001_0000"]
impl crate::Resettable for DCM_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x0001_0000;
}
