#[doc = "Register `RSA_PD_CTRL` reader"]
pub type R = crate::R<RSA_PD_CTRL_SPEC>;
#[doc = "Register `RSA_PD_CTRL` writer"]
pub type W = crate::W<RSA_PD_CTRL_SPEC>;
#[doc = "Field `RSA_MEM_PD` reader - Set this bit to power down RSA memory. This bit has the lowest priority. When Digital Signature occupies the RSA, this bit is invalid."]
pub type RSA_MEM_PD_R = crate::BitReader;
#[doc = "Field `RSA_MEM_PD` writer - Set this bit to power down RSA memory. This bit has the lowest priority. When Digital Signature occupies the RSA, this bit is invalid."]
pub type RSA_MEM_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSA_MEM_FORCE_PU` reader - Set this bit to force power up RSA memory. This bit has the second highest priority."]
pub type RSA_MEM_FORCE_PU_R = crate::BitReader;
#[doc = "Field `RSA_MEM_FORCE_PU` writer - Set this bit to force power up RSA memory. This bit has the second highest priority."]
pub type RSA_MEM_FORCE_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSA_MEM_FORCE_PD` reader - Set this bit to force power down RSA memory. This bit has the highest priority."]
pub type RSA_MEM_FORCE_PD_R = crate::BitReader;
#[doc = "Field `RSA_MEM_FORCE_PD` writer - Set this bit to force power down RSA memory. This bit has the highest priority."]
pub type RSA_MEM_FORCE_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set this bit to power down RSA memory. This bit has the lowest priority. When Digital Signature occupies the RSA, this bit is invalid."]
    #[inline(always)]
    pub fn rsa_mem_pd(&self) -> RSA_MEM_PD_R {
        RSA_MEM_PD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit to force power up RSA memory. This bit has the second highest priority."]
    #[inline(always)]
    pub fn rsa_mem_force_pu(&self) -> RSA_MEM_FORCE_PU_R {
        RSA_MEM_FORCE_PU_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to force power down RSA memory. This bit has the highest priority."]
    #[inline(always)]
    pub fn rsa_mem_force_pd(&self) -> RSA_MEM_FORCE_PD_R {
        RSA_MEM_FORCE_PD_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RSA_PD_CTRL")
            .field("rsa_mem_pd", &self.rsa_mem_pd())
            .field("rsa_mem_force_pu", &self.rsa_mem_force_pu())
            .field("rsa_mem_force_pd", &self.rsa_mem_force_pd())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to power down RSA memory. This bit has the lowest priority. When Digital Signature occupies the RSA, this bit is invalid."]
    #[inline(always)]
    #[must_use]
    pub fn rsa_mem_pd(&mut self) -> RSA_MEM_PD_W<RSA_PD_CTRL_SPEC> {
        RSA_MEM_PD_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to force power up RSA memory. This bit has the second highest priority."]
    #[inline(always)]
    #[must_use]
    pub fn rsa_mem_force_pu(&mut self) -> RSA_MEM_FORCE_PU_W<RSA_PD_CTRL_SPEC> {
        RSA_MEM_FORCE_PU_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to force power down RSA memory. This bit has the highest priority."]
    #[inline(always)]
    #[must_use]
    pub fn rsa_mem_force_pd(&mut self) -> RSA_MEM_FORCE_PD_W<RSA_PD_CTRL_SPEC> {
        RSA_MEM_FORCE_PD_W::new(self, 2)
    }
}
#[doc = "RSA memory remapping register\n\nYou can [`read`](crate::Reg::read) this register and get [`rsa_pd_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rsa_pd_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RSA_PD_CTRL_SPEC;
impl crate::RegisterSpec for RSA_PD_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rsa_pd_ctrl::R`](R) reader structure"]
impl crate::Readable for RSA_PD_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rsa_pd_ctrl::W`](W) writer structure"]
impl crate::Writable for RSA_PD_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RSA_PD_CTRL to value 0x01"]
impl crate::Resettable for RSA_PD_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
