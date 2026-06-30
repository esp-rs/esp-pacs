#[doc = "Register `ECDSA` reader"]
pub type R = crate::R<ECDSA_SPEC>;
#[doc = "Register `ECDSA` writer"]
pub type W = crate::W<ECDSA_SPEC>;
#[doc = "Field `CFG_ECDSA_P192_BLK` reader - Configures which block to use for ECDSA P192 key output."]
pub type CFG_ECDSA_P192_BLK_R = crate::FieldReader;
#[doc = "Field `CFG_ECDSA_P192_BLK` writer - Configures which block to use for ECDSA P192 key output."]
pub type CFG_ECDSA_P192_BLK_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CFG_ECDSA_P256_BLK` reader - Configures which block to use for ECDSA P256 key output."]
pub type CFG_ECDSA_P256_BLK_R = crate::FieldReader;
#[doc = "Field `CFG_ECDSA_P256_BLK` writer - Configures which block to use for ECDSA P256 key output."]
pub type CFG_ECDSA_P256_BLK_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CFG_ECDSA_P384_L_BLK` reader - Configures which block to use for ECDSA P384 key low part output."]
pub type CFG_ECDSA_P384_L_BLK_R = crate::FieldReader;
#[doc = "Field `CFG_ECDSA_P384_L_BLK` writer - Configures which block to use for ECDSA P384 key low part output."]
pub type CFG_ECDSA_P384_L_BLK_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CFG_ECDSA_P384_H_BLK` reader - Configures which block to use for ECDSA P256 key high part output."]
pub type CFG_ECDSA_P384_H_BLK_R = crate::FieldReader;
#[doc = "Field `CFG_ECDSA_P384_H_BLK` writer - Configures which block to use for ECDSA P256 key high part output."]
pub type CFG_ECDSA_P384_H_BLK_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CUR_ECDSA_P192_BLK` reader - Represents which block is used for ECDSA P192 key output."]
pub type CUR_ECDSA_P192_BLK_R = crate::FieldReader;
#[doc = "Field `CUR_ECDSA_P256_BLK` reader - Represents which block is used for ECDSA P256 key output."]
pub type CUR_ECDSA_P256_BLK_R = crate::FieldReader;
#[doc = "Field `CUR_ECDSA_P384_L_BLK` reader - Represents which block is used for ECDSA P384 key low part output."]
pub type CUR_ECDSA_P384_L_BLK_R = crate::FieldReader;
#[doc = "Field `CUR_ECDSA_P384_H_BLK` reader - Represents which block is used for ECDSA P384 key high part output."]
pub type CUR_ECDSA_P384_H_BLK_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Configures which block to use for ECDSA P192 key output."]
    #[inline(always)]
    pub fn cfg_ecdsa_p192_blk(&self) -> CFG_ECDSA_P192_BLK_R {
        CFG_ECDSA_P192_BLK_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Configures which block to use for ECDSA P256 key output."]
    #[inline(always)]
    pub fn cfg_ecdsa_p256_blk(&self) -> CFG_ECDSA_P256_BLK_R {
        CFG_ECDSA_P256_BLK_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Configures which block to use for ECDSA P384 key low part output."]
    #[inline(always)]
    pub fn cfg_ecdsa_p384_l_blk(&self) -> CFG_ECDSA_P384_L_BLK_R {
        CFG_ECDSA_P384_L_BLK_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Configures which block to use for ECDSA P256 key high part output."]
    #[inline(always)]
    pub fn cfg_ecdsa_p384_h_blk(&self) -> CFG_ECDSA_P384_H_BLK_R {
        CFG_ECDSA_P384_H_BLK_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Represents which block is used for ECDSA P192 key output."]
    #[inline(always)]
    pub fn cur_ecdsa_p192_blk(&self) -> CUR_ECDSA_P192_BLK_R {
        CUR_ECDSA_P192_BLK_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Represents which block is used for ECDSA P256 key output."]
    #[inline(always)]
    pub fn cur_ecdsa_p256_blk(&self) -> CUR_ECDSA_P256_BLK_R {
        CUR_ECDSA_P256_BLK_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Represents which block is used for ECDSA P384 key low part output."]
    #[inline(always)]
    pub fn cur_ecdsa_p384_l_blk(&self) -> CUR_ECDSA_P384_L_BLK_R {
        CUR_ECDSA_P384_L_BLK_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Represents which block is used for ECDSA P384 key high part output."]
    #[inline(always)]
    pub fn cur_ecdsa_p384_h_blk(&self) -> CUR_ECDSA_P384_H_BLK_R {
        CUR_ECDSA_P384_H_BLK_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ECDSA")
            .field("cfg_ecdsa_p192_blk", &self.cfg_ecdsa_p192_blk())
            .field("cfg_ecdsa_p256_blk", &self.cfg_ecdsa_p256_blk())
            .field("cfg_ecdsa_p384_l_blk", &self.cfg_ecdsa_p384_l_blk())
            .field("cfg_ecdsa_p384_h_blk", &self.cfg_ecdsa_p384_h_blk())
            .field("cur_ecdsa_p192_blk", &self.cur_ecdsa_p192_blk())
            .field("cur_ecdsa_p256_blk", &self.cur_ecdsa_p256_blk())
            .field("cur_ecdsa_p384_l_blk", &self.cur_ecdsa_p384_l_blk())
            .field("cur_ecdsa_p384_h_blk", &self.cur_ecdsa_p384_h_blk())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Configures which block to use for ECDSA P192 key output."]
    #[inline(always)]
    pub fn cfg_ecdsa_p192_blk(&mut self) -> CFG_ECDSA_P192_BLK_W<'_, ECDSA_SPEC> {
        CFG_ECDSA_P192_BLK_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Configures which block to use for ECDSA P256 key output."]
    #[inline(always)]
    pub fn cfg_ecdsa_p256_blk(&mut self) -> CFG_ECDSA_P256_BLK_W<'_, ECDSA_SPEC> {
        CFG_ECDSA_P256_BLK_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Configures which block to use for ECDSA P384 key low part output."]
    #[inline(always)]
    pub fn cfg_ecdsa_p384_l_blk(&mut self) -> CFG_ECDSA_P384_L_BLK_W<'_, ECDSA_SPEC> {
        CFG_ECDSA_P384_L_BLK_W::new(self, 8)
    }
    #[doc = "Bits 12:15 - Configures which block to use for ECDSA P256 key high part output."]
    #[inline(always)]
    pub fn cfg_ecdsa_p384_h_blk(&mut self) -> CFG_ECDSA_P384_H_BLK_W<'_, ECDSA_SPEC> {
        CFG_ECDSA_P384_H_BLK_W::new(self, 12)
    }
}
#[doc = "eFuse status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`ecdsa::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecdsa::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ECDSA_SPEC;
impl crate::RegisterSpec for ECDSA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ecdsa::R`](R) reader structure"]
impl crate::Readable for ECDSA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ecdsa::W`](W) writer structure"]
impl crate::Writable for ECDSA_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ECDSA to value 0"]
impl crate::Resettable for ECDSA_SPEC {}
