#[doc = "Register `AT_CMD_PRECNT` reader"]
pub type R = crate::R<AT_CMD_PRECNT_SPEC>;
#[doc = "Register `AT_CMD_PRECNT` writer"]
pub type W = crate::W<AT_CMD_PRECNT_SPEC>;
#[doc = "Field `PRE_IDLE_NUM` reader - This register is used to configure the idle duration time before the first at_cmd is received by receiver. when the the duration is less than this register value it will not take the next data received as at_cmd char."]
pub type PRE_IDLE_NUM_R = crate::FieldReader<u32>;
#[doc = "Field `PRE_IDLE_NUM` writer - This register is used to configure the idle duration time before the first at_cmd is received by receiver. when the the duration is less than this register value it will not take the next data received as at_cmd char."]
pub type PRE_IDLE_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - This register is used to configure the idle duration time before the first at_cmd is received by receiver. when the the duration is less than this register value it will not take the next data received as at_cmd char."]
    #[inline(always)]
    pub fn pre_idle_num(&self) -> PRE_IDLE_NUM_R {
        PRE_IDLE_NUM_R::new(self.bits & 0x00ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AT_CMD_PRECNT")
            .field("pre_idle_num", &self.pre_idle_num())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:23 - This register is used to configure the idle duration time before the first at_cmd is received by receiver. when the the duration is less than this register value it will not take the next data received as at_cmd char."]
    #[inline(always)]
    pub fn pre_idle_num(&mut self) -> PRE_IDLE_NUM_W<AT_CMD_PRECNT_SPEC> {
        PRE_IDLE_NUM_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`at_cmd_precnt::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`at_cmd_precnt::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AT_CMD_PRECNT_SPEC;
impl crate::RegisterSpec for AT_CMD_PRECNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`at_cmd_precnt::R`](R) reader structure"]
impl crate::Readable for AT_CMD_PRECNT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`at_cmd_precnt::W`](W) writer structure"]
impl crate::Writable for AT_CMD_PRECNT_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AT_CMD_PRECNT to value 0x0018_6a00"]
impl crate::Resettable for AT_CMD_PRECNT_SPEC {
    const RESET_VALUE: u32 = 0x0018_6a00;
}
