#[doc = "Register `CVSD_CONF2` reader"]
pub type R = crate::R<CVSD_CONF2_SPEC>;
#[doc = "Register `CVSD_CONF2` writer"]
pub type W = crate::W<CVSD_CONF2_SPEC>;
#[doc = "Field `CVSD_K` reader - "]
pub type CVSD_K_R = crate::FieldReader;
#[doc = "Field `CVSD_K` writer - "]
pub type CVSD_K_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CVSD_J` reader - "]
pub type CVSD_J_R = crate::FieldReader;
#[doc = "Field `CVSD_J` writer - "]
pub type CVSD_J_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CVSD_BETA` reader - "]
pub type CVSD_BETA_R = crate::FieldReader<u16>;
#[doc = "Field `CVSD_BETA` writer - "]
pub type CVSD_BETA_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `CVSD_H` reader - "]
pub type CVSD_H_R = crate::FieldReader;
#[doc = "Field `CVSD_H` writer - "]
pub type CVSD_H_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn cvsd_k(&self) -> CVSD_K_R {
        CVSD_K_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5"]
    #[inline(always)]
    pub fn cvsd_j(&self) -> CVSD_J_R {
        CVSD_J_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:15"]
    #[inline(always)]
    pub fn cvsd_beta(&self) -> CVSD_BETA_R {
        CVSD_BETA_R::new(((self.bits >> 6) & 0x03ff) as u16)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn cvsd_h(&self) -> CVSD_H_R {
        CVSD_H_R::new(((self.bits >> 16) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CVSD_CONF2")
            .field("cvsd_k", &self.cvsd_k())
            .field("cvsd_j", &self.cvsd_j())
            .field("cvsd_beta", &self.cvsd_beta())
            .field("cvsd_h", &self.cvsd_h())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2"]
    #[inline(always)]
    #[must_use]
    pub fn cvsd_k(&mut self) -> CVSD_K_W<CVSD_CONF2_SPEC> {
        CVSD_K_W::new(self, 0)
    }
    #[doc = "Bits 3:5"]
    #[inline(always)]
    #[must_use]
    pub fn cvsd_j(&mut self) -> CVSD_J_W<CVSD_CONF2_SPEC> {
        CVSD_J_W::new(self, 3)
    }
    #[doc = "Bits 6:15"]
    #[inline(always)]
    #[must_use]
    pub fn cvsd_beta(&mut self) -> CVSD_BETA_W<CVSD_CONF2_SPEC> {
        CVSD_BETA_W::new(self, 6)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    #[must_use]
    pub fn cvsd_h(&mut self) -> CVSD_H_W<CVSD_CONF2_SPEC> {
        CVSD_H_W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cvsd_conf2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cvsd_conf2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CVSD_CONF2_SPEC;
impl crate::RegisterSpec for CVSD_CONF2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cvsd_conf2::R`](R) reader structure"]
impl crate::Readable for CVSD_CONF2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cvsd_conf2::W`](W) writer structure"]
impl crate::Writable for CVSD_CONF2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CVSD_CONF2 to value 0x0005_02a4"]
impl crate::Resettable for CVSD_CONF2_SPEC {
    const RESET_VALUE: u32 = 0x0005_02a4;
}
