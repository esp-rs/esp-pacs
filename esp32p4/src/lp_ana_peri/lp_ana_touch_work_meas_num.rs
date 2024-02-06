#[doc = "Register `LP_ANA_TOUCH_WORK_MEAS_NUM` reader"]
pub type R = crate::R<LP_ANA_TOUCH_WORK_MEAS_NUM_SPEC>;
#[doc = "Register `LP_ANA_TOUCH_WORK_MEAS_NUM` writer"]
pub type W = crate::W<LP_ANA_TOUCH_WORK_MEAS_NUM_SPEC>;
#[doc = "Field `LP_ANA_TOUCH_MEAS_NUM2` reader - need_des"]
pub type LP_ANA_TOUCH_MEAS_NUM2_R = crate::FieldReader<u16>;
#[doc = "Field `LP_ANA_TOUCH_MEAS_NUM2` writer - need_des"]
pub type LP_ANA_TOUCH_MEAS_NUM2_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `LP_ANA_TOUCH_MEAS_NUM1` reader - need_des"]
pub type LP_ANA_TOUCH_MEAS_NUM1_R = crate::FieldReader<u16>;
#[doc = "Field `LP_ANA_TOUCH_MEAS_NUM1` writer - need_des"]
pub type LP_ANA_TOUCH_MEAS_NUM1_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `LP_ANA_TOUCH_MEAS_NUM0` reader - need_des"]
pub type LP_ANA_TOUCH_MEAS_NUM0_R = crate::FieldReader<u16>;
#[doc = "Field `LP_ANA_TOUCH_MEAS_NUM0` writer - need_des"]
pub type LP_ANA_TOUCH_MEAS_NUM0_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - need_des"]
    #[inline(always)]
    pub fn lp_ana_touch_meas_num2(&self) -> LP_ANA_TOUCH_MEAS_NUM2_R {
        LP_ANA_TOUCH_MEAS_NUM2_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:19 - need_des"]
    #[inline(always)]
    pub fn lp_ana_touch_meas_num1(&self) -> LP_ANA_TOUCH_MEAS_NUM1_R {
        LP_ANA_TOUCH_MEAS_NUM1_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    #[doc = "Bits 20:29 - need_des"]
    #[inline(always)]
    pub fn lp_ana_touch_meas_num0(&self) -> LP_ANA_TOUCH_MEAS_NUM0_R {
        LP_ANA_TOUCH_MEAS_NUM0_R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_ANA_TOUCH_WORK_MEAS_NUM")
            .field(
                "lp_ana_touch_meas_num2",
                &format_args!("{}", self.lp_ana_touch_meas_num2().bits()),
            )
            .field(
                "lp_ana_touch_meas_num1",
                &format_args!("{}", self.lp_ana_touch_meas_num1().bits()),
            )
            .field(
                "lp_ana_touch_meas_num0",
                &format_args!("{}", self.lp_ana_touch_meas_num0().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LP_ANA_TOUCH_WORK_MEAS_NUM_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:9 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_ana_touch_meas_num2(
        &mut self,
    ) -> LP_ANA_TOUCH_MEAS_NUM2_W<LP_ANA_TOUCH_WORK_MEAS_NUM_SPEC> {
        LP_ANA_TOUCH_MEAS_NUM2_W::new(self, 0)
    }
    #[doc = "Bits 10:19 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_ana_touch_meas_num1(
        &mut self,
    ) -> LP_ANA_TOUCH_MEAS_NUM1_W<LP_ANA_TOUCH_WORK_MEAS_NUM_SPEC> {
        LP_ANA_TOUCH_MEAS_NUM1_W::new(self, 10)
    }
    #[doc = "Bits 20:29 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_ana_touch_meas_num0(
        &mut self,
    ) -> LP_ANA_TOUCH_MEAS_NUM0_W<LP_ANA_TOUCH_WORK_MEAS_NUM_SPEC> {
        LP_ANA_TOUCH_MEAS_NUM0_W::new(self, 20)
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
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_ana_touch_work_meas_num::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_ana_touch_work_meas_num::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_ANA_TOUCH_WORK_MEAS_NUM_SPEC;
impl crate::RegisterSpec for LP_ANA_TOUCH_WORK_MEAS_NUM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_ana_touch_work_meas_num::R`](R) reader structure"]
impl crate::Readable for LP_ANA_TOUCH_WORK_MEAS_NUM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lp_ana_touch_work_meas_num::W`](W) writer structure"]
impl crate::Writable for LP_ANA_TOUCH_WORK_MEAS_NUM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LP_ANA_TOUCH_WORK_MEAS_NUM to value 0x0641_9064"]
impl crate::Resettable for LP_ANA_TOUCH_WORK_MEAS_NUM_SPEC {
    const RESET_VALUE: u32 = 0x0641_9064;
}
