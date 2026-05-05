#[doc = "Register `SLEEP_CONF0` reader"]
pub type R = crate::R<SLEEP_CONF0_SPEC>;
#[doc = "Register `SLEEP_CONF0` writer"]
pub type W = crate::W<SLEEP_CONF0_SPEC>;
#[doc = "Field `LP_REG_SLV_WK_CHAR0` reader - NA"]
pub type LP_REG_SLV_WK_CHAR0_R = crate::FieldReader;
#[doc = "Field `LP_REG_SLV_WK_CHAR0` writer - NA"]
pub type LP_REG_SLV_WK_CHAR0_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LP_REG_SLV_WK_CHAR_NUM` reader - NA"]
pub type LP_REG_SLV_WK_CHAR_NUM_R = crate::FieldReader;
#[doc = "Field `LP_REG_SLV_WK_CHAR_NUM` writer - NA"]
pub type LP_REG_SLV_WK_CHAR_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `LP_REG_SLV_WK_CHAR_MASK` reader - NA"]
pub type LP_REG_SLV_WK_CHAR_MASK_R = crate::FieldReader;
#[doc = "Field `LP_REG_SLV_WK_CHAR_MASK` writer - NA"]
pub type LP_REG_SLV_WK_CHAR_MASK_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `LP_REG_SLV_WK_MODE_SEL` reader - NA"]
pub type LP_REG_SLV_WK_MODE_SEL_R = crate::BitReader;
#[doc = "Field `LP_REG_SLV_WK_MODE_SEL` writer - NA"]
pub type LP_REG_SLV_WK_MODE_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_REG_SLEEP_EN` reader - NA"]
pub type LP_REG_SLEEP_EN_R = crate::BitReader;
#[doc = "Field `LP_REG_SLEEP_EN` writer - NA"]
pub type LP_REG_SLEEP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_REG_SLEEP_DIS_RXFIFO_WR_EN` reader - NA"]
pub type LP_REG_SLEEP_DIS_RXFIFO_WR_EN_R = crate::BitReader;
#[doc = "Field `LP_REG_SLEEP_DIS_RXFIFO_WR_EN` writer - NA"]
pub type LP_REG_SLEEP_DIS_RXFIFO_WR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_REG_SLEEP_WK_DATA_SEL` reader - NA"]
pub type LP_REG_SLEEP_WK_DATA_SEL_R = crate::BitReader;
#[doc = "Field `LP_REG_SLEEP_WK_DATA_SEL` writer - NA"]
pub type LP_REG_SLEEP_WK_DATA_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - NA"]
    #[inline(always)]
    pub fn lp_reg_slv_wk_char0(&self) -> LP_REG_SLV_WK_CHAR0_R {
        LP_REG_SLV_WK_CHAR0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - NA"]
    #[inline(always)]
    pub fn lp_reg_slv_wk_char_num(&self) -> LP_REG_SLV_WK_CHAR_NUM_R {
        LP_REG_SLV_WK_CHAR_NUM_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:15 - NA"]
    #[inline(always)]
    pub fn lp_reg_slv_wk_char_mask(&self) -> LP_REG_SLV_WK_CHAR_MASK_R {
        LP_REG_SLV_WK_CHAR_MASK_R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bit 16 - NA"]
    #[inline(always)]
    pub fn lp_reg_slv_wk_mode_sel(&self) -> LP_REG_SLV_WK_MODE_SEL_R {
        LP_REG_SLV_WK_MODE_SEL_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - NA"]
    #[inline(always)]
    pub fn lp_reg_sleep_en(&self) -> LP_REG_SLEEP_EN_R {
        LP_REG_SLEEP_EN_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - NA"]
    #[inline(always)]
    pub fn lp_reg_sleep_dis_rxfifo_wr_en(&self) -> LP_REG_SLEEP_DIS_RXFIFO_WR_EN_R {
        LP_REG_SLEEP_DIS_RXFIFO_WR_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - NA"]
    #[inline(always)]
    pub fn lp_reg_sleep_wk_data_sel(&self) -> LP_REG_SLEEP_WK_DATA_SEL_R {
        LP_REG_SLEEP_WK_DATA_SEL_R::new(((self.bits >> 19) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLEEP_CONF0")
            .field("lp_reg_slv_wk_char0", &self.lp_reg_slv_wk_char0())
            .field("lp_reg_slv_wk_char_num", &self.lp_reg_slv_wk_char_num())
            .field("lp_reg_slv_wk_char_mask", &self.lp_reg_slv_wk_char_mask())
            .field("lp_reg_slv_wk_mode_sel", &self.lp_reg_slv_wk_mode_sel())
            .field("lp_reg_sleep_en", &self.lp_reg_sleep_en())
            .field(
                "lp_reg_sleep_dis_rxfifo_wr_en",
                &self.lp_reg_sleep_dis_rxfifo_wr_en(),
            )
            .field("lp_reg_sleep_wk_data_sel", &self.lp_reg_sleep_wk_data_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - NA"]
    #[inline(always)]
    pub fn lp_reg_slv_wk_char0(&mut self) -> LP_REG_SLV_WK_CHAR0_W<'_, SLEEP_CONF0_SPEC> {
        LP_REG_SLV_WK_CHAR0_W::new(self, 0)
    }
    #[doc = "Bits 8:10 - NA"]
    #[inline(always)]
    pub fn lp_reg_slv_wk_char_num(&mut self) -> LP_REG_SLV_WK_CHAR_NUM_W<'_, SLEEP_CONF0_SPEC> {
        LP_REG_SLV_WK_CHAR_NUM_W::new(self, 8)
    }
    #[doc = "Bits 11:15 - NA"]
    #[inline(always)]
    pub fn lp_reg_slv_wk_char_mask(&mut self) -> LP_REG_SLV_WK_CHAR_MASK_W<'_, SLEEP_CONF0_SPEC> {
        LP_REG_SLV_WK_CHAR_MASK_W::new(self, 11)
    }
    #[doc = "Bit 16 - NA"]
    #[inline(always)]
    pub fn lp_reg_slv_wk_mode_sel(&mut self) -> LP_REG_SLV_WK_MODE_SEL_W<'_, SLEEP_CONF0_SPEC> {
        LP_REG_SLV_WK_MODE_SEL_W::new(self, 16)
    }
    #[doc = "Bit 17 - NA"]
    #[inline(always)]
    pub fn lp_reg_sleep_en(&mut self) -> LP_REG_SLEEP_EN_W<'_, SLEEP_CONF0_SPEC> {
        LP_REG_SLEEP_EN_W::new(self, 17)
    }
    #[doc = "Bit 18 - NA"]
    #[inline(always)]
    pub fn lp_reg_sleep_dis_rxfifo_wr_en(
        &mut self,
    ) -> LP_REG_SLEEP_DIS_RXFIFO_WR_EN_W<'_, SLEEP_CONF0_SPEC> {
        LP_REG_SLEEP_DIS_RXFIFO_WR_EN_W::new(self, 18)
    }
    #[doc = "Bit 19 - NA"]
    #[inline(always)]
    pub fn lp_reg_sleep_wk_data_sel(&mut self) -> LP_REG_SLEEP_WK_DATA_SEL_W<'_, SLEEP_CONF0_SPEC> {
        LP_REG_SLEEP_WK_DATA_SEL_W::new(self, 19)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`sleep_conf0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sleep_conf0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLEEP_CONF0_SPEC;
impl crate::RegisterSpec for SLEEP_CONF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sleep_conf0::R`](R) reader structure"]
impl crate::Readable for SLEEP_CONF0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sleep_conf0::W`](W) writer structure"]
impl crate::Writable for SLEEP_CONF0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SLEEP_CONF0 to value 0"]
impl crate::Resettable for SLEEP_CONF0_SPEC {}
