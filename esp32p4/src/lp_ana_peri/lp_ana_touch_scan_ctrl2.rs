#[doc = "Register `LP_ANA_TOUCH_SCAN_CTRL2` reader"]
pub type R = crate::R<LP_ANA_TOUCH_SCAN_CTRL2_SPEC>;
#[doc = "Register `LP_ANA_TOUCH_SCAN_CTRL2` writer"]
pub type W = crate::W<LP_ANA_TOUCH_SCAN_CTRL2_SPEC>;
#[doc = "Field `LP_ANA_TOUCH_TIMEOUT_NUM` reader - need_des"]
pub type LP_ANA_TOUCH_TIMEOUT_NUM_R = crate::FieldReader<u16>;
#[doc = "Field `LP_ANA_TOUCH_TIMEOUT_NUM` writer - need_des"]
pub type LP_ANA_TOUCH_TIMEOUT_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `LP_ANA_TOUCH_TIMEOUT_EN` reader - need_des"]
pub type LP_ANA_TOUCH_TIMEOUT_EN_R = crate::BitReader;
#[doc = "Field `LP_ANA_TOUCH_TIMEOUT_EN` writer - need_des"]
pub type LP_ANA_TOUCH_TIMEOUT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_ANA_TOUCH_OUT_RING` reader - need_des"]
pub type LP_ANA_TOUCH_OUT_RING_R = crate::FieldReader;
#[doc = "Field `LP_ANA_TOUCH_OUT_RING` writer - need_des"]
pub type LP_ANA_TOUCH_OUT_RING_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `LP_ANA_FREQ_SCAN_EN` reader - need_des"]
pub type LP_ANA_FREQ_SCAN_EN_R = crate::BitReader;
#[doc = "Field `LP_ANA_FREQ_SCAN_EN` writer - need_des"]
pub type LP_ANA_FREQ_SCAN_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_ANA_FREQ_SCAN_CNT_LIMIT` reader - need_des"]
pub type LP_ANA_FREQ_SCAN_CNT_LIMIT_R = crate::FieldReader;
#[doc = "Field `LP_ANA_FREQ_SCAN_CNT_LIMIT` writer - need_des"]
pub type LP_ANA_FREQ_SCAN_CNT_LIMIT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 6:21 - need_des"]
    #[inline(always)]
    pub fn lp_ana_touch_timeout_num(&self) -> LP_ANA_TOUCH_TIMEOUT_NUM_R {
        LP_ANA_TOUCH_TIMEOUT_NUM_R::new(((self.bits >> 6) & 0xffff) as u16)
    }
    #[doc = "Bit 22 - need_des"]
    #[inline(always)]
    pub fn lp_ana_touch_timeout_en(&self) -> LP_ANA_TOUCH_TIMEOUT_EN_R {
        LP_ANA_TOUCH_TIMEOUT_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 23:26 - need_des"]
    #[inline(always)]
    pub fn lp_ana_touch_out_ring(&self) -> LP_ANA_TOUCH_OUT_RING_R {
        LP_ANA_TOUCH_OUT_RING_R::new(((self.bits >> 23) & 0x0f) as u8)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn lp_ana_freq_scan_en(&self) -> LP_ANA_FREQ_SCAN_EN_R {
        LP_ANA_FREQ_SCAN_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:29 - need_des"]
    #[inline(always)]
    pub fn lp_ana_freq_scan_cnt_limit(&self) -> LP_ANA_FREQ_SCAN_CNT_LIMIT_R {
        LP_ANA_FREQ_SCAN_CNT_LIMIT_R::new(((self.bits >> 28) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_ANA_TOUCH_SCAN_CTRL2")
            .field(
                "lp_ana_touch_timeout_num",
                &format_args!("{}", self.lp_ana_touch_timeout_num().bits()),
            )
            .field(
                "lp_ana_touch_timeout_en",
                &format_args!("{}", self.lp_ana_touch_timeout_en().bit()),
            )
            .field(
                "lp_ana_touch_out_ring",
                &format_args!("{}", self.lp_ana_touch_out_ring().bits()),
            )
            .field(
                "lp_ana_freq_scan_en",
                &format_args!("{}", self.lp_ana_freq_scan_en().bit()),
            )
            .field(
                "lp_ana_freq_scan_cnt_limit",
                &format_args!("{}", self.lp_ana_freq_scan_cnt_limit().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LP_ANA_TOUCH_SCAN_CTRL2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 6:21 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_ana_touch_timeout_num(
        &mut self,
    ) -> LP_ANA_TOUCH_TIMEOUT_NUM_W<LP_ANA_TOUCH_SCAN_CTRL2_SPEC> {
        LP_ANA_TOUCH_TIMEOUT_NUM_W::new(self, 6)
    }
    #[doc = "Bit 22 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_ana_touch_timeout_en(
        &mut self,
    ) -> LP_ANA_TOUCH_TIMEOUT_EN_W<LP_ANA_TOUCH_SCAN_CTRL2_SPEC> {
        LP_ANA_TOUCH_TIMEOUT_EN_W::new(self, 22)
    }
    #[doc = "Bits 23:26 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_ana_touch_out_ring(
        &mut self,
    ) -> LP_ANA_TOUCH_OUT_RING_W<LP_ANA_TOUCH_SCAN_CTRL2_SPEC> {
        LP_ANA_TOUCH_OUT_RING_W::new(self, 23)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_ana_freq_scan_en(&mut self) -> LP_ANA_FREQ_SCAN_EN_W<LP_ANA_TOUCH_SCAN_CTRL2_SPEC> {
        LP_ANA_FREQ_SCAN_EN_W::new(self, 27)
    }
    #[doc = "Bits 28:29 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_ana_freq_scan_cnt_limit(
        &mut self,
    ) -> LP_ANA_FREQ_SCAN_CNT_LIMIT_W<LP_ANA_TOUCH_SCAN_CTRL2_SPEC> {
        LP_ANA_FREQ_SCAN_CNT_LIMIT_W::new(self, 28)
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
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_ana_touch_scan_ctrl2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_ana_touch_scan_ctrl2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_ANA_TOUCH_SCAN_CTRL2_SPEC;
impl crate::RegisterSpec for LP_ANA_TOUCH_SCAN_CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_ana_touch_scan_ctrl2::R`](R) reader structure"]
impl crate::Readable for LP_ANA_TOUCH_SCAN_CTRL2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lp_ana_touch_scan_ctrl2::W`](W) writer structure"]
impl crate::Writable for LP_ANA_TOUCH_SCAN_CTRL2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LP_ANA_TOUCH_SCAN_CTRL2 to value 0x37bf_ffc0"]
impl crate::Resettable for LP_ANA_TOUCH_SCAN_CTRL2_SPEC {
    const RESET_VALUE: u32 = 0x37bf_ffc0;
}
