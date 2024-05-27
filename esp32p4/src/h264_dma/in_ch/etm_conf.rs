///Register `ETM_CONF` reader
pub type R = crate::R<ETM_CONF_SPEC>;
///Register `ETM_CONF` writer
pub type W = crate::W<ETM_CONF_SPEC>;
///Field `IN_ETM_EN` reader - Set this bit to 1 to enable ETM task function
pub type IN_ETM_EN_R = crate::BitReader;
///Field `IN_ETM_EN` writer - Set this bit to 1 to enable ETM task function
pub type IN_ETM_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IN_ETM_LOOP_EN` reader - when this bit is 1, dscr can be processed after receiving a task
pub type IN_ETM_LOOP_EN_R = crate::BitReader;
///Field `IN_ETM_LOOP_EN` writer - when this bit is 1, dscr can be processed after receiving a task
pub type IN_ETM_LOOP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IN_DSCR_TASK_MAK` reader - ETM dscr_ready maximum cache numbers
pub type IN_DSCR_TASK_MAK_R = crate::FieldReader;
///Field `IN_DSCR_TASK_MAK` writer - ETM dscr_ready maximum cache numbers
pub type IN_DSCR_TASK_MAK_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    ///Bit 0 - Set this bit to 1 to enable ETM task function
    #[inline(always)]
    pub fn in_etm_en(&self) -> IN_ETM_EN_R {
        IN_ETM_EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - when this bit is 1, dscr can be processed after receiving a task
    #[inline(always)]
    pub fn in_etm_loop_en(&self) -> IN_ETM_LOOP_EN_R {
        IN_ETM_LOOP_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:3 - ETM dscr_ready maximum cache numbers
    #[inline(always)]
    pub fn in_dscr_task_mak(&self) -> IN_DSCR_TASK_MAK_R {
        IN_DSCR_TASK_MAK_R::new(((self.bits >> 2) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ETM_CONF")
            .field("in_etm_en", &self.in_etm_en())
            .field("in_etm_loop_en", &self.in_etm_loop_en())
            .field("in_dscr_task_mak", &self.in_dscr_task_mak())
            .finish()
    }
}
impl W {
    ///Bit 0 - Set this bit to 1 to enable ETM task function
    #[inline(always)]
    #[must_use]
    pub fn in_etm_en(&mut self) -> IN_ETM_EN_W<ETM_CONF_SPEC> {
        IN_ETM_EN_W::new(self, 0)
    }
    ///Bit 1 - when this bit is 1, dscr can be processed after receiving a task
    #[inline(always)]
    #[must_use]
    pub fn in_etm_loop_en(&mut self) -> IN_ETM_LOOP_EN_W<ETM_CONF_SPEC> {
        IN_ETM_LOOP_EN_W::new(self, 1)
    }
    ///Bits 2:3 - ETM dscr_ready maximum cache numbers
    #[inline(always)]
    #[must_use]
    pub fn in_dscr_task_mak(&mut self) -> IN_DSCR_TASK_MAK_W<ETM_CONF_SPEC> {
        IN_DSCR_TASK_MAK_W::new(self, 2)
    }
}
/**RX CHx ETM config register

You can [`read`](crate::generic::Reg::read) this register and get [`etm_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etm_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct ETM_CONF_SPEC;
impl crate::RegisterSpec for ETM_CONF_SPEC {
    type Ux = u32;
}
///`read()` method returns [`etm_conf::R`](R) reader structure
impl crate::Readable for ETM_CONF_SPEC {}
///`write(|w| ..)` method takes [`etm_conf::W`](W) writer structure
impl crate::Writable for ETM_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets ETM_CONF to value 0x04
impl crate::Resettable for ETM_CONF_SPEC {
    const RESET_VALUE: u32 = 0x04;
}
