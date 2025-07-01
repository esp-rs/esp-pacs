#[doc = "Register `HIST_SIZE` reader"]
pub type R = crate::R<HIST_SIZE_SPEC>;
#[doc = "Register `HIST_SIZE` writer"]
pub type W = crate::W<HIST_SIZE_SPEC>;
#[doc = "Field `HIST_Y_SIZE` reader - this field configures y direction size of subwindow"]
pub type HIST_Y_SIZE_R = crate::FieldReader<u16>;
#[doc = "Field `HIST_Y_SIZE` writer - this field configures y direction size of subwindow"]
pub type HIST_Y_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `HIST_X_SIZE` reader - this field configures x direction size of subwindow"]
pub type HIST_X_SIZE_R = crate::FieldReader<u16>;
#[doc = "Field `HIST_X_SIZE` writer - this field configures x direction size of subwindow"]
pub type HIST_X_SIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:8 - this field configures y direction size of subwindow"]
    #[inline(always)]
    pub fn hist_y_size(&self) -> HIST_Y_SIZE_R {
        HIST_Y_SIZE_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 16:24 - this field configures x direction size of subwindow"]
    #[inline(always)]
    pub fn hist_x_size(&self) -> HIST_X_SIZE_R {
        HIST_X_SIZE_R::new(((self.bits >> 16) & 0x01ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HIST_SIZE")
            .field("hist_y_size", &self.hist_y_size())
            .field("hist_x_size", &self.hist_x_size())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:8 - this field configures y direction size of subwindow"]
    #[inline(always)]
    pub fn hist_y_size(&mut self) -> HIST_Y_SIZE_W<HIST_SIZE_SPEC> {
        HIST_Y_SIZE_W::new(self, 0)
    }
    #[doc = "Bits 16:24 - this field configures x direction size of subwindow"]
    #[inline(always)]
    pub fn hist_x_size(&mut self) -> HIST_X_SIZE_W<HIST_SIZE_SPEC> {
        HIST_X_SIZE_W::new(self, 16)
    }
}
#[doc = "histogram sub-window size register\n\nYou can [`read`](crate::Reg::read) this register and get [`hist_size::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hist_size::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HIST_SIZE_SPEC;
impl crate::RegisterSpec for HIST_SIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hist_size::R`](R) reader structure"]
impl crate::Readable for HIST_SIZE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hist_size::W`](W) writer structure"]
impl crate::Writable for HIST_SIZE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HIST_SIZE to value 0x0012_0020"]
impl crate::Resettable for HIST_SIZE_SPEC {
    const RESET_VALUE: u32 = 0x0012_0020;
}
