#[doc = "Register `IN_ETM_CONF_CH2` reader"]
pub type R = crate::R<IN_ETM_CONF_CH2_SPEC>;
#[doc = "Register `IN_ETM_CONF_CH2` writer"]
pub type W = crate::W<IN_ETM_CONF_CH2_SPEC>;
#[doc = "Field `IN_ETM_EN_CH2` reader - Set this bit to 1 to enable ETM task function"]
pub type IN_ETM_EN_CH2_R = crate::BitReader;
#[doc = "Field `IN_ETM_EN_CH2` writer - Set this bit to 1 to enable ETM task function"]
pub type IN_ETM_EN_CH2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_ETM_LOOP_EN_CH2` reader - when this bit is 1, dscr can be processed after receiving a task"]
pub type IN_ETM_LOOP_EN_CH2_R = crate::BitReader;
#[doc = "Field `IN_ETM_LOOP_EN_CH2` writer - when this bit is 1, dscr can be processed after receiving a task"]
pub type IN_ETM_LOOP_EN_CH2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_DSCR_TASK_MAK_CH2` reader - ETM dscr_ready maximum cache numbers"]
pub type IN_DSCR_TASK_MAK_CH2_R = crate::FieldReader;
#[doc = "Field `IN_DSCR_TASK_MAK_CH2` writer - ETM dscr_ready maximum cache numbers"]
pub type IN_DSCR_TASK_MAK_CH2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Set this bit to 1 to enable ETM task function"]
    #[inline(always)]
    pub fn in_etm_en_ch2(&self) -> IN_ETM_EN_CH2_R {
        IN_ETM_EN_CH2_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - when this bit is 1, dscr can be processed after receiving a task"]
    #[inline(always)]
    pub fn in_etm_loop_en_ch2(&self) -> IN_ETM_LOOP_EN_CH2_R {
        IN_ETM_LOOP_EN_CH2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - ETM dscr_ready maximum cache numbers"]
    #[inline(always)]
    pub fn in_dscr_task_mak_ch2(&self) -> IN_DSCR_TASK_MAK_CH2_R {
        IN_DSCR_TASK_MAK_CH2_R::new(((self.bits >> 2) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN_ETM_CONF_CH2")
            .field(
                "in_etm_en_ch2",
                &format_args!("{}", self.in_etm_en_ch2().bit()),
            )
            .field(
                "in_etm_loop_en_ch2",
                &format_args!("{}", self.in_etm_loop_en_ch2().bit()),
            )
            .field(
                "in_dscr_task_mak_ch2",
                &format_args!("{}", self.in_dscr_task_mak_ch2().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IN_ETM_CONF_CH2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to 1 to enable ETM task function"]
    #[inline(always)]
    #[must_use]
    pub fn in_etm_en_ch2(&mut self) -> IN_ETM_EN_CH2_W<IN_ETM_CONF_CH2_SPEC> {
        IN_ETM_EN_CH2_W::new(self, 0)
    }
    #[doc = "Bit 1 - when this bit is 1, dscr can be processed after receiving a task"]
    #[inline(always)]
    #[must_use]
    pub fn in_etm_loop_en_ch2(&mut self) -> IN_ETM_LOOP_EN_CH2_W<IN_ETM_CONF_CH2_SPEC> {
        IN_ETM_LOOP_EN_CH2_W::new(self, 1)
    }
    #[doc = "Bits 2:3 - ETM dscr_ready maximum cache numbers"]
    #[inline(always)]
    #[must_use]
    pub fn in_dscr_task_mak_ch2(&mut self) -> IN_DSCR_TASK_MAK_CH2_W<IN_ETM_CONF_CH2_SPEC> {
        IN_DSCR_TASK_MAK_CH2_W::new(self, 2)
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
#[doc = "RX CH2 ETM config register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_etm_conf_ch2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`in_etm_conf_ch2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IN_ETM_CONF_CH2_SPEC;
impl crate::RegisterSpec for IN_ETM_CONF_CH2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_etm_conf_ch2::R`](R) reader structure"]
impl crate::Readable for IN_ETM_CONF_CH2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`in_etm_conf_ch2::W`](W) writer structure"]
impl crate::Writable for IN_ETM_CONF_CH2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IN_ETM_CONF_CH2 to value 0x04"]
impl crate::Resettable for IN_ETM_CONF_CH2_SPEC {
    const RESET_VALUE: Self::Ux = 0x04;
}
