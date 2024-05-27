#[doc = "Register `CVSD_CONF0` reader"]
pub type R = crate::R<CVSD_CONF0_SPEC>;
#[doc = "Register `CVSD_CONF0` writer"]
pub type W = crate::W<CVSD_CONF0_SPEC>;
#[doc = "Field `CVSD_Y_MAX` reader - "]
pub type CVSD_Y_MAX_R = crate::FieldReader<u16>;
#[doc = "Field `CVSD_Y_MAX` writer - "]
pub type CVSD_Y_MAX_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CVSD_Y_MIN` reader - "]
pub type CVSD_Y_MIN_R = crate::FieldReader<u16>;
#[doc = "Field `CVSD_Y_MIN` writer - "]
pub type CVSD_Y_MIN_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn cvsd_y_max(&self) -> CVSD_Y_MAX_R {
        CVSD_Y_MAX_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn cvsd_y_min(&self) -> CVSD_Y_MIN_R {
        CVSD_Y_MIN_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CVSD_CONF0")
            .field("cvsd_y_max", &self.cvsd_y_max())
            .field("cvsd_y_min", &self.cvsd_y_min())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    #[must_use]
    pub fn cvsd_y_max(&mut self) -> CVSD_Y_MAX_W<CVSD_CONF0_SPEC> {
        CVSD_Y_MAX_W::new(self, 0)
    }
    #[doc = "Bits 16:31"]
    #[inline(always)]
    #[must_use]
    pub fn cvsd_y_min(&mut self) -> CVSD_Y_MIN_W<CVSD_CONF0_SPEC> {
        CVSD_Y_MIN_W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cvsd_conf0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cvsd_conf0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CVSD_CONF0_SPEC;
impl crate::RegisterSpec for CVSD_CONF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cvsd_conf0::R`](R) reader structure"]
impl crate::Readable for CVSD_CONF0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cvsd_conf0::W`](W) writer structure"]
impl crate::Writable for CVSD_CONF0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CVSD_CONF0 to value 0x8000_7fff"]
impl crate::Resettable for CVSD_CONF0_SPEC {
    const RESET_VALUE: u32 = 0x8000_7fff;
}
