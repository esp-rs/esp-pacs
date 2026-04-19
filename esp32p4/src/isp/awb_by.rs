#[doc = "Register `AWB_BY` reader"]
pub type R = crate::R<AWB_BY_SPEC>;
#[doc = "Register `AWB_BY` writer"]
pub type W = crate::W<AWB_BY_SPEC>;
#[doc = "Field `AWB_Y_BSIZE` reader - Configures every block y size"]
pub type AWB_Y_BSIZE_R = crate::FieldReader<u16>;
#[doc = "Field `AWB_Y_BSIZE` writer - Configures every block y size"]
pub type AWB_Y_BSIZE_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `AWB_Y_START` reader - Configures first block start y address"]
pub type AWB_Y_START_R = crate::FieldReader<u16>;
#[doc = "Field `AWB_Y_START` writer - Configures first block start y address"]
pub type AWB_Y_START_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Configures every block y size"]
    #[inline(always)]
    pub fn awb_y_bsize(&self) -> AWB_Y_BSIZE_R {
        AWB_Y_BSIZE_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 12:23 - Configures first block start y address"]
    #[inline(always)]
    pub fn awb_y_start(&self) -> AWB_Y_START_R {
        AWB_Y_START_R::new(((self.bits >> 12) & 0x0fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AWB_BY")
            .field("awb_y_bsize", &self.awb_y_bsize())
            .field("awb_y_start", &self.awb_y_start())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:11 - Configures every block y size"]
    #[inline(always)]
    pub fn awb_y_bsize(&mut self) -> AWB_Y_BSIZE_W<'_, AWB_BY_SPEC> {
        AWB_Y_BSIZE_W::new(self, 0)
    }
    #[doc = "Bits 12:23 - Configures first block start y address"]
    #[inline(always)]
    pub fn awb_y_start(&mut self) -> AWB_Y_START_W<'_, AWB_BY_SPEC> {
        AWB_Y_START_W::new(self, 12)
    }
}
#[doc = "awb window register in y-direction\n\nYou can [`read`](crate::Reg::read) this register and get [`awb_by::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awb_by::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AWB_BY_SPEC;
impl crate::RegisterSpec for AWB_BY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`awb_by::R`](R) reader structure"]
impl crate::Readable for AWB_BY_SPEC {}
#[doc = "`write(|w| ..)` method takes [`awb_by::W`](W) writer structure"]
impl crate::Writable for AWB_BY_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AWB_BY to value 0"]
impl crate::Resettable for AWB_BY_SPEC {}
