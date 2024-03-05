#[doc = "Register `CH2_CFG1` reader"]
pub type R = crate::R<CH2_CFG1_SPEC>;
#[doc = "Register `CH2_CFG1` writer"]
pub type W = crate::W<CH2_CFG1_SPEC>;
#[doc = "Field `CH2_TT_FC` reader - NA"]
pub type CH2_TT_FC_R = crate::FieldReader;
#[doc = "Field `CH2_TT_FC` writer - NA"]
pub type CH2_TT_FC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CH2_HS_SEL_SRC` reader - NA"]
pub type CH2_HS_SEL_SRC_R = crate::BitReader;
#[doc = "Field `CH2_HS_SEL_SRC` writer - NA"]
pub type CH2_HS_SEL_SRC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2_HS_SEL_DST` reader - NA"]
pub type CH2_HS_SEL_DST_R = crate::BitReader;
#[doc = "Field `CH2_HS_SEL_DST` writer - NA"]
pub type CH2_HS_SEL_DST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2_SRC_HWHS_POL` reader - NA"]
pub type CH2_SRC_HWHS_POL_R = crate::BitReader;
#[doc = "Field `CH2_DST_HWHS_POL` reader - NA"]
pub type CH2_DST_HWHS_POL_R = crate::BitReader;
#[doc = "Field `CH2_SRC_PER` reader - NA"]
pub type CH2_SRC_PER_R = crate::FieldReader;
#[doc = "Field `CH2_SRC_PER` writer - NA"]
pub type CH2_SRC_PER_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CH2_DST_PER` reader - NA"]
pub type CH2_DST_PER_R = crate::FieldReader;
#[doc = "Field `CH2_DST_PER` writer - NA"]
pub type CH2_DST_PER_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CH2_CH_PRIOR` reader - NA"]
pub type CH2_CH_PRIOR_R = crate::FieldReader;
#[doc = "Field `CH2_CH_PRIOR` writer - NA"]
pub type CH2_CH_PRIOR_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CH2_LOCK_CH` reader - NA"]
pub type CH2_LOCK_CH_R = crate::BitReader;
#[doc = "Field `CH2_LOCK_CH_L` reader - NA"]
pub type CH2_LOCK_CH_L_R = crate::FieldReader;
#[doc = "Field `CH2_SRC_OSR_LMT` reader - NA"]
pub type CH2_SRC_OSR_LMT_R = crate::FieldReader;
#[doc = "Field `CH2_SRC_OSR_LMT` writer - NA"]
pub type CH2_SRC_OSR_LMT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CH2_DST_OSR_LMT` reader - NA"]
pub type CH2_DST_OSR_LMT_R = crate::FieldReader;
#[doc = "Field `CH2_DST_OSR_LMT` writer - NA"]
pub type CH2_DST_OSR_LMT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:2 - NA"]
    #[inline(always)]
    pub fn ch2_tt_fc(&self) -> CH2_TT_FC_R {
        CH2_TT_FC_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    pub fn ch2_hs_sel_src(&self) -> CH2_HS_SEL_SRC_R {
        CH2_HS_SEL_SRC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NA"]
    #[inline(always)]
    pub fn ch2_hs_sel_dst(&self) -> CH2_HS_SEL_DST_R {
        CH2_HS_SEL_DST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - NA"]
    #[inline(always)]
    pub fn ch2_src_hwhs_pol(&self) -> CH2_SRC_HWHS_POL_R {
        CH2_SRC_HWHS_POL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - NA"]
    #[inline(always)]
    pub fn ch2_dst_hwhs_pol(&self) -> CH2_DST_HWHS_POL_R {
        CH2_DST_HWHS_POL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:8 - NA"]
    #[inline(always)]
    pub fn ch2_src_per(&self) -> CH2_SRC_PER_R {
        CH2_SRC_PER_R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bits 12:13 - NA"]
    #[inline(always)]
    pub fn ch2_dst_per(&self) -> CH2_DST_PER_R {
        CH2_DST_PER_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 17:19 - NA"]
    #[inline(always)]
    pub fn ch2_ch_prior(&self) -> CH2_CH_PRIOR_R {
        CH2_CH_PRIOR_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bit 20 - NA"]
    #[inline(always)]
    pub fn ch2_lock_ch(&self) -> CH2_LOCK_CH_R {
        CH2_LOCK_CH_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - NA"]
    #[inline(always)]
    pub fn ch2_lock_ch_l(&self) -> CH2_LOCK_CH_L_R {
        CH2_LOCK_CH_L_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bits 23:26 - NA"]
    #[inline(always)]
    pub fn ch2_src_osr_lmt(&self) -> CH2_SRC_OSR_LMT_R {
        CH2_SRC_OSR_LMT_R::new(((self.bits >> 23) & 0x0f) as u8)
    }
    #[doc = "Bits 27:30 - NA"]
    #[inline(always)]
    pub fn ch2_dst_osr_lmt(&self) -> CH2_DST_OSR_LMT_R {
        CH2_DST_OSR_LMT_R::new(((self.bits >> 27) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH2_CFG1")
            .field("ch2_tt_fc", &format_args!("{}", self.ch2_tt_fc().bits()))
            .field(
                "ch2_hs_sel_src",
                &format_args!("{}", self.ch2_hs_sel_src().bit()),
            )
            .field(
                "ch2_hs_sel_dst",
                &format_args!("{}", self.ch2_hs_sel_dst().bit()),
            )
            .field(
                "ch2_src_hwhs_pol",
                &format_args!("{}", self.ch2_src_hwhs_pol().bit()),
            )
            .field(
                "ch2_dst_hwhs_pol",
                &format_args!("{}", self.ch2_dst_hwhs_pol().bit()),
            )
            .field(
                "ch2_src_per",
                &format_args!("{}", self.ch2_src_per().bits()),
            )
            .field(
                "ch2_dst_per",
                &format_args!("{}", self.ch2_dst_per().bits()),
            )
            .field(
                "ch2_ch_prior",
                &format_args!("{}", self.ch2_ch_prior().bits()),
            )
            .field("ch2_lock_ch", &format_args!("{}", self.ch2_lock_ch().bit()))
            .field(
                "ch2_lock_ch_l",
                &format_args!("{}", self.ch2_lock_ch_l().bits()),
            )
            .field(
                "ch2_src_osr_lmt",
                &format_args!("{}", self.ch2_src_osr_lmt().bits()),
            )
            .field(
                "ch2_dst_osr_lmt",
                &format_args!("{}", self.ch2_dst_osr_lmt().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CH2_CFG1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:2 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_tt_fc(&mut self) -> CH2_TT_FC_W<CH2_CFG1_SPEC> {
        CH2_TT_FC_W::new(self, 0)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_hs_sel_src(&mut self) -> CH2_HS_SEL_SRC_W<CH2_CFG1_SPEC> {
        CH2_HS_SEL_SRC_W::new(self, 3)
    }
    #[doc = "Bit 4 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_hs_sel_dst(&mut self) -> CH2_HS_SEL_DST_W<CH2_CFG1_SPEC> {
        CH2_HS_SEL_DST_W::new(self, 4)
    }
    #[doc = "Bits 7:8 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_src_per(&mut self) -> CH2_SRC_PER_W<CH2_CFG1_SPEC> {
        CH2_SRC_PER_W::new(self, 7)
    }
    #[doc = "Bits 12:13 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_dst_per(&mut self) -> CH2_DST_PER_W<CH2_CFG1_SPEC> {
        CH2_DST_PER_W::new(self, 12)
    }
    #[doc = "Bits 17:19 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_ch_prior(&mut self) -> CH2_CH_PRIOR_W<CH2_CFG1_SPEC> {
        CH2_CH_PRIOR_W::new(self, 17)
    }
    #[doc = "Bits 23:26 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_src_osr_lmt(&mut self) -> CH2_SRC_OSR_LMT_W<CH2_CFG1_SPEC> {
        CH2_SRC_OSR_LMT_W::new(self, 23)
    }
    #[doc = "Bits 27:30 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch2_dst_osr_lmt(&mut self) -> CH2_DST_OSR_LMT_W<CH2_CFG1_SPEC> {
        CH2_DST_OSR_LMT_W::new(self, 27)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch2_cfg1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch2_cfg1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH2_CFG1_SPEC;
impl crate::RegisterSpec for CH2_CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch2_cfg1::R`](R) reader structure"]
impl crate::Readable for CH2_CFG1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch2_cfg1::W`](W) writer structure"]
impl crate::Writable for CH2_CFG1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH2_CFG1 to value 0x0004_001b"]
impl crate::Resettable for CH2_CFG1_SPEC {
    const RESET_VALUE: u32 = 0x0004_001b;
}
