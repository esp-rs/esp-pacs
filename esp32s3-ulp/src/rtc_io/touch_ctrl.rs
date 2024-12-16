#[doc = "Register `TOUCH_CTRL` reader"]
pub type R = crate::R<TOUCH_CTRL_SPEC>;
#[doc = "Register `TOUCH_CTRL` writer"]
pub type W = crate::W<TOUCH_CTRL_SPEC>;
#[doc = "Field `IO_TOUCH_BUFSEL` reader - BUF_SEL when touch work without fsm"]
pub type IO_TOUCH_BUFSEL_R = crate::FieldReader;
#[doc = "Field `IO_TOUCH_BUFSEL` writer - BUF_SEL when touch work without fsm"]
pub type IO_TOUCH_BUFSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `IO_TOUCH_BUFMODE` reader - BUF_MODE when touch work without fsm"]
pub type IO_TOUCH_BUFMODE_R = crate::BitReader;
#[doc = "Field `IO_TOUCH_BUFMODE` writer - BUF_MODE when touch work without fsm"]
pub type IO_TOUCH_BUFMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - BUF_SEL when touch work without fsm"]
    #[inline(always)]
    pub fn io_touch_bufsel(&self) -> IO_TOUCH_BUFSEL_R {
        IO_TOUCH_BUFSEL_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - BUF_MODE when touch work without fsm"]
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
    #[doc = "Bits 0:3 - BUF_SEL when touch work without fsm"]
    #[inline(always)]
    pub fn io_touch_bufsel(&mut self) -> IO_TOUCH_BUFSEL_W<TOUCH_CTRL_SPEC> {
        IO_TOUCH_BUFSEL_W::new(self, 0)
    }
    #[doc = "Bit 4 - BUF_MODE when touch work without fsm"]
    #[inline(always)]
    pub fn io_touch_bufmode(&mut self) -> IO_TOUCH_BUFMODE_W<TOUCH_CTRL_SPEC> {
        IO_TOUCH_BUFMODE_W::new(self, 4)
    }
}
#[doc = "configure touch pad bufmode\n\nYou can [`read`](crate::Reg::read) this register and get [`touch_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`touch_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TOUCH_CTRL_SPEC;
impl crate::RegisterSpec for TOUCH_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`touch_ctrl::R`](R) reader structure"]
impl crate::Readable for TOUCH_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`touch_ctrl::W`](W) writer structure"]
impl crate::Writable for TOUCH_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TOUCH_CTRL to value 0"]
impl crate::Resettable for TOUCH_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
