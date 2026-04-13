#[doc = "Register `CH_DBG_EN` reader"]
pub type R = crate::R<CH_DBG_EN_SPEC>;
#[doc = "Register `CH_DBG_EN` writer"]
pub type W = crate::W<CH_DBG_EN_SPEC>;
#[doc = "Field `H264_OUT_CH0_DBG_EN` reader - configures whether to enable out channel 0 debug. 0: disable, 1: enable"]
pub type H264_OUT_CH0_DBG_EN_R = crate::BitReader;
#[doc = "Field `H264_OUT_CH0_DBG_EN` writer - configures whether to enable out channel 0 debug. 0: disable, 1: enable"]
pub type H264_OUT_CH0_DBG_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H264_OUT_CH1_DBG_EN` reader - configures whether to enable out channel 1 debug. 0: disable, 1: enable"]
pub type H264_OUT_CH1_DBG_EN_R = crate::BitReader;
#[doc = "Field `H264_OUT_CH1_DBG_EN` writer - configures whether to enable out channel 1 debug. 0: disable, 1: enable"]
pub type H264_OUT_CH1_DBG_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H264_OUT_CH2_DBG_EN` reader - configures whether to enable out channel 2 debug. 0: disable, 1: enable"]
pub type H264_OUT_CH2_DBG_EN_R = crate::BitReader;
#[doc = "Field `H264_OUT_CH2_DBG_EN` writer - configures whether to enable out channel 2 debug. 0: disable, 1: enable"]
pub type H264_OUT_CH2_DBG_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H264_OUT_CH3_DBG_EN` reader - configures whether to enable out channel 3 debug. 0: disable, 1: enable"]
pub type H264_OUT_CH3_DBG_EN_R = crate::BitReader;
#[doc = "Field `H264_OUT_CH3_DBG_EN` writer - configures whether to enable out channel 3 debug. 0: disable, 1: enable"]
pub type H264_OUT_CH3_DBG_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H264_OUT_CH4_DBG_EN` reader - configures whether to enable out channel 4 debug. 0: disable, 1: enable"]
pub type H264_OUT_CH4_DBG_EN_R = crate::BitReader;
#[doc = "Field `H264_OUT_CH4_DBG_EN` writer - configures whether to enable out channel 4 debug. 0: disable, 1: enable"]
pub type H264_OUT_CH4_DBG_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H264_IN_CH0_DBG_EN` reader - configures whether to enable in channel 0 debug. 0: disable, 1: enable"]
pub type H264_IN_CH0_DBG_EN_R = crate::BitReader;
#[doc = "Field `H264_IN_CH0_DBG_EN` writer - configures whether to enable in channel 0 debug. 0: disable, 1: enable"]
pub type H264_IN_CH0_DBG_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H264_IN_CH1_DBG_EN` reader - configures whether to enable in channel 1 debug. 0: disable, 1: enable"]
pub type H264_IN_CH1_DBG_EN_R = crate::BitReader;
#[doc = "Field `H264_IN_CH1_DBG_EN` writer - configures whether to enable in channel 1 debug. 0: disable, 1: enable"]
pub type H264_IN_CH1_DBG_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H264_IN_CH2_DBG_EN` reader - configures whether to enable in channel 2 debug. 0: disable, 1: enable"]
pub type H264_IN_CH2_DBG_EN_R = crate::BitReader;
#[doc = "Field `H264_IN_CH2_DBG_EN` writer - configures whether to enable in channel 2 debug. 0: disable, 1: enable"]
pub type H264_IN_CH2_DBG_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H264_IN_CH3_DBG_EN` reader - configures whether to enable in channel 3 debug. 0: disable, 1: enable"]
pub type H264_IN_CH3_DBG_EN_R = crate::BitReader;
#[doc = "Field `H264_IN_CH3_DBG_EN` writer - configures whether to enable in channel 3 debug. 0: disable, 1: enable"]
pub type H264_IN_CH3_DBG_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H264_IN_CH4_DBG_EN` reader - configures whether to enable in channel 4 debug. 0: disable, 1: enable"]
pub type H264_IN_CH4_DBG_EN_R = crate::BitReader;
#[doc = "Field `H264_IN_CH4_DBG_EN` writer - configures whether to enable in channel 4 debug. 0: disable, 1: enable"]
pub type H264_IN_CH4_DBG_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `H264_IN_CH5_DBG_EN` reader - configures whether to enable in channel 5 debug. 0: disable, 1: enable"]
pub type H264_IN_CH5_DBG_EN_R = crate::BitReader;
#[doc = "Field `H264_IN_CH5_DBG_EN` writer - configures whether to enable in channel 5 debug. 0: disable, 1: enable"]
pub type H264_IN_CH5_DBG_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - configures whether to enable out channel 0 debug. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn h264_out_ch0_dbg_en(&self) -> H264_OUT_CH0_DBG_EN_R {
        H264_OUT_CH0_DBG_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - configures whether to enable out channel 1 debug. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn h264_out_ch1_dbg_en(&self) -> H264_OUT_CH1_DBG_EN_R {
        H264_OUT_CH1_DBG_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - configures whether to enable out channel 2 debug. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn h264_out_ch2_dbg_en(&self) -> H264_OUT_CH2_DBG_EN_R {
        H264_OUT_CH2_DBG_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - configures whether to enable out channel 3 debug. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn h264_out_ch3_dbg_en(&self) -> H264_OUT_CH3_DBG_EN_R {
        H264_OUT_CH3_DBG_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - configures whether to enable out channel 4 debug. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn h264_out_ch4_dbg_en(&self) -> H264_OUT_CH4_DBG_EN_R {
        H264_OUT_CH4_DBG_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 16 - configures whether to enable in channel 0 debug. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn h264_in_ch0_dbg_en(&self) -> H264_IN_CH0_DBG_EN_R {
        H264_IN_CH0_DBG_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - configures whether to enable in channel 1 debug. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn h264_in_ch1_dbg_en(&self) -> H264_IN_CH1_DBG_EN_R {
        H264_IN_CH1_DBG_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - configures whether to enable in channel 2 debug. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn h264_in_ch2_dbg_en(&self) -> H264_IN_CH2_DBG_EN_R {
        H264_IN_CH2_DBG_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - configures whether to enable in channel 3 debug. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn h264_in_ch3_dbg_en(&self) -> H264_IN_CH3_DBG_EN_R {
        H264_IN_CH3_DBG_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - configures whether to enable in channel 4 debug. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn h264_in_ch4_dbg_en(&self) -> H264_IN_CH4_DBG_EN_R {
        H264_IN_CH4_DBG_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - configures whether to enable in channel 5 debug. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn h264_in_ch5_dbg_en(&self) -> H264_IN_CH5_DBG_EN_R {
        H264_IN_CH5_DBG_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH_DBG_EN")
            .field("h264_out_ch0_dbg_en", &self.h264_out_ch0_dbg_en())
            .field("h264_out_ch1_dbg_en", &self.h264_out_ch1_dbg_en())
            .field("h264_out_ch2_dbg_en", &self.h264_out_ch2_dbg_en())
            .field("h264_out_ch3_dbg_en", &self.h264_out_ch3_dbg_en())
            .field("h264_out_ch4_dbg_en", &self.h264_out_ch4_dbg_en())
            .field("h264_in_ch0_dbg_en", &self.h264_in_ch0_dbg_en())
            .field("h264_in_ch1_dbg_en", &self.h264_in_ch1_dbg_en())
            .field("h264_in_ch2_dbg_en", &self.h264_in_ch2_dbg_en())
            .field("h264_in_ch3_dbg_en", &self.h264_in_ch3_dbg_en())
            .field("h264_in_ch4_dbg_en", &self.h264_in_ch4_dbg_en())
            .field("h264_in_ch5_dbg_en", &self.h264_in_ch5_dbg_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - configures whether to enable out channel 0 debug. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn h264_out_ch0_dbg_en(&mut self) -> H264_OUT_CH0_DBG_EN_W<'_, CH_DBG_EN_SPEC> {
        H264_OUT_CH0_DBG_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - configures whether to enable out channel 1 debug. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn h264_out_ch1_dbg_en(&mut self) -> H264_OUT_CH1_DBG_EN_W<'_, CH_DBG_EN_SPEC> {
        H264_OUT_CH1_DBG_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - configures whether to enable out channel 2 debug. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn h264_out_ch2_dbg_en(&mut self) -> H264_OUT_CH2_DBG_EN_W<'_, CH_DBG_EN_SPEC> {
        H264_OUT_CH2_DBG_EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - configures whether to enable out channel 3 debug. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn h264_out_ch3_dbg_en(&mut self) -> H264_OUT_CH3_DBG_EN_W<'_, CH_DBG_EN_SPEC> {
        H264_OUT_CH3_DBG_EN_W::new(self, 3)
    }
    #[doc = "Bit 4 - configures whether to enable out channel 4 debug. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn h264_out_ch4_dbg_en(&mut self) -> H264_OUT_CH4_DBG_EN_W<'_, CH_DBG_EN_SPEC> {
        H264_OUT_CH4_DBG_EN_W::new(self, 4)
    }
    #[doc = "Bit 16 - configures whether to enable in channel 0 debug. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn h264_in_ch0_dbg_en(&mut self) -> H264_IN_CH0_DBG_EN_W<'_, CH_DBG_EN_SPEC> {
        H264_IN_CH0_DBG_EN_W::new(self, 16)
    }
    #[doc = "Bit 17 - configures whether to enable in channel 1 debug. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn h264_in_ch1_dbg_en(&mut self) -> H264_IN_CH1_DBG_EN_W<'_, CH_DBG_EN_SPEC> {
        H264_IN_CH1_DBG_EN_W::new(self, 17)
    }
    #[doc = "Bit 18 - configures whether to enable in channel 2 debug. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn h264_in_ch2_dbg_en(&mut self) -> H264_IN_CH2_DBG_EN_W<'_, CH_DBG_EN_SPEC> {
        H264_IN_CH2_DBG_EN_W::new(self, 18)
    }
    #[doc = "Bit 19 - configures whether to enable in channel 3 debug. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn h264_in_ch3_dbg_en(&mut self) -> H264_IN_CH3_DBG_EN_W<'_, CH_DBG_EN_SPEC> {
        H264_IN_CH3_DBG_EN_W::new(self, 19)
    }
    #[doc = "Bit 20 - configures whether to enable in channel 4 debug. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn h264_in_ch4_dbg_en(&mut self) -> H264_IN_CH4_DBG_EN_W<'_, CH_DBG_EN_SPEC> {
        H264_IN_CH4_DBG_EN_W::new(self, 20)
    }
    #[doc = "Bit 21 - configures whether to enable in channel 5 debug. 0: disable, 1: enable"]
    #[inline(always)]
    pub fn h264_in_ch5_dbg_en(&mut self) -> H264_IN_CH5_DBG_EN_W<'_, CH_DBG_EN_SPEC> {
        H264_IN_CH5_DBG_EN_W::new(self, 21)
    }
}
#[doc = "channel debug enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch_dbg_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_dbg_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH_DBG_EN_SPEC;
impl crate::RegisterSpec for CH_DBG_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch_dbg_en::R`](R) reader structure"]
impl crate::Readable for CH_DBG_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch_dbg_en::W`](W) writer structure"]
impl crate::Writable for CH_DBG_EN_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH_DBG_EN to value 0"]
impl crate::Resettable for CH_DBG_EN_SPEC {}
