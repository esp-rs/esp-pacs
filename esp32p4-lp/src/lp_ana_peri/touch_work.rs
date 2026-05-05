#[doc = "Register `TOUCH_WORK` reader"]
pub type R = crate::R<TOUCH_WORK_SPEC>;
#[doc = "Register `TOUCH_WORK` writer"]
pub type W = crate::W<TOUCH_WORK_SPEC>;
#[doc = "Field `DIV_NUM2` reader - need_des"]
pub type DIV_NUM2_R = crate::FieldReader;
#[doc = "Field `DIV_NUM2` writer - need_des"]
pub type DIV_NUM2_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DIV_NUM1` reader - need_des"]
pub type DIV_NUM1_R = crate::FieldReader;
#[doc = "Field `DIV_NUM1` writer - need_des"]
pub type DIV_NUM1_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DIV_NUM0` reader - need_des"]
pub type DIV_NUM0_R = crate::FieldReader;
#[doc = "Field `DIV_NUM0` writer - need_des"]
pub type DIV_NUM0_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TOUCH_OUT_SEL` reader - need_des"]
pub type TOUCH_OUT_SEL_R = crate::BitReader;
#[doc = "Field `TOUCH_OUT_SEL` writer - need_des"]
pub type TOUCH_OUT_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOUCH_OUT_RESET` writer - need_des"]
pub type TOUCH_OUT_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOUCH_OUT_GATE` reader - need_des"]
pub type TOUCH_OUT_GATE_R = crate::BitReader;
#[doc = "Field `TOUCH_OUT_GATE` writer - need_des"]
pub type TOUCH_OUT_GATE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 16:18 - need_des"]
    #[inline(always)]
    pub fn div_num2(&self) -> DIV_NUM2_R {
        DIV_NUM2_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:21 - need_des"]
    #[inline(always)]
    pub fn div_num1(&self) -> DIV_NUM1_R {
        DIV_NUM1_R::new(((self.bits >> 19) & 7) as u8)
    }
    #[doc = "Bits 22:24 - need_des"]
    #[inline(always)]
    pub fn div_num0(&self) -> DIV_NUM0_R {
        DIV_NUM0_R::new(((self.bits >> 22) & 7) as u8)
    }
    #[doc = "Bit 25 - need_des"]
    #[inline(always)]
    pub fn touch_out_sel(&self) -> TOUCH_OUT_SEL_R {
        TOUCH_OUT_SEL_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn touch_out_gate(&self) -> TOUCH_OUT_GATE_R {
        TOUCH_OUT_GATE_R::new(((self.bits >> 27) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TOUCH_WORK")
            .field("div_num2", &self.div_num2())
            .field("div_num1", &self.div_num1())
            .field("div_num0", &self.div_num0())
            .field("touch_out_sel", &self.touch_out_sel())
            .field("touch_out_gate", &self.touch_out_gate())
            .finish()
    }
}
impl W {
    #[doc = "Bits 16:18 - need_des"]
    #[inline(always)]
    pub fn div_num2(&mut self) -> DIV_NUM2_W<'_, TOUCH_WORK_SPEC> {
        DIV_NUM2_W::new(self, 16)
    }
    #[doc = "Bits 19:21 - need_des"]
    #[inline(always)]
    pub fn div_num1(&mut self) -> DIV_NUM1_W<'_, TOUCH_WORK_SPEC> {
        DIV_NUM1_W::new(self, 19)
    }
    #[doc = "Bits 22:24 - need_des"]
    #[inline(always)]
    pub fn div_num0(&mut self) -> DIV_NUM0_W<'_, TOUCH_WORK_SPEC> {
        DIV_NUM0_W::new(self, 22)
    }
    #[doc = "Bit 25 - need_des"]
    #[inline(always)]
    pub fn touch_out_sel(&mut self) -> TOUCH_OUT_SEL_W<'_, TOUCH_WORK_SPEC> {
        TOUCH_OUT_SEL_W::new(self, 25)
    }
    #[doc = "Bit 26 - need_des"]
    #[inline(always)]
    pub fn touch_out_reset(&mut self) -> TOUCH_OUT_RESET_W<'_, TOUCH_WORK_SPEC> {
        TOUCH_OUT_RESET_W::new(self, 26)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn touch_out_gate(&mut self) -> TOUCH_OUT_GATE_W<'_, TOUCH_WORK_SPEC> {
        TOUCH_OUT_GATE_W::new(self, 27)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`touch_work::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`touch_work::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TOUCH_WORK_SPEC;
impl crate::RegisterSpec for TOUCH_WORK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`touch_work::R`](R) reader structure"]
impl crate::Readable for TOUCH_WORK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`touch_work::W`](W) writer structure"]
impl crate::Writable for TOUCH_WORK_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TOUCH_WORK to value 0"]
impl crate::Resettable for TOUCH_WORK_SPEC {}
