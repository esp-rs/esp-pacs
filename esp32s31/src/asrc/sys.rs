#[doc = "Register `SYS` reader"]
pub type R = crate::R<SYS_SPEC>;
#[doc = "Register `SYS` writer"]
pub type W = crate::W<SYS_SPEC>;
#[doc = "Field `CLK_EN` reader - Reserved"]
pub type CLK_EN_R = crate::BitReader;
#[doc = "Field `CLK_EN` writer - Reserved"]
pub type CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHNL0_CLK_FO` reader - Set this bit to make channel0 clock free run."]
pub type CHNL0_CLK_FO_R = crate::BitReader;
#[doc = "Field `CHNL0_CLK_FO` writer - Set this bit to make channel0 clock free run."]
pub type CHNL0_CLK_FO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHNL1_CLK_FO` reader - Set this bit to make channel1 clock free run."]
pub type CHNL1_CLK_FO_R = crate::BitReader;
#[doc = "Field `CHNL1_CLK_FO` writer - Set this bit to make channel1 clock free run."]
pub type CHNL1_CLK_FO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHNL0_OUTFIFO_CLK_FO` reader - Set this bit to make channel0 outfifo clock free run."]
pub type CHNL0_OUTFIFO_CLK_FO_R = crate::BitReader;
#[doc = "Field `CHNL0_OUTFIFO_CLK_FO` writer - Set this bit to make channel0 outfifo clock free run."]
pub type CHNL0_OUTFIFO_CLK_FO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHNL0_INFIFO_CLK_FO` reader - Set this bit to make channel0 infifo clock free run."]
pub type CHNL0_INFIFO_CLK_FO_R = crate::BitReader;
#[doc = "Field `CHNL0_INFIFO_CLK_FO` writer - Set this bit to make channel0 infifo clock free run."]
pub type CHNL0_INFIFO_CLK_FO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHNL1_OUTFIFO_CLK_FO` reader - Set this bit to make channel1 outfifo clock free run."]
pub type CHNL1_OUTFIFO_CLK_FO_R = crate::BitReader;
#[doc = "Field `CHNL1_OUTFIFO_CLK_FO` writer - Set this bit to make channel1 outfifo clock free run."]
pub type CHNL1_OUTFIFO_CLK_FO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHNL1_INFIFO_CLK_FO` reader - Set this bit to make channel1 infifo clock free run."]
pub type CHNL1_INFIFO_CLK_FO_R = crate::BitReader;
#[doc = "Field `CHNL1_INFIFO_CLK_FO` writer - Set this bit to make channel1 infifo clock free run."]
pub type CHNL1_INFIFO_CLK_FO_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit to make channel0 clock free run."]
    #[inline(always)]
    pub fn chnl0_clk_fo(&self) -> CHNL0_CLK_FO_R {
        CHNL0_CLK_FO_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to make channel1 clock free run."]
    #[inline(always)]
    pub fn chnl1_clk_fo(&self) -> CHNL1_CLK_FO_R {
        CHNL1_CLK_FO_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set this bit to make channel0 outfifo clock free run."]
    #[inline(always)]
    pub fn chnl0_outfifo_clk_fo(&self) -> CHNL0_OUTFIFO_CLK_FO_R {
        CHNL0_OUTFIFO_CLK_FO_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set this bit to make channel0 infifo clock free run."]
    #[inline(always)]
    pub fn chnl0_infifo_clk_fo(&self) -> CHNL0_INFIFO_CLK_FO_R {
        CHNL0_INFIFO_CLK_FO_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Set this bit to make channel1 outfifo clock free run."]
    #[inline(always)]
    pub fn chnl1_outfifo_clk_fo(&self) -> CHNL1_OUTFIFO_CLK_FO_R {
        CHNL1_OUTFIFO_CLK_FO_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Set this bit to make channel1 infifo clock free run."]
    #[inline(always)]
    pub fn chnl1_infifo_clk_fo(&self) -> CHNL1_INFIFO_CLK_FO_R {
        CHNL1_INFIFO_CLK_FO_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYS")
            .field("clk_en", &self.clk_en())
            .field("chnl0_clk_fo", &self.chnl0_clk_fo())
            .field("chnl1_clk_fo", &self.chnl1_clk_fo())
            .field("chnl0_outfifo_clk_fo", &self.chnl0_outfifo_clk_fo())
            .field("chnl0_infifo_clk_fo", &self.chnl0_infifo_clk_fo())
            .field("chnl1_outfifo_clk_fo", &self.chnl1_outfifo_clk_fo())
            .field("chnl1_infifo_clk_fo", &self.chnl1_infifo_clk_fo())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn clk_en(&mut self) -> CLK_EN_W<'_, SYS_SPEC> {
        CLK_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to make channel0 clock free run."]
    #[inline(always)]
    pub fn chnl0_clk_fo(&mut self) -> CHNL0_CLK_FO_W<'_, SYS_SPEC> {
        CHNL0_CLK_FO_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to make channel1 clock free run."]
    #[inline(always)]
    pub fn chnl1_clk_fo(&mut self) -> CHNL1_CLK_FO_W<'_, SYS_SPEC> {
        CHNL1_CLK_FO_W::new(self, 2)
    }
    #[doc = "Bit 3 - Set this bit to make channel0 outfifo clock free run."]
    #[inline(always)]
    pub fn chnl0_outfifo_clk_fo(&mut self) -> CHNL0_OUTFIFO_CLK_FO_W<'_, SYS_SPEC> {
        CHNL0_OUTFIFO_CLK_FO_W::new(self, 3)
    }
    #[doc = "Bit 4 - Set this bit to make channel0 infifo clock free run."]
    #[inline(always)]
    pub fn chnl0_infifo_clk_fo(&mut self) -> CHNL0_INFIFO_CLK_FO_W<'_, SYS_SPEC> {
        CHNL0_INFIFO_CLK_FO_W::new(self, 4)
    }
    #[doc = "Bit 5 - Set this bit to make channel1 outfifo clock free run."]
    #[inline(always)]
    pub fn chnl1_outfifo_clk_fo(&mut self) -> CHNL1_OUTFIFO_CLK_FO_W<'_, SYS_SPEC> {
        CHNL1_OUTFIFO_CLK_FO_W::new(self, 5)
    }
    #[doc = "Bit 6 - Set this bit to make channel1 infifo clock free run."]
    #[inline(always)]
    pub fn chnl1_infifo_clk_fo(&mut self) -> CHNL1_INFIFO_CLK_FO_W<'_, SYS_SPEC> {
        CHNL1_INFIFO_CLK_FO_W::new(self, 6)
    }
}
#[doc = "Control and configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`sys::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYS_SPEC;
impl crate::RegisterSpec for SYS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys::R`](R) reader structure"]
impl crate::Readable for SYS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sys::W`](W) writer structure"]
impl crate::Writable for SYS_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYS to value 0"]
impl crate::Resettable for SYS_SPEC {}
