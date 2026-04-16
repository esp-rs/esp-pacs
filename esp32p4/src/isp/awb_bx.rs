#[doc = "Register `AWB_BX` reader"]
pub type R = crate::R<AWB_BX_SPEC>;
#[doc = "Register `AWB_BX` writer"]
pub type W = crate::W<AWB_BX_SPEC>;
#[doc = "Field `AWB_X_BSIZE` reader - Configures every block x size, min number is 4"]
pub type AWB_X_BSIZE_R = crate::FieldReader<u16>;
#[doc = "Field `AWB_X_BSIZE` writer - Configures every block x size, min number is 4"]
pub type AWB_X_BSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `AWB_X_START` reader - Configures first block start x address"]
pub type AWB_X_START_R = crate::FieldReader<u16>;
#[doc = "Field `AWB_X_START` writer - Configures first block start x address"]
pub type AWB_X_START_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Configures every block x size, min number is 4"]
    #[inline(always)]
    pub fn awb_x_bsize(&self) -> AWB_X_BSIZE_R {
        AWB_X_BSIZE_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:23 - Configures first block start x address"]
    #[inline(always)]
    pub fn awb_x_start(&self) -> AWB_X_START_R {
        AWB_X_START_R::new(((self.bits >> 12) & 0x0fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AWB_BX")
            .field("awb_x_bsize", &self.awb_x_bsize())
            .field("awb_x_start", &self.awb_x_start())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:11 - Configures every block x size, min number is 4"]
    #[inline(always)]
    pub fn awb_x_bsize(&mut self) -> AWB_X_BSIZE_W<'_, AWB_BX_SPEC> {
        AWB_X_BSIZE_W::new(self, 0)
    }
    #[doc = "Bits 12:23 - Configures first block start x address"]
    #[inline(always)]
    pub fn awb_x_start(&mut self) -> AWB_X_START_W<'_, AWB_BX_SPEC> {
        AWB_X_START_W::new(self, 12)
    }
}
#[doc = "awb window register in x-direction\n\nYou can [`read`](crate::Reg::read) this register and get [`awb_bx::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awb_bx::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AWB_BX_SPEC;
impl crate::RegisterSpec for AWB_BX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`awb_bx::R`](R) reader structure"]
impl crate::Readable for AWB_BX_SPEC {}
#[doc = "`write(|w| ..)` method takes [`awb_bx::W`](W) writer structure"]
impl crate::Writable for AWB_BX_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AWB_BX to value 0"]
impl crate::Resettable for AWB_BX_SPEC {}
