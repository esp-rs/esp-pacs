#[doc = "Register `REGDMA_ETM_CTRL` reader"]
pub type R = crate::R<REGDMA_ETM_CTRL_SPEC>;
#[doc = "Register `REGDMA_ETM_CTRL` writer"]
pub type W = crate::W<REGDMA_ETM_CTRL_SPEC>;
#[doc = "Field `ETM_START_0` writer - etm_start_0 reg"]
pub type ETM_START_0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETM_START_1` writer - etm_start_1 reg"]
pub type ETM_START_1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETM_START_2` writer - etm_start_2 reg"]
pub type ETM_START_2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETM_START_3` writer - etm_start_3 reg"]
pub type ETM_START_3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETM_LINK_SEL_0` reader - etm_link sel"]
pub type ETM_LINK_SEL_0_R = crate::FieldReader;
#[doc = "Field `ETM_LINK_SEL_0` writer - etm_link sel"]
pub type ETM_LINK_SEL_0_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ETM_LINK_SEL_1` reader - etm_link sel"]
pub type ETM_LINK_SEL_1_R = crate::FieldReader;
#[doc = "Field `ETM_LINK_SEL_1` writer - etm_link sel"]
pub type ETM_LINK_SEL_1_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ETM_LINK_SEL_2` reader - etm_link sel"]
pub type ETM_LINK_SEL_2_R = crate::FieldReader;
#[doc = "Field `ETM_LINK_SEL_2` writer - etm_link sel"]
pub type ETM_LINK_SEL_2_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ETM_LINK_SEL_3` reader - etm_link sel"]
pub type ETM_LINK_SEL_3_R = crate::FieldReader;
#[doc = "Field `ETM_LINK_SEL_3` writer - etm_link sel"]
pub type ETM_LINK_SEL_3_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ETM_BUSY_CAUSE` reader - debug"]
pub type ETM_BUSY_CAUSE_R = crate::FieldReader;
impl R {
    #[doc = "Bits 4:7 - etm_link sel"]
    #[inline(always)]
    pub fn etm_link_sel_0(&self) -> ETM_LINK_SEL_0_R {
        ETM_LINK_SEL_0_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - etm_link sel"]
    #[inline(always)]
    pub fn etm_link_sel_1(&self) -> ETM_LINK_SEL_1_R {
        ETM_LINK_SEL_1_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - etm_link sel"]
    #[inline(always)]
    pub fn etm_link_sel_2(&self) -> ETM_LINK_SEL_2_R {
        ETM_LINK_SEL_2_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - etm_link sel"]
    #[inline(always)]
    pub fn etm_link_sel_3(&self) -> ETM_LINK_SEL_3_R {
        ETM_LINK_SEL_3_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - debug"]
    #[inline(always)]
    pub fn etm_busy_cause(&self) -> ETM_BUSY_CAUSE_R {
        ETM_BUSY_CAUSE_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGDMA_ETM_CTRL")
            .field("etm_link_sel_0", &self.etm_link_sel_0())
            .field("etm_link_sel_1", &self.etm_link_sel_1())
            .field("etm_link_sel_2", &self.etm_link_sel_2())
            .field("etm_link_sel_3", &self.etm_link_sel_3())
            .field("etm_busy_cause", &self.etm_busy_cause())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - etm_start_0 reg"]
    #[inline(always)]
    pub fn etm_start_0(&mut self) -> ETM_START_0_W<'_, REGDMA_ETM_CTRL_SPEC> {
        ETM_START_0_W::new(self, 0)
    }
    #[doc = "Bit 1 - etm_start_1 reg"]
    #[inline(always)]
    pub fn etm_start_1(&mut self) -> ETM_START_1_W<'_, REGDMA_ETM_CTRL_SPEC> {
        ETM_START_1_W::new(self, 1)
    }
    #[doc = "Bit 2 - etm_start_2 reg"]
    #[inline(always)]
    pub fn etm_start_2(&mut self) -> ETM_START_2_W<'_, REGDMA_ETM_CTRL_SPEC> {
        ETM_START_2_W::new(self, 2)
    }
    #[doc = "Bit 3 - etm_start_3 reg"]
    #[inline(always)]
    pub fn etm_start_3(&mut self) -> ETM_START_3_W<'_, REGDMA_ETM_CTRL_SPEC> {
        ETM_START_3_W::new(self, 3)
    }
    #[doc = "Bits 4:7 - etm_link sel"]
    #[inline(always)]
    pub fn etm_link_sel_0(&mut self) -> ETM_LINK_SEL_0_W<'_, REGDMA_ETM_CTRL_SPEC> {
        ETM_LINK_SEL_0_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - etm_link sel"]
    #[inline(always)]
    pub fn etm_link_sel_1(&mut self) -> ETM_LINK_SEL_1_W<'_, REGDMA_ETM_CTRL_SPEC> {
        ETM_LINK_SEL_1_W::new(self, 8)
    }
    #[doc = "Bits 12:15 - etm_link sel"]
    #[inline(always)]
    pub fn etm_link_sel_2(&mut self) -> ETM_LINK_SEL_2_W<'_, REGDMA_ETM_CTRL_SPEC> {
        ETM_LINK_SEL_2_W::new(self, 12)
    }
    #[doc = "Bits 16:19 - etm_link sel"]
    #[inline(always)]
    pub fn etm_link_sel_3(&mut self) -> ETM_LINK_SEL_3_W<'_, REGDMA_ETM_CTRL_SPEC> {
        ETM_LINK_SEL_3_W::new(self, 16)
    }
}
#[doc = "ETM start ctrl reg\n\nYou can [`read`](crate::Reg::read) this register and get [`regdma_etm_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`regdma_etm_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGDMA_ETM_CTRL_SPEC;
impl crate::RegisterSpec for REGDMA_ETM_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`regdma_etm_ctrl::R`](R) reader structure"]
impl crate::Readable for REGDMA_ETM_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`regdma_etm_ctrl::W`](W) writer structure"]
impl crate::Writable for REGDMA_ETM_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REGDMA_ETM_CTRL to value 0"]
impl crate::Resettable for REGDMA_ETM_CTRL_SPEC {}
