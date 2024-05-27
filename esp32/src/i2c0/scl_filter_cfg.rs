#[doc = "Register `SCL_FILTER_CFG` reader"]
pub type R = crate::R<SCL_FILTER_CFG_SPEC>;
#[doc = "Register `SCL_FILTER_CFG` writer"]
pub type W = crate::W<SCL_FILTER_CFG_SPEC>;
#[doc = "Field `SCL_FILTER_THRES` reader - When input SCL's pulse width is smaller than this register value I2C ignores this pulse."]
pub type SCL_FILTER_THRES_R = crate::FieldReader;
#[doc = "Field `SCL_FILTER_THRES` writer - When input SCL's pulse width is smaller than this register value I2C ignores this pulse."]
pub type SCL_FILTER_THRES_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SCL_FILTER_EN` reader - This is the filter enable bit for SCL."]
pub type SCL_FILTER_EN_R = crate::BitReader;
#[doc = "Field `SCL_FILTER_EN` writer - This is the filter enable bit for SCL."]
pub type SCL_FILTER_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - When input SCL's pulse width is smaller than this register value I2C ignores this pulse."]
    #[inline(always)]
    pub fn scl_filter_thres(&self) -> SCL_FILTER_THRES_R {
        SCL_FILTER_THRES_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - This is the filter enable bit for SCL."]
    #[inline(always)]
    pub fn scl_filter_en(&self) -> SCL_FILTER_EN_R {
        SCL_FILTER_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SCL_FILTER_CFG")
            .field("scl_filter_thres", &self.scl_filter_thres())
            .field("scl_filter_en", &self.scl_filter_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - When input SCL's pulse width is smaller than this register value I2C ignores this pulse."]
    #[inline(always)]
    #[must_use]
    pub fn scl_filter_thres(&mut self) -> SCL_FILTER_THRES_W<SCL_FILTER_CFG_SPEC> {
        SCL_FILTER_THRES_W::new(self, 0)
    }
    #[doc = "Bit 3 - This is the filter enable bit for SCL."]
    #[inline(always)]
    #[must_use]
    pub fn scl_filter_en(&mut self) -> SCL_FILTER_EN_W<SCL_FILTER_CFG_SPEC> {
        SCL_FILTER_EN_W::new(self, 3)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`scl_filter_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scl_filter_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SCL_FILTER_CFG_SPEC;
impl crate::RegisterSpec for SCL_FILTER_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scl_filter_cfg::R`](R) reader structure"]
impl crate::Readable for SCL_FILTER_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`scl_filter_cfg::W`](W) writer structure"]
impl crate::Writable for SCL_FILTER_CFG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCL_FILTER_CFG to value 0x08"]
impl crate::Resettable for SCL_FILTER_CFG_SPEC {
    const RESET_VALUE: u32 = 0x08;
}
