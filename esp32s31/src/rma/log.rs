#[doc = "Register `LOG` reader"]
pub type R = crate::R<LOG_SPEC>;
#[doc = "Field `CERT_STATE` reader - Marks state of rma cert verification result. \\\\ 1: pass\\\\ 0: failed\\\\"]
pub type CERT_STATE_R = crate::BitReader;
#[doc = "Field `PUBLIC_KEY_STATE` reader - Marks state of rma public key verification result. \\\\ 1: pass\\\\ 0: failed\\\\"]
pub type PUBLIC_KEY_STATE_R = crate::BitReader;
#[doc = "Field `SIGN_STATE` reader - Marks state of rma sign verification result. \\\\ 1: pass\\\\ 0: failed\\\\"]
pub type SIGN_STATE_R = crate::BitReader;
#[doc = "Field `NONCE_STATE` reader - Marks error nonce config . \\\\ 0 : Use efuse_hash_sg in CERT_REQ or FAST_VEF but key invalid\\\\ 1 : Nonce config pass."]
pub type NONCE_STATE_R = crate::BitReader;
#[doc = "Field `KM_STATE` reader - Marks error KM config . \\\\ 0 : Use km but km key invalid\\\\ 1 : KM config pass\\\\"]
pub type KM_STATE_R = crate::BitReader;
#[doc = "Field `MODE_STATE` reader - Marks error mode config . \\\\ 0 : Invalid mode config\\\\ 1 : MODE config pass\\\\"]
pub type MODE_STATE_R = crate::BitReader;
#[doc = "Field `EFUSE_STATE` reader - Marks efuse state . \\\\ 0 : Efuse disable RMA module\\\\ 1 : Efuse pass\\\\"]
pub type EFUSE_STATE_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Marks state of rma cert verification result. \\\\ 1: pass\\\\ 0: failed\\\\"]
    #[inline(always)]
    pub fn cert_state(&self) -> CERT_STATE_R {
        CERT_STATE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Marks state of rma public key verification result. \\\\ 1: pass\\\\ 0: failed\\\\"]
    #[inline(always)]
    pub fn public_key_state(&self) -> PUBLIC_KEY_STATE_R {
        PUBLIC_KEY_STATE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Marks state of rma sign verification result. \\\\ 1: pass\\\\ 0: failed\\\\"]
    #[inline(always)]
    pub fn sign_state(&self) -> SIGN_STATE_R {
        SIGN_STATE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 28 - Marks error nonce config . \\\\ 0 : Use efuse_hash_sg in CERT_REQ or FAST_VEF but key invalid\\\\ 1 : Nonce config pass."]
    #[inline(always)]
    pub fn nonce_state(&self) -> NONCE_STATE_R {
        NONCE_STATE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Marks error KM config . \\\\ 0 : Use km but km key invalid\\\\ 1 : KM config pass\\\\"]
    #[inline(always)]
    pub fn km_state(&self) -> KM_STATE_R {
        KM_STATE_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Marks error mode config . \\\\ 0 : Invalid mode config\\\\ 1 : MODE config pass\\\\"]
    #[inline(always)]
    pub fn mode_state(&self) -> MODE_STATE_R {
        MODE_STATE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Marks efuse state . \\\\ 0 : Efuse disable RMA module\\\\ 1 : Efuse pass\\\\"]
    #[inline(always)]
    pub fn efuse_state(&self) -> EFUSE_STATE_R {
        EFUSE_STATE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LOG")
            .field("cert_state", &self.cert_state())
            .field("public_key_state", &self.public_key_state())
            .field("sign_state", &self.sign_state())
            .field("nonce_state", &self.nonce_state())
            .field("km_state", &self.km_state())
            .field("mode_state", &self.mode_state())
            .field("efuse_state", &self.efuse_state())
            .finish()
    }
}
#[doc = "Query result in rma\n\nYou can [`read`](crate::Reg::read) this register and get [`log::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LOG_SPEC;
impl crate::RegisterSpec for LOG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`log::R`](R) reader structure"]
impl crate::Readable for LOG_SPEC {}
#[doc = "`reset()` method sets LOG to value 0"]
impl crate::Resettable for LOG_SPEC {}
