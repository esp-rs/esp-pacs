///Register `TOUCH_CTRL` reader
pub type R = crate::R<TOUCH_CTRL_SPEC>;
///Register `TOUCH_CTRL` writer
pub type W = crate::W<TOUCH_CTRL_SPEC>;
///Field `IO_TOUCH_BUFSEL` reader - BUF_SEL when touch work without fsm
pub type IO_TOUCH_BUFSEL_R = crate::FieldReader;
///Field `IO_TOUCH_BUFSEL` writer - BUF_SEL when touch work without fsm
pub type IO_TOUCH_BUFSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `IO_TOUCH_BUFMODE` reader - BUF_MODE when touch work without fsm
pub type IO_TOUCH_BUFMODE_R = crate::BitReader;
///Field `IO_TOUCH_BUFMODE` writer - BUF_MODE when touch work without fsm
pub type IO_TOUCH_BUFMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:3 - BUF_SEL when touch work without fsm
    #[inline(always)]
    pub fn io_touch_bufsel(&self) -> IO_TOUCH_BUFSEL_R {
        IO_TOUCH_BUFSEL_R::new((self.bits & 0x0f) as u8)
    }
    ///Bit 4 - BUF_MODE when touch work without fsm
    #[inline(always)]
    pub fn io_touch_bufmode(&self) -> IO_TOUCH_BUFMODE_R {
        IO_TOUCH_BUFMODE_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TOUCH_CTRL")
            .field("io_touch_bufsel", &self.io_touch_bufsel())
            .field("io_touch_bufmode", &self.io_touch_bufmode())
            .finish()
    }
}
impl W {
    ///Bits 0:3 - BUF_SEL when touch work without fsm
    #[inline(always)]
    #[must_use]
    pub fn io_touch_bufsel(&mut self) -> IO_TOUCH_BUFSEL_W<TOUCH_CTRL_SPEC> {
        IO_TOUCH_BUFSEL_W::new(self, 0)
    }
    ///Bit 4 - BUF_MODE when touch work without fsm
    #[inline(always)]
    #[must_use]
    pub fn io_touch_bufmode(&mut self) -> IO_TOUCH_BUFMODE_W<TOUCH_CTRL_SPEC> {
        IO_TOUCH_BUFMODE_W::new(self, 4)
    }
}
/**configure touch pad bufmode

You can [`read`](crate::generic::Reg::read) this register and get [`touch_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`touch_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TOUCH_CTRL_SPEC;
impl crate::RegisterSpec for TOUCH_CTRL_SPEC {
    type Ux = u32;
}
///`read()` method returns [`touch_ctrl::R`](R) reader structure
impl crate::Readable for TOUCH_CTRL_SPEC {}
///`write(|w| ..)` method takes [`touch_ctrl::W`](W) writer structure
impl crate::Writable for TOUCH_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets TOUCH_CTRL to value 0
impl crate::Resettable for TOUCH_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
