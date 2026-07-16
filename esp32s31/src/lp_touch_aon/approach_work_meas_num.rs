#[doc = "Register `APPROACH_WORK_MEAS_NUM` reader"]
pub type R = crate::R<APPROACH_WORK_MEAS_NUM_SPEC>;
#[doc = "Register `APPROACH_WORK_MEAS_NUM` writer"]
pub type W = crate::W<APPROACH_WORK_MEAS_NUM_SPEC>;
#[doc = "Field `TOUCH_APPROACH_MEAS_NUM2` reader - need_des"]
pub type TOUCH_APPROACH_MEAS_NUM2_R = crate::FieldReader<u16>;
#[doc = "Field `TOUCH_APPROACH_MEAS_NUM2` writer - need_des"]
pub type TOUCH_APPROACH_MEAS_NUM2_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `TOUCH_APPROACH_MEAS_NUM1` reader - need_des"]
pub type TOUCH_APPROACH_MEAS_NUM1_R = crate::FieldReader<u16>;
#[doc = "Field `TOUCH_APPROACH_MEAS_NUM1` writer - need_des"]
pub type TOUCH_APPROACH_MEAS_NUM1_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `TOUCH_APPROACH_MEAS_NUM0` reader - need_des"]
pub type TOUCH_APPROACH_MEAS_NUM0_R = crate::FieldReader<u16>;
#[doc = "Field `TOUCH_APPROACH_MEAS_NUM0` writer - need_des"]
pub type TOUCH_APPROACH_MEAS_NUM0_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - need_des"]
    #[inline(always)]
    pub fn touch_approach_meas_num2(&self) -> TOUCH_APPROACH_MEAS_NUM2_R {
        TOUCH_APPROACH_MEAS_NUM2_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:19 - need_des"]
    #[inline(always)]
    pub fn touch_approach_meas_num1(&self) -> TOUCH_APPROACH_MEAS_NUM1_R {
        TOUCH_APPROACH_MEAS_NUM1_R::new(((self.bits >> 10) & 0x03ff) as u16)
    }
    #[doc = "Bits 20:29 - need_des"]
    #[inline(always)]
    pub fn touch_approach_meas_num0(&self) -> TOUCH_APPROACH_MEAS_NUM0_R {
        TOUCH_APPROACH_MEAS_NUM0_R::new(((self.bits >> 20) & 0x03ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APPROACH_WORK_MEAS_NUM")
            .field("touch_approach_meas_num2", &self.touch_approach_meas_num2())
            .field("touch_approach_meas_num1", &self.touch_approach_meas_num1())
            .field("touch_approach_meas_num0", &self.touch_approach_meas_num0())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:9 - need_des"]
    #[inline(always)]
    pub fn touch_approach_meas_num2(
        &mut self,
    ) -> TOUCH_APPROACH_MEAS_NUM2_W<'_, APPROACH_WORK_MEAS_NUM_SPEC> {
        TOUCH_APPROACH_MEAS_NUM2_W::new(self, 0)
    }
    #[doc = "Bits 10:19 - need_des"]
    #[inline(always)]
    pub fn touch_approach_meas_num1(
        &mut self,
    ) -> TOUCH_APPROACH_MEAS_NUM1_W<'_, APPROACH_WORK_MEAS_NUM_SPEC> {
        TOUCH_APPROACH_MEAS_NUM1_W::new(self, 10)
    }
    #[doc = "Bits 20:29 - need_des"]
    #[inline(always)]
    pub fn touch_approach_meas_num0(
        &mut self,
    ) -> TOUCH_APPROACH_MEAS_NUM0_W<'_, APPROACH_WORK_MEAS_NUM_SPEC> {
        TOUCH_APPROACH_MEAS_NUM0_W::new(self, 20)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`approach_work_meas_num::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`approach_work_meas_num::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APPROACH_WORK_MEAS_NUM_SPEC;
impl crate::RegisterSpec for APPROACH_WORK_MEAS_NUM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`approach_work_meas_num::R`](R) reader structure"]
impl crate::Readable for APPROACH_WORK_MEAS_NUM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`approach_work_meas_num::W`](W) writer structure"]
impl crate::Writable for APPROACH_WORK_MEAS_NUM_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APPROACH_WORK_MEAS_NUM to value 0x0641_9064"]
impl crate::Resettable for APPROACH_WORK_MEAS_NUM_SPEC {
    const RESET_VALUE: u32 = 0x0641_9064;
}
