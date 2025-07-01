#[doc = "Register `HIST_OFFS` reader"]
pub type R = crate::R<HIST_OFFS_SPEC>;
#[doc = "Register `HIST_OFFS` writer"]
pub type W = crate::W<HIST_OFFS_SPEC>;
#[doc = "Field `HIST_Y_OFFS` reader - this field configures y coordinate of first window"]
pub type HIST_Y_OFFS_R = crate::FieldReader<u16>;
#[doc = "Field `HIST_Y_OFFS` writer - this field configures y coordinate of first window"]
pub type HIST_Y_OFFS_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `HIST_X_OFFS` reader - this field configures x coordinate of first window"]
pub type HIST_X_OFFS_R = crate::FieldReader<u16>;
#[doc = "Field `HIST_X_OFFS` writer - this field configures x coordinate of first window"]
pub type HIST_X_OFFS_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - this field configures y coordinate of first window"]
    #[inline(always)]
    pub fn hist_y_offs(&self) -> HIST_Y_OFFS_R {
        HIST_Y_OFFS_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - this field configures x coordinate of first window"]
    #[inline(always)]
    pub fn hist_x_offs(&self) -> HIST_X_OFFS_R {
        HIST_X_OFFS_R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HIST_OFFS")
            .field("hist_y_offs", &self.hist_y_offs())
            .field("hist_x_offs", &self.hist_x_offs())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:11 - this field configures y coordinate of first window"]
    #[inline(always)]
    pub fn hist_y_offs(&mut self) -> HIST_Y_OFFS_W<HIST_OFFS_SPEC> {
        HIST_Y_OFFS_W::new(self, 0)
    }
    #[doc = "Bits 16:27 - this field configures x coordinate of first window"]
    #[inline(always)]
    pub fn hist_x_offs(&mut self) -> HIST_X_OFFS_W<HIST_OFFS_SPEC> {
        HIST_X_OFFS_W::new(self, 16)
    }
}
#[doc = "histogram window offsets register\n\nYou can [`read`](crate::Reg::read) this register and get [`hist_offs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hist_offs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HIST_OFFS_SPEC;
impl crate::RegisterSpec for HIST_OFFS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hist_offs::R`](R) reader structure"]
impl crate::Readable for HIST_OFFS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hist_offs::W`](W) writer structure"]
impl crate::Writable for HIST_OFFS_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HIST_OFFS to value 0"]
impl crate::Resettable for HIST_OFFS_SPEC {}
