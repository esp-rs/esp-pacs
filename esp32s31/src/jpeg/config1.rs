#[doc = "Register `CONFIG1` reader"]
pub type R = crate::R<CONFIG1_SPEC>;
#[doc = "Register `CONFIG1` writer"]
pub type W = crate::W<CONFIG1_SPEC>;
#[doc = "Field `ENCODE_TIMEOUT_THRES` reader - Configure the timeout period for encode can not output bitstream. The timeout periods =2 power (reg_encode_timeout_thres) -1"]
pub type ENCODE_TIMEOUT_THRES_R = crate::FieldReader;
#[doc = "Field `ENCODE_TIMEOUT_THRES` writer - Configure the timeout period for encode can not output bitstream. The timeout periods =2 power (reg_encode_timeout_thres) -1"]
pub type ENCODE_TIMEOUT_THRES_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Configure the timeout period for encode can not output bitstream. The timeout periods =2 power (reg_encode_timeout_thres) -1"]
    #[inline(always)]
    pub fn encode_timeout_thres(&self) -> ENCODE_TIMEOUT_THRES_R {
        ENCODE_TIMEOUT_THRES_R::new((self.bits & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONFIG1")
            .field("encode_timeout_thres", &self.encode_timeout_thres())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5 - Configure the timeout period for encode can not output bitstream. The timeout periods =2 power (reg_encode_timeout_thres) -1"]
    #[inline(always)]
    pub fn encode_timeout_thres(&mut self) -> ENCODE_TIMEOUT_THRES_W<'_, CONFIG1_SPEC> {
        ENCODE_TIMEOUT_THRES_W::new(self, 0)
    }
}
#[doc = "Control and configuration registers\n\nYou can [`read`](crate::Reg::read) this register and get [`config1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONFIG1_SPEC;
impl crate::RegisterSpec for CONFIG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config1::R`](R) reader structure"]
impl crate::Readable for CONFIG1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`config1::W`](W) writer structure"]
impl crate::Writable for CONFIG1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONFIG1 to value 0x20"]
impl crate::Resettable for CONFIG1_SPEC {
    const RESET_VALUE: u32 = 0x20;
}
