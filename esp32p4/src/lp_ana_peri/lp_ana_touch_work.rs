#[doc = "Register `LP_ANA_TOUCH_WORK` reader"]
pub type R = crate::R<LP_ANA_TOUCH_WORK_SPEC>;
#[doc = "Register `LP_ANA_TOUCH_WORK` writer"]
pub type W = crate::W<LP_ANA_TOUCH_WORK_SPEC>;
#[doc = "Field `LP_ANA_DIV_NUM2` reader - need_des"]
pub type LP_ANA_DIV_NUM2_R = crate::FieldReader;
#[doc = "Field `LP_ANA_DIV_NUM2` writer - need_des"]
pub type LP_ANA_DIV_NUM2_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `LP_ANA_DIV_NUM1` reader - need_des"]
pub type LP_ANA_DIV_NUM1_R = crate::FieldReader;
#[doc = "Field `LP_ANA_DIV_NUM1` writer - need_des"]
pub type LP_ANA_DIV_NUM1_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `LP_ANA_DIV_NUM0` reader - need_des"]
pub type LP_ANA_DIV_NUM0_R = crate::FieldReader;
#[doc = "Field `LP_ANA_DIV_NUM0` writer - need_des"]
pub type LP_ANA_DIV_NUM0_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `LP_ANA_TOUCH_OUT_SEL` reader - need_des"]
pub type LP_ANA_TOUCH_OUT_SEL_R = crate::BitReader;
#[doc = "Field `LP_ANA_TOUCH_OUT_SEL` writer - need_des"]
pub type LP_ANA_TOUCH_OUT_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_ANA_TOUCH_OUT_RESET` writer - need_des"]
pub type LP_ANA_TOUCH_OUT_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_ANA_TOUCH_OUT_GATE` reader - need_des"]
pub type LP_ANA_TOUCH_OUT_GATE_R = crate::BitReader;
#[doc = "Field `LP_ANA_TOUCH_OUT_GATE` writer - need_des"]
pub type LP_ANA_TOUCH_OUT_GATE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 16:18 - need_des"]
    #[inline(always)]
    pub fn lp_ana_div_num2(&self) -> LP_ANA_DIV_NUM2_R {
        LP_ANA_DIV_NUM2_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:21 - need_des"]
    #[inline(always)]
    pub fn lp_ana_div_num1(&self) -> LP_ANA_DIV_NUM1_R {
        LP_ANA_DIV_NUM1_R::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bits 22:24 - need_des"]
    #[inline(always)]
    pub fn lp_ana_div_num0(&self) -> LP_ANA_DIV_NUM0_R {
        LP_ANA_DIV_NUM0_R::new(((self.bits >> 22) & 7) as u8)
    }
    #[doc = "Bit 25 - need_des"]
    #[inline(always)]
    pub fn lp_ana_touch_out_sel(&self) -> LP_ANA_TOUCH_OUT_SEL_R {
        LP_ANA_TOUCH_OUT_SEL_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn lp_ana_touch_out_gate(&self) -> LP_ANA_TOUCH_OUT_GATE_R {
        LP_ANA_TOUCH_OUT_GATE_R::new(((self.bits >> 27) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_ANA_TOUCH_WORK")
            .field(
                "lp_ana_div_num2",
                &format_args!("{}", self.lp_ana_div_num2().bits()),
            )
            .field(
                "lp_ana_div_num1",
                &format_args!("{}", self.lp_ana_div_num1().bits()),
            )
            .field(
                "lp_ana_div_num0",
                &format_args!("{}", self.lp_ana_div_num0().bits()),
            )
            .field(
                "lp_ana_touch_out_sel",
                &format_args!("{}", self.lp_ana_touch_out_sel().bit()),
            )
            .field(
                "lp_ana_touch_out_gate",
                &format_args!("{}", self.lp_ana_touch_out_gate().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LP_ANA_TOUCH_WORK_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 16:18 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_ana_div_num2(&mut self) -> LP_ANA_DIV_NUM2_W<LP_ANA_TOUCH_WORK_SPEC> {
        LP_ANA_DIV_NUM2_W::new(self, 16)
    }
    #[doc = "Bits 19:21 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_ana_div_num1(&mut self) -> LP_ANA_DIV_NUM1_W<LP_ANA_TOUCH_WORK_SPEC> {
        LP_ANA_DIV_NUM1_W::new(self, 19)
    }
    #[doc = "Bits 22:24 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_ana_div_num0(&mut self) -> LP_ANA_DIV_NUM0_W<LP_ANA_TOUCH_WORK_SPEC> {
        LP_ANA_DIV_NUM0_W::new(self, 22)
    }
    #[doc = "Bit 25 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_ana_touch_out_sel(&mut self) -> LP_ANA_TOUCH_OUT_SEL_W<LP_ANA_TOUCH_WORK_SPEC> {
        LP_ANA_TOUCH_OUT_SEL_W::new(self, 25)
    }
    #[doc = "Bit 26 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_ana_touch_out_reset(&mut self) -> LP_ANA_TOUCH_OUT_RESET_W<LP_ANA_TOUCH_WORK_SPEC> {
        LP_ANA_TOUCH_OUT_RESET_W::new(self, 26)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_ana_touch_out_gate(&mut self) -> LP_ANA_TOUCH_OUT_GATE_W<LP_ANA_TOUCH_WORK_SPEC> {
        LP_ANA_TOUCH_OUT_GATE_W::new(self, 27)
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
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_ana_touch_work::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_ana_touch_work::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_ANA_TOUCH_WORK_SPEC;
impl crate::RegisterSpec for LP_ANA_TOUCH_WORK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_ana_touch_work::R`](R) reader structure"]
impl crate::Readable for LP_ANA_TOUCH_WORK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lp_ana_touch_work::W`](W) writer structure"]
impl crate::Writable for LP_ANA_TOUCH_WORK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LP_ANA_TOUCH_WORK to value 0"]
impl crate::Resettable for LP_ANA_TOUCH_WORK_SPEC {
    const RESET_VALUE: u32 = 0;
}
