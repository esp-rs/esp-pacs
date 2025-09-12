#[doc = "Register `CFG1` reader"]
pub type R = crate::R<CFG1_SPEC>;
#[doc = "Register `CFG1` writer"]
pub type W = crate::W<CFG1_SPEC>;
#[doc = "Field `CH1_TT_FC` reader - NA"]
pub type CH1_TT_FC_R = crate::FieldReader;
#[doc = "Field `CH1_TT_FC` writer - NA"]
pub type CH1_TT_FC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CH1_HS_SEL_SRC` reader - NA"]
pub type CH1_HS_SEL_SRC_R = crate::BitReader;
#[doc = "Field `CH1_HS_SEL_SRC` writer - NA"]
pub type CH1_HS_SEL_SRC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_HS_SEL_DST` reader - NA"]
pub type CH1_HS_SEL_DST_R = crate::BitReader;
#[doc = "Field `CH1_HS_SEL_DST` writer - NA"]
pub type CH1_HS_SEL_DST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_SRC_HWHS_POL` reader - NA"]
pub type CH1_SRC_HWHS_POL_R = crate::BitReader;
#[doc = "Field `CH1_DST_HWHS_POL` reader - NA"]
pub type CH1_DST_HWHS_POL_R = crate::BitReader;
#[doc = "Field `CH1_SRC_PER` reader - NA"]
pub type CH1_SRC_PER_R = crate::FieldReader;
#[doc = "Field `CH1_SRC_PER` writer - NA"]
pub type CH1_SRC_PER_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CH1_DST_PER` reader - NA"]
pub type CH1_DST_PER_R = crate::FieldReader;
#[doc = "Field `CH1_DST_PER` writer - NA"]
pub type CH1_DST_PER_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CH1_CH_PRIOR` reader - NA"]
pub type CH1_CH_PRIOR_R = crate::FieldReader;
#[doc = "Field `CH1_CH_PRIOR` writer - NA"]
pub type CH1_CH_PRIOR_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CH1_LOCK_CH` reader - NA"]
pub type CH1_LOCK_CH_R = crate::BitReader;
#[doc = "Field `CH1_LOCK_CH_L` reader - NA"]
pub type CH1_LOCK_CH_L_R = crate::FieldReader;
#[doc = "Field `CH1_SRC_OSR_LMT` reader - NA"]
pub type CH1_SRC_OSR_LMT_R = crate::FieldReader;
#[doc = "Field `CH1_SRC_OSR_LMT` writer - NA"]
pub type CH1_SRC_OSR_LMT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CH1_DST_OSR_LMT` reader - NA"]
pub type CH1_DST_OSR_LMT_R = crate::FieldReader;
#[doc = "Field `CH1_DST_OSR_LMT` writer - NA"]
pub type CH1_DST_OSR_LMT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:2 - NA"]
    #[inline(always)]
    pub fn ch1_tt_fc(&self) -> CH1_TT_FC_R {
        CH1_TT_FC_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    pub fn ch1_hs_sel_src(&self) -> CH1_HS_SEL_SRC_R {
        CH1_HS_SEL_SRC_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NA"]
    #[inline(always)]
    pub fn ch1_hs_sel_dst(&self) -> CH1_HS_SEL_DST_R {
        CH1_HS_SEL_DST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - NA"]
    #[inline(always)]
    pub fn ch1_src_hwhs_pol(&self) -> CH1_SRC_HWHS_POL_R {
        CH1_SRC_HWHS_POL_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - NA"]
    #[inline(always)]
    pub fn ch1_dst_hwhs_pol(&self) -> CH1_DST_HWHS_POL_R {
        CH1_DST_HWHS_POL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:8 - NA"]
    #[inline(always)]
    pub fn ch1_src_per(&self) -> CH1_SRC_PER_R {
        CH1_SRC_PER_R::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bits 12:13 - NA"]
    #[inline(always)]
    pub fn ch1_dst_per(&self) -> CH1_DST_PER_R {
        CH1_DST_PER_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 17:19 - NA"]
    #[inline(always)]
    pub fn ch1_ch_prior(&self) -> CH1_CH_PRIOR_R {
        CH1_CH_PRIOR_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bit 20 - NA"]
    #[inline(always)]
    pub fn ch1_lock_ch(&self) -> CH1_LOCK_CH_R {
        CH1_LOCK_CH_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - NA"]
    #[inline(always)]
    pub fn ch1_lock_ch_l(&self) -> CH1_LOCK_CH_L_R {
        CH1_LOCK_CH_L_R::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bits 23:26 - NA"]
    #[inline(always)]
    pub fn ch1_src_osr_lmt(&self) -> CH1_SRC_OSR_LMT_R {
        CH1_SRC_OSR_LMT_R::new(((self.bits >> 23) & 0x0f) as u8)
    }
    #[doc = "Bits 27:30 - NA"]
    #[inline(always)]
    pub fn ch1_dst_osr_lmt(&self) -> CH1_DST_OSR_LMT_R {
        CH1_DST_OSR_LMT_R::new(((self.bits >> 27) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CFG1")
            .field("ch1_tt_fc", &self.ch1_tt_fc())
            .field("ch1_hs_sel_src", &self.ch1_hs_sel_src())
            .field("ch1_hs_sel_dst", &self.ch1_hs_sel_dst())
            .field("ch1_src_hwhs_pol", &self.ch1_src_hwhs_pol())
            .field("ch1_dst_hwhs_pol", &self.ch1_dst_hwhs_pol())
            .field("ch1_src_per", &self.ch1_src_per())
            .field("ch1_dst_per", &self.ch1_dst_per())
            .field("ch1_ch_prior", &self.ch1_ch_prior())
            .field("ch1_lock_ch", &self.ch1_lock_ch())
            .field("ch1_lock_ch_l", &self.ch1_lock_ch_l())
            .field("ch1_src_osr_lmt", &self.ch1_src_osr_lmt())
            .field("ch1_dst_osr_lmt", &self.ch1_dst_osr_lmt())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - NA"]
    #[inline(always)]
    pub fn ch1_tt_fc(&mut self) -> CH1_TT_FC_W<'_, CFG1_SPEC> {
        CH1_TT_FC_W::new(self, 0)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    pub fn ch1_hs_sel_src(&mut self) -> CH1_HS_SEL_SRC_W<'_, CFG1_SPEC> {
        CH1_HS_SEL_SRC_W::new(self, 3)
    }
    #[doc = "Bit 4 - NA"]
    #[inline(always)]
    pub fn ch1_hs_sel_dst(&mut self) -> CH1_HS_SEL_DST_W<'_, CFG1_SPEC> {
        CH1_HS_SEL_DST_W::new(self, 4)
    }
    #[doc = "Bits 7:8 - NA"]
    #[inline(always)]
    pub fn ch1_src_per(&mut self) -> CH1_SRC_PER_W<'_, CFG1_SPEC> {
        CH1_SRC_PER_W::new(self, 7)
    }
    #[doc = "Bits 12:13 - NA"]
    #[inline(always)]
    pub fn ch1_dst_per(&mut self) -> CH1_DST_PER_W<'_, CFG1_SPEC> {
        CH1_DST_PER_W::new(self, 12)
    }
    #[doc = "Bits 17:19 - NA"]
    #[inline(always)]
    pub fn ch1_ch_prior(&mut self) -> CH1_CH_PRIOR_W<'_, CFG1_SPEC> {
        CH1_CH_PRIOR_W::new(self, 17)
    }
    #[doc = "Bits 23:26 - NA"]
    #[inline(always)]
    pub fn ch1_src_osr_lmt(&mut self) -> CH1_SRC_OSR_LMT_W<'_, CFG1_SPEC> {
        CH1_SRC_OSR_LMT_W::new(self, 23)
    }
    #[doc = "Bits 27:30 - NA"]
    #[inline(always)]
    pub fn ch1_dst_osr_lmt(&mut self) -> CH1_DST_OSR_LMT_W<'_, CFG1_SPEC> {
        CH1_DST_OSR_LMT_W::new(self, 27)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CFG1_SPEC;
impl crate::RegisterSpec for CFG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfg1::R`](R) reader structure"]
impl crate::Readable for CFG1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cfg1::W`](W) writer structure"]
impl crate::Writable for CFG1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFG1 to value 0x0006_001b"]
impl crate::Resettable for CFG1_SPEC {
    const RESET_VALUE: u32 = 0x0006_001b;
}
